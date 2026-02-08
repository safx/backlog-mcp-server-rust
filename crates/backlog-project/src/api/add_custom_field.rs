#[cfg(feature = "writable")]
use backlog_api_core::IntoRequest;
#[cfg(feature = "writable")]
use backlog_api_macros::ToFormParams;
#[cfg(feature = "writable")]
use backlog_core::{Date, ProjectIdOrKey, identifier::IssueTypeId};
#[cfg(feature = "writable")]
use backlog_domain_models::CustomFieldType;
#[cfg(feature = "writable")]
use backlog_issue::CustomFieldTypeId;

/// Response type for adding a custom field
#[cfg(feature = "writable")]
pub type AddCustomFieldResponse = CustomFieldType;

/// Parameters for adding a custom field to a project.
///
/// Corresponds to `POST /api/v2/projects/:projectIdOrKey/customFields`.
#[cfg(feature = "writable")]
#[derive(Debug, Clone, ToFormParams)]
pub struct AddCustomFieldParams {
    #[form(skip)]
    pub project_id_or_key: ProjectIdOrKey,

    // Required parameters - Don't use macro for type_id, handle manually
    #[form(skip)]
    pub type_id: CustomFieldTypeId,
    pub name: String,

    // Optional common parameters
    #[form(array, name = "applicableIssueTypes")]
    pub applicable_issue_types: Option<Vec<IssueTypeId>>,
    pub description: Option<String>,
    pub required: Option<bool>,

    // Numeric field specific parameters
    pub min: Option<f64>,
    pub max: Option<f64>,
    #[form(name = "initialValue")]
    pub initial_value: Option<f64>,
    pub unit: Option<String>,

    // Date field specific parameters
    #[form(name = "min")]
    pub min_date: Option<Date>,
    #[form(name = "max")]
    pub max_date: Option<Date>,
    #[form(name = "initialValueType")]
    pub initial_value_type: Option<i32>,
    #[form(name = "initialDate")]
    pub initial_date: Option<Date>,
    #[form(name = "initialShift")]
    pub initial_shift: Option<i32>,

    // List field specific parameters
    #[form(array, name = "items")]
    pub items: Option<Vec<String>>,
    #[form(name = "allowInput")]
    pub allow_input: Option<bool>,
    #[form(name = "allowAddItem")]
    pub allow_add_item: Option<bool>,
}

#[cfg(feature = "writable")]
impl AddCustomFieldParams {
    /// Creates parameters for a text custom field.
    pub fn text(project_id_or_key: impl Into<ProjectIdOrKey>, name: impl Into<String>) -> Self {
        Self {
            project_id_or_key: project_id_or_key.into(),
            type_id: CustomFieldTypeId::Text,
            name: name.into(),
            applicable_issue_types: None,
            description: None,
            required: None,
            min: None,
            max: None,
            initial_value: None,
            unit: None,
            min_date: None,
            max_date: None,
            initial_value_type: None,
            initial_date: None,
            initial_shift: None,
            items: None,
            allow_input: None,
            allow_add_item: None,
        }
    }

    /// Creates parameters for a textarea custom field.
    pub fn textarea(project_id_or_key: impl Into<ProjectIdOrKey>, name: impl Into<String>) -> Self {
        Self {
            project_id_or_key: project_id_or_key.into(),
            type_id: CustomFieldTypeId::TextArea,
            name: name.into(),
            applicable_issue_types: None,
            description: None,
            required: None,
            min: None,
            max: None,
            initial_value: None,
            unit: None,
            min_date: None,
            max_date: None,
            initial_value_type: None,
            initial_date: None,
            initial_shift: None,
            items: None,
            allow_input: None,
            allow_add_item: None,
        }
    }

    /// Creates parameters for a numeric custom field.
    pub fn numeric(project_id_or_key: impl Into<ProjectIdOrKey>, name: impl Into<String>) -> Self {
        Self {
            project_id_or_key: project_id_or_key.into(),
            type_id: CustomFieldTypeId::Numeric,
            name: name.into(),
            applicable_issue_types: None,
            description: None,
            required: None,
            min: None,
            max: None,
            initial_value: None,
            unit: None,
            min_date: None,
            max_date: None,
            initial_value_type: None,
            initial_date: None,
            initial_shift: None,
            items: None,
            allow_input: None,
            allow_add_item: None,
        }
    }

    /// Creates parameters for a date custom field.
    pub fn date(project_id_or_key: impl Into<ProjectIdOrKey>, name: impl Into<String>) -> Self {
        Self {
            project_id_or_key: project_id_or_key.into(),
            type_id: CustomFieldTypeId::Date,
            name: name.into(),
            applicable_issue_types: None,
            description: None,
            required: None,
            min: None,
            max: None,
            initial_value: None,
            unit: None,
            min_date: None,
            max_date: None,
            initial_value_type: None,
            initial_date: None,
            initial_shift: None,
            items: None,
            allow_input: None,
            allow_add_item: None,
        }
    }

    /// Creates parameters for a single-select list custom field.
    pub fn single_list(
        project_id_or_key: impl Into<ProjectIdOrKey>,
        name: impl Into<String>,
        items: Vec<String>,
    ) -> Self {
        Self {
            project_id_or_key: project_id_or_key.into(),
            type_id: CustomFieldTypeId::SingleList,
            name: name.into(),
            applicable_issue_types: None,
            description: None,
            required: None,
            min: None,
            max: None,
            initial_value: None,
            unit: None,
            min_date: None,
            max_date: None,
            initial_value_type: None,
            initial_date: None,
            initial_shift: None,
            items: Some(items),
            allow_input: None,
            allow_add_item: None,
        }
    }

    /// Creates parameters for a multiple-select list custom field.
    pub fn multiple_list(
        project_id_or_key: impl Into<ProjectIdOrKey>,
        name: impl Into<String>,
        items: Vec<String>,
    ) -> Self {
        Self {
            project_id_or_key: project_id_or_key.into(),
            type_id: CustomFieldTypeId::MultipleList,
            name: name.into(),
            applicable_issue_types: None,
            description: None,
            required: None,
            min: None,
            max: None,
            initial_value: None,
            unit: None,
            min_date: None,
            max_date: None,
            initial_value_type: None,
            initial_date: None,
            initial_shift: None,
            items: Some(items),
            allow_input: None,
            allow_add_item: None,
        }
    }

    /// Creates parameters for a checkbox custom field.
    pub fn checkbox(project_id_or_key: impl Into<ProjectIdOrKey>, name: impl Into<String>) -> Self {
        Self {
            project_id_or_key: project_id_or_key.into(),
            type_id: CustomFieldTypeId::CheckBox,
            name: name.into(),
            applicable_issue_types: None,
            description: None,
            required: None,
            min: None,
            max: None,
            initial_value: None,
            unit: None,
            min_date: None,
            max_date: None,
            initial_value_type: None,
            initial_date: None,
            initial_shift: None,
            items: None,
            allow_input: None,
            allow_add_item: None,
        }
    }

    /// Creates parameters for a radio button custom field.
    pub fn radio(project_id_or_key: impl Into<ProjectIdOrKey>, name: impl Into<String>) -> Self {
        Self {
            project_id_or_key: project_id_or_key.into(),
            type_id: CustomFieldTypeId::Radio,
            name: name.into(),
            applicable_issue_types: None,
            description: None,
            required: None,
            min: None,
            max: None,
            initial_value: None,
            unit: None,
            min_date: None,
            max_date: None,
            initial_value_type: None,
            initial_date: None,
            initial_shift: None,
            items: None,
            allow_input: None,
            allow_add_item: None,
        }
    }

    /// Sets the description of the custom field.
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Sets whether the field is required.
    pub fn with_required(mut self, required: bool) -> Self {
        self.required = Some(required);
        self
    }

    /// Sets the applicable issue types.
    pub fn with_applicable_issue_types(mut self, issue_types: Vec<IssueTypeId>) -> Self {
        self.applicable_issue_types = Some(issue_types);
        self
    }

    /// Sets numeric field parameters.
    pub fn with_numeric_settings(
        mut self,
        min: Option<f64>,
        max: Option<f64>,
        initial_value: Option<f64>,
        unit: Option<String>,
    ) -> Self {
        self.min = min;
        self.max = max;
        self.initial_value = initial_value;
        self.unit = unit;
        self
    }

    /// Sets date field parameters.
    pub fn with_date_settings(
        mut self,
        min: Option<Date>,
        max: Option<Date>,
        initial_value_type: Option<i32>,
        initial_date: Option<Date>,
        initial_shift: Option<i32>,
    ) -> Self {
        self.min_date = min;
        self.max_date = max;
        self.initial_value_type = initial_value_type;
        self.initial_date = initial_date;
        self.initial_shift = initial_shift;
        self
    }

    /// Sets whether to allow adding new items for list fields.
    pub fn with_allow_add_item(mut self, allow: bool) -> Self {
        self.allow_add_item = Some(allow);
        self
    }

    /// Sets whether to allow direct input for list fields.
    pub fn with_allow_input(mut self, allow: bool) -> Self {
        self.allow_input = Some(allow);
        self
    }
}

#[cfg(feature = "writable")]
impl IntoRequest for AddCustomFieldParams {
    fn path(&self) -> String {
        format!("/api/v2/projects/{}/customFields", self.project_id_or_key)
    }

    fn method(&self) -> backlog_api_core::HttpMethod {
        backlog_api_core::HttpMethod::Post
    }

    fn to_form(&self) -> impl serde::Serialize {
        let mut form_params = Vec::<(String, String)>::from(self);
        // Manually add typeId as numeric value
        form_params.insert(0, ("typeId".to_string(), (self.type_id as i8).to_string()));
        form_params
    }
}
