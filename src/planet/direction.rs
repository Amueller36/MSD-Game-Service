use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Direction {
    NORTH,
    EAST,
    SOUTH,
    WEST,
}
