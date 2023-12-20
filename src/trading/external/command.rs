use serde::{Deserialize, Serialize};

use crate::trading::external::command_object::CommandObject;
use crate::trading::external::command_type::CommandType;

#[derive(Serialize, Deserialize, Debug)]
struct Command {
    player_id: String,
    command_type: CommandType,
    command_object: CommandObject,
}