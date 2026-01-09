use backlog_core::identifier::{IssueTypeId, ProjectId};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

#[cfg(feature = "schemars")]
use schemars::JsonSchema;

/// Represents an issue type in Backlog.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct IssueType {
    pub id: IssueTypeId,
    pub project_id: ProjectId,
    pub name: String,
    pub color: String,
    pub display_order: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template_summary: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template_description: Option<String>,
}

/// Represents valid colors for issue types in Backlog.
/// These are the only colors supported by the Backlog API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub enum IssueTypeColor {
    #[serde(rename = "#e30000")]
    Red,
    #[serde(rename = "#990000")]
    DarkRed,
    #[serde(rename = "#934981")]
    Purple,
    #[serde(rename = "#814fbc")]
    Violet,
    #[serde(rename = "#2779ca")]
    Blue,
    #[serde(rename = "#007e9a")]
    Teal,
    #[serde(rename = "#7ea800")]
    Green,
    #[serde(rename = "#ff9200")]
    Orange,
    #[serde(rename = "#ff3265")]
    Pink,
    #[serde(rename = "#666665")]
    Gray,
}

impl IssueTypeColor {
    /// Returns the hex color code as a string slice.
    pub fn as_hex(&self) -> &'static str {
        match self {
            Self::Red => "#e30000",
            Self::DarkRed => "#990000",
            Self::Purple => "#934981",
            Self::Violet => "#814fbc",
            Self::Blue => "#2779ca",
            Self::Teal => "#007e9a",
            Self::Green => "#7ea800",
            Self::Orange => "#ff9200",
            Self::Pink => "#ff3265",
            Self::Gray => "#666665",
        }
    }

    /// Returns the human-readable name of the color.
    pub fn name(&self) -> &'static str {
        match self {
            Self::Red => "red",
            Self::DarkRed => "dark-red",
            Self::Purple => "purple",
            Self::Violet => "violet",
            Self::Blue => "blue",
            Self::Teal => "teal",
            Self::Green => "green",
            Self::Orange => "orange",
            Self::Pink => "pink",
            Self::Gray => "gray",
        }
    }

    /// Returns all available issue type colors.
    pub fn all_colors() -> &'static [IssueTypeColor] {
        &[
            Self::Red,
            Self::DarkRed,
            Self::Purple,
            Self::Violet,
            Self::Blue,
            Self::Teal,
            Self::Green,
            Self::Orange,
            Self::Pink,
            Self::Gray,
        ]
    }

    /// Returns all available color names.
    pub fn all_names() -> Vec<&'static str> {
        Self::all_colors().iter().map(|c| c.name()).collect()
    }

    /// Returns all available hex codes.
    pub fn all_hex_codes() -> Vec<&'static str> {
        Self::all_colors().iter().map(|c| c.as_hex()).collect()
    }
}

impl fmt::Display for IssueTypeColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_hex())
    }
}

impl FromStr for IssueTypeColor {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            // Accept both hex codes and color names
            "#e30000" | "red" => Ok(Self::Red),
            "#990000" | "dark-red" => Ok(Self::DarkRed),
            "#934981" | "purple" => Ok(Self::Purple),
            "#814fbc" | "violet" => Ok(Self::Violet),
            "#2779ca" | "blue" => Ok(Self::Blue),
            "#007e9a" | "teal" => Ok(Self::Teal),
            "#7ea800" | "green" => Ok(Self::Green),
            "#ff9200" | "orange" => Ok(Self::Orange),
            "#ff3265" | "pink" => Ok(Self::Pink),
            "#666665" | "gray" => Ok(Self::Gray),
            _ => Err(format!(
                "Invalid issue type color: '{}'. Valid colors: {}",
                s,
                Self::all_names().join(", ")
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_issue_type_color_as_hex() {
        assert_eq!(IssueTypeColor::Red.as_hex(), "#e30000");
        assert_eq!(IssueTypeColor::Blue.as_hex(), "#2779ca");
        assert_eq!(IssueTypeColor::Gray.as_hex(), "#666665");
    }

    #[test]
    fn test_issue_type_color_name() {
        assert_eq!(IssueTypeColor::Red.name(), "red");
        assert_eq!(IssueTypeColor::DarkRed.name(), "dark-red");
        assert_eq!(IssueTypeColor::Gray.name(), "gray");
    }

    #[test]
    fn test_issue_type_color_from_str_hex() {
        assert_eq!(
            IssueTypeColor::from_str("#e30000").expect("should parse hex color"),
            IssueTypeColor::Red
        );
        assert_eq!(
            IssueTypeColor::from_str("#2779ca").expect("should parse hex color"),
            IssueTypeColor::Blue
        );
        assert_eq!(
            IssueTypeColor::from_str("#666665").expect("should parse hex color"),
            IssueTypeColor::Gray
        );
    }

    #[test]
    fn test_issue_type_color_from_str_name() {
        assert_eq!(
            IssueTypeColor::from_str("red").expect("should parse color name"),
            IssueTypeColor::Red
        );
        assert_eq!(
            IssueTypeColor::from_str("dark-red").expect("should parse color name"),
            IssueTypeColor::DarkRed
        );
        assert_eq!(
            IssueTypeColor::from_str("gray").expect("should parse color name"),
            IssueTypeColor::Gray
        );
    }

    #[test]
    fn test_issue_type_color_from_str_invalid() {
        assert!(IssueTypeColor::from_str("invalid").is_err());
        assert!(IssueTypeColor::from_str("#000000").is_err());
    }

    #[test]
    fn test_issue_type_color_all_colors() {
        let colors = IssueTypeColor::all_colors();
        assert_eq!(colors.len(), 10);
        assert!(colors.contains(&IssueTypeColor::Red));
        assert!(colors.contains(&IssueTypeColor::Gray));
    }

    #[test]
    fn test_issue_type_color_display() {
        assert_eq!(IssueTypeColor::Red.to_string(), "#e30000");
        assert_eq!(IssueTypeColor::Blue.to_string(), "#2779ca");
    }

    #[test]
    fn test_issue_type_color_serialize() {
        let color = IssueTypeColor::Red;
        let json = serde_json::to_string(&color).expect("should serialize color");
        assert_eq!(json, r##""#e30000""##);
    }

    #[test]
    fn test_issue_type_color_deserialize() {
        let json = r##""#2779ca""##;
        let color: IssueTypeColor =
            serde_json::from_str(json).expect("should deserialize color from JSON");
        assert_eq!(color, IssueTypeColor::Blue);
    }

    #[test]
    fn test_issue_type_deserialize() {
        let json = r##"{"id":1,"projectId":100,"name":"Bug","color":"#e30000","displayOrder":0}"##;
        let issue_type: IssueType =
            serde_json::from_str(json).expect("should deserialize IssueType from JSON");
        assert_eq!(issue_type.name, "Bug");
        assert_eq!(issue_type.color, "#e30000");
    }

    #[test]
    fn test_issue_type_deserialize_with_templates() {
        let json = r##"{"id":2,"projectId":100,"name":"Task","color":"#2779ca","displayOrder":1,"templateSummary":"Summary","templateDescription":"Description"}"##;
        let issue_type: IssueType =
            serde_json::from_str(json).expect("should deserialize IssueType with templates");
        assert_eq!(issue_type.template_summary, Some("Summary".to_string()));
    }
}
