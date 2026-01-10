use std::fmt;

/// Error type for parsing color strings
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseColorError {
    input: String,
    color_type: &'static str,
    valid_colors: &'static str,
}

impl ParseColorError {
    pub fn new(
        input: impl Into<String>,
        color_type: &'static str,
        valid_colors: &'static str,
    ) -> Self {
        Self {
            input: input.into(),
            color_type,
            valid_colors,
        }
    }
}

impl fmt::Display for ParseColorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Invalid {} color: '{}'. Valid colors: {}",
            self.color_type, self.input, self.valid_colors
        )
    }
}

impl std::error::Error for ParseColorError {}
