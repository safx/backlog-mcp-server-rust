use backlog_core::identifier::CustomListItemId;
use serde::{Deserialize, Serialize};

#[cfg(feature = "schemars")]
use schemars::JsonSchema;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct ListItem {
    pub id: CustomListItemId,
    pub name: String,
    pub display_order: i32,
}
