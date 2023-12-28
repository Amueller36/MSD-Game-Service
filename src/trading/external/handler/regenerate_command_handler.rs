use crate::game::game_state::GameState;
use crate::trading::external::command_type::CommandType;

pub fn handle_regenerate_commands(game_state: &mut GameState) {
    let round_state = game_state.round_states.get_mut(&game_state.current_round).unwrap();
    let player_states = &mut round_state.player_name_player_map;

    for player in player_states.values_mut() {
        let players_robots = &mut player.robots;
        if let Some(regenerate_commands) = player.commands.get_mut(&CommandType::REGENERATE) {
            while let Some(regenerate_command) = regenerate_commands.pop_front() {
                let robot_id = regenerate_command.command_object.robot_id.expect("Robot id is required");
                let robot = players_robots.get_mut(&robot_id).expect("Robot not found");
                robot.regenerate();
            }
        }
    }
}

