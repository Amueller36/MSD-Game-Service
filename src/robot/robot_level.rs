use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub enum RobotLevel {
    LEVEL0,
    LEVEL1,
    LEVEL2,
    LEVEL3,
    LEVEL4,
    LEVEL5,
}
impl RobotLevel {
    pub fn get_int_value(&self) -> u32 {
        match self {
            RobotLevel::LEVEL0 => 0,
            RobotLevel::LEVEL1 => 1,
            RobotLevel::LEVEL2 => 2,
            RobotLevel::LEVEL3 => 3,
            RobotLevel::LEVEL4 => 4,
            RobotLevel::LEVEL5 => 5,
        }
    }
}

impl Default for RobotLevel {
    fn default() -> Self {
        RobotLevel::LEVEL0
    }
}
