use crate::error::{Error, Result};
use backlog_api_client::client::BacklogApiClient;
use backlog_core::{
    ProjectIdOrKey,
    identifier::{CustomFieldId, CustomFieldItemId, Identifier},
};
use backlog_domain_models::CustomFieldType;
use backlog_issue::models::CustomFieldInput;
use backlog_project::GetCustomFieldListParams;
use serde_json::Value;
use std::collections::HashMap;
use std::str::FromStr;

pub async fn resolve_custom_fields(
    client: &BacklogApiClient,
    project_id_or_key: &ProjectIdOrKey,
    fields_by_name: HashMap<String, Value>,
) -> Result<HashMap<CustomFieldId, CustomFieldInput>> {
    let params = GetCustomFieldListParams::new(project_id_or_key.clone());
    let custom_fields = client.project().get_custom_field_list(params).await?;

    let fields_by_name_map: HashMap<String, CustomFieldType> = custom_fields
        .into_iter()
        .map(|field| (field.name.clone(), field))
        .collect();

    let mut result = HashMap::new();
    for (field_name, value) in fields_by_name {
        match fields_by_name_map.get(&field_name) {
            Some(field_def) => {
                let input = convert_value_to_input(field_def, &value, &field_name)?;
                result.insert(field_def.id, input);
            }
            None => {
                return Err(Error::Server(format!(
                    "Custom field '{field_name}' not found in project"
                )));
            }
        }
    }

    Ok(result)
}

fn convert_value_to_input(
    field: &CustomFieldType,
    value: &Value,
    field_name: &str,
) -> Result<CustomFieldInput> {
    use backlog_domain_models::CustomFieldSettings;

    match &field.settings {
        CustomFieldSettings::Text => match value {
            Value::String(s) => Ok(CustomFieldInput::Text(s.clone())),
            _ => Err(Error::Server(format!(
                "Custom field '{field_name}' expects a string value"
            ))),
        },
        CustomFieldSettings::TextArea => match value {
            Value::String(s) => Ok(CustomFieldInput::TextArea(s.clone())),
            _ => Err(Error::Server(format!(
                "Custom field '{field_name}' expects a string value"
            ))),
        },
        CustomFieldSettings::Numeric(_) => match value {
            Value::Number(n) => {
                let float_value = n.as_f64().ok_or_else(|| {
                    Error::Server(format!(
                        "Custom field '{field_name}' expects a numeric value"
                    ))
                })?;
                Ok(CustomFieldInput::Numeric(float_value))
            }
            _ => Err(Error::Server(format!(
                "Custom field '{field_name}' expects a numeric value"
            ))),
        },
        CustomFieldSettings::Date(_) => match value {
            Value::String(s) => {
                let date = backlog_core::Date::from_str(s).map_err(|_| {
                    Error::Server(format!(
                        "Custom field '{field_name}' expects date in yyyy-MM-dd format"
                    ))
                })?;
                Ok(CustomFieldInput::Date(date.into()))
            }
            _ => Err(Error::Server(format!(
                "Custom field '{field_name}' expects a date string in yyyy-MM-dd format"
            ))),
        },
        CustomFieldSettings::SingleList(settings) => {
            let (item_name, other_value) = parse_single_list_value(value, field_name)?;

            let item = settings
                .items
                .iter()
                .find(|i| i.name == item_name)
                .ok_or_else(|| {
                    Error::Server(format!(
                        "Custom field '{}': option '{}' not found. Available options: {}",
                        field_name,
                        item_name,
                        settings
                            .items
                            .iter()
                            .map(|i| format!("'{}'", i.name))
                            .collect::<Vec<_>>()
                            .join(", ")
                    ))
                })?;

            Ok(CustomFieldInput::SingleList {
                id: CustomFieldItemId::new(item.id.value()),
                other_value,
            })
        }
        CustomFieldSettings::MultipleList(settings) => {
            let (item_names, other_value) = parse_multiple_list_value(value, field_name)?;

            let mut ids = Vec::new();
            for name in item_names {
                let item = settings
                    .items
                    .iter()
                    .find(|i| i.name == name)
                    .ok_or_else(|| {
                        Error::Server(format!(
                            "Custom field '{}': option '{}' not found. Available options: {}",
                            field_name,
                            name,
                            settings
                                .items
                                .iter()
                                .map(|i| format!("'{}'", i.name))
                                .collect::<Vec<_>>()
                                .join(", ")
                        ))
                    })?;
                ids.push(CustomFieldItemId::new(item.id.value()));
            }

            Ok(CustomFieldInput::MultipleList { ids, other_value })
        }
        CustomFieldSettings::Checkbox(settings) => match value {
            Value::Array(arr) => {
                let mut ids = Vec::new();
                for v in arr {
                    match v {
                        Value::String(name) => {
                            let item = settings
                                .items
                                .iter()
                                .find(|i| i.name == *name)
                                .ok_or_else(|| {
                                    Error::Server(format!(
                                        "Custom field '{}': option '{}' not found. Available options: {}",
                                        field_name,
                                        name,
                                        settings
                                            .items
                                            .iter()
                                            .map(|i| format!("'{}'", i.name))
                                            .collect::<Vec<_>>()
                                            .join(", ")
                                    ))
                                })?;
                            ids.push(CustomFieldItemId::new(item.id.value()));
                        }
                        _ => {
                            return Err(Error::Server(format!(
                                "Custom field '{field_name}' expects an array of strings"
                            )));
                        }
                    }
                }
                Ok(CustomFieldInput::CheckBox(ids))
            }
            _ => Err(Error::Server(format!(
                "Custom field '{field_name}' expects an array of strings"
            ))),
        },
        CustomFieldSettings::Radio(settings) => {
            match value {
                Value::String(name) => {
                    let item = settings.items.iter().find(|i| i.name == *name).ok_or_else(
                        || {
                            Error::Server(format!(
                                "Custom field '{}': option '{}' not found. Available options: {}",
                                field_name,
                                name,
                                settings
                                    .items
                                    .iter()
                                    .map(|i| format!("'{}'", i.name))
                                    .collect::<Vec<_>>()
                                    .join(", ")
                            ))
                        },
                    )?;
                    Ok(CustomFieldInput::Radio {
                        id: CustomFieldItemId::new(item.id.value()),
                        other_value: None,
                    })
                }
                _ => Err(Error::Server(format!(
                    "Custom field '{field_name}' expects a string value"
                ))),
            }
        }
    }
}

fn parse_single_list_value(value: &Value, field_name: &str) -> Result<(String, Option<String>)> {
    match value {
        Value::String(s) => Ok((s.clone(), None)),
        Value::Object(obj) => {
            let name = obj
                .get("name")
                .and_then(|v| v.as_str())
                .ok_or_else(|| {
                    Error::Server(format!(
                        "Custom field '{field_name}' object must have a 'name' field"
                    ))
                })?
                .to_string();
            let other = obj
                .get("other")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());
            Ok((name, other))
        }
        _ => Err(Error::Server(format!(
            "Custom field '{field_name}' expects a string or object with 'name' field"
        ))),
    }
}

fn parse_multiple_list_value(
    value: &Value,
    field_name: &str,
) -> Result<(Vec<String>, Option<String>)> {
    match value {
        Value::Array(arr) => {
            let names: Vec<String> = arr
                .iter()
                .map(|v| match v {
                    Value::String(s) => Ok(s.clone()),
                    _ => Err(Error::Server(format!(
                        "Custom field '{field_name}' array must contain strings"
                    ))),
                })
                .collect::<Result<Vec<_>>>()?;
            Ok((names, None))
        }
        Value::Object(obj) => {
            // Object format: { items: ["name1", "name2"], other: "other value" }
            let items = obj.get("items").and_then(|v| v.as_array()).ok_or_else(|| {
                Error::Server(format!(
                    "Custom field '{field_name}' object must have an 'items' array"
                ))
            })?;
            let names: Vec<String> = items
                .iter()
                .map(|v| match v {
                    Value::String(s) => Ok(s.clone()),
                    _ => Err(Error::Server(format!(
                        "Custom field '{field_name}' items array must contain strings"
                    ))),
                })
                .collect::<Result<Vec<_>>>()?;
            let other = obj
                .get("other")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());
            Ok((names, other))
        }
        _ => Err(Error::Server(format!(
            "Custom field '{field_name}' expects an array or object with 'items' array"
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_parse_single_list_value_string() {
        let value = json!("High");
        let (name, other) = parse_single_list_value(&value, "Priority").unwrap();
        assert_eq!(name, "High");
        assert_eq!(other, None);
    }

    #[test]
    fn test_parse_single_list_value_object() {
        let value = json!({"name": "Other", "other": "Custom value"});
        let (name, other) = parse_single_list_value(&value, "Priority").unwrap();
        assert_eq!(name, "Other");
        assert_eq!(other, Some("Custom value".to_string()));
    }

    #[test]
    fn test_parse_multiple_list_value_array() {
        let value = json!(["Backend", "Frontend"]);
        let (names, other) = parse_multiple_list_value(&value, "Tags").unwrap();
        assert_eq!(names, vec!["Backend", "Frontend"]);
        assert_eq!(other, None);
    }

    #[test]
    fn test_parse_multiple_list_value_object() {
        let value = json!({"items": ["Backend", "Frontend"], "other": "Custom tag"});
        let (names, other) = parse_multiple_list_value(&value, "Tags").unwrap();
        assert_eq!(names, vec!["Backend", "Frontend"]);
        assert_eq!(other, Some("Custom tag".to_string()));
    }
}
