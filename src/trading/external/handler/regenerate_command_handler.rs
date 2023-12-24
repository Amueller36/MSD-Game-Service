use crate::game::game_state::GameState;
use crate::player::PlayerState;
use crate::trading::external::command::Command;

pub fn handle_regenerate_command(player: &mut PlayerState, command: Command,) {
    let robot_id = command.command_object.robot_id.expect("Robot id is required");
    let robot = player.robots.get_mut(&robot_id).expect("Robot not found");
    robot.regenerate();
}