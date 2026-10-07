#[cfg(feature = "writable")]
use backlog_api_core::{HttpMethod, IntoRequest};
#[cfg(feature = "writable")]
use backlog_core::ProjectIdOrKey;
#[cfg(feature = "writable")]
use serde::Serialize;

#[cfg(feature = "writable")]
use backlog_api_macros::ToFormParams;

#[cfg(feature = "writable")]
pub type UpdateStatusOrderResponse = Vec<backlog_domain_models::Status>;

#[cfg(feature = "writable")]
#[derive(Debug, Clone, ToFormParams)]
pub struct UpdateStatusOrderParams {
    #[form(skip)]
    pub project_id_or_key: ProjectIdOrKey,
    #[form(array, name = "statusId")]
    pub status_ids: Vec<backlog_core::identifier::StatusId>,
}

#[cfg(feature = "writable")]
impl UpdateStatusOrderParams {
    pub fn new(
        project_id_or_key: impl Into<ProjectIdOrKey>,
        status_ids: Vec<backlog_core::identifier::StatusId>,
    ) -> Self {
        Self {
            project_id_or_key: project_id_or_key.into(),
            status_ids,
        }
    }
}

#[cfg(feature = "writable")]
impl IntoRequest for UpdateStatusOrderParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Patch
    }

    fn path(&self) -> String {
        format!(
            "/api/v2/projects/{}/statuses/updateDisplayOrder",
            self.project_id_or_key
        )
    }

    fn to_form(&self) -> impl Serialize {
        Vec::<(String, String)>::from(self)
    }
}
