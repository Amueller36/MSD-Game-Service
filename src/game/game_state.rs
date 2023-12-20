use std::collections::HashMap;
use rayon::prelude::*;

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

    pub fn get_planet_as_mut(&mut self, planet_id : &Uuid) -> Option<&mut Planet> {
        if let Some((x,y)) = self.indices.get(&planet_id) {
            return self.planets[*x][*y].as_mut();
        }
        None
    }
    
    pub fn get_planet(&self, planet_id : &Uuid) -> Option<&Planet> {
        if let Some((x,y)) = self.indices.get(&planet_id) {
            return self.planets[*x][*y].as_ref();
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

    pub fn get_robots_for_current_round(&mut self, player_name: &str) -> Option<&mut HashMap<Uuid,Robot>> {
        if let Some(round_state) = self.round_states.get_mut(&self.current_round) {
            return round_state.player_robot_map.get_mut(player_name);
        }
        None
    }

    pub fn get_player_for_current_round(&mut self, player_name: &str) -> Option<&mut Player> {
        if let Some(round_state) = self.round_states.get_mut(&self.current_round) {
            return round_state.player_name_player_map.get_mut(player_name);
        }
        None
    }

    pub fn get_robots_for_current_round_by_robot_id(&mut self, robot_id: &Uuid) -> Option<&mut HashMap<Uuid,Robot>> {
        if let Some(round_state) = self.round_states.get_mut(&self.current_round) {
            for robots in round_state.player_robot_map.values_mut() {
                if robots.contains_key(&robot_id) {
                    return Some(robots);
                }
            }
        }
        None
    }

    pub fn get_robot_for_current_round_by_robot_id(&mut self, robot_id: &Uuid) -> Option<&mut Robot> {
        if let Some(round_state) = self.round_states.get_mut(&self.current_round) {
            for robots in round_state.player_robot_map.values_mut() {
                if let Some(robot) = robots.get_mut(&robot_id) {
                    return Some(robot);
                }
            }
        }
        None
    }

    pub fn get_robot_planet_as_mut(&mut self, robot_id: &Uuid) -> Option<&mut Planet> {
        if let Some(round_state) = self.round_states.get_mut(&self.current_round) {
            for robots in round_state.player_robot_map.values() {
                if let Some(robot) = robots.get(&robot_id) {
                    return round_state.map.get_planet_as_mut(&robot.planet_id);
                }
            }
        }
        None
    }
    
    pub fn get_robot_planet(&self, robot_id: &Uuid) -> Option<&Planet> {
        if let Some(round_state) = self.round_states.get(&self.current_round) {
            for robots in round_state.player_robot_map.values() {
                if let Some(robot) = robots.get(&robot_id) {
                    return round_state.map.get_planet(&robot.planet_id);
                }
            }
        }
        None
    }
}
