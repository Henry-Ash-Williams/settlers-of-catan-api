use crate::board::Board;
use crate::Player;
use crate::{bank::Bank, player::PlayerColour};

use anyhow::{anyhow, Result};
use rand::{thread_rng, Rng};

use uuid::Uuid;

pub enum GameState {
    Setup,
    Running,
    Complete,
}

pub struct Game {
    players: Vec<Player>,
    board: Board,
    bank: Bank,
    state: GameState,
    turn_no: usize,
}

impl Game {
    pub fn new() -> Self {
        Game {
            players: Vec::new(),
            board: Board::new(),
            bank: Bank::new(),
            state: GameState::Setup,
            turn_no: 0,
        }
    }

    pub fn get_game_id(&self) -> Result<Uuid> {
        match self.state {
            GameState::Setup => Ok(Uuid::new_v4()),
            GameState::Running => Err(anyhow!("Cannot get Uuid for a game currently in progress")),
            GameState::Complete => Err(anyhow!("Cannot get Uuid for a finished game")),
        }
    }

    pub fn roll_dice(&self) -> (u8, u8) {
        let mut rng = thread_rng();
        (rng.gen_range(1..6), rng.gen_range(1..6))
    }

    pub fn get_player(&self, colour: &PlayerColour) -> Result<&Player> {
        self.players
            .iter()
            .find(|player| player.colour() == colour)
            .ok_or(anyhow!("Could not find that player"))
    }

    pub fn get_player_mut(&mut self, colour: PlayerColour) -> Result<&mut Player> {
        self.players
            .iter_mut()
            .find(|player| *player.colour() == colour)
            .ok_or(anyhow!("Could not find that player"))
    }

    pub fn finalize_trade(&mut self, trade_id: Uuid) -> Result<()> {
        match self.bank.get_trade_mut(trade_id) {
            Some(trade) => {
                let offering = trade.offering().clone();
                let wants = trade.wants().clone();

                let from = self.get_player_mut(trade.get_offering_player())?;
                *from.resources_mut() += wants - offering;

                let to = self.get_player_mut(trade.get_trade_partner()?)?;
                *to.resources_mut() += offering - wants;

                Ok(())
            }
            None => Err(anyhow!("Could not find trade with that ID")),
        }
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}
