use serde_repr::{Deserialize_repr, Serialize_repr};
use std::fmt;

#[cfg(feature = "schemars")]
use schemars::JsonSchema;

#[repr(i8)]
#[derive(Eq, PartialEq, Debug, Clone, Copy, Hash, Serialize_repr, Deserialize_repr)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub enum CustomFieldTypeId {
    Text = 1,
    TextArea = 2,
    Numeric = 3,
    Date = 4,
    SingleList = 5,
    MultipleList = 6,
    CheckBox = 7,
    Radio = 8,
}

impl fmt::Display for CustomFieldTypeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            CustomFieldTypeId::Text => "text",
            CustomFieldTypeId::TextArea => "textarea",
            CustomFieldTypeId::Numeric => "number",
            CustomFieldTypeId::Date => "date",
            CustomFieldTypeId::SingleList => "single_list",
            CustomFieldTypeId::MultipleList => "multiple_list",
            CustomFieldTypeId::CheckBox => "checkbox",
            CustomFieldTypeId::Radio => "radio",
        };
        write!(f, "{s}")
    }
}
