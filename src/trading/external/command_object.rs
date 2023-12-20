use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
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
    pub item_quantity: Option<u16>,
}