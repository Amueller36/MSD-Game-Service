use std::collections::HashSet;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Player {
    pub player_name: String,
    pub money: u32,
    pub visited_planets: HashSet<Uuid>,
}