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
}
