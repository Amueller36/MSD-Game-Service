use std::collections::{HashMap, HashSet, VecDeque};

use serde::{Deserialize, Serialize, Serializer};
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
    #[serde(serialize_with = "sort_and_serialize_robots")]
    pub robots : HashMap<Uuid,Robot>,
    pub commands: HashMap<CommandType, VecDeque<Command>>,
    pub killed_robots : HashMap<Uuid,Vec<(String,Robot)>> // OurRobotId -> Enemy_player_name, Enemy_robot
}

fn sort_and_serialize_robots<S>(
    robots: &HashMap<Uuid, Robot>,
    serializer: S,
) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
{
    let mut robots: Vec<_> = robots.iter().collect();
    robots.sort_by_key(|&(uuid, _)| uuid);
    let sorted_robots: HashMap<String, &Robot> = robots
        .into_iter()
        .map(|(uuid, robot)| (uuid.to_string(), robot))
        .collect();
    sorted_robots.serialize(serializer)
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Money{
    pub amount: u32,
}