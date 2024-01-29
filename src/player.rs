use std::collections::{HashMap, HashSet, VecDeque};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::robot::robot::Robot;
use crate::trading::external::command::Command;
use crate::trading::external::command_type::CommandType;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PlayerState {
    pub player_name: String,
    pub money: Money,
    pub total_money_made: Money,
    pub visited_planets: HashSet<Uuid>,
    pub robots : HashMap<Uuid,Robot>,
    pub commands: HashMap<CommandType, VecDeque<Command>>,
    pub killed_robots : HashMap<Uuid,(String,Robot)>
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Money{
    pub amount: u32,
}