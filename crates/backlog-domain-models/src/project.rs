use backlog_core::{ProjectKey, TextFormattingRule, identifier::ProjectId};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub id: ProjectId,
    pub project_key: ProjectKey,
    pub name: String,
    pub chart_enabled: bool,
    pub subtasking_enabled: bool,
    pub project_leader_can_edit_project_leader: bool,
    pub use_wiki: bool,
    pub use_file_sharing: bool,
    pub use_wiki_tree_view: bool,
    pub use_original_image_size_at_wiki: bool,
    pub text_formatting_rule: TextFormattingRule,
    pub archived: bool,
    pub display_order: i32,
    pub use_dev_attributes: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project_deserialize() {
        let json = r##"{"id":1,"projectKey":"TEST","name":"Test Project","chartEnabled":true,"subtaskingEnabled":true,"projectLeaderCanEditProjectLeader":false,"useWiki":true,"useFileSharing":true,"useWikiTreeView":false,"useOriginalImageSizeAtWiki":false,"textFormattingRule":"markdown","archived":false,"displayOrder":0,"useDevAttributes":false}"##;
        let project: Project =
            serde_json::from_str(json).expect("should deserialize Project from JSON");
        assert_eq!(project.name, "Test Project");
        assert!(project.chart_enabled);
        assert!(project.use_wiki);
    }
}
