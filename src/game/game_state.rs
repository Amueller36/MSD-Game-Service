use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use crate::planet::planet::Planet;
use crate::robot::robot::Robot;

#[derive(Serialize, Deserialize, Debug)]
pub struct GameState {
    pub game_id: Uuid,
    pub robots: HashMap<Uuid, Vec<Robot>>, //playerId -> robot
    pub participating_players: Vec<Uuid>,
    pub current_round: u32,
    pub map: Vec<Vec<Option<Planet>>>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct RoundState {
    pub round: u32,
    pub robots: HashMap<Uuid, Vec<Robot>>,
    pub map: HashMap<Uuid, Planet>,
}

pub struct Map {
    planets : Vec<Vec<Option<Planet>>>,
    indices : HashMap<Uuid, (usize,usize)>,
}

impl Map {
    fn new(planets : Vec<Vec<Option<Planet>>>) -> Map{
        let mut indices = HashMap::new();
        for (x, row) in planets.iter().enumerate() {
            for (y, planet) in row.iter().enumerate() {
                if let Some(planet) = planet {
                    indices.insert(planet.planet_id, (x,y));
                }
            }
        }
        Map{
            planets,
            indices,
        }
    }

}

impl GameState {
    pub fn new(game_id: Uuid, map: Vec<Vec<Option<Planet>>>) -> GameState {
        GameState {
            game_id,
            robots: HashMap::new(),
            participating_players : Vec::new(),
            current_round: 0,
            map,
        }
    }
}
