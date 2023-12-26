use tracing::log::{error, info};
use uuid::Uuid;
use crate::game::game_state::GameState;
use crate::player::PlayerState;
use crate::robot::robot::Robot;
use crate::robot::robot_level::RobotLevel;
use crate::robot::robot_levels::RobotLevels;
use crate::trading::external::command::Command;
use crate::trading::external::command_type::CommandType;

pub fn handle_buy_commands(game_state: &mut GameState) {
    //TODO:"Handle buying of robots and spawning them on random planet, and buying health/energy restores
    let round_state = game_state.round_states.get_mut(&game_state.current_round).unwrap();
    let map = &round_state.map;
    let mut player_name_player_map = round_state.player_name_player_map.values_mut();
    for player in player_name_player_map {
        let money = &mut player.money;
        if let Some(buy_commands) = player.commands.get_mut(&CommandType::BUYING) {
            if buy_commands.is_empty() {
                continue;
            }
            for command in buy_commands.drain(..) {
                if money.amount == 0 {
                    error!("Player {} has no money and cannot buy {:?}", player.player_name, command);
                    continue;
                }
                let item_name = command.command_object.item_name.expect("Item name not present");
                let item_quantity = command.command_object.item_quantity.expect("Item amount not present");
                let upgrade_or_item = parse_item_name(&item_name, item_quantity).expect("Item name was not present in buy command");
                info!("Player {} wants to buy {:?}", player.player_name, upgrade_or_item);
                match upgrade_or_item {
                    UpgradeOrItem::Upgrade(upgrade_type, level) => {
                        let upgrade_cost = RobotLevels::get_cost_for_level(&level);
                        if money.amount < upgrade_cost {
                            error!("Player {} has not enough money to buy {:?} with cost {}", &player.player_name, &upgrade_type, &upgrade_cost);
                            continue;
                        }
                        money.amount -= upgrade_cost;
                        let robot_id = command.command_object.robot_id.expect("Robot id was missing in buy command");
                        let robot = player.robots.get_mut(&robot_id)
                            .unwrap_or_else(|| panic!("Robot with id {} does not exist", &robot_id));
                        match upgrade_type {
                            UpgradeType::Health => {
                                if robot.levels.health_level < level {
                                    robot.levels.health_level = level;
                                    robot.health = robot.levels.get_health_for_level();
                                } else {
                                    error!("Player {} tried to buy health upgrade {:?} for robot {} but it already has a higher level", &player.player_name, &level, &robot_id);
                                }
                            }
                            UpgradeType::Energy => {
                                if robot.levels.energy_level < level {
                                    robot.levels.energy_level = level;
                                    robot.energy = robot.levels.get_energy_for_level();
                                } else {
                                    error!("Player {} tried to buy energy upgrade {:?} for robot {} but it already has a higher level", &player.player_name, &level, &robot_id);
                                }
                            }
                            UpgradeType::EnergyRegen => {
                                if robot.levels.energy_regen_level < level {
                                    robot.levels.energy_regen_level = level;
                                } else {
                                    error!("Player {} tried to buy energy regen upgrade {:?} for robot {} but it already has a higher level", player.player_name, level, robot_id);
                                }
                            }
                            UpgradeType::Damage => {
                                if robot.levels.damage_level < level {
                                    robot.levels.damage_level = level;
                                } else {
                                    error!("Player {} tried to buy damage upgrade {:?} for robot {} but it already has a higher level", player.player_name, level, robot_id);
                                }
                            }
                            UpgradeType::Mining => {
                                if robot.levels.mining_level < level {
                                    robot.levels.mining_level = level;
                                } else {
                                    error!("Player {} tried to buy mining upgrade {:?} for robot {} but it already has a higher level", player.player_name, level, robot_id);
                                }
                            }
                            UpgradeType::MiningSpeed => {
                                if robot.levels.mining_speed_level < level {
                                    robot.levels.mining_speed_level = level;
                                } else {
                                    error!("Player {} tried to buy mining speed upgrade {:?} for robot {} but it already has a higher level", player.player_name, level, robot_id);
                                }
                            }
                            UpgradeType::Storage => {
                                if robot.levels.storage_level < level {
                                    robot.levels.storage_level = level;
                                } else {
                                    error!("Player {} tried to buy storage upgrade {:?} for robot {} but it already has a higher level", player.player_name, level, robot_id);
                                }
                            }
                        }
                    }
                    UpgradeOrItem::Item(item) => {
                        let item_cost = item.get_cost();
                        if money.amount < item_cost {
                            error!("Player {} has not enough money to buy {:?} with cost {}", player.player_name, item, item_cost);
                            continue;
                        }
                        money.amount -= item_cost;
                        match item {
                            Item::Robot(amount) => {
                                //choose random planet on map and spawn robot there
                                for _ in 0..=amount {
                                    let rand = rand::random::<usize>() % map.indices.len();
                                    let planet_id = map.indices.keys().nth(rand).expect("Planet Index not found, probably out of bounds");
                                    let robot = Robot::new(
                                        Uuid::new_v4(),
                                        *planet_id,
                                    );
                                    player.robots.insert(robot.robot_id, robot);
                                    info!("Player {} bought robot and spawned it on planet {}", player.player_name, planet_id);
                                }
                            }
                            Item::HealthRestore => {
                                let robot_id = command.command_object.robot_id.expect("Robot id was missing in buy command");
                                let robot = player.robots.get_mut(&robot_id)
                                    .expect(&*format!("Robot with id {} does not exist", robot_id));
                                robot.health = robot.levels.get_health_for_level();
                            }
                            Item::EnergyRestore => {
                                let robot_id = command.command_object.robot_id.expect("Robot id was missing in buy command");
                                let robot = player.robots.get_mut(&robot_id)
                                    .expect(&*format!("Robot with id {} does not exist", robot_id));
                                robot.energy = robot.levels.get_energy_for_level();
                            }
                        }
                    }
                }
            }
        }
    }
}

fn parse_item_name(item_name: &str, amount: u16) -> Option<UpgradeOrItem> {
    let item_name = item_name.to_lowercase();
    let parts: Vec<&str> = item_name.split('_').collect();

    match item_name.as_str() {
        "robot" => Some(UpgradeOrItem::Item(Item::Robot(amount))),
        "health_restore" => Some(UpgradeOrItem::Item(Item::HealthRestore)),
        "energy_restore" => Some(UpgradeOrItem::Item(Item::EnergyRestore)),
        _ => {
            if parts.len() < 2 {
                error!("Item name {} was not recognized", item_name);
                return None;
            }

            let upgrade_type_str = parts[..parts.len() - 1].join("_");
            let level_str = parts[parts.len() - 1];

            let upgrade_type = match upgrade_type_str.as_str() {
                "health" => UpgradeType::Health,
                "energy" => UpgradeType::Energy,
                "energy_regen" => UpgradeType::EnergyRegen,
                "damage" => UpgradeType::Damage,
                "mining" => UpgradeType::Mining,
                "mining_speed" => UpgradeType::MiningSpeed,
                "storage" => UpgradeType::Storage,
                _ => {
                    error!("Upgrade type {} was not recognized", upgrade_type_str);
                    return None;
                }
            };

            let level = match level_str {
                "1" => RobotLevel::LEVEL1,
                "2" => RobotLevel::LEVEL2,
                "3" => RobotLevel::LEVEL3,
                "4" => RobotLevel::LEVEL4,
                "5" => RobotLevel::LEVEL5,
                _ => {
                    error!("Upgrade level {} was not recognized", level_str);
                    return None;
                }
            };

            Some(UpgradeOrItem::Upgrade(upgrade_type, level))
        }
    }
}

#[derive(Debug, Clone,PartialEq)]
pub enum UpgradeOrItem {
    Upgrade(UpgradeType, RobotLevel),
    Item(Item),
}

#[derive(Debug, Clone,PartialEq)]
pub enum UpgradeType {
    Health,
    Energy,
    EnergyRegen,
    Damage,
    Mining,
    MiningSpeed,
    Storage,
}

#[derive(Debug, Clone,PartialEq)]
pub enum Item {
    Robot(u16),
    HealthRestore,
    EnergyRestore,
}

impl Item {
    pub fn get_cost(&self) -> u32 {
        match self {
            Item::Robot(i) => 100 * *i as u32,
            Item::HealthRestore => 75,
            Item::EnergyRestore => 50,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_item_name() {
        assert_eq!(parse_item_name("health_1", 1), Some(UpgradeOrItem::Upgrade(UpgradeType::Health, RobotLevel::LEVEL1)));
        assert_eq!(parse_item_name("health_2", 1), Some(UpgradeOrItem::Upgrade(UpgradeType::Health, RobotLevel::LEVEL2)));
        assert_eq!(parse_item_name("health_3", 1), Some(UpgradeOrItem::Upgrade(UpgradeType::Health, RobotLevel::LEVEL3)));
        assert_eq!(parse_item_name("health_4", 1), Some(UpgradeOrItem::Upgrade(UpgradeType::Health, RobotLevel::LEVEL4)));
        assert_eq!(parse_item_name("health_5", 1), Some(UpgradeOrItem::Upgrade(UpgradeType::Health, RobotLevel::LEVEL5)));
        assert_eq!(parse_item_name("energy_1", 1), Some(UpgradeOrItem::Upgrade(UpgradeType::Energy, RobotLevel::LEVEL1)));
        assert_eq!(parse_item_name("energy_2", 1), Some(UpgradeOrItem::Upgrade(UpgradeType::Energy, RobotLevel::LEVEL2)));
        assert_eq!(parse_item_name("energy_3", 1), Some(UpgradeOrItem::Upgrade(UpgradeType::Energy, RobotLevel::LEVEL3)));
        assert_eq!(parse_item_name("energy_4", 1), Some(UpgradeOrItem::Upgrade(UpgradeType::Energy, RobotLevel::LEVEL4)));
        assert_eq!(parse_item_name("energy_5", 1), Some(UpgradeOrItem::Upgrade(UpgradeType::Energy, RobotLevel::LEVEL5)));
        assert_eq!(parse_item_name("energy_regen_1", 1), Some(UpgradeOrItem::Upgrade(UpgradeType::EnergyRegen, RobotLevel::LEVEL1)));

    }
}