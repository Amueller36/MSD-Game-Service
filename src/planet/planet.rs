use crate::planet::resource::Resource;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use crate::planet::direction::Direction;

#[derive(Serialize, Deserialize, Debug)]
pub struct Planet {
    planet_id: Uuid,
    movement_difficulty: u8,
    resources: HashMap<Resource, u32>,
    neighbours: HashMap<Direction, Planet>,
}
