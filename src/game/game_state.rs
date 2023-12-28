use std::collections::HashMap;
use rayon::prelude::*;

use serde::{Deserialize, Serialize};
use tracing::log::info;
use uuid::Uuid;

use crate::planet::planet::Planet;
use crate::player::PlayerState;
use crate::robot::robot::Robot;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GameState {
    pub game_id: Uuid,
    pub participating_players: Vec<String>,
    pub current_round: u16,
    pub status: GameStatus,
    pub max_rounds: u16,
    pub max_players: u8,
    pub round_states: HashMap<u16, RoundState>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum GameStatus {
    Created,
    Started,
    Ended,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RoundState {
    pub round_number: u32,
    pub player_name_player_map: HashMap<String, PlayerState>,
    pub map: GameMap,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GameMap {
    pub planets: Vec<Vec<Option<Planet>>>,
    pub indices: HashMap<Uuid, (usize, usize)>,
}

impl GameMap {
    fn new(planets: Vec<Vec<Option<Planet>>>) -> GameMap {
        let mut indices = HashMap::new();
        for (x, row) in planets.iter().enumerate() {
            for (y, planet) in row.iter().enumerate() {
                if let Some(planet) = planet {
                    indices.insert(planet.planet_id, (x, y));
                }
            }
        }
        GameMap {
            planets,
            indices,
        }
    }

    pub fn get_planet_as_mut(&mut self, planet_id: &Uuid) -> Option<&mut Planet> {
        if let Some((x, y)) = self.indices.get(&planet_id) {
            return self.planets[*x][*y].as_mut();
        }
        None
    }

    pub fn get_planet(&self, planet_id: &Uuid) -> Option<&Planet> {
        if let Some((x, y)) = self.indices.get(&planet_id) {
            return self.planets[*x][*y].as_ref();
        }
        None
    }
}

impl GameState {
    pub fn new(game_id: Uuid, max_rounds: u16, max_players: u8, planets: Vec<Vec<Option<Planet>>>) -> GameState {
        let initial_round = RoundState {
            round_number: 0,
            player_name_player_map: HashMap::new(),
            map: GameMap::new(planets),
        };
        let mut round_states = HashMap::new();
        round_states.insert(0, initial_round);
        GameState {
            game_id,
            round_states: round_states,
            status: GameStatus::Created,
            participating_players: Vec::new(),
            current_round: 0,
            max_rounds,
            max_players: max_players,
        }
    }

    pub fn get_robots_for_current_round(&mut self, player_name: &str) -> Option<&mut HashMap<Uuid, Robot>> {
        if let Some(round_state) = self.round_states.get_mut(&self.current_round) {
            return Some(&mut round_state.player_name_player_map.get_mut(player_name).expect(&*format!("Player {} does not exist", player_name)).robots);
        }
        None
    }

    pub fn get_player_for_current_round(&mut self, player_name: &str) -> Option<&mut PlayerState> {
        if let Some(round_state) = self.round_states.get_mut(&self.current_round) {
            return round_state.player_name_player_map.get_mut(player_name);
        }
        None
    }

    pub fn get_robots_for_current_round_by_robot_id(&mut self, robot_id: &Uuid) -> Option<&mut HashMap<Uuid, Robot>> {
        if let Some(round_state) = self.round_states.get_mut(&self.current_round) {
            for robots in round_state.player_name_player_map.values_mut().map(|player| &mut player.robots) {
                if robots.contains_key(&robot_id) {
                    return Some(robots);
                }
            }
        }
        None
    }

    pub fn get_robot_for_current_round_by_robot_id(&mut self, robot_id: &Uuid) -> Option<&mut Robot> {
        if let Some(round_state) = self.round_states.get_mut(&self.current_round) {
            for robots in round_state.player_name_player_map.values_mut().map(|player| &mut player.robots) {
                if let Some(robot) = robots.get_mut(&robot_id) {
                    return Some(robot);
                }
            }
        }
        None
    }
    pub fn get_robot_for_current_round_by_player_id_and_robot_id(&mut self, player_name: &str, robot_id: &Uuid) -> Option<&mut Robot> {
        if let Some(round_state) = self.round_states.get_mut(&self.current_round) {
            if let Some(player) = round_state.player_name_player_map.get_mut(player_name) {
                if let Some(robot) = player.robots.get_mut(&robot_id) {
                    return Some(robot);
                }
            }
        }
        None
    }

    pub fn get_robot_planet_as_mut_by_robot_id(&mut self, robot_id: &Uuid) -> Option<&mut Planet> {
        if let Some(round_state) = self.round_states.get_mut(&self.current_round) {
            for robots in round_state.player_name_player_map.values_mut().map(|player| &mut player.robots) {
                if let Some(robot) = robots.get(&robot_id) {
                    return round_state.map.get_planet_as_mut(&robot.planet_id);
                }
            }
        }
        None
    }

    pub fn get_robot_planet_as_mut_by_robot_id_and_player_name(&mut self, player_name: &str, robot_id: &Uuid) -> Option<&mut Planet> {
        if let Some(round_state) = self.round_states.get_mut(&self.current_round) {
            if let Some(player) = round_state.player_name_player_map.get_mut(player_name) {
                if let Some(robot) = player.robots.get(&robot_id) {
                    return round_state.map.get_planet_as_mut(&robot.planet_id);
                }
            }
        }
        None
    }

    pub fn get_robot_planet(&mut self, robot_id: &Uuid) -> Option<&Planet> {
        if let Some(round_state) = self.round_states.get_mut(&self.current_round) {
            for robots in round_state.player_name_player_map.values_mut().map(|player| &mut player.robots) {
                if let Some(robot) = robots.get(&robot_id) {
                    return round_state.map.get_planet(&robot.planet_id);
                }
            }
        }
        None
    }

    pub fn start_next_round(&mut self) -> bool {
        if self.current_round < self.max_rounds {
            self.current_round += 1;
            info!("Starting round {}", self.current_round);
            return true
        }
        self.status = GameStatus::Ended;
        false
    }
}
