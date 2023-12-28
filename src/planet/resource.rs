use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Hash, Clone)]
pub enum Resource {
    COAL,
    IRON,
    GEM,
    GOLD,
    PLATINUM,
}

impl Resource {
    pub(crate) fn variants() -> Vec<Resource> {
        vec![
            Resource::COAL,
            Resource::IRON,
            Resource::GEM,
            Resource::GOLD,
            Resource::PLATINUM,
        ]
    }

    pub(crate) fn movement_difficulty(&self) -> u8 {
        match self {
            Resource::COAL => 1,
            Resource::IRON => 1,
            Resource::GEM => 2,
            Resource::GOLD => 3,
            Resource::PLATINUM => 3,
        }
    }

    pub fn get_selling_value(&self) -> u32 {
        match self {
            Resource::COAL => 5,
            Resource::IRON => 15,
            Resource::GEM => 30,
            Resource::GOLD => 50,
            Resource::PLATINUM => 60,
        }
    }
}
