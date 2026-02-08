use backlog_core::identifier::CustomFieldItemId;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[cfg(feature = "schemars")]
use schemars::JsonSchema;

/// Represents a value for a custom field list item.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct CustomFieldListItem {
    /// The ID of the list item.
    pub id: CustomFieldItemId,
    /// The name of the list item.
    pub name: String,
}

/// Represents different types of custom field values with strong typing.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[non_exhaustive]
pub enum CustomFieldValue {
    /// Text field value (TypeId: 1)
    Text(String),
    /// TextArea field value (TypeId: 2)
    TextArea(String),
    /// Numeric field value (TypeId: 3)
    Numeric(f64),
    /// Date field value (TypeId: 4)
    Date(NaiveDate),
    /// Single selection list field value (TypeId: 5)
    SingleList {
        /// The selected item
        item: CustomFieldListItem,
        /// Optional "other" value when "other" option is selected
        other_value: Option<String>,
    },
    /// Multiple selection list field value (TypeId: 6)
    MultipleList {
        /// The selected items
        items: Vec<CustomFieldListItem>,
        /// Optional "other" value when "other" option is selected
        other_value: Option<String>,
    },
    /// Checkbox field value (TypeId: 7)
    CheckBox(Vec<CustomFieldListItem>),
    /// Radio button field value (TypeId: 8)
    Radio {
        /// The selected item
        item: CustomFieldListItem,
        /// Optional "other" value when "other" option is selected
        other_value: Option<String>,
    },
}

impl CustomFieldValue {
    /// Convert to form parameter value for API requests.
    /// Returns (value, optional_other_value) tuple.
    pub fn to_form_value(&self) -> (String, Option<String>) {
        match self {
            CustomFieldValue::Text(s) | CustomFieldValue::TextArea(s) => (s.clone(), None),
            CustomFieldValue::Numeric(n) => (n.to_string(), None),
            CustomFieldValue::Date(d) => (d.format("%Y-%m-%d").to_string(), None),
            CustomFieldValue::SingleList { item, other_value } => {
                (item.id.to_string(), other_value.clone())
            }
            CustomFieldValue::MultipleList { items, other_value } => {
                let id_strings: Vec<String> =
                    items.iter().map(|item| item.id.to_string()).collect();
                (id_strings.join(","), other_value.clone())
            }
            CustomFieldValue::CheckBox(items) => {
                let id_strings: Vec<String> =
                    items.iter().map(|item| item.id.to_string()).collect();
                (id_strings.join(","), None)
            }
            CustomFieldValue::Radio { item, other_value } => {
                (item.id.to_string(), other_value.clone())
            }
        }
    }
}

/// Input type for custom fields when creating or updating issues.
/// This is used for form serialization where we only need IDs, not full objects.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[non_exhaustive]
pub enum CustomFieldInput {
    /// Text field input (TypeId: 1)
    Text(String),
    /// TextArea field input (TypeId: 2)
    TextArea(String),
    /// Numeric field input (TypeId: 3)
    Numeric(f64),
    /// Date field input (TypeId: 4)
    Date(NaiveDate),
    /// Single selection list field input (TypeId: 5)
    SingleList {
        /// The ID of the selected item
        id: CustomFieldItemId,
        /// Optional "other" value
        other_value: Option<String>,
    },
    /// Multiple selection list field input (TypeId: 6)
    MultipleList {
        /// The IDs of the selected items
        ids: Vec<CustomFieldItemId>,
        /// Optional "other" value
        other_value: Option<String>,
    },
    /// Checkbox field input (TypeId: 7)
    CheckBox(Vec<CustomFieldItemId>),
    /// Radio button field input (TypeId: 8)
    Radio {
        /// The ID of the selected item
        id: CustomFieldItemId,
        /// Optional "other" value
        other_value: Option<String>,
    },
}

impl CustomFieldInput {
    /// Convert to form parameter value for API requests.
    /// Returns (value, optional_other_value) tuple.
    pub fn to_form_value(&self) -> (String, Option<String>) {
        match self {
            CustomFieldInput::Text(s) | CustomFieldInput::TextArea(s) => (s.clone(), None),
            CustomFieldInput::Numeric(n) => (n.to_string(), None),
            CustomFieldInput::Date(d) => (d.format("%Y-%m-%d").to_string(), None),
            CustomFieldInput::SingleList { id, other_value } => {
                (id.to_string(), other_value.clone())
            }
            CustomFieldInput::MultipleList { ids, other_value } => {
                // For form submission, multiple values need special handling
                // This will be handled in the form serialization logic
                let id_strings: Vec<String> = ids.iter().map(|id| id.to_string()).collect();
                (id_strings.join(","), other_value.clone())
            }
            CustomFieldInput::CheckBox(ids) => {
                let id_strings: Vec<String> = ids.iter().map(|id| id.to_string()).collect();
                (id_strings.join(","), None)
            }
            CustomFieldInput::Radio { id, other_value } => (id.to_string(), other_value.clone()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_custom_field_list_item() {
        let item = CustomFieldListItem {
            id: CustomFieldItemId::new(123),
            name: "Option A".to_string(),
        };
        assert_eq!(item.id, CustomFieldItemId::new(123));
        assert_eq!(item.name, "Option A");
    }

    #[test]
    fn test_custom_field_value_text() {
        let value = CustomFieldValue::Text("Hello, World!".to_string());
        let (form_value, other) = value.to_form_value();
        assert_eq!(form_value, "Hello, World!");
        assert_eq!(other, None);
    }

    #[test]
    fn test_custom_field_value_numeric() {
        let value = CustomFieldValue::Numeric(42.5);
        let (form_value, other) = value.to_form_value();
        assert_eq!(form_value, "42.5");
        assert_eq!(other, None);
    }

    #[test]
    fn test_custom_field_value_date() {
        let date = NaiveDate::from_ymd_opt(2024, 6, 24).unwrap();
        let value = CustomFieldValue::Date(date);
        let (form_value, other) = value.to_form_value();
        assert_eq!(form_value, "2024-06-24");
        assert_eq!(other, None);
    }

    #[test]
    fn test_custom_field_value_single_list() {
        let item = CustomFieldListItem {
            id: CustomFieldItemId::new(456),
            name: "Selected Option".to_string(),
        };
        let value = CustomFieldValue::SingleList {
            item,
            other_value: Some("Additional info".to_string()),
        };
        let (form_value, other) = value.to_form_value();
        assert_eq!(form_value, "456");
        assert_eq!(other, Some("Additional info".to_string()));
    }

    #[test]
    fn test_custom_field_value_multiple_list() {
        let items = vec![
            CustomFieldListItem {
                id: CustomFieldItemId::new(100),
                name: "Option 1".to_string(),
            },
            CustomFieldListItem {
                id: CustomFieldItemId::new(200),
                name: "Option 2".to_string(),
            },
        ];
        let value = CustomFieldValue::MultipleList {
            items,
            other_value: None,
        };
        let (form_value, other) = value.to_form_value();
        assert_eq!(form_value, "100,200");
        assert_eq!(other, None);
    }

    #[test]
    fn test_custom_field_input_text() {
        let input = CustomFieldInput::Text("Test input".to_string());
        let (form_value, other) = input.to_form_value();
        assert_eq!(form_value, "Test input");
        assert_eq!(other, None);
    }

    #[test]
    fn test_custom_field_input_date() {
        let date = NaiveDate::from_ymd_opt(2024, 12, 31).unwrap();
        let input = CustomFieldInput::Date(date);
        let (form_value, other) = input.to_form_value();
        assert_eq!(form_value, "2024-12-31");
        assert_eq!(other, None);
    }

    #[test]
    fn test_custom_field_input_single_list() {
        let input = CustomFieldInput::SingleList {
            id: CustomFieldItemId::new(789),
            other_value: Some("Other description".to_string()),
        };
        let (form_value, other) = input.to_form_value();
        assert_eq!(form_value, "789");
        assert_eq!(other, Some("Other description".to_string()));
    }

    #[test]
    fn test_custom_field_input_checkbox() {
        let input = CustomFieldInput::CheckBox(vec![
            CustomFieldItemId::new(10),
            CustomFieldItemId::new(20),
            CustomFieldItemId::new(30),
        ]);
        let (form_value, other) = input.to_form_value();
        assert_eq!(form_value, "10,20,30");
        assert_eq!(other, None);
    }
}
