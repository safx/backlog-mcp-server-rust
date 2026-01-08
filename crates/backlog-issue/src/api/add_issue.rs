#[cfg(feature = "writable")]
use super::custom_field_utils::CustomFieldFormSerializer;
#[cfg(feature = "writable")]
use crate::models::{CustomFieldInput, Issue};
#[cfg(feature = "writable")]
use backlog_api_core::{Error as ApiError, HttpMethod, IntoRequest};
#[cfg(feature = "writable")]
use backlog_api_macros::ToFormParams;
#[cfg(feature = "writable")]
use backlog_core::identifier::{
    AttachmentId, CategoryId, CustomFieldId, IssueId, IssueTypeId, MilestoneId, PriorityId,
    ProjectId, UserId,
};
#[cfg(feature = "writable")]
use chrono::{DateTime, Utc};
#[cfg(feature = "writable")]
use derive_builder::Builder;
#[cfg(feature = "writable")]
use serde::Serialize;
#[cfg(feature = "writable")]
use std::collections::HashMap;

/// Response type for adding a new issue
#[cfg(feature = "writable")]
pub type AddIssueResponse = Issue;

#[cfg(feature = "writable")]
#[derive(Debug, Builder, ToFormParams)]
#[builder(build_fn(error = "ApiError"))]
pub struct AddIssueParams {
    #[builder(setter(into))]
    #[form(name = "projectId")]
    pub project_id: ProjectId,
    #[builder(setter(into))]
    pub summary: String,
    #[builder(default, setter(into, strip_option))]
    #[form(name = "parentIssueId")]
    pub parent_issue_id: Option<IssueId>,
    #[builder(default, setter(into, strip_option))]
    pub description: Option<String>,
    #[builder(default, setter(into, strip_option))]
    #[form(name = "startDate", date_format = "%Y-%m-%d")]
    pub start_date: Option<DateTime<Utc>>,
    #[builder(default, setter(into, strip_option))]
    #[form(name = "dueDate", date_format = "%Y-%m-%d")]
    pub due_date: Option<DateTime<Utc>>,
    #[builder(default, setter(into, strip_option))]
    #[form(name = "estimatedHours")]
    pub estimated_hours: Option<f32>,
    #[builder(default, setter(into, strip_option))]
    #[form(name = "actualHours")]
    pub actual_hours: Option<f32>,
    #[builder(setter(into))]
    #[form(name = "issueTypeId")]
    pub issue_type_id: IssueTypeId,
    #[builder(default, setter(into, strip_option))]
    #[form(array, name = "categoryId")]
    pub category_id: Option<Vec<CategoryId>>,
    #[builder(default, setter(into, strip_option))]
    #[form(array, name = "versionId")]
    pub version_id: Option<Vec<MilestoneId>>,
    #[builder(default, setter(into, strip_option))]
    #[form(array, name = "milestoneId")]
    pub milestone_id: Option<Vec<MilestoneId>>,
    #[builder(setter(into))]
    #[form(name = "priorityId")]
    pub priority_id: PriorityId,
    #[builder(default, setter(into, strip_option))]
    #[form(name = "assigneeId")]
    pub assignee_id: Option<UserId>,
    #[builder(default, setter(into, strip_option))]
    #[form(array, name = "notifyUserId")]
    pub notify_user_id: Option<Vec<UserId>>,
    #[builder(default, setter(into, strip_option))]
    #[form(array, name = "attachmentId")]
    pub attachment_id: Option<Vec<AttachmentId>>,
    #[builder(default, setter(custom))]
    #[form(skip)]
    pub custom_fields: Option<HashMap<CustomFieldId, CustomFieldInput>>,
}

#[cfg(feature = "writable")]
impl AddIssueParamsBuilder {
    /// Set custom fields for the issue
    pub fn custom_fields(
        &mut self,
        custom_fields: HashMap<CustomFieldId, CustomFieldInput>,
    ) -> &mut Self {
        self.custom_fields = Some(Some(custom_fields));
        self
    }

    /// Add a single custom field
    pub fn custom_field(&mut self, field_id: CustomFieldId, value: CustomFieldInput) -> &mut Self {
        let mut fields = self.custom_fields.clone().flatten().unwrap_or_default();
        fields.insert(field_id, value);
        self.custom_fields = Some(Some(fields));
        self
    }
}

#[cfg(feature = "writable")]
impl IntoRequest for AddIssueParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Post
    }

    fn path(&self) -> String {
        "/api/v2/issues".to_string()
    }

    fn to_form(&self) -> impl Serialize {
        let mut params: Vec<(String, String)> = self.into();
        self.custom_fields.serialize_custom_fields(&mut params);
        params
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use backlog_core::identifier::CustomFieldItemId;
    use chrono::{NaiveDate, TimeZone};

    // Helper function to get form params including custom fields
    fn get_form_params(params: &AddIssueParams) -> Vec<(String, String)> {
        // First get standard params
        let mut form_params: Vec<(String, String)> = params.into();

        // Then add custom fields manually
        if let Some(ref custom_fields) = params.custom_fields {
            for (field_id, input) in custom_fields {
                let (value, other_value) = input.to_form_value();

                match input {
                    CustomFieldInput::MultipleList { ids, .. } => {
                        for id in ids {
                            form_params.push((format!("customField_{field_id}"), id.to_string()));
                        }
                    }
                    CustomFieldInput::CheckBox(ids) => {
                        for id in ids {
                            form_params.push((format!("customField_{field_id}"), id.to_string()));
                        }
                    }
                    _ => {
                        form_params.push((format!("customField_{field_id}"), value));
                    }
                }

                if let Some(other) = other_value {
                    form_params.push((format!("customField_{field_id}_otherValue"), other));
                }
            }
        }

        form_params
    }

    #[test]
    fn test_datetime_formatting_with_macros() {
        let params = AddIssueParamsBuilder::default()
            .project_id(ProjectId::new(1))
            .summary("Test Issue".to_string())
            .issue_type_id(IssueTypeId::new(1))
            .priority_id(PriorityId::new(1))
            .start_date(
                chrono::Utc
                    .with_ymd_and_hms(2024, 6, 24, 12, 30, 45)
                    .unwrap(),
            )
            .due_date(
                chrono::Utc
                    .with_ymd_and_hms(2024, 12, 31, 23, 59, 59)
                    .unwrap(),
            )
            .build()
            .unwrap();

        let form_params: Vec<(String, String)> = (&params).into();

        // Check that dates are properly formatted
        let start_date_param = form_params.iter().find(|(key, _)| key == "startDate");
        assert!(start_date_param.is_some());
        assert_eq!(start_date_param.unwrap().1, "2024-06-24");

        let due_date_param = form_params.iter().find(|(key, _)| key == "dueDate");
        assert!(due_date_param.is_some());
        assert_eq!(due_date_param.unwrap().1, "2024-12-31");
    }

    #[test]
    fn test_custom_fields_text() {
        let mut builder = AddIssueParamsBuilder::default();
        builder
            .project_id(ProjectId::new(1))
            .summary("Test Issue".to_string())
            .issue_type_id(IssueTypeId::new(1))
            .priority_id(PriorityId::new(1))
            .custom_field(
                CustomFieldId::new(10),
                CustomFieldInput::Text("Sample text".to_string()),
            );

        let params = builder.build().unwrap();
        let form_params = get_form_params(&params);

        let custom_field = form_params.iter().find(|(key, _)| key == "customField_10");
        assert!(custom_field.is_some());
        assert_eq!(custom_field.unwrap().1, "Sample text");
    }

    #[test]
    fn test_custom_fields_date() {
        let mut builder = AddIssueParamsBuilder::default();
        builder
            .project_id(ProjectId::new(1))
            .summary("Test Issue".to_string())
            .issue_type_id(IssueTypeId::new(1))
            .priority_id(PriorityId::new(1))
            .custom_field(
                CustomFieldId::new(20),
                CustomFieldInput::Date(NaiveDate::from_ymd_opt(2024, 6, 24).unwrap()),
            );

        let params = builder.build().unwrap();
        let form_params = get_form_params(&params);

        let custom_field = form_params.iter().find(|(key, _)| key == "customField_20");
        assert!(custom_field.is_some());
        assert_eq!(custom_field.unwrap().1, "2024-06-24");
    }

    #[test]
    fn test_custom_fields_single_list_with_other() {
        let mut builder = AddIssueParamsBuilder::default();
        builder
            .project_id(ProjectId::new(1))
            .summary("Test Issue".to_string())
            .issue_type_id(IssueTypeId::new(1))
            .priority_id(PriorityId::new(1))
            .custom_field(
                CustomFieldId::new(30),
                CustomFieldInput::SingleList {
                    id: CustomFieldItemId::new(123),
                    other_value: Some("Other description".to_string()),
                },
            );

        let params = builder.build().unwrap();
        let form_params = get_form_params(&params);

        let custom_field = form_params.iter().find(|(key, _)| key == "customField_30");
        assert!(custom_field.is_some());
        assert_eq!(custom_field.unwrap().1, "123");

        let other_value = form_params
            .iter()
            .find(|(key, _)| key == "customField_30_otherValue");
        assert!(other_value.is_some());
        assert_eq!(other_value.unwrap().1, "Other description");
    }

    #[test]
    fn test_custom_fields_multiple_list() {
        let mut builder = AddIssueParamsBuilder::default();
        builder
            .project_id(ProjectId::new(1))
            .summary("Test Issue".to_string())
            .issue_type_id(IssueTypeId::new(1))
            .priority_id(PriorityId::new(1))
            .custom_field(
                CustomFieldId::new(40),
                CustomFieldInput::MultipleList {
                    ids: vec![
                        CustomFieldItemId::new(100),
                        CustomFieldItemId::new(200),
                        CustomFieldItemId::new(300),
                    ],
                    other_value: None,
                },
            );

        let params = builder.build().unwrap();
        let form_params = get_form_params(&params);

        // Multiple list should create separate parameters for each ID
        let custom_fields: Vec<_> = form_params
            .iter()
            .filter(|(key, _)| key == "customField_40")
            .collect();
        assert_eq!(custom_fields.len(), 3);
        assert!(custom_fields.iter().any(|(_, v)| v == "100"));
        assert!(custom_fields.iter().any(|(_, v)| v == "200"));
        assert!(custom_fields.iter().any(|(_, v)| v == "300"));
    }
}
