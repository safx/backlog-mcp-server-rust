use crate::models::SharedFile;
use backlog_api_core::IntoRequest;
use backlog_core::ProjectIdOrKey;
use derive_builder::Builder;
use percent_encoding::{AsciiSet, CONTROLS, utf8_percent_encode};
use serde::Serialize;

/// Characters that must be escaped inside a single URL path segment (WHATWG path-segment set).
const PATH_SEGMENT: &AsciiSet = &CONTROLS
    .add(b' ')
    .add(b'"')
    .add(b'#')
    .add(b'%')
    .add(b'<')
    .add(b'>')
    .add(b'?')
    .add(b'`')
    .add(b'{')
    .add(b'}');

/// Response type for getting shared files list
pub type GetSharedFilesListResponse = Vec<SharedFile>;

/// Parameters for getting shared files list
///
/// Corresponds to `GET /api/v2/projects/:projectIdOrKey/files/metadata/:path`.
#[derive(Debug, Clone, PartialEq, Builder, Serialize)]
#[serde(rename_all = "camelCase")]
#[builder(setter(strip_option))]
pub struct GetSharedFilesListParams {
    /// Project ID or key
    #[builder(setter(into))]
    #[serde(skip)]
    pub project_id_or_key: ProjectIdOrKey,

    /// Path to the directory
    #[builder(setter(into))]
    #[serde(skip)]
    pub path: String,

    /// Sort order for the files ("asc" or "desc")
    #[builder(default, setter(into))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,

    /// Offset for pagination
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,

    /// Number of files to retrieve (1-100, default: 20)
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u32>,
}

impl IntoRequest for GetSharedFilesListParams {
    fn path(&self) -> String {
        let encoded_path = self
            .path
            .split('/')
            .map(|seg| utf8_percent_encode(seg, PATH_SEGMENT).to_string())
            .collect::<Vec<_>>()
            .join("/");
        format!(
            "/api/v2/projects/{}/files/metadata/{}",
            self.project_id_or_key, encoded_path
        )
    }

    fn to_query(&self) -> impl serde::Serialize {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use backlog_core::identifier::ProjectId;

    #[test]
    fn test_path_percent_encodes_special_characters() {
        let params = GetSharedFilesListParamsBuilder::default()
            .project_id_or_key(ProjectIdOrKey::Id(ProjectId::new(1)))
            .path("docs/a?b#c 100%".to_string())
            .build()
            .unwrap();

        assert_eq!(
            params.path(),
            "/api/v2/projects/1/files/metadata/docs/a%3Fb%23c%20100%25"
        );
    }

    #[test]
    fn test_path_keeps_already_safe_path_unchanged() {
        let params = GetSharedFilesListParamsBuilder::default()
            .project_id_or_key(ProjectIdOrKey::Id(ProjectId::new(1)))
            .path("docs/2024".to_string())
            .build()
            .unwrap();

        assert_eq!(params.path(), "/api/v2/projects/1/files/metadata/docs/2024");
    }
}
