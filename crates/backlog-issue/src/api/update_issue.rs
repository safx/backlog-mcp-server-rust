#[cfg(feature = "writable")]
use super::custom_field_utils::CustomFieldFormSerializer;
#[cfg(feature = "writable")]
use crate::models::{CustomFieldInput, Issue};
#[cfg(feature = "writable")]
use backlog_api_core::{Error as ApiError, HttpMethod, IntoRequest};
#[cfg(feature = "writable")]
use backlog_api_macros::ToFormParams;
#[cfg(feature = "writable")]
use backlog_core::{
    ApiDate, IssueIdOrKey,
    identifier::{
        AttachmentId, CategoryId, CustomFieldId, IssueId, IssueTypeId, MilestoneId, PriorityId,
        ResolutionId, UserId,
    },
};
#[cfg(feature = "writable")]
use derive_builder::Builder;
#[cfg(feature = "writable")]
use serde::Serialize;
#[cfg(feature = "writable")]
use std::collections::HashMap;

/// Response type for updating an issue
#[cfg(feature = "writable")]
pub type UpdateIssueResponse = Issue;

#[cfg(feature = "writable")]
#[derive(Debug, Clone, Builder, ToFormParams)]
#[builder(setter(strip_option, into))]
#[builder(build_fn(error = "ApiError"))]
pub struct UpdateIssueParams {
    #[builder(setter(into))]
    #[form(skip)]
    pub issue_id_or_key: IssueIdOrKey,
    #[builder(default, setter(into, strip_option))]
    pub summary: Option<String>,
    #[builder(default, setter(into, strip_option))]
    #[form(name = "parentIssueId")]
    pub parent_issue_id: Option<IssueId>,
    #[builder(default, setter(into, strip_option))]
    pub description: Option<String>,
    #[builder(default, setter(into, strip_option))]
    #[form(name = "startDate")]
    pub start_date: Option<ApiDate>,
    #[builder(default, setter(into, strip_option))]
    #[form(name = "dueDate")]
    pub due_date: Option<ApiDate>,
    #[builder(default, setter(into, strip_option))]
    #[form(name = "estimatedHours")]
    pub estimated_hours: Option<f32>,
    #[builder(default, setter(into, strip_option))]
    #[form(name = "actualHours")]
    pub actual_hours: Option<f32>,
    #[builder(default, setter(into, strip_option))]
    #[form(name = "issueTypeId")]
    pub issue_type_id: Option<IssueTypeId>,
    #[builder(default, setter(into, strip_option))]
    #[form(array, name = "categoryId")]
    pub category_id: Option<Vec<CategoryId>>,
    #[builder(default, setter(into, strip_option))]
    #[form(array, name = "versionId")]
    pub version_id: Option<Vec<MilestoneId>>,
    #[builder(default, setter(into, strip_option))]
    #[form(array, name = "milestoneId")]
    pub milestone_id: Option<Vec<MilestoneId>>,
    #[builder(default, setter(into, strip_option))]
    #[form(name = "priorityId")]
    pub priority_id: Option<PriorityId>,
    #[builder(default, setter(into, strip_option))]
    #[form(name = "assigneeId")]
    pub assignee_id: Option<UserId>,
    #[builder(default, setter(into, strip_option))]
    #[form(array, name = "notifiedUserId")]
    pub notified_user_id: Option<Vec<UserId>>,
    #[builder(default, setter(into, strip_option))]
    #[form(array, name = "attachmentId")]
    pub attachment_id: Option<Vec<AttachmentId>>,
    #[builder(default, setter(into, strip_option))]
    #[form(name = "statusId")]
    pub status_id: Option<String>,
    #[builder(default, setter(into, strip_option))]
    #[form(name = "resolutionId")]
    pub resolution_id: Option<ResolutionId>,
    #[builder(default, setter(into, strip_option))]
    pub comment: Option<String>,
    #[builder(default, setter(custom))]
    #[form(skip)]
    pub custom_fields: Option<HashMap<CustomFieldId, CustomFieldInput>>,
}

#[cfg(feature = "writable")]
impl UpdateIssueParamsBuilder {
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
impl IntoRequest for UpdateIssueParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Patch
    }

    fn path(&self) -> String {
        format!("/api/v2/issues/{}", self.issue_id_or_key)
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
    use chrono::NaiveDate;

    // Helper function to get form params including custom fields
    fn get_form_params(params: &UpdateIssueParams) -> Vec<(String, String)> {
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
    fn test_update_custom_fields_text() {
        let mut builder = UpdateIssueParamsBuilder::default();
        builder.issue_id_or_key(IssueId::new(123)).custom_field(
            CustomFieldId::new(10),
            CustomFieldInput::Text("Updated text".to_string()),
        );

        let params = builder.build().unwrap();
        let form_params = get_form_params(&params);

        let custom_field = form_params.iter().find(|(key, _)| key == "customField_10");
        assert!(custom_field.is_some());
        assert_eq!(custom_field.unwrap().1, "Updated text");
    }

    #[test]
    fn test_update_custom_fields_date() {
        let mut builder = UpdateIssueParamsBuilder::default();
        builder.issue_id_or_key(IssueId::new(123)).custom_field(
            CustomFieldId::new(30),
            CustomFieldInput::Date(NaiveDate::from_ymd_opt(2024, 12, 31).unwrap()),
        );

        let params = builder.build().unwrap();
        let form_params = get_form_params(&params);

        let custom_field = form_params.iter().find(|(key, _)| key == "customField_30");
        assert!(custom_field.is_some());
        assert_eq!(custom_field.unwrap().1, "2024-12-31");
    }
}
