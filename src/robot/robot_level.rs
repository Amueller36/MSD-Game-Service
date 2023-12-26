use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize,Debug,Clone, PartialEq, PartialOrd)]
pub enum RobotLevel{
    LEVEL0,
    LEVEL1,
    LEVEL2,
    LEVEL3,
    LEVEL4,
    LEVEL5
}
