#[cfg(feature = "writable")]
use backlog_api_core::{HttpMethod, IntoRequest};
#[cfg(feature = "writable")]
use backlog_core::{ProjectIdOrKey, TextFormattingRule};
#[cfg(feature = "writable")]
use backlog_domain_models::Project;
#[cfg(feature = "writable")]
use serde::Serialize;

#[cfg(feature = "writable")]
pub type UpdateProjectResponse = Project;

#[cfg(feature = "writable")]
#[derive(Debug, Clone)]
pub struct UpdateProjectParams {
    project_id_or_key: ProjectIdOrKey,
    pub name: Option<String>,
    pub key: Option<String>,
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
    pub text_formatting_rule: Option<TextFormattingRule>,
    pub archived: Option<bool>,
    pub use_dev_attributes: Option<bool>,
}

#[cfg(feature = "writable")]
impl UpdateProjectParams {
    pub fn new(project_id_or_key: impl Into<ProjectIdOrKey>) -> Self {
        Self {
            project_id_or_key: project_id_or_key.into(),
            name: None,
            key: None,
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
            archived: None,
            use_dev_attributes: None,
        }
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn key(mut self, key: impl Into<String>) -> Self {
        self.key = Some(key.into());
        self
    }

    pub fn chart_enabled(mut self, enabled: bool) -> Self {
        self.chart_enabled = Some(enabled);
        self
    }

    pub fn use_resolved_for_chart(mut self, use_resolved: bool) -> Self {
        self.use_resolved_for_chart = Some(use_resolved);
        self
    }

    pub fn subtasking_enabled(mut self, enabled: bool) -> Self {
        self.subtasking_enabled = Some(enabled);
        self
    }

    pub fn project_leader_can_edit_project_leader(mut self, can_edit: bool) -> Self {
        self.project_leader_can_edit_project_leader = Some(can_edit);
        self
    }

    pub fn use_wiki(mut self, use_wiki: bool) -> Self {
        self.use_wiki = Some(use_wiki);
        self
    }

    pub fn use_file_sharing(mut self, use_file_sharing: bool) -> Self {
        self.use_file_sharing = Some(use_file_sharing);
        self
    }

    pub fn use_wiki_tree_view(mut self, use_tree_view: bool) -> Self {
        self.use_wiki_tree_view = Some(use_tree_view);
        self
    }

    pub fn use_subversion(mut self, use_svn: bool) -> Self {
        self.use_subversion = Some(use_svn);
        self
    }

    pub fn use_git(mut self, use_git: bool) -> Self {
        self.use_git = Some(use_git);
        self
    }

    pub fn use_original_image_size_at_wiki(mut self, use_original: bool) -> Self {
        self.use_original_image_size_at_wiki = Some(use_original);
        self
    }

    pub fn text_formatting_rule(mut self, rule: TextFormattingRule) -> Self {
        self.text_formatting_rule = Some(rule);
        self
    }

    pub fn archived(mut self, archived: bool) -> Self {
        self.archived = Some(archived);
        self
    }

    pub fn use_dev_attributes(mut self, use_dev: bool) -> Self {
        self.use_dev_attributes = Some(use_dev);
        self
    }
}

#[cfg(feature = "writable")]
impl From<&UpdateProjectParams> for Vec<(String, String)> {
    fn from(params: &UpdateProjectParams) -> Self {
        let mut seq = Vec::new();

        if let Some(name) = &params.name {
            seq.push(("name".to_string(), name.clone()));
        }

        if let Some(key) = &params.key {
            seq.push(("key".to_string(), key.clone()));
        }

        if let Some(enabled) = params.chart_enabled {
            seq.push(("chartEnabled".to_string(), enabled.to_string()));
        }

        if let Some(use_resolved) = params.use_resolved_for_chart {
            seq.push(("useResolvedForChart".to_string(), use_resolved.to_string()));
        }

        if let Some(enabled) = params.subtasking_enabled {
            seq.push(("subtaskingEnabled".to_string(), enabled.to_string()));
        }

        if let Some(can_edit) = params.project_leader_can_edit_project_leader {
            seq.push((
                "projectLeaderCanEditProjectLeader".to_string(),
                can_edit.to_string(),
            ));
        }

        if let Some(use_wiki) = params.use_wiki {
            seq.push(("useWiki".to_string(), use_wiki.to_string()));
        }

        if let Some(use_file_sharing) = params.use_file_sharing {
            seq.push(("useFileSharing".to_string(), use_file_sharing.to_string()));
        }

        if let Some(use_tree_view) = params.use_wiki_tree_view {
            seq.push(("useWikiTreeView".to_string(), use_tree_view.to_string()));
        }

        if let Some(use_svn) = params.use_subversion {
            seq.push(("useSubversion".to_string(), use_svn.to_string()));
        }

        if let Some(use_git) = params.use_git {
            seq.push(("useGit".to_string(), use_git.to_string()));
        }

        if let Some(use_original) = params.use_original_image_size_at_wiki {
            seq.push((
                "useOriginalImageSizeAtWiki".to_string(),
                use_original.to_string(),
            ));
        }

        if let Some(rule) = &params.text_formatting_rule {
            seq.push(("textFormattingRule".to_string(), rule.to_string()));
        }

        if let Some(archived) = params.archived {
            seq.push(("archived".to_string(), archived.to_string()));
        }

        if let Some(use_dev) = params.use_dev_attributes {
            seq.push(("useDevAttributes".to_string(), use_dev.to_string()));
        }

        seq
    }
}

#[cfg(feature = "writable")]
impl IntoRequest for UpdateProjectParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Patch
    }

    fn path(&self) -> String {
        format!("/api/v2/projects/{}", self.project_id_or_key)
    }

    fn to_form(&self) -> impl Serialize {
        let params_vec: Vec<(String, String)> = self.into();
        params_vec
    }
}
