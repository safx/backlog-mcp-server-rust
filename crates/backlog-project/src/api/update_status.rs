#[cfg(feature = "writable")]
use backlog_api_core::{HttpMethod, IntoRequest};
#[cfg(feature = "writable")]
use backlog_core::ProjectIdOrKey;
#[cfg(feature = "writable")]
use serde::Serialize;

#[cfg(feature = "writable")]
pub type UpdateStatusResponse = backlog_domain_models::Status;

#[cfg(feature = "writable")]
#[derive(Debug, Clone, Serialize)]
pub struct UpdateStatusParams {
    #[serde(skip)]
    pub project_id_or_key: ProjectIdOrKey,
    #[serde(skip)]
    pub status_id: backlog_core::identifier::StatusId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<backlog_domain_models::StatusColor>,
}

#[cfg(feature = "writable")]
impl UpdateStatusParams {
    pub fn new(
        project_id_or_key: impl Into<ProjectIdOrKey>,
        status_id: impl Into<backlog_core::identifier::StatusId>,
    ) -> Self {
        Self {
            project_id_or_key: project_id_or_key.into(),
            status_id: status_id.into(),
            name: None,
            color: None,
        }
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn color(mut self, color: backlog_domain_models::StatusColor) -> Self {
        self.color = Some(color);
        self
    }
}

#[cfg(feature = "writable")]
impl IntoRequest for UpdateStatusParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Patch
    }

    fn path(&self) -> String {
        format!(
            "/api/v2/projects/{}/statuses/{}",
            self.project_id_or_key, self.status_id
        )
    }

    fn to_form(&self) -> impl Serialize {
        self
    }
}
