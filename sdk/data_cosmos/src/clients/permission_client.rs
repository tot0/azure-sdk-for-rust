use super::*;
use crate::prelude::*;
use crate::resources::permission::{PermissionMode, PermissionResponse};
use crate::resources::ResourceType;
use crate::ReadonlyString;
use azure_core::{Context, Pipeline, Request};

/// A client for Cosmos permission resources.
#[derive(Debug, Clone)]
pub struct PermissionClient {
    user_client: UserClient,
    permission_name: ReadonlyString,
}

impl PermissionClient {
    pub(crate) fn new<S: Into<ReadonlyString>>(
        user_client: UserClient,
        permission_name: S,
    ) -> Self {
        Self {
            user_client,
            permission_name: permission_name.into(),
        }
    }

    /// Get a [`CosmosClient`
    pub fn cosmos_client(&self) -> &CosmosClient {
        self.user_client.cosmos_client()
    }

    /// Get a [`DatabaseClient`]
    pub fn database_client(&self) -> &DatabaseClient {
        self.user_client.database_client()
    }

    /// Get the [`UserClient`]
    pub fn user_client(&self) -> &UserClient {
        &self.user_client
    }

    /// Get the permission's name
    pub fn permission_name(&self) -> &str {
        &self.permission_name
    }

    /// Create the permission
    pub fn create_permission(&self, permission_mode: PermissionMode) -> CreatePermissionBuilder {
        CreatePermissionBuilder::new(self.clone(), permission_mode)
    }

    /// Replace the permission
    pub async fn replace_permission(
        &self,
        ctx: Context,
        options: ReplacePermissionOptions,
        permission_mode: &PermissionMode,
    ) -> crate::Result<PermissionResponse> {
        let mut request = self.prepare_request_with_permission_name(http::Method::PUT);

        options.decorate_request(&mut request, self.permission_name(), permission_mode)?;

        let response = self
            .pipeline()
            .send(ctx.clone().insert(ResourceType::Permissions), &mut request)
            .await?;

        Ok(PermissionResponse::try_from(response).await?)
    }

    /// Get the permission
    pub fn get_permission(&self) -> GetPermissionBuilder {
        GetPermissionBuilder::new(self.clone())
    }

    /// Delete the permission
    pub fn delete_permission(&self) -> DeletePermissionBuilder {
        DeletePermissionBuilder::new(self.clone())
    }

    pub(crate) fn prepare_request_with_permission_name(&self, method: http::Method) -> Request {
        self.cosmos_client().prepare_request_pipeline(
            &format!(
                "dbs/{}/users/{}/permissions/{}",
                self.database_client().database_name(),
                self.user_client().user_name(),
                self.permission_name()
            ),
            method,
        )
    }

    pub(crate) fn pipeline(&self) -> &Pipeline {
        self.cosmos_client().pipeline()
    }
}
