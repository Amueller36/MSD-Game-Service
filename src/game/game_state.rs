use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::planet::planet::Planet;
use crate::player::Player;
use crate::robot::robot::Robot;

#[derive(Serialize, Deserialize, Debug)]
pub struct GameState {
    pub game_id: Uuid,
    pub participating_players: Vec<Player>,
    pub current_round: u16,
    pub status: GameStatus,
    pub max_rounds: u16,
    pub max_players: u8,
    pub round_states: HashMap<u16, RoundState>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum GameStatus {
    Created,
    Started,
    Ended,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct RoundState {
    pub round_number: u32,
    pub player_robot_map: HashMap<String, HashMap<Uuid,Robot>>, // player_name -> robot_id -> robot
    pub player_name_player_map: HashMap<String, Player>,
    pub map: GameMap,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GameMap {
    pub planets : Vec<Vec<Option<Planet>>>,
    pub indices : HashMap<Uuid, (usize,usize)>,
}

impl GameMap {
    fn new(planets : Vec<Vec<Option<Planet>>>) -> GameMap {
        let mut indices = HashMap::new();
        for (x, row) in planets.iter().enumerate() {
            for (y, planet) in row.iter().enumerate() {
                if let Some(planet) = planet {
                    indices.insert(planet.planet_id, (x,y));
                }
            }
        }
        GameMap {
            planets,
            indices,
        }
    }

    pub fn get_planet(&mut self, planet_id : Uuid) -> Option<&mut Planet> {
        if let Some((x,y)) = self.indices.get(&planet_id) {
            return self.planets[*x][*y].as_mut();
        }
        None
    }

}

impl GameState {
    pub fn new(game_id: Uuid, max_rounds: u16, max_players: u8 ,planets: Vec<Vec<Option<Planet>>>) -> GameState {
        let initial_round = RoundState {
            round_number: 0,
            player_robot_map: HashMap::new(),
            player_name_player_map: HashMap::new(),
            map: GameMap::new(planets),
        };
        let mut round_states = HashMap::new();
        round_states.insert(0, initial_round);
        GameState {
            game_id,
            round_states: round_states,
            status: GameStatus::Created,
            participating_players : Vec::new(),
            current_round: 0,
            max_rounds,
            max_players: max_players
        }
    }
}
