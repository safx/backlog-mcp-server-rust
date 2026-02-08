use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub enum TextFormattingRule {
    Backlog,
    Markdown,
}

impl std::fmt::Display for TextFormattingRule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Backlog => write!(f, "backlog"),
            Self::Markdown => write!(f, "markdown"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_formatting_rule_deserialization() {
        let backlog: TextFormattingRule = serde_json::from_str("\"backlog\"").unwrap();
        assert_eq!(backlog, TextFormattingRule::Backlog);

        let markdown: TextFormattingRule = serde_json::from_str("\"markdown\"").unwrap();
        assert_eq!(markdown, TextFormattingRule::Markdown);
    }

    #[test]
    fn test_text_formatting_rule_serialization() {
        assert_eq!(
            serde_json::to_string(&TextFormattingRule::Backlog).unwrap(),
            "\"backlog\""
        );
        assert_eq!(
            serde_json::to_string(&TextFormattingRule::Markdown).unwrap(),
            "\"markdown\""
        );
    }

    #[test]
    fn test_text_formatting_rule_display() {
        assert_eq!(TextFormattingRule::Backlog.to_string(), "backlog");
        assert_eq!(TextFormattingRule::Markdown.to_string(), "markdown");
    }

    #[test]
    fn test_text_formatting_rule_round_trip() {
        for rule in [TextFormattingRule::Backlog, TextFormattingRule::Markdown] {
            let json = serde_json::to_string(&rule).unwrap();
            let deserialized: TextFormattingRule = serde_json::from_str(&json).unwrap();
            assert_eq!(rule, deserialized);
        }
    }

    #[test]
    fn test_text_formatting_rule_deserialization_invalid() {
        assert!(serde_json::from_str::<TextFormattingRule>("\"html\"").is_err());
        assert!(serde_json::from_str::<TextFormattingRule>("\"Markdown\"").is_err()); // case sensitive
        assert!(serde_json::from_str::<TextFormattingRule>("\"BACKLOG\"").is_err());
    }
}
