use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CommandObject {
    #[serde(skip_serializing_if = "Option::is_none", alias = "robotId")]
    pub robot_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "planetId")]
    pub planet_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "targetId")]
    pub target_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "itemName")]
    pub item_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "itemQuantity")]
    pub item_quantity: Option<u16>,
}