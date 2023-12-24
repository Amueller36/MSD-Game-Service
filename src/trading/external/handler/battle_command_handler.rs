use std::collections::HashMap;
use std::sync::Arc;
use rayon::prelude::*;
use tokio::sync::Mutex;
use tracing::error;
use uuid::Uuid;
use crate::game::game_state::GameState;
use crate::player::PlayerState;
use crate::trading::external::command::Command;
use crate::trading::external::command_type::CommandType;

struct DamageReport {
    robot_id: Uuid,
    damage_to_take: u32,
}

pub async fn calculate_damage_for_round(players: Arc<Mutex<&HashMap<String, PlayerState>>>) -> Vec<DamageReport> {
    let player_states = players.lock().await;

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
                        robot_id: target_id,
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
    for damage_report in damage_reports {
        let target_robot = game_state.get_robot_for_current_round_by_robot_id(&damage_report.robot_id).expect("Target robot not found");
        target_robot.take_damage(damage_report.damage_to_take);
    }
}