#[cfg(feature = "writable")]
use backlog_api_core::{HttpMethod, IntoRequest};
#[cfg(feature = "writable")]
use backlog_core::ProjectIdOrKey;
#[cfg(feature = "writable")]
use serde::Serialize;

#[cfg(feature = "writable")]
pub type AddStatusResponse = backlog_domain_models::Status;

#[cfg(feature = "writable")]
#[derive(Debug, Clone, Serialize)]
pub struct AddStatusParams {
    #[serde(skip)]
    pub project_id_or_key: ProjectIdOrKey,
    pub name: String,
    pub color: backlog_domain_models::StatusColor,
}

#[cfg(feature = "writable")]
impl AddStatusParams {
    pub fn new(
        project_id_or_key: impl Into<ProjectIdOrKey>,
        name: impl Into<String>,
        color: backlog_domain_models::StatusColor,
    ) -> Self {
        Self {
            project_id_or_key: project_id_or_key.into(),
            name: name.into(),
            color,
        }
    }
}

#[cfg(feature = "writable")]
impl IntoRequest for AddStatusParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Post
    }

    fn path(&self) -> String {
        format!("/api/v2/projects/{}/statuses", self.project_id_or_key)
    }

    fn to_form(&self) -> impl Serialize {
        self
    }
}
