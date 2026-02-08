use serde_repr::{Deserialize_repr, Serialize_repr};
use std::fmt;

/// Parent-child issue filtering condition
///
/// Defines conditions used in Backlog API's `parentChild` parameter.
#[repr(u8)]
#[derive(Eq, PartialEq, Debug, Clone, Copy, Hash, Serialize_repr, Deserialize_repr, Default)]
pub enum ParentChildCondition {
    /// All issues (no filtering by parent/child status)
    #[default]
    All = 0,

    /// Exclude Child Issues
    ExcludeChildIssue = 1,

    /// Child Issues only
    ChildIssue = 2,

    /// Neither Parent nor Child Issues
    NeitherParentIssueNorChildIssue = 3,

    /// Parent Issues only
    ParentIssue = 4,
}

impl ParentChildCondition {
    /// Get description text for the condition
    pub fn description(&self) -> &'static str {
        match self {
            ParentChildCondition::All => "All issues (no filtering by parent/child status)",
            ParentChildCondition::ExcludeChildIssue => "Exclude Child Issues",
            ParentChildCondition::ChildIssue => "Child Issues only",
            ParentChildCondition::NeitherParentIssueNorChildIssue => {
                "Neither Parent nor Child Issues"
            }
            ParentChildCondition::ParentIssue => "Parent Issues only",
        }
    }

    /// Get all available conditions
    pub fn all() -> &'static [ParentChildCondition] {
        &[
            ParentChildCondition::All,
            ParentChildCondition::ExcludeChildIssue,
            ParentChildCondition::ChildIssue,
            ParentChildCondition::NeitherParentIssueNorChildIssue,
            ParentChildCondition::ParentIssue,
        ]
    }
}

impl fmt::Display for ParentChildCondition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", *self as u8)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_parent_child_condition_values() {
        assert_eq!(ParentChildCondition::All as u8, 0);
        assert_eq!(ParentChildCondition::ExcludeChildIssue as u8, 1);
        assert_eq!(ParentChildCondition::ChildIssue as u8, 2);
        assert_eq!(
            ParentChildCondition::NeitherParentIssueNorChildIssue as u8,
            3
        );
        assert_eq!(ParentChildCondition::ParentIssue as u8, 4);
    }

    #[test]
    fn test_serialization() {
        assert_eq!(
            serde_json::to_string(&ParentChildCondition::All).unwrap(),
            "0"
        );
        assert_eq!(
            serde_json::to_string(&ParentChildCondition::ChildIssue).unwrap(),
            "2"
        );
        assert_eq!(
            serde_json::to_string(&ParentChildCondition::ParentIssue).unwrap(),
            "4"
        );
    }

    #[test]
    fn test_deserialization() {
        assert_eq!(
            serde_json::from_str::<ParentChildCondition>("0").unwrap(),
            ParentChildCondition::All
        );
        assert_eq!(
            serde_json::from_str::<ParentChildCondition>("2").unwrap(),
            ParentChildCondition::ChildIssue
        );
        assert_eq!(
            serde_json::from_str::<ParentChildCondition>("4").unwrap(),
            ParentChildCondition::ParentIssue
        );
    }

    #[test]
    fn test_description() {
        assert_eq!(
            ParentChildCondition::All.description(),
            "All issues (no filtering by parent/child status)"
        );
        assert_eq!(
            ParentChildCondition::ExcludeChildIssue.description(),
            "Exclude Child Issues"
        );
        assert_eq!(
            ParentChildCondition::ChildIssue.description(),
            "Child Issues only"
        );
        assert_eq!(
            ParentChildCondition::NeitherParentIssueNorChildIssue.description(),
            "Neither Parent nor Child Issues"
        );
        assert_eq!(
            ParentChildCondition::ParentIssue.description(),
            "Parent Issues only"
        );
    }

    #[test]
    fn test_all_variants() {
        let all_conditions = ParentChildCondition::all();
        assert_eq!(all_conditions.len(), 5);
        assert!(all_conditions.contains(&ParentChildCondition::All));
        assert!(all_conditions.contains(&ParentChildCondition::ChildIssue));
        assert!(all_conditions.contains(&ParentChildCondition::ParentIssue));
    }

    #[test]
    fn test_default() {
        assert_eq!(ParentChildCondition::default(), ParentChildCondition::All);
    }

    #[test]
    fn test_display() {
        assert_eq!(ParentChildCondition::All.to_string(), "0");
        assert_eq!(ParentChildCondition::ExcludeChildIssue.to_string(), "1");
        assert_eq!(ParentChildCondition::ChildIssue.to_string(), "2");
        assert_eq!(
            ParentChildCondition::NeitherParentIssueNorChildIssue.to_string(),
            "3"
        );
        assert_eq!(ParentChildCondition::ParentIssue.to_string(), "4");
    }
}
