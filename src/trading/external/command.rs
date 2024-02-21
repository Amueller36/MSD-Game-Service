use serde::{Deserialize, Serialize};

use crate::trading::external::command_object::CommandObject;
use crate::trading::external::command_type::CommandType;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, Hash)]
pub struct Command {
    pub player_name: String,
    pub game_id: String,
    pub command_type: CommandType,
    pub command_object: CommandObject,
}

impl PartialEq for Command {
    fn eq(&self, other: &Self) -> bool {
        // Beispiel: Vergleichen Sie nur player_name und game_id für Gleichheit
        self.player_name == other.player_name && self.game_id == other.game_id && self.command_type == other.command_type && self.command_object == other.command_object
        // Fügen Sie weitere Vergleiche hinzu, wenn andere Felder ebenfalls berücksichtigt werden sollen
    }
}