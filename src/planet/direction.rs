use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Hash, Clone)]
pub enum Direction {
    NORTH,
    EAST,
    SOUTH,
    WEST,
}

impl Direction {
    pub fn get_opposite(&self) -> Direction {
        match self {
            Direction::NORTH => Direction::SOUTH,
            Direction::EAST => Direction::WEST,
            Direction::SOUTH => Direction::NORTH,
            Direction::WEST => Direction::EAST,
        }
    }

    pub fn variants() -> Vec<Direction> {
        vec![
            Direction::NORTH,
            Direction::EAST,
            Direction::SOUTH,
            Direction::WEST,
        ]
    }

    pub fn get_direction_from_string(string: &str) -> Option<Direction> {
        match string {
            "NORTH" => Some(Direction::NORTH),
            "EAST" => Some(Direction::EAST),
            "SOUTH" => Some(Direction::SOUTH),
            "WEST" => Some(Direction::WEST),
            _ => None,
        }
    }
}