#[cfg(feature = "writable")]
use crate::models::CustomFieldInput;
#[cfg(feature = "writable")]
use backlog_core::identifier::CustomFieldId;
#[cfg(feature = "writable")]
use std::collections::HashMap;

/// Extension trait for serializing custom fields to form parameters
///
/// This trait provides a method to serialize custom field values into form parameters
/// that can be sent to the Backlog API. It handles the special case where `MultipleList`
/// and `CheckBox` fields need to generate multiple form parameters with the same key.
#[cfg(feature = "writable")]
pub trait CustomFieldFormSerializer {
    /// Serialize custom fields to form parameters
    ///
    /// Appends custom field parameters to the provided vector. For most custom field types,
    /// a single parameter is added in the format `customField_{id}`. However, for `MultipleList`
    /// and `CheckBox` types, multiple parameters with the same key are added, one for each value.
    ///
    /// If a custom field has an "other value" (for list types), an additional parameter
    /// `customField_{id}_otherValue` is appended.
    fn serialize_custom_fields(&self, params: &mut Vec<(String, String)>);
}

#[cfg(feature = "writable")]
impl CustomFieldFormSerializer for Option<HashMap<CustomFieldId, CustomFieldInput>> {
    fn serialize_custom_fields(&self, params: &mut Vec<(String, String)>) {
        if let Some(custom_fields) = self {
            for (field_id, input) in custom_fields {
                let (value, other_value) = input.to_form_value();

                // Handle multiple values differently from single values
                match input {
                    CustomFieldInput::MultipleList { ids, .. } => {
                        // Multiple values need to be added as separate parameters
                        for id in ids {
                            params.push((format!("customField_{field_id}"), id.to_string()));
                        }
                    }
                    CustomFieldInput::CheckBox(ids) => {
                        // Multiple values need to be added as separate parameters
                        for id in ids {
                            params.push((format!("customField_{field_id}"), id.to_string()));
                        }
                    }
                    _ => {
                        params.push((format!("customField_{field_id}"), value));
                    }
                }

                if let Some(other) = other_value {
                    params.push((format!("customField_{field_id}_otherValue"), other));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::CustomFieldInput;
    use backlog_core::identifier::{CustomFieldId, CustomFieldItemId};
    use chrono::NaiveDate;

    #[test]
    fn test_serialize_none_custom_fields() {
        let custom_fields: Option<HashMap<CustomFieldId, CustomFieldInput>> = None;
        let mut params = Vec::new();
        custom_fields.serialize_custom_fields(&mut params);
        assert_eq!(params.len(), 0);
    }

    #[test]
    fn test_serialize_empty_custom_fields() {
        let custom_fields: Option<HashMap<CustomFieldId, CustomFieldInput>> = Some(HashMap::new());
        let mut params = Vec::new();
        custom_fields.serialize_custom_fields(&mut params);
        assert_eq!(params.len(), 0);
    }

    #[test]
    fn test_serialize_text_custom_field() {
        let mut custom_fields = HashMap::new();
        custom_fields.insert(
            CustomFieldId::new(100),
            CustomFieldInput::Text("Test Value".to_string()),
        );
        let custom_fields = Some(custom_fields);

        let mut params = Vec::new();
        custom_fields.serialize_custom_fields(&mut params);

        assert_eq!(params.len(), 1);
        assert_eq!(
            params[0],
            ("customField_100".to_string(), "Test Value".to_string())
        );
    }

    #[test]
    fn test_serialize_numeric_custom_field() {
        let mut custom_fields = HashMap::new();
        custom_fields.insert(CustomFieldId::new(101), CustomFieldInput::Numeric(123.45));
        let custom_fields = Some(custom_fields);

        let mut params = Vec::new();
        custom_fields.serialize_custom_fields(&mut params);

        assert_eq!(params.len(), 1);
        assert_eq!(
            params[0],
            ("customField_101".to_string(), "123.45".to_string())
        );
    }

    #[test]
    fn test_serialize_date_custom_field() {
        let mut custom_fields = HashMap::new();
        custom_fields.insert(
            CustomFieldId::new(102),
            CustomFieldInput::Date(NaiveDate::from_ymd_opt(2024, 1, 15).unwrap()),
        );
        let custom_fields = Some(custom_fields);

        let mut params = Vec::new();
        custom_fields.serialize_custom_fields(&mut params);

        assert_eq!(params.len(), 1);
        assert_eq!(
            params[0],
            ("customField_102".to_string(), "2024-01-15".to_string())
        );
    }

    #[test]
    fn test_serialize_single_list_with_other() {
        let mut custom_fields = HashMap::new();
        custom_fields.insert(
            CustomFieldId::new(103),
            CustomFieldInput::SingleList {
                id: CustomFieldItemId::new(200),
                other_value: Some("Custom value".to_string()),
            },
        );
        let custom_fields = Some(custom_fields);

        let mut params = Vec::new();
        custom_fields.serialize_custom_fields(&mut params);

        assert_eq!(params.len(), 2);
        assert_eq!(
            params[0],
            ("customField_103".to_string(), "200".to_string())
        );
        assert_eq!(
            params[1],
            (
                "customField_103_otherValue".to_string(),
                "Custom value".to_string()
            )
        );
    }

    #[test]
    fn test_serialize_multiple_list() {
        let mut custom_fields = HashMap::new();
        custom_fields.insert(
            CustomFieldId::new(104),
            CustomFieldInput::MultipleList {
                ids: vec![
                    CustomFieldItemId::new(201),
                    CustomFieldItemId::new(202),
                    CustomFieldItemId::new(203),
                ],
                other_value: None,
            },
        );
        let custom_fields = Some(custom_fields);

        let mut params = Vec::new();
        custom_fields.serialize_custom_fields(&mut params);

        assert_eq!(params.len(), 3);
        assert_eq!(
            params[0],
            ("customField_104".to_string(), "201".to_string())
        );
        assert_eq!(
            params[1],
            ("customField_104".to_string(), "202".to_string())
        );
        assert_eq!(
            params[2],
            ("customField_104".to_string(), "203".to_string())
        );
    }

    #[test]
    fn test_serialize_multiple_list_with_other() {
        let mut custom_fields = HashMap::new();
        custom_fields.insert(
            CustomFieldId::new(105),
            CustomFieldInput::MultipleList {
                ids: vec![CustomFieldItemId::new(210), CustomFieldItemId::new(211)],
                other_value: Some("Other value".to_string()),
            },
        );
        let custom_fields = Some(custom_fields);

        let mut params = Vec::new();
        custom_fields.serialize_custom_fields(&mut params);

        assert_eq!(params.len(), 3);
        assert_eq!(
            params[0],
            ("customField_105".to_string(), "210".to_string())
        );
        assert_eq!(
            params[1],
            ("customField_105".to_string(), "211".to_string())
        );
        assert_eq!(
            params[2],
            (
                "customField_105_otherValue".to_string(),
                "Other value".to_string()
            )
        );
    }

    #[test]
    fn test_serialize_checkbox() {
        let mut custom_fields = HashMap::new();
        custom_fields.insert(
            CustomFieldId::new(106),
            CustomFieldInput::CheckBox(vec![
                CustomFieldItemId::new(301),
                CustomFieldItemId::new(302),
            ]),
        );
        let custom_fields = Some(custom_fields);

        let mut params = Vec::new();
        custom_fields.serialize_custom_fields(&mut params);

        assert_eq!(params.len(), 2);
        assert_eq!(
            params[0],
            ("customField_106".to_string(), "301".to_string())
        );
        assert_eq!(
            params[1],
            ("customField_106".to_string(), "302".to_string())
        );
    }

    #[test]
    fn test_serialize_radio_with_other() {
        let mut custom_fields = HashMap::new();
        custom_fields.insert(
            CustomFieldId::new(107),
            CustomFieldInput::Radio {
                id: CustomFieldItemId::new(400),
                other_value: Some("Radio other".to_string()),
            },
        );
        let custom_fields = Some(custom_fields);

        let mut params = Vec::new();
        custom_fields.serialize_custom_fields(&mut params);

        assert_eq!(params.len(), 2);
        assert_eq!(
            params[0],
            ("customField_107".to_string(), "400".to_string())
        );
        assert_eq!(
            params[1],
            (
                "customField_107_otherValue".to_string(),
                "Radio other".to_string()
            )
        );
    }

    #[test]
    fn test_serialize_mixed_custom_fields() {
        let mut custom_fields = HashMap::new();
        custom_fields.insert(
            CustomFieldId::new(100),
            CustomFieldInput::Text("Text".to_string()),
        );
        custom_fields.insert(CustomFieldId::new(101), CustomFieldInput::Numeric(42.0));
        custom_fields.insert(
            CustomFieldId::new(102),
            CustomFieldInput::MultipleList {
                ids: vec![CustomFieldItemId::new(500), CustomFieldItemId::new(501)],
                other_value: None,
            },
        );
        let custom_fields = Some(custom_fields);

        let mut params = Vec::new();
        custom_fields.serialize_custom_fields(&mut params);

        // Total: 1 (text) + 1 (numeric) + 2 (multiple list) = 4
        assert_eq!(params.len(), 4);

        // Note: HashMap doesn't guarantee order, so we need to check existence
        assert!(params.contains(&("customField_100".to_string(), "Text".to_string())));
        assert!(params.contains(&("customField_101".to_string(), "42".to_string())));
        assert!(params.contains(&("customField_102".to_string(), "500".to_string())));
        assert!(params.contains(&("customField_102".to_string(), "501".to_string())));
    }
}
