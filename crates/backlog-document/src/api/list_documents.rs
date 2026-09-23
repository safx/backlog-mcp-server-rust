use crate::models::Document;
use backlog_api_core::IntoRequest;
use backlog_api_macros::ToFormParams;
use backlog_core::identifier::ProjectId;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Response type for listing documents
pub type ListDocumentsResponse = Vec<Document>;

/// Parameters for listing documents
///
/// Corresponds to `GET /api/v2/documents`.
#[derive(Debug, Builder, Clone, PartialEq, ToFormParams)]
#[builder(setter(strip_option))]
pub struct ListDocumentsParams {
    /// Numeric project IDs. Omit to search all participating projects.
    #[builder(default, setter(into))]
    #[form(array, name = "projectId")]
    pub project_ids: Option<Vec<ProjectId>>,
    #[builder(default, setter(into))]
    pub keyword: Option<String>,
    #[builder(default, setter(into))]
    pub sort: Option<DocumentSortKey>,
    #[builder(default, setter(into))]
    pub order: Option<DocumentOrder>, // Sort order
    #[builder(default = "Some(0)")]
    pub offset: Option<u32>,
    #[builder(default)]
    pub count: Option<u32>,
}

impl ListDocumentsParams {
    pub fn validate(&self) -> backlog_core::Result<()> {
        if self.project_ids.as_ref().is_some_and(Vec::is_empty) {
            return Err(backlog_core::Error::InvalidParameter(
                "project_ids must not be empty; omit it to search all participating projects"
                    .into(),
            ));
        }
        if self.count.is_some_and(|count| !(1..=100).contains(&count)) {
            return Err(backlog_core::Error::InvalidParameter(
                "count must be between 1 and 100".into(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub enum DocumentSortKey {
    #[serde(rename = "created")]
    Created,
    #[serde(rename = "updated")]
    Updated,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub enum DocumentOrder {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}

impl fmt::Display for DocumentSortKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DocumentSortKey::Created => write!(f, "created"),
            DocumentSortKey::Updated => write!(f, "updated"),
        }
    }
}

impl fmt::Display for DocumentOrder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DocumentOrder::Asc => write!(f, "asc"),
            DocumentOrder::Desc => write!(f, "desc"),
        }
    }
}

impl From<ListDocumentsParams> for Vec<(String, String)> {
    fn from(params: ListDocumentsParams) -> Self {
        (&params).into()
    }
}

impl IntoRequest for ListDocumentsParams {
    fn path(&self) -> String {
        "/api/v2/documents".to_string()
    }

    fn to_query(&self) -> impl Serialize {
        <Vec<(String, String)>>::from(self)
    }
}
