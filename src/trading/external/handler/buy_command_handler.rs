use crate::game::game_state::GameState;
use crate::player::PlayerState;
use crate::trading::external::command::Command;

pub fn handle_buy_command(player: &mut PlayerState, command: Command, game_state : &mut GameState) {
    todo!("Handle buying of robots and spawning them on random planet, and buying health/energy restores")
}