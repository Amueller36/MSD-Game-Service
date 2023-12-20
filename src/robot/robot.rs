use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::planet::planet::Planet;
use crate::planet::resource::Resource;
use crate::robot::robot_levels::RobotLevels;

#[derive(Serialize,Deserialize,Debug)]
pub struct Robot {
    robot_id: Uuid,
    health: u32,
    energy: u32,
    levels: RobotLevels,
    planet : Planet,
    inventory: HashMap<Resource,u32>
}

impl Robot {
    fn is_inventory_full(&self) -> bool {
        todo!()
    }

    fn move_to_planet(&mut self, planet: Planet) {
        todo!()

    }

    fn add_resource_to_inventory(&mut self, resource: Resource, amount: u32) {
        todo!()
    }

    fn take_damage(&mut self, amount: u32){
        todo!()
    }
}