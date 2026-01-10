use crate::error::ParseColorError;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

#[cfg(feature = "schemars")]
use schemars::JsonSchema;

/// Represents valid colors for statuses in Backlog.
/// These are the only colors supported by the Backlog API for statuses.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub enum StatusColor {
    #[serde(rename = "#ea2c00")]
    Red,
    #[serde(rename = "#e87758")]
    Coral,
    #[serde(rename = "#e07b9a")]
    Pink,
    #[serde(rename = "#868cb7")]
    LightPurple,
    #[serde(rename = "#3b9dbd")]
    Blue,
    #[serde(rename = "#4caf93")]
    Green,
    #[serde(rename = "#b0be3c")]
    LightGreen,
    #[serde(rename = "#eda62a")]
    Orange,
    #[serde(rename = "#f42858")]
    Magenta,
    #[serde(rename = "#393939")]
    DarkGray,
}

impl StatusColor {
    /// Returns the hex color code as a string slice.
    pub fn as_hex(&self) -> &'static str {
        match self {
            Self::Red => "#ea2c00",
            Self::Coral => "#e87758",
            Self::Pink => "#e07b9a",
            Self::LightPurple => "#868cb7",
            Self::Blue => "#3b9dbd",
            Self::Green => "#4caf93",
            Self::LightGreen => "#b0be3c",
            Self::Orange => "#eda62a",
            Self::Magenta => "#f42858",
            Self::DarkGray => "#393939",
        }
    }

    /// Returns the human-readable name of the color.
    pub fn name(&self) -> &'static str {
        match self {
            Self::Red => "red",
            Self::Coral => "coral",
            Self::Pink => "pink",
            Self::LightPurple => "light-purple",
            Self::Blue => "blue",
            Self::Green => "green",
            Self::LightGreen => "light-green",
            Self::Orange => "orange",
            Self::Magenta => "magenta",
            Self::DarkGray => "dark-gray",
        }
    }

    /// Returns all available status colors.
    pub fn all_colors() -> &'static [StatusColor] {
        &[
            Self::Red,
            Self::Coral,
            Self::Pink,
            Self::LightPurple,
            Self::Blue,
            Self::Green,
            Self::LightGreen,
            Self::Orange,
            Self::Magenta,
            Self::DarkGray,
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

impl fmt::Display for StatusColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_hex())
    }
}

impl FromStr for StatusColor {
    type Err = ParseColorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            // Accept both hex codes and color names
            "#ea2c00" | "red" => Ok(Self::Red),
            "#e87758" | "coral" => Ok(Self::Coral),
            "#e07b9a" | "pink" => Ok(Self::Pink),
            "#868cb7" | "light-purple" => Ok(Self::LightPurple),
            "#3b9dbd" | "blue" => Ok(Self::Blue),
            "#4caf93" | "green" => Ok(Self::Green),
            "#b0be3c" | "light-green" => Ok(Self::LightGreen),
            "#eda62a" | "orange" => Ok(Self::Orange),
            "#f42858" | "magenta" => Ok(Self::Magenta),
            "#393939" | "dark-gray" => Ok(Self::DarkGray),
            _ => Err(ParseColorError::new(
                s,
                "status",
                "red, coral, pink, light-purple, blue, green, light-green, orange, magenta, dark-gray",
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_color_as_hex() {
        assert_eq!(StatusColor::Red.as_hex(), "#ea2c00");
        assert_eq!(StatusColor::Blue.as_hex(), "#3b9dbd");
        assert_eq!(StatusColor::DarkGray.as_hex(), "#393939");
    }

    #[test]
    fn test_status_color_name() {
        assert_eq!(StatusColor::Red.name(), "red");
        assert_eq!(StatusColor::LightPurple.name(), "light-purple");
        assert_eq!(StatusColor::DarkGray.name(), "dark-gray");
    }

    #[test]
    fn test_status_color_from_str_hex() {
        assert_eq!(StatusColor::from_str("#ea2c00").unwrap(), StatusColor::Red);
        assert_eq!(StatusColor::from_str("#3b9dbd").unwrap(), StatusColor::Blue);
        assert_eq!(
            StatusColor::from_str("#393939").unwrap(),
            StatusColor::DarkGray
        );
    }

    #[test]
    fn test_status_color_from_str_name() {
        assert_eq!(StatusColor::from_str("red").unwrap(), StatusColor::Red);
        assert_eq!(
            StatusColor::from_str("light-purple").unwrap(),
            StatusColor::LightPurple
        );
        assert_eq!(
            StatusColor::from_str("dark-gray").unwrap(),
            StatusColor::DarkGray
        );
    }

    #[test]
    fn test_status_color_from_str_invalid() {
        assert!(StatusColor::from_str("invalid").is_err());
        assert!(StatusColor::from_str("#000000").is_err());
    }

    #[test]
    fn test_status_color_all_colors() {
        let colors = StatusColor::all_colors();
        assert_eq!(colors.len(), 10);
        assert!(colors.contains(&StatusColor::Red));
        assert!(colors.contains(&StatusColor::DarkGray));
    }

    #[test]
    fn test_status_color_display() {
        assert_eq!(StatusColor::Red.to_string(), "#ea2c00");
        assert_eq!(StatusColor::Blue.to_string(), "#3b9dbd");
    }

    #[test]
    fn test_status_color_serialize() {
        let color = StatusColor::Red;
        let json = serde_json::to_string(&color).unwrap();
        assert_eq!(json, "\"#ea2c00\"");
    }

    #[test]
    fn test_status_color_deserialize() {
        let json = "\"#3b9dbd\"";
        let color: StatusColor = serde_json::from_str(json).unwrap();
        assert_eq!(color, StatusColor::Blue);
    }
}
