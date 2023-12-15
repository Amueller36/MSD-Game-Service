use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum CommandType {
    MOVEMENT,
    BATTLE,
    MINING,
    REGENERATE,
    BUYING,
    SELLING,
}
