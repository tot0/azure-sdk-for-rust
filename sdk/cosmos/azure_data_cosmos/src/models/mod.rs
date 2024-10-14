// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Model types sent to and received from the Cosmos DB API.

use azure_core::{date::OffsetDateTime, Model};
use serde::{de::DeserializeOwned, Deserialize, Deserializer};

#[cfg(doc)]
use crate::{
    clients::{ContainerClient, ContainerClientMethods, DatabaseClientMethods},
    CosmosClientMethods,
};

mod container_properties;
mod item;

pub use container_properties::*;
pub use item::*;

fn deserialize_cosmos_timestamp<'de, D>(deserializer: D) -> Result<Option<OffsetDateTime>, D::Error>
where
    D: Deserializer<'de>,
{
    let seconds_since_epoch = Option::<i64>::deserialize(deserializer)?;
    match seconds_since_epoch {
        None => Ok(None),
        Some(seconds) => Ok(Some(OffsetDateTime::from_unix_timestamp(seconds).map_err(
            |_| {
                use serde::de::Error;
                D::Error::invalid_value(
                    serde::de::Unexpected::Signed(seconds),
                    &"a valid timestamp",
                )
            },
        )?)),
    }
}

/// A page of query results, where each item is of type `T`.
#[non_exhaustive]
#[derive(Clone, Default, Debug, Deserialize)]
pub struct QueryResults<T> {
    #[serde(alias = "Documents")]
    pub items: Vec<T>,
}

impl<T: DeserializeOwned> azure_core::Model for QueryResults<T> {
    async fn from_response_body(
        body: azure_core::ResponseBody,
    ) -> typespec_client_core::Result<Self> {
        body.json().await
    }
}

/// A page of results from [`CosmosClient::query_databases`](crate::clients::CosmosClient::query_databases())
#[non_exhaustive]
#[derive(Clone, Default, Debug, Deserialize, Model)]
pub struct DatabaseQueryResults {
    #[serde(alias = "Databases")]
    pub databases: Vec<DatabaseProperties>,
}

/// A page of results from [`DatabaseClient::query_containers`](crate::clients::DatabaseClient::query_containers())
#[non_exhaustive]
#[derive(Clone, Default, Debug, Deserialize, Model)]
pub struct ContainerQueryResults {
    #[serde(alias = "DocumentCollections")]
    pub containers: Vec<ContainerProperties>,
}

/// Common system properties returned for most Cosmos DB resources.
#[non_exhaustive]
#[derive(Clone, Default, Debug, Deserialize, PartialEq, Eq)]
pub struct SystemProperties {
    /// The entity tag associated with the resource.
    #[serde(rename = "_etag")]
    pub etag: Option<azure_core::Etag>,

    /// The self-link associated with the resource.
    #[serde(rename = "_self")]
    pub self_link: Option<String>,

    /// The system-generated unique identifier associated with the resource.
    #[serde(rename = "_rid")]
    pub resource_id: Option<String>,

    /// A [`OffsetDateTime`] representing the last modified time of the resource.
    #[serde(rename = "_ts")]
    #[serde(deserialize_with = "deserialize_cosmos_timestamp")]
    pub last_modified: Option<OffsetDateTime>,
}

/// Properties of a Cosmos DB database.
///
/// Returned by [`DatabaseClient::read()`](crate::clients::DatabaseClient::read()).
#[non_exhaustive]
#[derive(Model, Clone, Default, Debug, Deserialize, PartialEq, Eq)]
pub struct DatabaseProperties {
    /// The ID of the database.
    pub id: String,

    /// A [`SystemProperties`] object containing common system properties for the database.
    #[serde(flatten)]
    pub system_properties: SystemProperties,
}
