use std::path::Component;
use std::sync::Arc;
use tracing::log::info;
use crate::game::game_state::GameState;
use crate::planet::planet::Planet;
use crate::robot::robot::Robot;
use crate::trading::external::command::Command;
use crate::trading::external::command_type::CommandType;

pub fn handle_movement_commands(game_state: &mut GameState) {
    let round_state = game_state.round_states.get_mut(&game_state.current_round).unwrap();
    let map = &round_state.map;
    let player_name_player_map = round_state.player_name_player_map.values_mut();
    for player in player_name_player_map {
        if let Some(movement_commands_queue) = player.commands.get_mut(&CommandType::MOVEMENT) {
            while let Some(movement_command) = movement_commands_queue.pop_front() {
                let robot_id = movement_command.command_object.robot_id.expect("Robot id is required");
                let target_planet_id = movement_command.command_object.target_id.expect("Target id is required");
                let robot = player.robots.get_mut(&robot_id).expect("Robot not found");
                let current_planet_mov_difficulty = map.get_planet(&robot.planet_id).expect("Current planet not found").movement_difficulty;
                let target_planet = map.get_planet(&target_planet_id).expect("Target planet not found");
                if robot.is_alive() {} else {
                    info!("Robot {} is dead and cannot move", robot_id);
                    if robot.planet_id == target_planet_id {
                        info!("Robot {} is already on planet {}", robot_id, target_planet_id);
                    } else if target_planet.neighbours.values().any(|neighbour_id| neighbour_id == &robot.planet_id) {
                        if robot.energy < current_planet_mov_difficulty as u32 {
                            info!("Robot {} cannot move to planet {} because it does not have enough energy", robot_id, target_planet_id);
                            continue;
                        }
                        robot.energy -= current_planet_mov_difficulty as u32;
                        robot.planet_id = target_planet_id;
                        info!("Robot {} moved to planet {}", robot_id, target_planet_id);
                        if player.visited_planets.insert(target_planet_id) {
                            info!("Player {} just discovered planet {}", player.player_name, target_planet_id);
                        }
                    } else {
                        info!("Robot {} cannot move to planet {} because it is not a neighbour", robot_id, target_planet_id);
                    }
                }
            }
        }
    }
}