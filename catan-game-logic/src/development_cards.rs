use rand::{thread_rng, Rng};

use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Hash)]
#[serde(rename_all = "snake_case")]
pub enum DevelopmentCard {
    YearOfPlenty,
    Monopoly,
    Knight,
    RoadBuilding,
    HiddenVictoryPoint,
}

impl DevelopmentCard {
    pub fn random() -> Self {
        let mut rng = thread_rng();
        let variants = [
            DevelopmentCard::YearOfPlenty,
            DevelopmentCard::Monopoly,
            DevelopmentCard::Knight,
            DevelopmentCard::RoadBuilding,
            DevelopmentCard::HiddenVictoryPoint,
        ];
        let idx = rng.gen_range(0..variants.len());
        variants[idx]
    }
}
