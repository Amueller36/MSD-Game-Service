use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::trading::external::command_object::CommandObject;
use crate::trading::external::command_type::CommandType;

#[derive(Serialize, Deserialize, Debug)]
pub struct Command {
    pub player_name: String,
    pub game_id: String,
    pub command_type: CommandType,
    pub command_object: CommandObject,
}