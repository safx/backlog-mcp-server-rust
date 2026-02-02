use serde::{Deserialize, Serialize};

#[cfg(feature = "schemars")]
use schemars::JsonSchema;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub enum Language {
    #[serde(rename = "ja")]
    Japanese,
    #[serde(rename = "en")]
    English,
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Japanese => write!(f, "ja"),
            Self::English => write!(f, "en"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_deserialization() {
        let ja: Language = serde_json::from_str("\"ja\"").unwrap();
        assert_eq!(ja, Language::Japanese);

        let en: Language = serde_json::from_str("\"en\"").unwrap();
        assert_eq!(en, Language::English);
    }

    #[test]
    fn test_language_serialization() {
        assert_eq!(
            serde_json::to_string(&Language::Japanese).unwrap(),
            "\"ja\""
        );
        assert_eq!(serde_json::to_string(&Language::English).unwrap(), "\"en\"");
    }

    #[test]
    fn test_language_display() {
        assert_eq!(Language::Japanese.to_string(), "ja");
        assert_eq!(Language::English.to_string(), "en");
    }

    #[test]
    fn test_language_round_trip() {
        for lang in [Language::Japanese, Language::English] {
            let json = serde_json::to_string(&lang).unwrap();
            let deserialized: Language = serde_json::from_str(&json).unwrap();
            assert_eq!(lang, deserialized);
        }
    }

    #[test]
    fn test_language_deserialization_invalid() {
        assert!(serde_json::from_str::<Language>("\"fr\"").is_err());
        assert!(serde_json::from_str::<Language>("\"JA\"").is_err()); // case sensitive
        assert!(serde_json::from_str::<Language>("\"japanese\"").is_err());
    }
}
