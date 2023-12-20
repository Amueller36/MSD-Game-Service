use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::planet::planet::Planet;
use crate::planet::resource::Resource;
use crate::robot::robot_levels::RobotLevels;

#[derive(Serialize,Deserialize,Debug)]
pub struct Robot {
    pub robot_id: Uuid,
    pub planet_id: Uuid,
    pub health: u32,
    pub energy: u32,
    pub levels: RobotLevels,
    pub inventory: HashMap<Resource,u32>
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