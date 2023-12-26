use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Hash, PartialEq, Eq, Copy)]
pub enum CommandType {
    MOVEMENT,
    BATTLE,
    MINING,
    REGENERATE,
    BUYING,
    SELLING,
}
