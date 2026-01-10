use crate::models::{CustomFieldListItem, CustomFieldTypeId, CustomFieldValue};
use backlog_core::identifier::CustomFieldId;
use serde::{Deserialize, Deserializer};
use serde_json::Value;

#[cfg(feature = "schemars")]
use schemars::JsonSchema;

/// Raw structure for deserializing custom field from JSON
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RawCustomField {
    id: CustomFieldId,
    field_type_id: CustomFieldTypeId,
    name: String,
    value: Value,
    #[serde(default)]
    other_value: Option<Value>,
}

/// Represents a custom field associated with an issue with strongly typed values.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct CustomFieldWithValue {
    /// The ID of the custom field.
    pub id: CustomFieldId,
    /// The field type ID.
    pub field_type_id: CustomFieldTypeId,
    /// The name of the custom field.
    pub name: String,
    /// The strongly typed value of the custom field.
    pub value: CustomFieldValue,
}

impl<'de> Deserialize<'de> for CustomFieldWithValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = RawCustomField::deserialize(deserializer)?;

        let value = match raw.field_type_id {
            CustomFieldTypeId::Text => CustomFieldValue::Text(
                raw.value
                    .as_str()
                    .ok_or_else(|| {
                        serde::de::Error::custom(format!(
                            "Expected string for Text field '{}' (id: {})",
                            raw.name, raw.id
                        ))
                    })?
                    .to_string(),
            ),
            CustomFieldTypeId::TextArea => CustomFieldValue::TextArea(
                raw.value
                    .as_str()
                    .ok_or_else(|| {
                        serde::de::Error::custom(format!(
                            "Expected string for TextArea field '{}' (id: {})",
                            raw.name, raw.id
                        ))
                    })?
                    .to_string(),
            ),
            CustomFieldTypeId::Numeric => {
                CustomFieldValue::Numeric(raw.value.as_f64().ok_or_else(|| {
                    serde::de::Error::custom(format!(
                        "Expected number for Numeric field '{}' (id: {})",
                        raw.name, raw.id
                    ))
                })?)
            }
            CustomFieldTypeId::Date => {
                let date_str = raw.value.as_str().ok_or_else(|| {
                    serde::de::Error::custom(format!(
                        "Expected string for Date field '{}' (id: {})",
                        raw.name, raw.id
                    ))
                })?;
                let date =
                    chrono::NaiveDate::parse_from_str(date_str, "%Y-%m-%d").map_err(|e| {
                        serde::de::Error::custom(format!(
                            "Invalid date format for field '{}' (id: {}): {e}",
                            raw.name, raw.id
                        ))
                    })?;
                CustomFieldValue::Date(date)
            }
            CustomFieldTypeId::SingleList => {
                let item: CustomFieldListItem = serde_json::from_value(raw.value).map_err(|e| {
                    serde::de::Error::custom(format!(
                        "Failed to parse SingleList item for field '{}' (id: {}): {e}",
                        raw.name, raw.id
                    ))
                })?;
                let other_value = raw
                    .other_value
                    .as_ref()
                    .and_then(|v| v.as_str())
                    .map(String::from);
                CustomFieldValue::SingleList { item, other_value }
            }
            CustomFieldTypeId::MultipleList => {
                let items: Vec<CustomFieldListItem> =
                    serde_json::from_value(raw.value).map_err(|e| {
                        serde::de::Error::custom(format!(
                            "Failed to parse MultipleList items for field '{}' (id: {}): {e}",
                            raw.name, raw.id
                        ))
                    })?;
                let other_value = raw
                    .other_value
                    .as_ref()
                    .and_then(|v| v.as_str())
                    .map(String::from);
                CustomFieldValue::MultipleList { items, other_value }
            }
            CustomFieldTypeId::CheckBox => {
                let items: Vec<CustomFieldListItem> =
                    serde_json::from_value(raw.value).map_err(|e| {
                        serde::de::Error::custom(format!(
                            "Failed to parse CheckBox items for field '{}' (id: {}): {e}",
                            raw.name, raw.id
                        ))
                    })?;
                CustomFieldValue::CheckBox(items)
            }
            CustomFieldTypeId::Radio => {
                let item: CustomFieldListItem = serde_json::from_value(raw.value).map_err(|e| {
                    serde::de::Error::custom(format!(
                        "Failed to parse Radio item for field '{}' (id: {}): {e}",
                        raw.name, raw.id
                    ))
                })?;
                let other_value = raw
                    .other_value
                    .as_ref()
                    .and_then(|v| v.as_str())
                    .map(String::from);
                CustomFieldValue::Radio { item, other_value }
            }
        };

        Ok(CustomFieldWithValue {
            id: raw.id,
            field_type_id: raw.field_type_id,
            name: raw.name,
            value,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use backlog_core::identifier::CustomFieldItemId;
    use chrono::NaiveDate;

    #[test]
    fn test_deserialize_text_field() {
        let json = r#"{
            "id": 1,
            "fieldTypeId": 1,
            "name": "テキストフィールド",
            "value": "サンプルテキスト"
        }"#;

        let field: CustomFieldWithValue = serde_json::from_str(json).unwrap();
        assert_eq!(field.id, CustomFieldId::new(1));
        assert_eq!(field.field_type_id, CustomFieldTypeId::Text);
        assert_eq!(field.name, "テキストフィールド");

        match field.value {
            CustomFieldValue::Text(ref s) => assert_eq!(s, "サンプルテキスト"),
            _ => panic!("Expected Text variant"),
        }
    }

    #[test]
    fn test_deserialize_numeric_field() {
        let json = r#"{
            "id": 2,
            "fieldTypeId": 3,
            "name": "数値フィールド",
            "value": 42.5
        }"#;

        let field: CustomFieldWithValue = serde_json::from_str(json).unwrap();
        assert_eq!(field.field_type_id, CustomFieldTypeId::Numeric);

        match field.value {
            CustomFieldValue::Numeric(n) => assert_eq!(n, 42.5),
            _ => panic!("Expected Numeric variant"),
        }
    }

    #[test]
    fn test_deserialize_date_field() {
        let json = r#"{
            "id": 3,
            "fieldTypeId": 4,
            "name": "日付フィールド",
            "value": "2024-06-24"
        }"#;

        let field: CustomFieldWithValue = serde_json::from_str(json).unwrap();
        assert_eq!(field.field_type_id, CustomFieldTypeId::Date);

        match field.value {
            CustomFieldValue::Date(d) => {
                assert_eq!(d, NaiveDate::from_ymd_opt(2024, 6, 24).unwrap());
            }
            _ => panic!("Expected Date variant"),
        }
    }

    #[test]
    fn test_deserialize_single_list_field() {
        let json = r#"{
            "id": 4,
            "fieldTypeId": 5,
            "name": "単一選択リスト",
            "value": {"id": 123, "name": "選択肢A"},
            "otherValue": "その他の説明"
        }"#;

        let field: CustomFieldWithValue = serde_json::from_str(json).unwrap();
        assert_eq!(field.field_type_id, CustomFieldTypeId::SingleList);

        match &field.value {
            CustomFieldValue::SingleList { item, other_value } => {
                assert_eq!(item.id, CustomFieldItemId::new(123));
                assert_eq!(item.name, "選択肢A");
                assert_eq!(other_value, &Some("その他の説明".to_string()));
            }
            _ => panic!("Expected SingleList variant"),
        }
    }

    #[test]
    fn test_deserialize_multiple_list_field() {
        let json = r#"{
            "id": 5,
            "fieldTypeId": 6,
            "name": "複数選択リスト",
            "value": [
                {"id": 100, "name": "選択肢1"},
                {"id": 200, "name": "選択肢2"}
            ]
        }"#;

        let field: CustomFieldWithValue = serde_json::from_str(json).unwrap();
        assert_eq!(field.field_type_id, CustomFieldTypeId::MultipleList);

        match &field.value {
            CustomFieldValue::MultipleList { items, other_value } => {
                assert_eq!(items.len(), 2);
                assert_eq!(items[0].id, CustomFieldItemId::new(100));
                assert_eq!(items[1].id, CustomFieldItemId::new(200));
                assert_eq!(other_value, &None);
            }
            _ => panic!("Expected MultipleList variant"),
        }
    }
}
