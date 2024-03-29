use rayon::prelude::IntoParallelRefMutIterator;
use tracing::error;
use tracing::log::{debug, info};
use uuid::Uuid;

use crate::game::game_state::GameState;
use crate::robot::robot::Robot;
use crate::trading::external::command_type::CommandType;

pub struct DamageReport {
    attacker_id: Uuid,
    attacker_name: String,
    defender_id: Uuid,
    damage_to_take: u32,
}

pub async fn calculate_damage_for_round(game_state: &mut GameState) -> Vec<DamageReport> {
    let round_state = game_state.round_states.get_mut(&game_state.current_round).unwrap();

    round_state.player_name_player_map.iter_mut().flat_map(|(_, player)| {
        player.commands.get_mut(&CommandType::BATTLE).into_iter().flat_map(|commands| {
            commands.drain(..).filter_map(|command| {
                let attacker_id = command.command_object.robot_id.expect("Attacker id is required");
                let target_id = command.command_object.target_id.expect("Target id is required");
                if attacker_id == target_id {
                    error!("Robot {} with id {} cannot attack itself", &player.player_name, &attacker_id);
                    return None;
                }

                player.robots.get(&attacker_id).and_then(|attacker_robot| {
                    if attacker_robot.is_alive() {
                        let energy_needed_for_attack = attacker_robot.levels.damage_level.get_int_value() + 1;
                        if attacker_robot.energy < energy_needed_for_attack {
                            error!("Robot {} with id {} does not have enough energy to attack", &player.player_name, &attacker_id);
                            return None;
                        }
                        let attacker_damage = attacker_robot.levels.get_damage_for_level();
                        Some(DamageReport {
                            attacker_id,
                            attacker_name: command.player_name,
                            defender_id: target_id,
                            damage_to_take: attacker_damage,
                        })
                    } else {
                        error!("Attacker robot of {} with id {} is dead; it cannot attack", &player.player_name, &attacker_id);
                        None
                    }
                })
            })
        })
    }).collect()
}

pub fn apply_damage_for_round(damage_reports: Vec<DamageReport>, game_state: &mut GameState) {
    pub struct KillReport {
        pub attacker_robot_id: Uuid,
        pub attacker_name: String,
        pub player_name_whose_robot_got_killed: String,
        pub killed_robot: Robot,
    }
    let mut kill_reports = Vec::new();
    for damage_report in damage_reports {
        if let Some(attackers_robots) = game_state.get_robots_for_current_round(&damage_report.attacker_name) {
            if let Some(attacker_robot) = attackers_robots.get_mut(&damage_report.attacker_id) {
                let energy_cost_for_attack = attacker_robot.levels.damage_level.get_int_value() + 1;
                attacker_robot.energy -= energy_cost_for_attack; //TODO: Eigentlich muss das weiter unten hin, nach dem check

                if let Some((target_robot, target_player_name)) = game_state.get_robot_and_playername_for_current_round_by_robot_id(&damage_report.defender_id) {
                    if damage_report.attacker_id.clone() == target_robot.robot_id.clone() {
                        error!("Robot of {} with id {} cannot attack itself", damage_report.attacker_name, damage_report.attacker_id);
                        continue;
                    }
                    target_robot.take_damage(damage_report.damage_to_take);
                    if !target_robot.is_alive() {
                        debug!("Robot {} of player {} was killed by {} {}", damage_report.defender_id, target_player_name, damage_report.attacker_name, damage_report.attacker_id);
                        let kill_report = KillReport {
                            attacker_name: damage_report.attacker_name.clone(),
                            attacker_robot_id: damage_report.attacker_id,
                            player_name_whose_robot_got_killed: target_player_name.clone(),
                            killed_robot: target_robot.clone(), // Consider avoiding clone
                        };
                        if kill_report.attacker_robot_id == kill_report.killed_robot.robot_id {
                            error!("Robot of {} with id {} cannot kill itself", kill_report.attacker_name, kill_report.attacker_robot_id);
                        } else {
                            kill_reports.push(kill_report);
                        }
                    }
                } else {
                    error!("Target robot not found for ID {}", damage_report.defender_id);
                }
            } else {
                error!("Attacker robot not found for ID {}", damage_report.attacker_id);
            }
        } else {
            error!("Attacker robot not found for ID {}", damage_report.attacker_id);
        }
    }

    for report in kill_reports {
        if let Some(player_state) = game_state.get_player_for_current_round_as_mut(&report.attacker_name) {
            let mut existing_killed_robots = player_state.killed_robots.entry(report.attacker_robot_id).or_insert_with(Vec::new);
            existing_killed_robots.push((report.player_name_whose_robot_got_killed, report.killed_robot));
        } else {
            error!("Player {} (Attacker) not found", report.attacker_name);
        }
    }
}

pub fn delete_commands_for_dead_robots(game_state: &mut GameState) {
    let round_state = game_state.round_states.get_mut(&game_state.current_round).unwrap();
    let player_states = &mut round_state.player_name_player_map;

    for player in player_states.values_mut() {
        let players_robots = &mut player.robots;
        //clear mining and regenerating commands for dead robots, in case they had such commands
        for (robot_id, robot) in players_robots.iter_mut() {
            if !robot.is_alive() {
                if let Some(mining_commands) = player.commands.get_mut(&CommandType::MINING) {
                    mining_commands.retain(|command| command.command_object.robot_id.unwrap() != *robot_id);
                }
                if let Some(regenerating_commands) = player.commands.get_mut(&CommandType::REGENERATE) {
                    regenerating_commands.retain(|command| command.command_object.robot_id.unwrap() != *robot_id);
                }
            }
        }
    }
}