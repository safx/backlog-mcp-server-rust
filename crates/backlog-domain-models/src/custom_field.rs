use backlog_core::{
    Date,
    identifier::{CustomFieldId, IssueTypeId, ProjectId},
};
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;

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

/// Deserialize an optional field from a borrowed JSON value.
///
/// Missing and `null` become `None`; a value of the wrong type is an error naming `field`.
fn optional_field<'a, T, E>(value: Option<&'a Value>, field: &str) -> Result<Option<T>, E>
where
    T: Deserialize<'a>,
    E: serde::de::Error,
{
    value
        .map(Option::<T>::deserialize)
        .transpose()
        .map(Option::flatten)
        .map_err(|e| E::custom(format!("invalid {field}: {e}")))
}

fn numeric_settings<E: serde::de::Error>(
    min: Option<&Value>,
    max: Option<&Value>,
    initial_value: Option<f64>,
    unit: Option<String>,
) -> Result<NumericSettings, E> {
    Ok(NumericSettings {
        min: optional_field(min, "min")?,
        max: optional_field(max, "max")?,
        initial_value,
        unit,
    })
}

fn date_settings<E: serde::de::Error>(
    min: Option<&Value>,
    max: Option<&Value>,
    initial_value_type: Option<InitialDate>,
    initial_shift: Option<i32>,
    initial_date: Option<&Value>,
) -> Result<DateSettings, E> {
    let (initial_value_type, initial_shift, initial_date) =
        normalize_initial_date(initial_value_type, initial_shift, initial_date)?;
    Ok(DateSettings {
        min: optional_field(min, "min")?,
        max: optional_field(max, "max")?,
        initial_value_type,
        initial_shift,
        initial_date,
    })
}

/// Initial value mode, shift days, and date of a date field.
type InitialDateParts = (Option<InitialDate>, Option<i32>, Option<Date>);

/// Normalize the initial value of a date field into its flat representation.
///
/// API responses carry it as an `initialDate` object `{id, shift, date}` (see backlog4j
/// `DateValueSetting`). The flat `initialValueType` / `initialShift` plus a date-string
/// `initialDate` is still accepted. When the object is present, all three values come
/// from it and the top-level mode and shift are not used as fallbacks.
fn normalize_initial_date<E: serde::de::Error>(
    initial_value_type: Option<InitialDate>,
    initial_shift: Option<i32>,
    initial_date: Option<&Value>,
) -> Result<InitialDateParts, E> {
    let Some(Value::Object(object)) = initial_date else {
        let date = optional_field(initial_date, "initialDate")?;
        return Ok((initial_value_type, initial_shift, date));
    };
    let id: i64 = optional_field(object.get("id"), "initialDate.id")?
        .ok_or_else(|| E::custom("invalid initialDate.id: missing"))?;
    let mode = InitialDate::from_api_value(id)
        .ok_or_else(|| E::custom(format!("invalid initialDate.id: unknown value {id}")))?;
    Ok((
        Some(mode),
        optional_field(object.get("shift"), "initialDate.shift")?,
        optional_field(object.get("date"), "initialDate.date")?,
    ))
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
    min: Option<Value>,
    max: Option<Value>,
    initial_value: Option<f64>,
    unit: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RawDateFieldType {
    #[serde(flatten)]
    base: RawCustomFieldBase,
    min: Option<Value>,
    max: Option<Value>,
    initial_value_type: Option<InitialDate>,
    initial_shift: Option<i32>,
    initial_date: Option<Value>,
}

impl RawDateFieldType {
    fn into_date_settings<'de, D>(self) -> Result<(RawCustomFieldBase, DateSettings), D::Error>
    where
        D: Deserializer<'de>,
    {
        let settings = date_settings::<D::Error>(
            self.min.as_ref(),
            self.max.as_ref(),
            self.initial_value_type,
            self.initial_shift,
            self.initial_date.as_ref(),
        )?;
        Ok((self.base, settings))
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
        date_settings::<D::Error>(
            self.min.as_ref(),
            self.max.as_ref(),
            self.initial_value_type,
            self.initial_shift,
            self.initial_date.as_ref(),
        )
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

/// Initial value mode of a date custom field.
///
/// Wire values follow the Backlog API (`initialValueType` / `initialDate.id`):
/// 1 = today, 2 = today + `initialShift` days, 3 = the specified `initialDate`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub enum InitialDate {
    #[serde(rename = "today")]
    Today,
    #[serde(rename = "shifted")]
    Shifted,
    #[serde(rename = "specified")]
    Specified,
}

impl InitialDate {
    fn from_api_value(value: i64) -> Option<Self> {
        match value {
            1 => Some(Self::Today),
            2 => Some(Self::Shifted),
            3 => Some(Self::Specified),
            _ => None,
        }
    }
}

impl<'de> Deserialize<'de> for InitialDate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum InitialDateHelper {
            Integer(i64),
            String(String),
        }

        match InitialDateHelper::deserialize(deserializer)? {
            InitialDateHelper::Integer(i) => InitialDate::from_api_value(i)
                .ok_or_else(|| serde::de::Error::custom(format!("Unknown InitialDate value: {i}"))),
            InitialDateHelper::String(s) => match s.as_str() {
                "today" => Ok(InitialDate::Today),
                "shifted" => Ok(InitialDate::Shifted),
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

        // Otherwise, try the tagged format (string typeId). Anything with an integer
        // typeId returned above, so an untagged fallback here could never succeed.
        let tagged: RawTaggedCustomFieldType =
            serde_json::from_value(value).map_err(serde::de::Error::custom)?;
        CustomFieldType::from_tagged::<D>(tagged)
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
            RawTaggedCustomFieldType::Numeric(raw) => {
                let settings = numeric_settings::<D::Error>(
                    raw.min.as_ref(),
                    raw.max.as_ref(),
                    raw.initial_value,
                    raw.unit,
                )?;
                (raw.base, CustomFieldSettings::Numeric(settings))
            }
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
            3 => CustomFieldSettings::Numeric(numeric_settings::<D::Error>(
                untagged.min.as_ref(),
                untagged.max.as_ref(),
                untagged.initial_value,
                untagged.unit.clone(),
            )?),
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
    use serde_json::json;

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
    fn test_initial_date_deserialize_valid() {
        for (json, expected) in [
            ("1", InitialDate::Today),
            ("2", InitialDate::Shifted),
            ("3", InitialDate::Specified),
            ("\"today\"", InitialDate::Today),
            ("\"shifted\"", InitialDate::Shifted),
            ("\"specified\"", InitialDate::Specified),
        ] {
            let result: InitialDate =
                serde_json::from_str(json).unwrap_or_else(|e| panic!("{json}: {e}"));
            assert_eq!(result, expected, "{json}");
        }
    }

    #[test]
    fn test_initial_date_deserialize_invalid() {
        for json in [
            "0",
            "4",
            "99",
            "-1",
            "\"2\"",
            "\"tomorrow\"",
            "\"yesterday\"",
            "\"Today\"",
            "\"invalid\"",
            "\"\"",
        ] {
            assert!(
                serde_json::from_str::<InitialDate>(json).is_err(),
                "{json} should be invalid"
            );
        }
    }

    #[test]
    fn test_initial_date_serialize_roundtrip() {
        for (value, json) in [
            (InitialDate::Today, "\"today\""),
            (InitialDate::Shifted, "\"shifted\""),
            (InitialDate::Specified, "\"specified\""),
        ] {
            assert_eq!(serde_json::to_string(&value).expect("serialize"), json);
            let restored: InitialDate = serde_json::from_str(json).expect("deserialize");
            assert_eq!(restored, value);
        }
    }

    // ============================================
    // Numeric / Date settings validation (both typeId forms)
    // ============================================

    /// Parse the same custom field with an integer typeId and with a string typeId.
    fn parse_both_type_id_forms(
        type_id: u8,
        extra: &serde_json::Value,
    ) -> [Result<CustomFieldType, serde_json::Error>; 2] {
        [json!(type_id), json!(type_id.to_string())].map(|type_id| {
            let mut field = json!({
                "id": 4,
                "projectId": 100,
                "typeId": type_id,
                "name": "Field",
                "description": "",
                "required": false,
                "applicableIssueTypes": null,
                "displayOrder": 0
            });
            let object = field.as_object_mut().expect("object");
            for (key, value) in extra.as_object().expect("extra must be an object") {
                object.insert(key.clone(), value.clone());
            }
            serde_json::from_value(field)
        })
    }

    fn parse_settings(type_id: u8, extra: serde_json::Value) -> CustomFieldSettings {
        let [integer, string] = parse_both_type_id_forms(type_id, &extra);
        let integer = integer.unwrap_or_else(|e| panic!("{extra} (integer typeId): {e}"));
        let string = string.unwrap_or_else(|e| panic!("{extra} (string typeId): {e}"));
        assert_eq!(integer, string, "{extra}: typeId forms disagree");
        integer.settings
    }

    fn assert_rejected(type_id: u8, extra: serde_json::Value, field: &str) {
        for result in parse_both_type_id_forms(type_id, &extra) {
            let error = match result {
                Ok(parsed) => panic!("{extra} should be rejected, got {parsed:?}"),
                Err(error) => error.to_string(),
            };
            assert!(
                error.contains(field),
                "{extra}: error {error:?} should mention {field:?}"
            );
        }
    }

    fn date(s: &str) -> Option<Date> {
        Some(s.parse().expect("valid date"))
    }

    #[test]
    fn test_numeric_min_max_validation() {
        for (extra, min, max) in [
            (json!({}), None, None),
            (json!({"min": null, "max": null}), None, None),
            (json!({"min": 0, "max": 100.5}), Some(0.0), Some(100.5)),
        ] {
            let CustomFieldSettings::Numeric(settings) = parse_settings(3, extra.clone()) else {
                panic!("{extra}: expected Numeric settings");
            };
            assert_eq!((settings.min, settings.max), (min, max), "{extra}");
        }

        for field in ["min", "max"] {
            for value in [
                json!("not-a-number"),
                json!("1"),
                json!(true),
                json!([]),
                json!({}),
            ] {
                assert_rejected(3, json!({ field: value }), field);
            }
        }
    }

    #[test]
    fn test_date_min_max_validation() {
        let CustomFieldSettings::Date(settings) = parse_settings(
            4,
            json!({"min": "2025-01-01", "max": "2025-12-31T00:00:00Z"}),
        ) else {
            panic!("expected Date settings");
        };
        assert_eq!(settings.min, date("2025-01-01"));
        assert_eq!(settings.max, date("2025-12-31"));

        let CustomFieldSettings::Date(settings) = parse_settings(4, json!({"min": null})) else {
            panic!("expected Date settings");
        };
        assert_eq!((settings.min, settings.max), (None, None));

        for field in ["min", "max"] {
            for value in [
                json!(12345),
                json!(false),
                json!([]),
                json!({}),
                json!("not-a-date"),
            ] {
                assert_rejected(4, json!({ field: value }), field);
            }
        }
    }

    #[test]
    fn test_date_initial_value_normalization() {
        use InitialDate::{Shifted, Specified, Today};
        let cases = [
            // initialDate object (API response form)
            (
                json!({"initialDate": {"id": 1, "date": "2025-06-15T00:00:00Z"}}),
                (Some(Today), None, date("2025-06-15")),
            ),
            (
                json!({"initialDate": {"id": 2, "shift": -7}}),
                (Some(Shifted), Some(-7), None),
            ),
            (
                json!({"initialDate": {"id": 2, "shift": 0}}),
                (Some(Shifted), Some(0), None),
            ),
            (
                json!({"initialDate": {"id": 2, "shift": 7, "date": null}}),
                (Some(Shifted), Some(7), None),
            ),
            (
                json!({"initialDate": {"id": 3, "shift": null, "date": "2025-06-15", "extra": 1}}),
                (Some(Specified), None, date("2025-06-15")),
            ),
            // The object wins as a whole; top-level values are not used as fallbacks
            (
                json!({"initialValueType": 1, "initialShift": 5, "initialDate": {"id": 2}}),
                (Some(Shifted), None, None),
            ),
            (
                json!({
                    "initialValueType": 2,
                    "initialShift": 5,
                    "initialDate": {"id": 3, "date": "2025-06-15"}
                }),
                (Some(Specified), None, date("2025-06-15")),
            ),
            // Flat form (date string + top-level mode and shift)
            (
                json!({"initialValueType": 2, "initialShift": 7, "initialDate": "2025-06-15"}),
                (Some(Shifted), Some(7), date("2025-06-15")),
            ),
            (
                json!({"initialValueType": "shifted", "initialShift": 7}),
                (Some(Shifted), Some(7), None),
            ),
            (
                json!({"initialValueType": 3, "initialDate": null}),
                (Some(Specified), None, None),
            ),
            (json!({}), (None, None, None)),
        ];

        for (extra, expected) in cases {
            let CustomFieldSettings::Date(settings) = parse_settings(4, extra.clone()) else {
                panic!("{extra}: expected Date settings");
            };
            let actual = (
                settings.initial_value_type,
                settings.initial_shift,
                settings.initial_date,
            );
            assert_eq!(actual, expected, "{extra}");
        }
    }

    #[test]
    fn test_date_initial_value_rejections() {
        let cases = [
            (json!({"initialDate": 20250615}), "initialDate"),
            (json!({"initialDate": true}), "initialDate"),
            (json!({"initialDate": []}), "initialDate"),
            (json!({"initialDate": "not-a-date"}), "initialDate"),
            (json!({"initialDate": {}}), "initialDate.id"),
            (json!({"initialDate": {"id": null}}), "initialDate.id"),
            (json!({"initialDate": {"id": "2"}}), "initialDate.id"),
            (json!({"initialDate": {"id": "today"}}), "initialDate.id"),
            (json!({"initialDate": {"id": 2.0}}), "initialDate.id"),
            (json!({"initialDate": {"id": 0}}), "initialDate.id"),
            (json!({"initialDate": {"id": 4}}), "initialDate.id"),
            (
                json!({"initialDate": {"id": 2, "shift": "7"}}),
                "initialDate.shift",
            ),
            (
                json!({"initialDate": {"id": 2, "shift": 1.5}}),
                "initialDate.shift",
            ),
            (
                json!({"initialDate": {"id": 2, "shift": 4294967296_i64}}),
                "initialDate.shift",
            ),
            (
                json!({"initialDate": {"id": 3, "date": 20250615}}),
                "initialDate.date",
            ),
            (
                json!({"initialDate": {"id": 3, "date": {}}}),
                "initialDate.date",
            ),
            (
                json!({"initialDate": {"id": 3, "date": "not-a-date"}}),
                "initialDate.date",
            ),
            // Invalid top-level values are rejected even when a valid object is present
            (
                json!({"initialValueType": 4, "initialDate": {"id": 1}}),
                "InitialDate",
            ),
            (json!({"initialValueType": "tomorrow"}), "InitialDate"),
            (json!({"initialShift": "7", "initialDate": {"id": 1}}), ""),
        ];

        for (extra, field) in cases {
            assert_rejected(4, extra, field);
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
