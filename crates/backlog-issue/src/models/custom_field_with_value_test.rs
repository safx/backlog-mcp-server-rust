#[cfg(test)]
mod additional_tests {
    use crate::models::{CustomFieldTypeId, CustomFieldValue, CustomFieldWithValue};
    use backlog_core::identifier::{CustomFieldId, CustomFieldItemId};

    #[test]
    fn test_deserialize_text_field_with_special_chars() {
        let json = r#"{
            "id": 1,
            "fieldTypeId": 1,
            "name": "Special Text",
            "value": "Line 1\nLine 2\t\tTab\r\n\"Quoted\""
        }"#;

        let field: CustomFieldWithValue = serde_json::from_str(json).unwrap();
        match field.value {
            CustomFieldValue::Text(ref s) => {
                assert_eq!(s, "Line 1\nLine 2\t\tTab\r\n\"Quoted\"");
            }
            _ => panic!("Expected Text variant"),
        }
    }

    #[test]
    fn test_deserialize_numeric_field_integer() {
        let json = r#"{
            "id": 2,
            "fieldTypeId": 3,
            "name": "Integer Number",
            "value": 42
        }"#;

        let field: CustomFieldWithValue = serde_json::from_str(json).unwrap();
        match field.value {
            CustomFieldValue::Numeric(n) => assert_eq!(n, 42.0),
            _ => panic!("Expected Numeric variant"),
        }
    }

    #[test]
    fn test_deserialize_numeric_field_scientific_notation() {
        let json = r#"{
            "id": 3,
            "fieldTypeId": 3,
            "name": "Scientific",
            "value": 1.23e-4
        }"#;

        let field: CustomFieldWithValue = serde_json::from_str(json).unwrap();
        match field.value {
            CustomFieldValue::Numeric(n) => assert_eq!(n, 0.000123),
            _ => panic!("Expected Numeric variant"),
        }
    }

    #[test]
    fn test_deserialize_invalid_date_format() {
        let json = r#"{
            "id": 4,
            "fieldTypeId": 4,
            "name": "Bad Date",
            "value": "2024/06/24"
        }"#;

        let result: Result<CustomFieldWithValue, _> = serde_json::from_str(json);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Invalid date format")
        );
    }

    #[test]
    fn test_deserialize_invalid_numeric_value() {
        let json = r#"{
            "id": 5,
            "fieldTypeId": 3,
            "name": "Bad Number",
            "value": "not a number"
        }"#;

        let result: Result<CustomFieldWithValue, _> = serde_json::from_str(json);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Expected number for Numeric field")
        );
    }

    #[test]
    fn test_deserialize_single_list_missing_fields() {
        let json = r#"{
            "id": 6,
            "fieldTypeId": 5,
            "name": "Bad List",
            "value": {"id": 123}
        }"#;

        let result: Result<CustomFieldWithValue, _> = serde_json::from_str(json);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Failed to parse SingleList item")
        );
    }

    #[test]
    fn test_deserialize_multiple_list_empty() {
        let json = r#"{
            "id": 7,
            "fieldTypeId": 6,
            "name": "Empty List",
            "value": []
        }"#;

        let field: CustomFieldWithValue = serde_json::from_str(json).unwrap();
        match &field.value {
            CustomFieldValue::MultipleList { items, other_value } => {
                assert_eq!(items.len(), 0);
                assert_eq!(other_value, &None);
            }
            _ => panic!("Expected MultipleList variant"),
        }
    }

    #[test]
    fn test_deserialize_checkbox_with_single_item() {
        let json = r#"{
            "id": 8,
            "fieldTypeId": 7,
            "name": "Single Checkbox",
            "value": [{"id": 1, "name": "Option 1"}]
        }"#;

        let field: CustomFieldWithValue = serde_json::from_str(json).unwrap();
        match &field.value {
            CustomFieldValue::CheckBox(items) => {
                assert_eq!(items.len(), 1);
                assert_eq!(items[0].id, CustomFieldItemId::new(1));
                assert_eq!(items[0].name, "Option 1");
            }
            _ => panic!("Expected CheckBox variant"),
        }
    }

    #[test]
    fn test_deserialize_radio_without_other_value() {
        let json = r#"{
            "id": 9,
            "fieldTypeId": 8,
            "name": "Radio Field",
            "value": {"id": 789, "name": "Selected"}
        }"#;

        let field: CustomFieldWithValue = serde_json::from_str(json).unwrap();
        match &field.value {
            CustomFieldValue::Radio { item, other_value } => {
                assert_eq!(item.id, CustomFieldItemId::new(789));
                assert_eq!(item.name, "Selected");
                assert_eq!(other_value, &None);
            }
            _ => panic!("Expected Radio variant"),
        }
    }

    #[test]
    fn test_deserialize_with_null_other_value() {
        let json = r#"{
            "id": 10,
            "fieldTypeId": 5,
            "name": "List with null other",
            "value": {"id": 123, "name": "Item"},
            "otherValue": null
        }"#;

        let field: CustomFieldWithValue = serde_json::from_str(json).unwrap();
        match &field.value {
            CustomFieldValue::SingleList { item, other_value } => {
                assert_eq!(item.id, CustomFieldItemId::new(123));
                assert_eq!(other_value, &None);
            }
            _ => panic!("Expected SingleList variant"),
        }
    }

    #[test]
    fn test_deserialize_missing_other_value_field() {
        // This should work as otherValue is optional
        let json = r#"{
            "id": 11,
            "fieldTypeId": 8,
            "name": "Radio without otherValue field",
            "value": {"id": 456, "name": "Option"}
        }"#;

        let field: CustomFieldWithValue = serde_json::from_str(json).unwrap();
        match &field.value {
            CustomFieldValue::Radio { item, other_value } => {
                assert_eq!(item.id, CustomFieldItemId::new(456));
                assert_eq!(other_value, &None);
            }
            _ => panic!("Expected Radio variant"),
        }
    }

    #[test]
    fn test_field_equality() {
        let field1 = CustomFieldWithValue {
            id: CustomFieldId::new(1),
            field_type_id: CustomFieldTypeId::Text,
            name: "Field".to_string(),
            value: CustomFieldValue::Text("Value".to_string()),
        };

        let field2 = CustomFieldWithValue {
            id: CustomFieldId::new(1),
            field_type_id: CustomFieldTypeId::Text,
            name: "Field".to_string(),
            value: CustomFieldValue::Text("Value".to_string()),
        };

        assert_eq!(field1, field2);
    }

    #[test]
    fn test_field_inequality() {
        let field1 = CustomFieldWithValue {
            id: CustomFieldId::new(1),
            field_type_id: CustomFieldTypeId::Text,
            name: "Field".to_string(),
            value: CustomFieldValue::Text("Value1".to_string()),
        };

        let field2 = CustomFieldWithValue {
            id: CustomFieldId::new(1),
            field_type_id: CustomFieldTypeId::Text,
            name: "Field".to_string(),
            value: CustomFieldValue::Text("Value2".to_string()),
        };

        assert_ne!(field1, field2);
    }
}
