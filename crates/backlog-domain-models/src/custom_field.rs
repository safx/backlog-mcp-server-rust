use backlog_core::{
    Date,
    identifier::{CustomFieldId, IssueTypeId, ProjectId},
};
use serde::{Deserialize, Deserializer, Serialize, de::Error as _};

#[cfg(feature = "schemars")]
use schemars::JsonSchema;

#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct CustomFieldType {
    pub id: CustomFieldId,
    pub project_id: ProjectId,
    pub name: String,
    pub description: String,
    pub required: bool,
    pub applicable_issue_types: Option<Vec<IssueTypeId>>,
    pub display_order: i64,
    #[serde(flatten)]
    pub settings: CustomFieldSettings,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub enum CustomFieldSettings {
    Text,
    TextArea,
    Numeric(NumericSettings),
    Date(DateSettings),
    SingleList(ListSettings),
    MultipleList(ListSettings),
    Checkbox(ListSettings),
    Radio(ListSettings),
}

// Raw types for deserializing typeId-based JSON
#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum RawCustomFieldType {
    Tagged(RawTaggedCustomFieldType),
    Untagged(RawUntaggedCustomFieldType),
}

// Custom Field Type IDs (Backlog API)
// 1: Text, 2: TextArea, 3: Numeric, 4: Date
// 5: SingleList, 6: MultipleList, 7: Checkbox, 8: Radio
#[derive(Debug, Deserialize)]
#[serde(tag = "typeId")]
enum RawTaggedCustomFieldType {
    #[serde(rename = "1")]
    Text(RawTextFieldType),
    #[serde(rename = "2")]
    TextArea(RawTextAreaFieldType),
    #[serde(rename = "3")]
    Numeric(RawNumericFieldType),
    #[serde(rename = "4")]
    Date(RawDateFieldType),
    #[serde(rename = "5")]
    SingleList(RawListFieldType),
    #[serde(rename = "6")]
    MultipleList(RawListFieldType),
    #[serde(rename = "7")]
    Checkbox(RawListFieldType),
    #[serde(rename = "8")]
    Radio(RawListFieldType),
}

/// Common fields shared across all custom field types
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RawCustomFieldBase {
    id: CustomFieldId,
    project_id: ProjectId,
    name: String,
    description: String,
    required: bool,
    applicable_issue_types: Option<Vec<IssueTypeId>>,
    display_order: i64,
}

impl RawCustomFieldBase {
    fn into_custom_field(self, settings: CustomFieldSettings) -> CustomFieldType {
        CustomFieldType {
            id: self.id,
            project_id: self.project_id,
            name: self.name,
            description: self.description,
            required: self.required,
            applicable_issue_types: self.applicable_issue_types,
            display_order: self.display_order,
            settings,
        }
    }
}

/// Parse an optional date string with error context
fn parse_optional_date<'de, D>(
    date_str: Option<&str>,
    field_name: &str,
) -> Result<Option<Date>, D::Error>
where
    D: Deserializer<'de>,
{
    use std::str::FromStr;
    date_str
        .map(Date::from_str)
        .transpose()
        .map_err(|e| D::Error::custom(format!("Failed to parse {} date: {}", field_name, e)))
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RawTextFieldType {
    #[serde(flatten)]
    base: RawCustomFieldBase,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RawTextAreaFieldType {
    #[serde(flatten)]
    base: RawCustomFieldBase,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RawNumericFieldType {
    #[serde(flatten)]
    base: RawCustomFieldBase,
    min: Option<f64>,
    max: Option<f64>,
    initial_value: Option<f64>,
    unit: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RawDateFieldType {
    #[serde(flatten)]
    base: RawCustomFieldBase,
    min: Option<String>,
    max: Option<String>,
    initial_value_type: Option<InitialDate>,
    initial_shift: Option<i32>,
    initial_date: Option<String>,
}

impl RawDateFieldType {
    fn into_date_settings<'de, D>(self) -> Result<(RawCustomFieldBase, DateSettings), D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok((
            self.base,
            DateSettings {
                min: parse_optional_date::<D>(self.min.as_deref(), "min")?,
                max: parse_optional_date::<D>(self.max.as_deref(), "max")?,
                initial_value_type: self.initial_value_type,
                initial_shift: self.initial_shift,
                initial_date: parse_optional_date::<D>(self.initial_date.as_deref(), "initial")?,
            },
        ))
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RawListFieldType {
    #[serde(flatten)]
    base: RawCustomFieldBase,
    items: Vec<ListItem>,
    #[serde(default)]
    allow_add_item: Option<bool>,
    #[serde(default)]
    allow_input: Option<bool>,
}

impl RawListFieldType {
    fn into_list_settings(self) -> (RawCustomFieldBase, ListSettings) {
        (
            self.base,
            ListSettings {
                items: self.items,
                allow_input: self.allow_input,
                allow_add_item: self.allow_add_item,
            },
        )
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RawUntaggedCustomFieldType {
    id: CustomFieldId,
    project_id: ProjectId,
    #[serde(rename = "typeId")]
    type_id: i32,
    name: String,
    description: String,
    required: bool,
    applicable_issue_types: Option<Vec<IssueTypeId>>,
    #[serde(default)]
    #[allow(dead_code)]
    use_issue_type: Option<bool>,
    display_order: i64,
    // Optional fields for different types
    #[serde(default)]
    min: Option<serde_json::Value>,
    #[serde(default)]
    max: Option<serde_json::Value>,
    #[serde(default)]
    initial_value: Option<f64>,
    #[serde(default)]
    unit: Option<String>,
    #[serde(default)]
    initial_value_type: Option<InitialDate>,
    #[serde(default)]
    initial_shift: Option<i32>,
    #[serde(default)]
    initial_date: Option<serde_json::Value>,
    #[serde(default)]
    items: Option<Vec<ListItem>>,
    #[serde(default)]
    allow_add_item: Option<bool>,
    #[serde(default)]
    allow_input: Option<bool>,
}

impl RawUntaggedCustomFieldType {
    fn to_list_settings(&self) -> ListSettings {
        ListSettings {
            items: self.items.clone().unwrap_or_default(),
            allow_input: self.allow_input,
            allow_add_item: self.allow_add_item,
        }
    }

    fn to_date_settings<'de, D>(&self) -> Result<DateSettings, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(DateSettings {
            min: parse_optional_date::<D>(self.min.as_ref().and_then(|v| v.as_str()), "min")?,
            max: parse_optional_date::<D>(self.max.as_ref().and_then(|v| v.as_str()), "max")?,
            initial_value_type: self.initial_value_type.clone(),
            initial_shift: self.initial_shift,
            initial_date: parse_optional_date::<D>(
                self.initial_date.as_ref().and_then(|v| v.as_str()),
                "initial",
            )?,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct NumericSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_value: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct DateSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<Date>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<Date>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_value_type: Option<InitialDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_shift: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_date: Option<Date>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct ListSettings {
    pub items: Vec<ListItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_input: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_add_item: Option<bool>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct ListItem {
    pub id: backlog_core::identifier::CustomFieldItemId,
    pub name: String,
    pub display_order: i32,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub enum InitialDate {
    #[serde(rename = "today")]
    Today,
    #[serde(rename = "tomorrow")]
    Tomorrow,
    #[serde(rename = "yesterday")]
    Yesterday,
    #[serde(rename = "specified")]
    Specified,
}

impl<'de> Deserialize<'de> for InitialDate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum InitialDateHelper {
            Integer(i32),
            String(String),
        }

        match InitialDateHelper::deserialize(deserializer)? {
            InitialDateHelper::Integer(i) => match i {
                1 => Ok(InitialDate::Today),
                2 => Ok(InitialDate::Tomorrow),
                3 => Ok(InitialDate::Yesterday),
                4 => Ok(InitialDate::Specified),
                _ => Err(serde::de::Error::custom(format!(
                    "Unknown InitialDate value: {i}"
                ))),
            },
            InitialDateHelper::String(s) => match s.as_str() {
                "today" => Ok(InitialDate::Today),
                "tomorrow" => Ok(InitialDate::Tomorrow),
                "yesterday" => Ok(InitialDate::Yesterday),
                "specified" => Ok(InitialDate::Specified),
                _ => Err(serde::de::Error::custom(format!(
                    "Unknown InitialDate string: {s}"
                ))),
            },
        }
    }
}

impl<'de> Deserialize<'de> for CustomFieldType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Try to deserialize as untagged first
        #[derive(Deserialize)]
        struct Peek {
            #[serde(rename = "typeId")]
            type_id: Option<i32>,
        }

        // First, deserialize to a JSON Value to inspect the structure
        let value = serde_json::Value::deserialize(deserializer)?;

        // Check if it has a typeId field at the root level
        if let Ok(peek) = serde_json::from_value::<Peek>(value.clone())
            && peek.type_id.is_some()
        {
            // This is an untagged format (API response)
            let untagged: RawUntaggedCustomFieldType =
                serde_json::from_value(value).map_err(serde::de::Error::custom)?;
            return CustomFieldType::from_untagged::<D>(untagged);
        }

        // Otherwise, try the tagged format
        let raw: RawCustomFieldType =
            serde_json::from_value(value).map_err(serde::de::Error::custom)?;

        match raw {
            RawCustomFieldType::Tagged(tagged) => CustomFieldType::from_tagged::<D>(tagged),
            RawCustomFieldType::Untagged(untagged) => CustomFieldType::from_untagged::<D>(untagged),
        }
    }
}

impl CustomFieldType {
    fn from_tagged<'de, D>(tagged: RawTaggedCustomFieldType) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let (base, settings) = match tagged {
            RawTaggedCustomFieldType::Text(raw) => (raw.base, CustomFieldSettings::Text),
            RawTaggedCustomFieldType::TextArea(raw) => (raw.base, CustomFieldSettings::TextArea),
            RawTaggedCustomFieldType::Numeric(raw) => (
                raw.base,
                CustomFieldSettings::Numeric(NumericSettings {
                    min: raw.min,
                    max: raw.max,
                    initial_value: raw.initial_value,
                    unit: raw.unit,
                }),
            ),
            RawTaggedCustomFieldType::Date(raw) => {
                let (base, settings) = raw.into_date_settings::<D>()?;
                (base, CustomFieldSettings::Date(settings))
            }
            RawTaggedCustomFieldType::SingleList(raw) => {
                let (base, settings) = raw.into_list_settings();
                (base, CustomFieldSettings::SingleList(settings))
            }
            RawTaggedCustomFieldType::MultipleList(raw) => {
                let (base, settings) = raw.into_list_settings();
                (base, CustomFieldSettings::MultipleList(settings))
            }
            RawTaggedCustomFieldType::Checkbox(raw) => {
                let (base, settings) = raw.into_list_settings();
                (base, CustomFieldSettings::Checkbox(settings))
            }
            RawTaggedCustomFieldType::Radio(raw) => {
                let (base, settings) = raw.into_list_settings();
                (base, CustomFieldSettings::Radio(settings))
            }
        };

        Ok(base.into_custom_field(settings))
    }
}

impl CustomFieldType {
    fn from_untagged<'de, D>(untagged: RawUntaggedCustomFieldType) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let settings = match untagged.type_id {
            1 => CustomFieldSettings::Text,
            2 => CustomFieldSettings::TextArea,
            3 => CustomFieldSettings::Numeric(NumericSettings {
                min: untagged.min.as_ref().and_then(|v| v.as_f64()),
                max: untagged.max.as_ref().and_then(|v| v.as_f64()),
                initial_value: untagged.initial_value,
                unit: untagged.unit.clone(),
            }),
            4 => CustomFieldSettings::Date(untagged.to_date_settings::<D>()?),
            5 => CustomFieldSettings::SingleList(untagged.to_list_settings()),
            6 => CustomFieldSettings::MultipleList(untagged.to_list_settings()),
            7 => CustomFieldSettings::Checkbox(untagged.to_list_settings()),
            8 => CustomFieldSettings::Radio(untagged.to_list_settings()),
            _ => {
                return Err(serde::de::Error::custom(format!(
                    "Unknown typeId: {}",
                    untagged.type_id
                )));
            }
        };

        Ok(CustomFieldType {
            id: untagged.id,
            project_id: untagged.project_id,
            name: untagged.name,
            description: untagged.description,
            required: untagged.required,
            applicable_issue_types: untagged.applicable_issue_types,
            display_order: untagged.display_order,
            settings,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use backlog_core::identifier::Identifier;

    #[test]
    fn test_custom_field_type_creation() {
        let field = CustomFieldType {
            id: CustomFieldId::new(1),
            project_id: ProjectId::new(10),
            name: "Test Field".to_string(),
            description: "A test custom field".to_string(),
            required: true,
            applicable_issue_types: Some(vec![IssueTypeId::new(1), IssueTypeId::new(2)]),
            display_order: 1,
            settings: CustomFieldSettings::Text,
        };

        assert_eq!(field.id.value(), 1);
        assert_eq!(field.project_id.value(), 10);
        assert_eq!(field.name, "Test Field");
        assert!(field.required);
    }

    #[test]
    fn test_numeric_settings_serialization() {
        let settings = NumericSettings {
            min: Some(0.0),
            max: Some(100.0),
            initial_value: Some(50.0),
            unit: Some("%".to_string()),
        };

        let json = serde_json::to_string(&settings).unwrap();
        assert!(json.contains("\"min\":0.0"));
        assert!(json.contains("\"max\":100.0"));
        assert!(json.contains("\"initialValue\":50.0"));
        assert!(json.contains("\"unit\":\"%\""));
    }

    #[test]
    fn test_date_settings_with_initial_date() {
        let settings = DateSettings {
            min: None,
            max: None,
            initial_value_type: Some(InitialDate::Today),
            initial_shift: None,
            initial_date: None,
        };

        let json = serde_json::to_string(&settings).unwrap();
        assert!(json.contains("\"initialValueType\":\"today\""));
    }

    #[test]
    fn test_list_settings_serialization() {
        use backlog_core::identifier::CustomFieldItemId;
        let settings = ListSettings {
            items: vec![
                ListItem {
                    id: CustomFieldItemId::new(1),
                    name: "Option 1".to_string(),
                    display_order: 1,
                },
                ListItem {
                    id: CustomFieldItemId::new(2),
                    name: "Option 2".to_string(),
                    display_order: 2,
                },
            ],
            allow_input: Some(true),
            allow_add_item: Some(false),
        };

        let json = serde_json::to_string(&settings).unwrap();
        assert!(json.contains("\"items\":["));
        assert!(json.contains("\"allowInput\":true"));
        assert!(json.contains("\"allowAddItem\":false"));
    }

    #[test]
    fn test_custom_field_settings_enum() {
        let text_settings = CustomFieldSettings::Text;
        assert!(matches!(text_settings, CustomFieldSettings::Text));

        let numeric_settings = CustomFieldSettings::Numeric(NumericSettings {
            min: Some(0.0),
            max: None,
            initial_value: None,
            unit: None,
        });

        if let CustomFieldSettings::Numeric(settings) = numeric_settings {
            assert_eq!(settings.min, Some(0.0));
        } else {
            panic!("Expected Numeric settings");
        }
    }

    // ============================================
    // InitialDate Tests
    // ============================================

    #[test]
    fn test_initial_date_deserialize_integer_today() {
        let result: InitialDate = serde_json::from_str("1").expect("should deserialize 1 as Today");
        assert_eq!(result, InitialDate::Today);
    }

    #[test]
    fn test_initial_date_deserialize_integer_tomorrow() {
        let result: InitialDate =
            serde_json::from_str("2").expect("should deserialize 2 as Tomorrow");
        assert_eq!(result, InitialDate::Tomorrow);
    }

    #[test]
    fn test_initial_date_deserialize_integer_yesterday() {
        let result: InitialDate =
            serde_json::from_str("3").expect("should deserialize 3 as Yesterday");
        assert_eq!(result, InitialDate::Yesterday);
    }

    #[test]
    fn test_initial_date_deserialize_integer_specified() {
        let result: InitialDate =
            serde_json::from_str("4").expect("should deserialize 4 as Specified");
        assert_eq!(result, InitialDate::Specified);
    }

    #[test]
    fn test_initial_date_deserialize_string_today() {
        let result: InitialDate =
            serde_json::from_str("\"today\"").expect("should deserialize 'today'");
        assert_eq!(result, InitialDate::Today);
    }

    #[test]
    fn test_initial_date_deserialize_string_tomorrow() {
        let result: InitialDate =
            serde_json::from_str("\"tomorrow\"").expect("should deserialize 'tomorrow'");
        assert_eq!(result, InitialDate::Tomorrow);
    }

    #[test]
    fn test_initial_date_deserialize_string_yesterday() {
        let result: InitialDate =
            serde_json::from_str("\"yesterday\"").expect("should deserialize 'yesterday'");
        assert_eq!(result, InitialDate::Yesterday);
    }

    #[test]
    fn test_initial_date_deserialize_string_specified() {
        let result: InitialDate =
            serde_json::from_str("\"specified\"").expect("should deserialize 'specified'");
        assert_eq!(result, InitialDate::Specified);
    }

    #[test]
    fn test_initial_date_deserialize_invalid_integer() {
        let result = serde_json::from_str::<InitialDate>("0");
        assert!(result.is_err(), "0 should be invalid");

        let result = serde_json::from_str::<InitialDate>("5");
        assert!(result.is_err(), "5 should be invalid");

        let result = serde_json::from_str::<InitialDate>("99");
        assert!(result.is_err(), "99 should be invalid");

        let result = serde_json::from_str::<InitialDate>("-1");
        assert!(result.is_err(), "-1 should be invalid");
    }

    #[test]
    fn test_initial_date_deserialize_invalid_string() {
        let result = serde_json::from_str::<InitialDate>("\"invalid\"");
        assert!(result.is_err(), "'invalid' should fail");

        let result = serde_json::from_str::<InitialDate>("\"Today\"");
        assert!(result.is_err(), "'Today' (capitalized) should fail");

        let result = serde_json::from_str::<InitialDate>("\"\"");
        assert!(result.is_err(), "empty string should fail");
    }

    #[test]
    fn test_initial_date_serialize() {
        assert_eq!(
            serde_json::to_string(&InitialDate::Today).expect("should serialize Today"),
            "\"today\""
        );
        assert_eq!(
            serde_json::to_string(&InitialDate::Tomorrow).expect("should serialize Tomorrow"),
            "\"tomorrow\""
        );
        assert_eq!(
            serde_json::to_string(&InitialDate::Yesterday).expect("should serialize Yesterday"),
            "\"yesterday\""
        );
        assert_eq!(
            serde_json::to_string(&InitialDate::Specified).expect("should serialize Specified"),
            "\"specified\""
        );
    }

    #[test]
    fn test_initial_date_roundtrip() {
        for original in [
            InitialDate::Today,
            InitialDate::Tomorrow,
            InitialDate::Yesterday,
            InitialDate::Specified,
        ] {
            let json = serde_json::to_string(&original).expect("should serialize");
            let deserialized: InitialDate =
                serde_json::from_str(&json).expect("should deserialize");
            assert_eq!(original, deserialized);
        }
    }

    // ============================================
    // CustomFieldType Deserialization Tests (API Response Format)
    // ============================================

    #[test]
    fn test_custom_field_type_deserialize_text() {
        let json = r#"{
            "id": 1,
            "projectId": 100,
            "typeId": 1,
            "name": "Text Field",
            "description": "A text field",
            "required": false,
            "applicableIssueTypes": [1, 2],
            "displayOrder": 0
        }"#;

        let field: CustomFieldType =
            serde_json::from_str(json).expect("should deserialize Text custom field");

        assert_eq!(field.id.value(), 1);
        assert_eq!(field.project_id.value(), 100);
        assert_eq!(field.name, "Text Field");
        assert_eq!(field.description, "A text field");
        assert!(!field.required);
        assert_eq!(
            field.applicable_issue_types,
            Some(vec![IssueTypeId::new(1), IssueTypeId::new(2)])
        );
        assert_eq!(field.display_order, 0);
        assert!(matches!(field.settings, CustomFieldSettings::Text));
    }

    #[test]
    fn test_custom_field_type_deserialize_textarea() {
        let json = r#"{
            "id": 2,
            "projectId": 100,
            "typeId": 2,
            "name": "TextArea Field",
            "description": "",
            "required": true,
            "applicableIssueTypes": [],
            "displayOrder": 1
        }"#;

        let field: CustomFieldType =
            serde_json::from_str(json).expect("should deserialize TextArea custom field");

        assert_eq!(field.id.value(), 2);
        assert!(field.required);
        assert!(matches!(field.settings, CustomFieldSettings::TextArea));
    }

    #[test]
    fn test_custom_field_type_deserialize_numeric() {
        let json = r#"{
            "id": 3,
            "projectId": 100,
            "typeId": 3,
            "name": "Estimate Hours",
            "description": "Estimated work hours",
            "required": false,
            "applicableIssueTypes": null,
            "displayOrder": 2,
            "min": 0.0,
            "max": 100.0,
            "initialValue": 8.0,
            "unit": "hours"
        }"#;

        let field: CustomFieldType =
            serde_json::from_str(json).expect("should deserialize Numeric custom field");

        assert_eq!(field.id.value(), 3);
        assert_eq!(field.name, "Estimate Hours");

        if let CustomFieldSettings::Numeric(settings) = &field.settings {
            assert_eq!(settings.min, Some(0.0));
            assert_eq!(settings.max, Some(100.0));
            assert_eq!(settings.initial_value, Some(8.0));
            assert_eq!(settings.unit, Some("hours".to_string()));
        } else {
            panic!("Expected Numeric settings");
        }
    }

    #[test]
    fn test_custom_field_type_deserialize_numeric_minimal() {
        let json = r#"{
            "id": 3,
            "projectId": 100,
            "typeId": 3,
            "name": "Points",
            "description": "",
            "required": false,
            "applicableIssueTypes": null,
            "displayOrder": 0
        }"#;

        let field: CustomFieldType = serde_json::from_str(json)
            .expect("should deserialize Numeric custom field without optional fields");

        if let CustomFieldSettings::Numeric(settings) = &field.settings {
            assert_eq!(settings.min, None);
            assert_eq!(settings.max, None);
            assert_eq!(settings.initial_value, None);
            assert_eq!(settings.unit, None);
        } else {
            panic!("Expected Numeric settings");
        }
    }

    #[test]
    fn test_custom_field_type_deserialize_date() {
        let json = r#"{
            "id": 4,
            "projectId": 100,
            "typeId": 4,
            "name": "Due Date",
            "description": "Expected completion date",
            "required": true,
            "applicableIssueTypes": [1],
            "displayOrder": 3,
            "min": "2024-01-01",
            "max": "2024-12-31",
            "initialValueType": 1,
            "initialShift": 7,
            "initialDate": "2024-06-15"
        }"#;

        let field: CustomFieldType =
            serde_json::from_str(json).expect("should deserialize Date custom field");

        assert_eq!(field.id.value(), 4);
        assert_eq!(field.name, "Due Date");
        assert!(field.required);

        if let CustomFieldSettings::Date(settings) = &field.settings {
            assert!(settings.min.is_some());
            assert!(settings.max.is_some());
            assert_eq!(settings.initial_value_type, Some(InitialDate::Today));
            assert_eq!(settings.initial_shift, Some(7));
            assert!(settings.initial_date.is_some());
        } else {
            panic!("Expected Date settings");
        }
    }

    #[test]
    fn test_custom_field_type_deserialize_date_minimal() {
        let json = r#"{
            "id": 4,
            "projectId": 100,
            "typeId": 4,
            "name": "Start Date",
            "description": "",
            "required": false,
            "applicableIssueTypes": null,
            "displayOrder": 0
        }"#;

        let field: CustomFieldType = serde_json::from_str(json)
            .expect("should deserialize Date custom field without optional fields");

        if let CustomFieldSettings::Date(settings) = &field.settings {
            assert_eq!(settings.min, None);
            assert_eq!(settings.max, None);
            assert_eq!(settings.initial_value_type, None);
            assert_eq!(settings.initial_shift, None);
            assert_eq!(settings.initial_date, None);
        } else {
            panic!("Expected Date settings");
        }
    }

    #[test]
    fn test_custom_field_type_deserialize_single_list() {
        let json = r#"{
            "id": 5,
            "projectId": 100,
            "typeId": 5,
            "name": "Priority Level",
            "description": "Select priority",
            "required": false,
            "applicableIssueTypes": null,
            "displayOrder": 4,
            "items": [
                {"id": 1, "name": "Low", "displayOrder": 0},
                {"id": 2, "name": "Medium", "displayOrder": 1},
                {"id": 3, "name": "High", "displayOrder": 2}
            ],
            "allowInput": false,
            "allowAddItem": true
        }"#;

        let field: CustomFieldType =
            serde_json::from_str(json).expect("should deserialize SingleList custom field");

        assert_eq!(field.id.value(), 5);

        if let CustomFieldSettings::SingleList(settings) = &field.settings {
            assert_eq!(settings.items.len(), 3);
            assert_eq!(settings.items[0].name, "Low");
            assert_eq!(settings.items[1].name, "Medium");
            assert_eq!(settings.items[2].name, "High");
            assert_eq!(settings.allow_input, Some(false));
            assert_eq!(settings.allow_add_item, Some(true));
        } else {
            panic!("Expected SingleList settings");
        }
    }

    #[test]
    fn test_custom_field_type_deserialize_multiple_list() {
        let json = r#"{
            "id": 6,
            "projectId": 100,
            "typeId": 6,
            "name": "Tags",
            "description": "Multiple tags",
            "required": false,
            "applicableIssueTypes": null,
            "displayOrder": 5,
            "items": [
                {"id": 10, "name": "Frontend", "displayOrder": 0},
                {"id": 11, "name": "Backend", "displayOrder": 1}
            ],
            "allowInput": true
        }"#;

        let field: CustomFieldType =
            serde_json::from_str(json).expect("should deserialize MultipleList custom field");

        if let CustomFieldSettings::MultipleList(settings) = &field.settings {
            assert_eq!(settings.items.len(), 2);
            assert_eq!(settings.allow_input, Some(true));
            assert_eq!(settings.allow_add_item, None);
        } else {
            panic!("Expected MultipleList settings");
        }
    }

    #[test]
    fn test_custom_field_type_deserialize_checkbox() {
        let json = r#"{
            "id": 7,
            "projectId": 100,
            "typeId": 7,
            "name": "Features",
            "description": "Check applicable features",
            "required": false,
            "applicableIssueTypes": null,
            "displayOrder": 6,
            "items": [
                {"id": 20, "name": "Feature A", "displayOrder": 0},
                {"id": 21, "name": "Feature B", "displayOrder": 1}
            ]
        }"#;

        let field: CustomFieldType =
            serde_json::from_str(json).expect("should deserialize Checkbox custom field");

        if let CustomFieldSettings::Checkbox(settings) = &field.settings {
            assert_eq!(settings.items.len(), 2);
        } else {
            panic!("Expected Checkbox settings");
        }
    }

    #[test]
    fn test_custom_field_type_deserialize_radio() {
        let json = r#"{
            "id": 8,
            "projectId": 100,
            "typeId": 8,
            "name": "Environment",
            "description": "Select environment",
            "required": true,
            "applicableIssueTypes": null,
            "displayOrder": 7,
            "items": [
                {"id": 30, "name": "Development", "displayOrder": 0},
                {"id": 31, "name": "Staging", "displayOrder": 1},
                {"id": 32, "name": "Production", "displayOrder": 2}
            ]
        }"#;

        let field: CustomFieldType =
            serde_json::from_str(json).expect("should deserialize Radio custom field");

        assert!(field.required);

        if let CustomFieldSettings::Radio(settings) = &field.settings {
            assert_eq!(settings.items.len(), 3);
            assert_eq!(settings.items[2].name, "Production");
        } else {
            panic!("Expected Radio settings");
        }
    }

    #[test]
    fn test_custom_field_type_deserialize_invalid_type_id() {
        let json = r#"{
            "id": 99,
            "projectId": 100,
            "typeId": 99,
            "name": "Invalid",
            "description": "",
            "required": false,
            "applicableIssueTypes": null,
            "displayOrder": 0
        }"#;

        let result = serde_json::from_str::<CustomFieldType>(json);
        assert!(result.is_err(), "typeId 99 should be invalid");
    }

    #[test]
    fn test_custom_field_type_deserialize_date_with_invalid_date_format() {
        let json = r#"{
            "id": 4,
            "projectId": 100,
            "typeId": 4,
            "name": "Bad Date",
            "description": "",
            "required": false,
            "applicableIssueTypes": null,
            "displayOrder": 0,
            "min": "not-a-date"
        }"#;

        let result = serde_json::from_str::<CustomFieldType>(json);
        assert!(result.is_err(), "invalid date format should fail");
    }
}
