use crate::{
    Error, ProjectKey,
    identifier::{Identifier, ProjectId},
};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ProjectIdOrKey {
    Id(ProjectId),
    Key(ProjectKey),
    EitherIdOrKey(ProjectId, ProjectKey),
}

impl FromStr for ProjectIdOrKey {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match (u32::from_str(s), ProjectKey::from_str(s)) {
            (Ok(id), Ok(key)) if id > 0 => Ok(Self::EitherIdOrKey(ProjectId::new(id), key)),
            (Ok(id), Err(_)) if id > 0 => Ok(Self::Id(ProjectId::new(id))),
            (Err(_), Ok(key)) => Ok(Self::Key(key)),
            _ => Err(Error::InvalidProjectIdOrKey(s.to_string())),
        }
    }
}

impl std::fmt::Display for ProjectIdOrKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProjectIdOrKey::Id(id) => write!(f, "{}", id.value()),
            ProjectIdOrKey::Key(key) => write!(f, "{key}"),
            ProjectIdOrKey::EitherIdOrKey(id, _) => write!(f, "{}", id.value()),
        }
    }
}

impl From<ProjectId> for ProjectIdOrKey {
    fn from(id: ProjectId) -> Self {
        ProjectIdOrKey::Id(id)
    }
}

impl From<ProjectKey> for ProjectIdOrKey {
    fn from(key: ProjectKey) -> Self {
        ProjectIdOrKey::Key(key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_project_id_or_key_from_str_either() {
        // Numeric strings are valid both as ID and ProjectKey → EitherIdOrKey
        let id_or_key = ProjectIdOrKey::from_str("123").unwrap();
        assert!(matches!(id_or_key, ProjectIdOrKey::EitherIdOrKey(_, _)));
    }

    #[test]
    fn test_project_id_or_key_from_str_key() {
        // Strings with alphabetic characters → Key variant
        let id_or_key = ProjectIdOrKey::from_str("MYPROJECT").unwrap();
        assert!(matches!(id_or_key, ProjectIdOrKey::Key(_)));
    }

    #[test]
    fn test_project_id_or_key_from_str_invalid() {
        // Invalid formats
        assert!(ProjectIdOrKey::from_str("").is_err());
        assert!(ProjectIdOrKey::from_str("invalid-key").is_err()); // Hyphen not allowed
        assert!(ProjectIdOrKey::from_str("lowercase").is_err()); // Lowercase not allowed
        assert!(ProjectIdOrKey::from_str("0").is_err()); // Zero not allowed
    }

    #[test]
    fn test_project_id_or_key_display_id() {
        let id_or_key = ProjectIdOrKey::Id(ProjectId::new(456));
        assert_eq!(format!("{}", id_or_key), "456");
    }

    #[test]
    fn test_project_id_or_key_display_key() {
        let project_key = ProjectKey::from_str("PROJ").unwrap();
        let id_or_key = ProjectIdOrKey::Key(project_key);
        assert_eq!(format!("{}", id_or_key), "PROJ");
    }

    #[test]
    fn test_project_id_or_key_display_either() {
        // EitherIdOrKey prefers to display as ID
        let id_or_key = ProjectIdOrKey::from_str("999").unwrap();
        assert_eq!(format!("{}", id_or_key), "999");
    }

    #[test]
    fn test_project_id_or_key_from_types() {
        // From<ProjectId>
        let id_or_key: ProjectIdOrKey = ProjectId::new(789).into();
        assert!(matches!(id_or_key, ProjectIdOrKey::Id(_)));

        // From<ProjectKey>
        let project_key = ProjectKey::from_str("TEST").unwrap();
        let id_or_key: ProjectIdOrKey = project_key.into();
        assert!(matches!(id_or_key, ProjectIdOrKey::Key(_)));
    }

    #[test]
    fn test_project_id_or_key_serde_id() {
        let id_or_key = ProjectIdOrKey::Id(ProjectId::new(123));
        let json = serde_json::to_string(&id_or_key).unwrap();
        assert_eq!(json, "123");

        let deserialized: ProjectIdOrKey = serde_json::from_str(&json).unwrap();
        assert_eq!(id_or_key, deserialized);
    }

    #[test]
    fn test_project_id_or_key_serde_key() {
        let project_key = ProjectKey::from_str("PROJ").unwrap();
        let id_or_key = ProjectIdOrKey::Key(project_key);
        let json = serde_json::to_string(&id_or_key).unwrap();
        assert_eq!(json, "\"PROJ\"");

        let deserialized: ProjectIdOrKey = serde_json::from_str(&json).unwrap();
        assert_eq!(id_or_key, deserialized);
    }
}
