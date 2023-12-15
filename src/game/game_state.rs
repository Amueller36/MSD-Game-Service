use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use crate::planet::planet::Planet;
use crate::robot::robot::Robot;

#[derive(Serialize, Deserialize, Debug)]
struct GameState {
    game_id: Uuid,
    robots: HashMap<Uuid, Vec<Robot>>, //playerId -> robot
    participating_players: Vec<Uuid>,
    map: HashMap<Uuid, Planet>,
}
