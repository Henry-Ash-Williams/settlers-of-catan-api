use serde::{Deserialize, Serialize};

use crate::{development_cards::DevelopmentCard, resources::Resources};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlayerColour {
    Red,
    Green,
    Blue,
    Purple,
    Custom { r: u8, g: u8, b: u8 },
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct Player {
    colour: PlayerColour,
    resources: Resources,
    development_cards: Vec<DevelopmentCard>,
    victory_points: usize,
}

impl Player {
    pub fn new(colour: PlayerColour) -> Self {
        Self {
            colour,
            resources: Resources::new(),
            development_cards: Vec::new(),
            victory_points: 0,
        }
    }

    pub fn resources(&self) -> &Resources {
        &self.resources
    }

    pub fn resources_mut(&mut self) -> &mut Resources {
        &mut self.resources
    }

    pub fn colour(&self) -> &PlayerColour {
        &self.colour
    }
}
