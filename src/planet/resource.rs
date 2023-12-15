use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
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
}
