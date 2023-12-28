use serde::{Deserialize, Serialize};

use crate::planet::resource::Resource;
use crate::robot::robot_level::RobotLevel;

#[derive(Serialize,Deserialize,Debug, Clone)]
pub struct RobotLevels {
    pub health_level : RobotLevel,
    pub damage_level : RobotLevel,
    pub mining_level : RobotLevel,
    pub mining_speed_level : RobotLevel,
    pub energy_level: RobotLevel,
    pub energy_regen_level: RobotLevel,
    pub storage_level : RobotLevel
}

impl Default for RobotLevels{
    fn default() -> Self {
        RobotLevels {
            health_level : RobotLevel::LEVEL0,
            damage_level : RobotLevel::LEVEL0,
            mining_level : RobotLevel::LEVEL0,
            mining_speed_level : RobotLevel::LEVEL0,
            energy_level: RobotLevel::LEVEL0,
            energy_regen_level: RobotLevel::LEVEL0,
            storage_level : RobotLevel::LEVEL0
        }
    }

}

impl RobotLevels {

    pub fn get_cost_for_level(level : &RobotLevel) -> u32 {
        match level {
            RobotLevel::LEVEL0 => {0}
            RobotLevel::LEVEL1 => {50}
            RobotLevel::LEVEL2 => {300}
            RobotLevel::LEVEL3 => {1500}
            RobotLevel::LEVEL4 => {4000}
            RobotLevel::LEVEL5 => {15000}
        }
    }
    pub fn get_health_for_level(&self) -> u32 {
        match self.health_level {
            RobotLevel::LEVEL0 => {10}
            RobotLevel::LEVEL1 => {25}
            RobotLevel::LEVEL2 => {50}
            RobotLevel::LEVEL3 => {100}
            RobotLevel::LEVEL4 => {200}
            RobotLevel::LEVEL5 => {500}
        }
    }

    pub fn get_damage_for_level(&self) -> u32 {
        match self.damage_level {
            RobotLevel::LEVEL0 => {1}
            RobotLevel::LEVEL1 => {2}
            RobotLevel::LEVEL2 => {5}
            RobotLevel::LEVEL3 => {10}
            RobotLevel::LEVEL4 => {20}
            RobotLevel::LEVEL5 => {50}
        }
    }

    pub fn get_mining_amount_for_level(&self) -> u32 {
        match self.mining_speed_level {
            RobotLevel::LEVEL0 => {2}
            RobotLevel::LEVEL1 => {5}
            RobotLevel::LEVEL2 => {10}
            RobotLevel::LEVEL3 => {15}
            RobotLevel::LEVEL4 => {20}
            RobotLevel::LEVEL5 => {40}
        }
    }

    pub fn get_minable_resoures (&self) -> Vec<Resource> {
        let mut resources = Vec::with_capacity(Resource::variants().len());
        match self.mining_level {
            RobotLevel::LEVEL0 => resources.push(Resource::COAL),
            RobotLevel::LEVEL1 => {
                resources.push(Resource::COAL);
                resources.push(Resource::IRON);
            }
            RobotLevel::LEVEL2 => {
                resources.push(Resource::COAL);
                resources.push(Resource::IRON);
                resources.push(Resource::GEM);
            }
            RobotLevel::LEVEL3 => {
                resources.push(Resource::COAL);
                resources.push(Resource::IRON);
                resources.push(Resource::GEM);
                resources.push(Resource::GOLD);
            }
            RobotLevel::LEVEL4 => {
                resources.push(Resource::COAL);
                resources.push(Resource::IRON);
                resources.push(Resource::GEM);
                resources.push(Resource::GOLD);
                resources.push(Resource::PLATINUM);
            }
            RobotLevel::LEVEL5 => {
                resources.push(Resource::COAL);
                resources.push(Resource::IRON);
                resources.push(Resource::GEM);
                resources.push(Resource::GOLD);
                resources.push(Resource::PLATINUM);
            }

        }
        return resources
    }


    pub fn get_energy_for_level(&self) -> u32 {
        match self.energy_level {
            RobotLevel::LEVEL0 => {20}
            RobotLevel::LEVEL1 => {30}
            RobotLevel::LEVEL2 => {40}
            RobotLevel::LEVEL3 => {60}
            RobotLevel::LEVEL4 => {100}
            RobotLevel::LEVEL5 => {200}
        }
    }

    pub fn get_energy_regen_for_level(&self) -> u32 {
        match self.energy_regen_level {
            RobotLevel::LEVEL0 => {4}
            RobotLevel::LEVEL1 => {6}
            RobotLevel::LEVEL2 => {8}
            RobotLevel::LEVEL3 => {10}
            RobotLevel::LEVEL4 => {15}
            RobotLevel::LEVEL5 => {20}
        }
    }

    pub fn get_storage_for_level(&self) -> u32 {
        match self.storage_level {
            RobotLevel::LEVEL0 => {20}
            RobotLevel::LEVEL1 => {50}
            RobotLevel::LEVEL2 => {100}
            RobotLevel::LEVEL3 => {200}
            RobotLevel::LEVEL4 => {400}
            RobotLevel::LEVEL5 => {1000}
        }
    }
}