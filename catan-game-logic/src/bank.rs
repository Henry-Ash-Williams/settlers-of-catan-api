use std::collections::HashMap;
use std::mem::variant_count;

pub(self) use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::development_cards::*;
use crate::player::PlayerColour;
use crate::resources::*;
use crate::trade::Trade;

use DevelopmentCard::*;

pub const TOTAL_RESOURCES: usize = 19;

/// Bank handles distributing resources and development cards, and trades
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Bank {
    development_cards: HashMap<DevelopmentCard, usize>,
    resources: Resources,
    #[serde(with = "uuid_map")]
    trades: HashMap<Uuid, Trade>,
}

mod uuid_map {
    use crate::trade::Trade;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use std::collections::HashMap;
    use uuid::Uuid;

    pub fn serialize<S>(map: &HashMap<Uuid, Trade>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let new_hm: HashMap<String, &Trade> = map.iter().map(|(k, v)| (k.to_string(), v)).collect();
        new_hm.serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<HashMap<Uuid, Trade>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let vec: HashMap<String, Trade> = HashMap::deserialize(deserializer).unwrap();
        let map: HashMap<Uuid, Trade> = vec
            .into_iter()
            .map(|(k, v)| (Uuid::parse_str(&k).unwrap(), v))
            .collect();
        Ok(map)
    }
}

impl Bank {
    /// Create a new instance of bank with the correct number of total resources and development cards
    pub fn new() -> Self {
        Bank {
            development_cards: HashMap::from([
                (YearOfPlenty, 2),
                (RoadBuilding, 2),
                (Monopoly, 2),
                (HiddenVictoryPoint, 5),
                (Knight, 14),
            ]),
            resources: Resources::new_with_amount(TOTAL_RESOURCES),
            trades: HashMap::new(),
        }
    }

    /// Select a random development card, and distribute it to the player
    /// fails if there are no more development cards to distribute
    pub fn distribute_random_development_card(&mut self) -> Result<DevelopmentCard> {
        let mut i = 0;
        loop {
            let dev_card_kind = DevelopmentCard::random();
            let dev_card = self.development_cards.get_mut(&dev_card_kind);
            match dev_card {
                Some(n) if *n > 0 => {
                    *n -= 1;
                    break Ok(dev_card_kind);
                }
                Some(_) | None => (),
            };
            i += 1;

            if i == variant_count::<DevelopmentCard>() {
                break Err(anyhow!("No development cards available"));
            }
        }
    }

    /// Distribute an amount of a specific resource
    pub fn distribute_resource(&mut self, kind: ResourceKind, amount: usize) -> Result<Resources> {
        if (self.resources[kind] as i32) - (amount as i32) < 0 {
            return Err(anyhow!("Cannot distribute that amount of resources"));
        };

        let mut distributed_resources = Resources::new();
        distributed_resources[kind] = amount;
        self.resources[kind] -= amount;

        Ok(distributed_resources)
    }

    pub fn propose_trade_with_bank(&mut self, player: PlayerColour, wants: Resources) {
        let requirements = wants * 4;

        let _trade_id = self.propose_trade(player, requirements, wants);
        todo!()
    }

    pub fn return_resources(&mut self, resources: Resources) {
        self.resources += resources;
    }

    pub fn return_dev_card(&mut self, kind: DevelopmentCard) {
        *self.development_cards.get_mut(&kind).unwrap() += 1;
    }

    pub fn get_trade(&self, trade_id: Uuid) -> Option<&Trade> {
        self.trades.get(&trade_id)
    }

    pub fn get_trade_mut(&mut self, trade_id: Uuid) -> Option<&mut Trade> {
        self.trades.get_mut(&trade_id)
    }

    /// Propose a new trade to the other players
    ///
    /// creates a new instance of a `Trade` object, and insert it into the `trades` hashmap
    pub fn propose_trade(
        &mut self,
        from: PlayerColour,
        offering: Resources,
        wants: Resources,
    ) -> Uuid {
        let t = Trade::new(from, offering, wants);
        let uuid = Uuid::new_v4();
        self.trades.insert(uuid, t);
        uuid
    }

    /// Indicate a player is willing to make a trade
    pub fn accept_trade(&mut self, trade_id: Uuid, accepted_by: PlayerColour) -> Result<()> {
        let trade = self.trades.get_mut(&trade_id);

        if trade.is_none() {
            return Err(anyhow!("Trade not found"));
        };

        trade.unwrap().accept(accepted_by)?;

        Ok(())
    }

    /// Indicate that the player offering the trade is willing to finalize the player
    pub fn finalize_trade(&mut self, trade_id: Uuid, player: PlayerColour) -> Result<()> {
        let trade = self.trades.get_mut(&trade_id);

        if trade.is_none() {
            return Err(anyhow!("Trade not found"));
        }

        trade.unwrap().confirm_recipient(player)?;

        Ok(())
    }
}

impl Default for Bank {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{resources::Resources, *};

    #[test]
    fn test_init() {
        let b = Bank::new();

        assert_eq!(b.resources[Ore], 19);
        assert_eq!(b.resources[Wool], 19);
        assert_eq!(b.resources[Grain], 19);
        assert_eq!(b.resources[Lumber], 19);
        assert_eq!(b.resources[Brick], 19);

        assert_eq!(b.development_cards.get(&YearOfPlenty), Some(&2));
        assert_eq!(b.development_cards.get(&Monopoly), Some(&2));
        assert_eq!(b.development_cards.get(&Knight), Some(&14));
        assert_eq!(b.development_cards.get(&RoadBuilding), Some(&2));
        assert_eq!(b.development_cards.get(&HiddenVictoryPoint), Some(&5));
    }

    #[test]
    fn test_dev_card_distribution() {
        let mut b = Bank::new();
        let dev_card = b.distribute_random_development_card();

        assert!(dev_card.is_ok());
    }

    #[test]
    fn test_resource_distribution() {
        let mut b = Bank::new();
        let resources = b.distribute_resource(Ore, 5);

        assert!(resources.is_ok_and(|r| r == Resources::new_explicit(5, 0, 0, 0, 0)));
        assert_eq!(b.resources[Ore], 14);

        let more_resources = b.distribute_resource(Ore, 20);
        assert!(more_resources.is_err());
        assert_eq!(b.resources[Ore], 14);
    }

    #[test]
    fn test_resource_return() {
        let mut b = Bank::new();
        let resources = b.distribute_resource(Ore, 4).unwrap();
        assert_eq!(b.resources[Ore], 15);
        b.return_resources(resources);
        assert_eq!(b.resources[Ore], 19);
    }

    #[test]
    fn test_propose_trade() {
        let mut b = Bank::new();
        let p1 = player::PlayerColour::Red;
        let trade_id = b.propose_trade(
            p1,
            Resources::new_explicit(0, 0, 1, 0, 1),
            Resources::new_explicit(2, 0, 0, 0, 0),
        );
        assert_eq!(b.trades.len(), 1);
        assert!(b.get_trade(trade_id).is_some());
    }

    #[test]
    fn test_accept_trade() {
        let mut b = Bank::new();
        let p1 = player::PlayerColour::Red;
        let p2 = player::PlayerColour::Blue;
        let trade_id = b.propose_trade(
            p1,
            Resources::new_explicit(0, 0, 1, 0, 1),
            Resources::new_explicit(2, 0, 0, 0, 0),
        );
        assert!(b.accept_trade(trade_id, p2).is_ok());
        assert_eq!(
            *b.get_trade(trade_id).unwrap().state(),
            trade::TradeState::Proposed
        );
    }

    #[test]
    fn test_finalize_trade() {
        let mut b = Bank::new();
        let p1 = player::PlayerColour::Red;
        let p2 = player::PlayerColour::Blue;
        let trade_id = b.propose_trade(
            p1,
            Resources::new_explicit(0, 0, 1, 0, 1),
            Resources::new_explicit(2, 0, 0, 0, 0),
        );
        let _ = b.accept_trade(trade_id, p2);
        let _ = b.finalize_trade(trade_id, p2);

        assert_eq!(
            *b.get_trade(trade_id).unwrap().state(),
            trade::TradeState::LockedIn
        )
    }

    #[test]
    fn test_return_dev_card() {
        let mut b = Bank::new();
        let dc = b.distribute_random_development_card();

        assert!(dc.is_ok());
        b.return_dev_card(dc.unwrap());
    }
}
