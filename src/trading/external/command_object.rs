use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash)]
pub struct CommandObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub robot_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub planet_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_quantity: Option<u32>,
}