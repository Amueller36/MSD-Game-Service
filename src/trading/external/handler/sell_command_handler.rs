use tracing::log::error;

use crate::game::game_state::GameState;
use crate::trading::external::command_type::CommandType;

pub fn handle_selling_commands(game_state: &mut GameState) {
    for player in game_state.round_states.get_mut(&game_state.current_round)
        .expect(&format!("No game state found for Round {}", &game_state.current_round))
        .player_name_player_map.values_mut() {

        // Überprüfen, ob Verkaufsbefehle vorhanden sind
        if let Some(selling_commands) = player.commands.get_mut(&CommandType::SELLING) {
            if selling_commands.is_empty() {
                continue; // Keine Verkaufsbefehle, gehe zum nächsten Spieler
            }

            let money = &mut player.money;
            let total_money_made = &mut player.total_money_made;
            for command in selling_commands.drain(..) {
                let robot_id = command.command_object.robot_id
                    .expect("Robot id was missing in selling command");
                let robot = player.robots.get_mut(&robot_id)
                    .unwrap_or_else(|| panic!("Robot with id {} does not exist", robot_id));

                if !robot.is_alive() {
                    error!("Robot with id {} is dead it cannot sell", robot_id);
                    continue;
                }
                if robot.inventory.is_empty() {
                    error!("Robot with id {} has an empty inventory and cannot sell", robot_id);
                    continue;
                }
                let resource_values = robot.get_storage_value();
                if resource_values == 0 {
                    error!("Robot with id {} has an empty inventory and tried to sell!", robot_id);
                    continue;
                }
                robot.money_made += resource_values;
                total_money_made.amount += resource_values;
                money.amount += resource_values;
                robot.inventory.clear();
            }
        }
    }
}