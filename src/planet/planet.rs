use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::planet::direction::Direction;
use crate::planet::resource::Resource;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Planet {
    pub planet_id: Uuid,
    pub movement_difficulty: u8,
    pub resources: Option<(Resource, u32)>,
    pub neighbours: HashMap<Direction, Uuid>,
}

impl Planet {
    pub fn new(planet_id: Uuid, movement_difficulty: u8) -> Planet {
        Planet {
            planet_id,
            movement_difficulty,
            resources: None,
            neighbours: HashMap::new(),
        }
    }

    pub fn get_planet_id(&self) -> Uuid {
        self.planet_id
    }

    pub fn get_movement_difficulty(&self) -> u8 {
        self.movement_difficulty
    }


    pub fn get_neighbours(&self) -> &HashMap<Direction, Uuid> {
        &self.neighbours
    }

    pub fn set_planet_id(&mut self, planet_id: Uuid) {
        self.planet_id = planet_id;
    }

    pub fn set_movement_difficulty(&mut self, movement_difficulty: u8) {
        self.movement_difficulty = movement_difficulty;
    }

    pub fn set_neighbours(&mut self, neighbours: HashMap<Direction, Uuid>) {
        self.neighbours = neighbours;
    }

    pub fn set_neighbour(&mut self, direction: Direction, neighbour: Uuid) {
        self.neighbours.insert(direction, neighbour);
    }
}

