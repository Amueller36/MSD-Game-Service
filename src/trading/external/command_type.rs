use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Hash, PartialEq, Eq)]
pub enum CommandType {
    MOVEMENT,
    BATTLE,
    MINING,
    REGENERATE,
    BUYING,
    SELLING,
}
