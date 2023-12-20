use std::path::Component;
use std::sync::Arc;
use crate::game::game_state::GameState;
use crate::trading::external::command::Command;

pub fn handle_movement_command(command: Command, game_state: &mut GameState) -> Result<GameState, String> {
    let robot_id = command.command_object.robot_id.ok_or("Robot id was missing in movement command".to_string())?;
    let robot_current_planet = game_state.get_robot_planet_as_mut(&robot_id).ok_or(format!("Robot with id {} does not exist", robot_id))?;
    let destination_planet_id = command.command_object.planet_id.ok_or("Planet id was missing in movement command".to_string())?;

    if !robot_current_planet.neighbours.values().any(|neighbor_planet_id| neighbor_planet_id == &destination_planet_id) {
        return Err(format!("Robot with id {} cannot move to planet with id {}", robot_id, destination_planet_id));
    }

    let robot = game_state.get_robot_for_current_round_by_robot_id(&robot_id).ok_or(format!("Robot with id {} does not exist", robot_id))?;
    robot.planet_id = destination_planet_id;

    todo!("Notiz an mich selbst: Command Priorisierung nach Command Ablauf. Wenn ein Roboter bspw stirbt, andere commands die auf ihn referenzieren abbrechen/droppen")
}