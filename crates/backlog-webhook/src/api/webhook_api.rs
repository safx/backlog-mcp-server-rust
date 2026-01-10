#[cfg(feature = "writable")]
use crate::api::add_webhook::{AddWebhookParams, AddWebhookParamsBuilder, AddWebhookResponse};
#[cfg(feature = "writable")]
use crate::api::delete_webhook::{DeleteWebhookParams, DeleteWebhookResponse};
use crate::api::get_webhook::{GetWebhookParams, GetWebhookResponse};
use crate::api::get_webhook_list::{GetWebhookListParams, GetWebhookListResponse};
#[cfg(feature = "writable")]
use crate::api::update_webhook::{
    UpdateWebhookParams, UpdateWebhookParamsBuilder, UpdateWebhookResponse,
};
use backlog_api_core::Result;
use backlog_core::{ProjectIdOrKey, id::WebhookId};
use client::Client;

#[derive(Debug, Clone)]
pub struct WebhookApi(Client);

impl WebhookApi {
    pub fn new(client: Client) -> Self {
        Self(client)
    }

    /// Get list of webhooks in a project.
    /// Corresponds to `GET /api/v2/projects/:projectIdOrKey/webhooks`.
    pub async fn get_webhook_list(
        &self,
        project_id_or_key: impl Into<ProjectIdOrKey>,
    ) -> Result<GetWebhookListResponse> {
        let params = GetWebhookListParams {
            project_id_or_key: project_id_or_key.into(),
        };
        self.0.execute(params).await
    }

    /// Get webhook information.
    /// Corresponds to `GET /api/v2/projects/:projectIdOrKey/webhooks/:webhookId`.
    pub async fn get_webhook(
        &self,
        project_id_or_key: impl Into<ProjectIdOrKey>,
        webhook_id: impl Into<WebhookId>,
    ) -> Result<GetWebhookResponse> {
        let params = GetWebhookParams {
            project_id_or_key: project_id_or_key.into(),
            webhook_id: webhook_id.into(),
        };
        self.0.execute(params).await
    }

    /// Update webhook information.
    /// Corresponds to `PATCH /api/v2/projects/:projectIdOrKey/webhooks/:webhookId`.
    #[cfg(feature = "writable")]
    pub fn update_webhook(
        &self,
        project_id_or_key: impl Into<ProjectIdOrKey>,
        webhook_id: impl Into<WebhookId>,
    ) -> UpdateWebhookParamsBuilder {
        let mut builder = UpdateWebhookParamsBuilder::default();
        builder.project_id_or_key(project_id_or_key.into());
        builder.webhook_id(webhook_id.into());
        builder
    }

    /// Execute update webhook request with params.
    #[cfg(feature = "writable")]
    pub async fn execute_update_webhook(
        &self,
        params: UpdateWebhookParams,
    ) -> Result<UpdateWebhookResponse> {
        self.0.execute(params).await
    }

    /// Add a new webhook to a project.
    /// Corresponds to `POST /api/v2/projects/:projectIdOrKey/webhooks`.
    #[cfg(feature = "writable")]
    pub fn add_webhook(
        &self,
        project_id_or_key: impl Into<ProjectIdOrKey>,
    ) -> AddWebhookParamsBuilder {
        let mut builder = AddWebhookParamsBuilder::default();
        builder.project_id_or_key(project_id_or_key.into());
        builder
    }

    /// Execute add webhook request with params.
    #[cfg(feature = "writable")]
    pub async fn execute_add_webhook(
        &self,
        params: AddWebhookParams,
    ) -> Result<AddWebhookResponse> {
        self.0.execute(params).await
    }

    /// Delete a webhook from a project.
    /// Corresponds to `DELETE /api/v2/projects/:projectIdOrKey/webhooks/:webhookId`.
    #[cfg(feature = "writable")]
    pub async fn delete_webhook(
        &self,
        project_id_or_key: impl Into<ProjectIdOrKey>,
        webhook_id: impl Into<WebhookId>,
    ) -> Result<DeleteWebhookResponse> {
        let params = DeleteWebhookParams::new(project_id_or_key, webhook_id);
        self.0.execute(params).await
    }
}
