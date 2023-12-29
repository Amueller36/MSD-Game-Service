use std::collections::HashMap;
use std::sync::Arc;
use rayon::prelude::*;
use tokio::sync::Mutex;
use tracing::error;
use tracing::log::info;
use uuid::Uuid;
use crate::game::game_state::GameState;
use crate::player::PlayerState;
use crate::robot::robot::Robot;
use crate::trading::external::command::Command;
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

                player.robots.get(&attacker_id).and_then(|attacker_robot| {
                    if attacker_robot.is_alive() {
                        let attacker_damage = attacker_robot.levels.get_damage_for_level();
                        Some(DamageReport {
                            attacker_id,
                            attacker_name: player.player_name.clone(), // consider if clone is necessary here
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
        pub killed_robot : Robot
    }
    let mut kill_reports = Vec::new();
    for damage_report in damage_reports {
        if let Some((target_robot, target_player_name)) = game_state.get_robot_and_playername_for_current_round_by_robot_id(&damage_report.defender_id) {
            target_robot.take_damage(damage_report.damage_to_take);
            if !target_robot.is_alive() {
                info!("Robot {} of player {} was killed by {}", damage_report.defender_id, target_player_name, damage_report.attacker_name);
                kill_reports.push(KillReport {
                    attacker_name: damage_report.attacker_name.clone(),
                    attacker_robot_id: damage_report.attacker_id,
                    player_name_whose_robot_got_killed: target_player_name.clone(),
                    killed_robot: target_robot.clone() // Consider avoiding clone
                });
            }
        } else {
            error!("Target robot not found for ID {}", damage_report.defender_id);
        }
    }

    for report in kill_reports {
        if let Some(player_state) = game_state.get_player_for_current_round_as_mut(&report.attacker_name) {
            player_state.killed_robots.insert(report.attacker_robot_id, (report.player_name_whose_robot_got_killed, report.killed_robot));
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
        //clear mining and regenerating commands for dead robots
        for (robot_id, robot) in players_robots.iter_mut() {
            if !robot.is_alive() {
                if let Some(mining_commands) = player.commands.get_mut(&CommandType::MINING) {
                    mining_commands.retain(|command| command.command_object.robot_id.unwrap() != *robot_id);
                } else {
                    error!("Mining commands are required");
                }
                if let Some(regenerating_commands) = player.commands.get_mut(&CommandType::REGENERATE) {
                    regenerating_commands.retain(|command| command.command_object.robot_id.unwrap() != *robot_id);
                } else {
                    error!("Regenerating commands are required");
                }
            }
        }
    }
}