use std::collections::HashMap;
use std::sync::Arc;
use rayon::prelude::*;
use tokio::sync::Mutex;
use tracing::error;
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
    let player_states = &round_state.player_name_player_map;

    player_states.par_iter().flat_map(|(_, player)| {
        let mut local_damage_reports = Vec::new();
        let commands = player.commands.get(&CommandType::BATTLE).expect("Battle commands are required");

        for command in commands {
            let attacker_id = command.command_object.robot_id.expect("Attacker id is required");
            let target_id = command.command_object.target_id.expect("Target id is required");

            if let Some(attacker_robot) = player.robots.get(&attacker_id) {
                if attacker_robot.is_alive() {
                    let attacker_damage = attacker_robot.levels.get_damage_for_level();
                    local_damage_reports.push(DamageReport {
                        attacker_id,
                        attacker_name: player.player_name.clone(),
                        defender_id: target_id,
                        damage_to_take: attacker_damage,
                    });
                } else {
                    error!("Attacker robot of {} with id {} is dead it cannot attack", &player.player_name, &attacker_id);
                }
            }
        }
        local_damage_reports
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
        let (target_robot, target_player_name) = game_state.get_robot_and_playername_for_current_round_by_robot_id(&damage_report.defender_id).expect("Target robot not found");
        target_robot.take_damage(damage_report.damage_to_take);
        if !target_robot.is_alive() {
            kill_reports.push(KillReport {
                attacker_name: damage_report.attacker_name,
                attacker_robot_id: damage_report.attacker_id,
                player_name_whose_robot_got_killed: target_player_name,
                killed_robot: target_robot.clone()
            });
        }
    }

    for report in kill_reports {
        let player_state = game_state.get_player_for_current_round_as_mut(&report.attacker_name).expect(format!("Player {} (Attacker) not found", report.attacker_name).as_str());
        player_state.killed_robots.insert(report.attacker_robot_id, (report.player_name_whose_robot_got_killed, report.killed_robot));
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
                let mining_commands = player.commands.get_mut(&CommandType::MINING).expect("Mining commands are required");
                mining_commands.retain(|command| command.command_object.robot_id.unwrap() != *robot_id);

                let regenerating_commands = player.commands.get_mut(&CommandType::REGENERATE).expect("Regenerating commands are required");
                regenerating_commands.retain(|command| command.command_object.robot_id.unwrap() != *robot_id);
            }
        }

    }
}