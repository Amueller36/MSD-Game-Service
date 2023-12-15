use serde::{Deserialize, Serialize};
use crate::planet::resource::Resource;
use crate::robot::robot_level::RobotLevel;
use crate::robot::robot_level::RobotLevel::LEVEL1;

#[derive(Serialize,Deserialize,Debug)]
pub struct RobotLevels {
    health_level : RobotLevel,
    damage_level : RobotLevel,
    mining_level : RobotLevel,
    mining_speed_level : RobotLevel,
    energy_level: RobotLevel,
    energy_regen_level: RobotLevel,
    storage_level : RobotLevel
}

impl RobotLevels {
    fn get_health_for_level(&self) -> u32 {
        match self.health_level {
            RobotLevel::LEVEL0 => {0}
            RobotLevel::LEVEL1 => {2}
            RobotLevel::LEVEL2 => {4}
            RobotLevel::LEVEL3 => {6}
            RobotLevel::LEVEL4 => {9}
            RobotLevel::LEVEL5 => {10}
        }
    }

    fn get_damage_for_level(&self) -> u32 {
        match self.damage_level {
            RobotLevel::LEVEL0 => {0}
            RobotLevel::LEVEL1 => {2}
            RobotLevel::LEVEL2 => {4}
            RobotLevel::LEVEL3 => {6}
            RobotLevel::LEVEL4 => {9}
            RobotLevel::LEVEL5 => {10}
        }
    }

    fn get_mining_amount_for_level(&self) -> u32 {
        match self.mining_level {
            RobotLevel::LEVEL0 => {0}
            RobotLevel::LEVEL1 => {2}
            RobotLevel::LEVEL2 => {4}
            RobotLevel::LEVEL3 => {6}
            RobotLevel::LEVEL4 => {9}
            RobotLevel::LEVEL5 => {10}
        }
    }

    fn get_minable_resoures (&self) -> Vec<Resource> {
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

    fn get_mining_speed_for_level(&self) -> u32 {
        match self.mining_speed_level {
            RobotLevel::LEVEL0 => {0}
            RobotLevel::LEVEL1 => {2}
            RobotLevel::LEVEL2 => {4}
            RobotLevel::LEVEL3 => {6}
            RobotLevel::LEVEL4 => {9}
            RobotLevel::LEVEL5 => {10}
        }
    }

    fn get_energy_for_level(&self) -> u32 {
        match self.energy_level {
            RobotLevel::LEVEL0 => {0}
            RobotLevel::LEVEL1 => {2}
            RobotLevel::LEVEL2 => {4}
            RobotLevel::LEVEL3 => {6}
            RobotLevel::LEVEL4 => {9}
            RobotLevel::LEVEL5 => {10}
        }
    }

    fn get_energy_regen_for_level(&self) -> u32 {
        match self.energy_regen_level {
            RobotLevel::LEVEL0 => {0}
            RobotLevel::LEVEL1 => {2}
            RobotLevel::LEVEL2 => {4}
            RobotLevel::LEVEL3 => {6}
            RobotLevel::LEVEL4 => {9}
            RobotLevel::LEVEL5 => {10}
        }
    }

    fn get_storage_for_level(&self) -> u32 {
        match self.storage_level {
            RobotLevel::LEVEL0 => {0}
            RobotLevel::LEVEL1 => {2}
            RobotLevel::LEVEL2 => {4}
            RobotLevel::LEVEL3 => {6}
            RobotLevel::LEVEL4 => {9}
            RobotLevel::LEVEL5 => {10}
        }
    }
}