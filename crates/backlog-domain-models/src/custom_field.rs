use backlog_core::{
    Date,
    identifier::{CustomFieldId, IssueTypeId, ProjectId},
};
use serde::{Deserialize, Deserializer, Serialize};

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
        use std::str::FromStr;

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
            RawTaggedCustomFieldType::Date(raw) => (
                raw.base,
                CustomFieldSettings::Date(DateSettings {
                    min: raw.min.and_then(|s| Date::from_str(&s).ok()),
                    max: raw.max.and_then(|s| Date::from_str(&s).ok()),
                    initial_value_type: raw.initial_value_type,
                    initial_shift: raw.initial_shift,
                    initial_date: raw.initial_date.and_then(|s| Date::from_str(&s).ok()),
                }),
            ),
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
                unit: untagged.unit,
            }),
            4 => {
                use std::str::FromStr;
                CustomFieldSettings::Date(DateSettings {
                    min: untagged
                        .min
                        .as_ref()
                        .and_then(|s| s.as_str())
                        .and_then(|s| Date::from_str(s).ok()),
                    max: untagged
                        .max
                        .as_ref()
                        .and_then(|s| s.as_str())
                        .and_then(|s| Date::from_str(s).ok()),
                    initial_value_type: untagged.initial_value_type,
                    initial_shift: untagged.initial_shift,
                    initial_date: untagged
                        .initial_date
                        .as_ref()
                        .and_then(|s| s.as_str())
                        .and_then(|s| Date::from_str(s).ok()),
                })
            }
            5 => CustomFieldSettings::SingleList(ListSettings {
                items: untagged.items.unwrap_or_default(),
                allow_input: untagged.allow_input,
                allow_add_item: untagged.allow_add_item,
            }),
            6 => CustomFieldSettings::MultipleList(ListSettings {
                items: untagged.items.unwrap_or_default(),
                allow_input: untagged.allow_input,
                allow_add_item: untagged.allow_add_item,
            }),
            7 => CustomFieldSettings::Checkbox(ListSettings {
                items: untagged.items.unwrap_or_default(),
                allow_input: untagged.allow_input,
                allow_add_item: untagged.allow_add_item,
            }),
            8 => CustomFieldSettings::Radio(ListSettings {
                items: untagged.items.unwrap_or_default(),
                allow_input: untagged.allow_input,
                allow_add_item: untagged.allow_add_item,
            }),
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
}
