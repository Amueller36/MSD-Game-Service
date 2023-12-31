use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use tracing::log::warn;
use uuid::Uuid;

use crate::planet::planet::Planet;
use crate::planet::resource::Resource;
use crate::robot::robot_levels::RobotLevels;

#[derive(Serialize,Deserialize,Debug, Clone)]
pub struct Robot {
    pub robot_id: Uuid,
    pub planet_id: Uuid,
    pub health: u32,
    pub energy: u32,
    pub levels: RobotLevels,
    pub inventory: HashMap<Resource,u32>
}

impl Robot {

    pub fn new (robot_id: Uuid, planet_id: Uuid) -> Robot {
        let levels = RobotLevels::default();
        Robot {
            robot_id,
            planet_id,
            health: levels.get_health_for_level(),
            energy: levels.get_energy_for_level(),
            levels,
            inventory: HashMap::new()
        }
    }
    pub fn is_alive(&self) -> bool {
        self.health > 0
    }
    pub fn is_inventory_full(&self) -> bool {
       self.get_free_storage_space() == 0
    }

    pub fn regenerate(&mut self) {
        if !self.is_alive() {
            warn!("Robot {} is dead and cannot regenerate", self.robot_id);
            return;
        }
        if self.energy + self.levels.get_energy_regen_for_level() > self.levels.get_energy_for_level() {
            self.energy = self.levels.get_energy_for_level();
        } else {
            self.energy += self.levels.get_energy_regen_for_level();
        }
    }

    pub fn get_inventory_value(&self) -> u32 {
        let mut inventory_value = 0;
        for (resource, amount) in &self.inventory {
            inventory_value += resource.get_selling_value() * amount;
        }
        inventory_value
    }

    pub fn get_free_storage_space(&self) -> u32 {
        let used_inventory_space = self.inventory.iter().fold(0, |acc, (_, amount)| acc + amount);
        self.levels.get_storage_for_level() - used_inventory_space
    }


    pub fn add_resource_to_inventory(&mut self, resource: &Resource, amount: &u32) {
        if let Some(current_amount) = self.inventory.get_mut(&resource) {
            *current_amount += *amount;
        } else {
            let resource = resource.clone();
            self.inventory.insert(resource, *amount);
        }
    }

    pub fn take_damage(&mut self, amount: u32){
        if self.health - amount > 0 {
            self.health -= amount;
        } else {
            self.health = 0;
        }
    }
}