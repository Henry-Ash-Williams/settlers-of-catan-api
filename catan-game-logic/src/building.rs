use Building::*;

use crate::resources::Resources;

use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Building {
    Settlement,
    City,
    Road,
}

impl Building {
    pub fn get_resource_cost(&self) -> Resources {
        match *self {
            Settlement => Resources::new_explicit(0, 1, 1, 1, 1),
            City => Resources::new_explicit(3, 2, 0, 0, 0),
            Road => Resources::new_explicit(0, 0, 0, 1, 1),
        }
    }
}
