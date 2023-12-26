use std::ops::Deref;
use tracing::error;
use crate::game::game_state::GameState;
use crate::trading::external::command::Command;

// pub fn handle_mining_command(mining_command: Command, mut game_state: GameState) -> Option<GameState>{
//     let robot_id = mining_command.command_object.robot_id.expect("Robot id was missing in mining command");
//     let robot = game_state.get_robot_for_current_round_by_player_id_and_robot_id(&mining_command.player_name, &robot_id)
//         .unwrap_or_else(|| panic!("Robot with id {} does not exist", robot_id));
//
//     if !robot.is_alive() {
//         error!("Robot with id {} is dead it cannot mine", robot_id);
//         return None;
//     }
//
//     let planet_id = robot.planet_id;
//     let command_planet_id = mining_command.command_object.planet_id.expect("Planet id was missing in mining command");
//     if planet_id != command_planet_id {
//         error!("Robot with id {} is on planet with id {} but mining command was for planet with id {}", robot_id, planet_id, command_planet_id);
//         return None;
//     }
//
//     if robot.is_inventory_full() {
//         error!("Robot with id {} has a full inventory and cannot mine", robot_id);
//         return None;
//     }
//
//     let mining_amount_for_level = robot.levels.get_mining_amount_for_level();
//     let planet = game_state.round_states[&game_state.current_round.clone()].map.planets.get(0).unwrap().get_mut(0)
//         .unwrap_or_else(|| panic!("Planet with id {} does not exist", planet_id))
//         .as_mut().unwrap();
//
//     let potential_mining_amount = std::cmp::min(robot.get_free_inventory_space(), mining_amount_for_level);
//
//     if let Some((resource, resource_amount)) = &mut planet.resources {
//         if *resource_amount == 0 {
//             error!("No resources left to mine on planet {}", planet_id);
//             return None;
//         }
//
//         let mining_amount = std::cmp::min( , *resource_amount);
//         robot.add_resource_to_inventory(resource, &mining_amount);
//         *resource_amount -= mining_amount;
//
//         if *resource_amount == 0 {
//             planet.resources = None;
//         }
//     }
//     Some(game_state)
// }
