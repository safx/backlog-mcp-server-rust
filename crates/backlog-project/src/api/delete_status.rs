#[cfg(feature = "writable")]
use backlog_api_core::{HttpMethod, IntoRequest};
#[cfg(feature = "writable")]
use backlog_core::ProjectIdOrKey;
#[cfg(feature = "writable")]
use serde::Serialize;

#[cfg(feature = "writable")]
pub type DeleteStatusResponse = backlog_domain_models::Status;

#[cfg(feature = "writable")]
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteStatusParams {
    #[serde(skip)]
    pub project_id_or_key: ProjectIdOrKey,
    #[serde(skip)]
    pub status_id: backlog_core::identifier::StatusId,
    pub substitute_status_id: backlog_core::identifier::StatusId,
}

#[cfg(feature = "writable")]
impl DeleteStatusParams {
    pub fn new(
        project_id_or_key: impl Into<ProjectIdOrKey>,
        status_id: impl Into<backlog_core::identifier::StatusId>,
        substitute_status_id: impl Into<backlog_core::identifier::StatusId>,
    ) -> Self {
        Self {
            project_id_or_key: project_id_or_key.into(),
            status_id: status_id.into(),
            substitute_status_id: substitute_status_id.into(),
        }
    }
}

#[cfg(feature = "writable")]
impl IntoRequest for DeleteStatusParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Delete
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
