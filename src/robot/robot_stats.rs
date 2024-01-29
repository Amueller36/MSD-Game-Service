use serde::{Deserialize, Serialize};
use crate::planet::resource::Resource;
use crate::robot::robot_levels::RobotLevels;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RobotStats {
    pub damage: u32,
    pub max_health: u32,
    pub max_energy: u32,
    pub energy_regen: u32,
    pub mining_speed: u32,
    pub max_storage: u32,
    pub mineable_resources: Vec<Resource>,
}

impl RobotStats {
    pub fn from_robot_levels(levels: &RobotLevels) -> RobotStats {
        RobotStats {
            damage: RobotLevels::get_damage_for_level(&levels),
            max_health: RobotLevels::get_health_for_level(&levels),
            max_energy: RobotLevels::get_energy_for_level(&levels),
            energy_regen: RobotLevels::get_energy_regen_for_level(&levels),
            mining_speed: RobotLevels::get_mining_speed_for_level(&levels),
            max_storage: RobotLevels::get_storage_for_level(&levels),
            mineable_resources: RobotLevels::get_mineable_resoures(&levels),
        }
    }
}

impl Default for RobotStats {
    fn default() -> Self {
        RobotStats {
            damage: RobotLevels::get_damage_for_level(&RobotLevels::default()),
            max_health: RobotLevels::get_health_for_level(&RobotLevels::default()),
            max_energy: RobotLevels::get_energy_for_level(&RobotLevels::default()),
            energy_regen: RobotLevels::get_energy_regen_for_level(&RobotLevels::default()),
            mining_speed: RobotLevels::get_mining_speed_for_level(&RobotLevels::default()),
            max_storage: RobotLevels::get_storage_for_level(&RobotLevels::default()),
            mineable_resources: RobotLevels::get_mineable_resoures(&RobotLevels::default()),
        }
    }
}