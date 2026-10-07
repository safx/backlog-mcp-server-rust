#[cfg(feature = "writable")]
use backlog_api_core::{HttpMethod, IntoRequest};
#[cfg(feature = "writable")]
use serde::Serialize;

#[cfg(feature = "writable")]
pub type AddProjectResponse = backlog_domain_models::Project;

#[cfg(feature = "writable")]
#[derive(Debug, Clone)]
pub struct AddProjectParams {
    pub name: String,
    pub key: String,
    pub chart_enabled: Option<bool>,
    pub use_resolved_for_chart: Option<bool>,
    pub subtasking_enabled: Option<bool>,
    pub project_leader_can_edit_project_leader: Option<bool>,
    pub use_wiki: Option<bool>,
    pub use_file_sharing: Option<bool>,
    pub use_wiki_tree_view: Option<bool>,
    pub use_subversion: Option<bool>,
    pub use_git: Option<bool>,
    pub use_original_image_size_at_wiki: Option<bool>,
    pub text_formatting_rule: Option<String>,
    pub use_dev_attributes: Option<bool>,
}

#[cfg(feature = "writable")]
impl AddProjectParams {
    pub fn new(name: impl Into<String>, key: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            key: key.into(),
            chart_enabled: None,
            use_resolved_for_chart: None,
            subtasking_enabled: None,
            project_leader_can_edit_project_leader: None,
            use_wiki: None,
            use_file_sharing: None,
            use_wiki_tree_view: None,
            use_subversion: None,
            use_git: None,
            use_original_image_size_at_wiki: None,
            text_formatting_rule: None,
            use_dev_attributes: None,
        }
    }

    pub fn chart_enabled(mut self, enabled: bool) -> Self {
        self.chart_enabled = Some(enabled);
        self
    }

    pub fn use_resolved_for_chart(mut self, enabled: bool) -> Self {
        self.use_resolved_for_chart = Some(enabled);
        self
    }

    pub fn subtasking_enabled(mut self, enabled: bool) -> Self {
        self.subtasking_enabled = Some(enabled);
        self
    }

    pub fn project_leader_can_edit_project_leader(mut self, enabled: bool) -> Self {
        self.project_leader_can_edit_project_leader = Some(enabled);
        self
    }

    pub fn use_wiki(mut self, enabled: bool) -> Self {
        self.use_wiki = Some(enabled);
        self
    }

    pub fn use_file_sharing(mut self, enabled: bool) -> Self {
        self.use_file_sharing = Some(enabled);
        self
    }

    pub fn use_wiki_tree_view(mut self, enabled: bool) -> Self {
        self.use_wiki_tree_view = Some(enabled);
        self
    }

    pub fn use_subversion(mut self, enabled: bool) -> Self {
        self.use_subversion = Some(enabled);
        self
    }

    pub fn use_git(mut self, enabled: bool) -> Self {
        self.use_git = Some(enabled);
        self
    }

    pub fn use_original_image_size_at_wiki(mut self, enabled: bool) -> Self {
        self.use_original_image_size_at_wiki = Some(enabled);
        self
    }

    pub fn text_formatting_rule(mut self, rule: impl Into<String>) -> Self {
        self.text_formatting_rule = Some(rule.into());
        self
    }

    pub fn use_dev_attributes(mut self, enabled: bool) -> Self {
        self.use_dev_attributes = Some(enabled);
        self
    }
}

#[cfg(feature = "writable")]
impl From<&AddProjectParams> for Vec<(String, String)> {
    fn from(params: &AddProjectParams) -> Self {
        let mut seq = Vec::new();
        seq.push(("name".to_string(), params.name.clone()));
        seq.push(("key".to_string(), params.key.clone()));

        if let Some(enabled) = params.chart_enabled {
            seq.push(("chartEnabled".to_string(), enabled.to_string()));
        }
        if let Some(enabled) = params.use_resolved_for_chart {
            seq.push(("useResolvedForChart".to_string(), enabled.to_string()));
        }
        if let Some(enabled) = params.subtasking_enabled {
            seq.push(("subtaskingEnabled".to_string(), enabled.to_string()));
        }
        if let Some(enabled) = params.project_leader_can_edit_project_leader {
            seq.push((
                "projectLeaderCanEditProjectLeader".to_string(),
                enabled.to_string(),
            ));
        }
        if let Some(enabled) = params.use_wiki {
            seq.push(("useWiki".to_string(), enabled.to_string()));
        }
        if let Some(enabled) = params.use_file_sharing {
            seq.push(("useFileSharing".to_string(), enabled.to_string()));
        }
        if let Some(enabled) = params.use_wiki_tree_view {
            seq.push(("useWikiTreeView".to_string(), enabled.to_string()));
        }
        if let Some(enabled) = params.use_subversion {
            seq.push(("useSubversion".to_string(), enabled.to_string()));
        }
        if let Some(enabled) = params.use_git {
            seq.push(("useGit".to_string(), enabled.to_string()));
        }
        if let Some(enabled) = params.use_original_image_size_at_wiki {
            seq.push((
                "useOriginalImageSizeAtWiki".to_string(),
                enabled.to_string(),
            ));
        }
        if let Some(rule) = &params.text_formatting_rule {
            seq.push(("textFormattingRule".to_string(), rule.clone()));
        }
        if let Some(enabled) = params.use_dev_attributes {
            seq.push(("useDevAttributes".to_string(), enabled.to_string()));
        }
        seq
    }
}

#[cfg(feature = "writable")]
impl IntoRequest for AddProjectParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Post
    }

    fn path(&self) -> String {
        "/api/v2/projects".to_string()
    }

    fn to_form(&self) -> impl Serialize {
        let params: Vec<(String, String)> = self.into();
        params
    }
}
