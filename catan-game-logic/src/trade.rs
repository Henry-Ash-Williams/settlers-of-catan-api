use crate::{player::PlayerColour, resources::Resources};

use anyhow::{anyhow, Result};

pub enum TradeState {
    Proposed,
    LockedIn,
    Accepted,
}

use TradeState::*;

pub struct Trade {
    from: PlayerColour,
    accepted_by: Vec<PlayerColour>,
    to: Option<PlayerColour>,
    offering: Resources,
    wants: Resources,
    state: TradeState,
}

impl Trade {
    pub fn new(from: PlayerColour, offering: Resources, wants: Resources) -> Self {
        Trade {
            from,
            to: None,
            accepted_by: Vec::new(),
            offering,
            wants,
            state: Proposed,
        }
    }

    /// Indicate a player is willing to make this trade
    pub fn accept(&mut self, accepted_by: PlayerColour) -> Result<()> {
        match self.state {
            Proposed => {
                self.accepted_by.push(accepted_by);
                Ok(())
            }
            LockedIn | Accepted => Err(anyhow!("Cannot accept trade offer at this stage")),
        }
    }

    /// Indicate the player offering the trade accepts the trade from a player
    pub fn confirm_recipient(&mut self, player: PlayerColour) -> Result<()> {
        match self.state {
            Proposed => {
                self.to = Some(player);
                self.state = LockedIn;

                Ok(())
            }
            LockedIn | Accepted => Err(anyhow!(
                "Cannot confirm the recipient for trade offer at this stage"
            )),
        }
    }

    /// Swap the items between the two players
    pub fn complete(&mut self) -> Result<()> {
        match self.state {
            Proposed => return Err(anyhow!("Missing trade recipient")),
            Accepted => return Err(anyhow!("This trade has already been accepted")),
            _ => (),
        };
        self.state = Accepted;
        Ok(())
    }

    pub fn get_offering_player(&self) -> PlayerColour {
        self.from
    }

    pub fn get_trade_partner(&self) -> Result<PlayerColour> {
        match self.state {
            Proposed => Err(anyhow!("No trade partner")),
            _ => Ok(self.to.unwrap()),
        }
    }

    pub fn offering(&self) -> &Resources {
        &self.offering
    }

    pub fn wants(&self) -> &Resources {
        &self.wants
    }
}
