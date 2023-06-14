use crate::board::Board;
use crate::resources::Resources;
use crate::trade::TradeState::*;
use crate::Player;
use crate::{bank::Bank, player::PlayerColour};

use anyhow::{anyhow, Result};
use rand::{thread_rng, Rng};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Copy, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum GameState {
    Setup,
    Running,
    Complete,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
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

    pub fn add_player(&mut self, colour: PlayerColour) {
        self.players.push(Player::new(colour));
    }

    pub fn roll_dice() -> (u8, u8) {
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

    /// Handle the final step of trading, moving the resources between the two players
    pub fn finalize_trade(&mut self, trade_id: Uuid) -> Result<()> {
        let mut trade = match self.bank.get_trade_mut(trade_id) {
            Some(trade) => trade.clone(),
            None => return Err(anyhow!("Could not find trade with that ID")),
        };

        match trade.state() {
            LockedIn => (),
            Accepted | Proposed => return Err(anyhow!("Cannot finalize trade at this time")),
        };

        *trade.state_mut() = Accepted;

        let offering: Resources = *trade.offering();
        let wants: Resources = *trade.wants();
        let offering_player = trade.get_offering_player();
        let trade_partner = trade.get_trade_partner()?;

        {
            let from = self.get_player_mut(offering_player)?;
            if *from.resources() < offering {
                return Err(anyhow!("Not enough resources to make this trade"));
            } else {
                *from.resources_mut() += wants;
                *from.resources_mut() -= offering;
            }
        }

        {
            let to = self.get_player_mut(trade_partner)?;
            if *to.resources() < wants {
                return Err(anyhow!("Not enough resources to make this trade"));
            } else {
                *to.resources_mut() += offering;
                *to.resources_mut() -= wants;
            }
        }

        Ok(())
    }

    pub fn get_bank(&self) -> &Bank {
        &self.bank
    }

    pub fn get_bank_mut(&mut self) -> &mut Bank {
        &mut self.bank
    }
}

impl Default for Game {
    fn default() -> Self {
        Self {
            players: Vec::new(),
            board: Board::default(),
            bank: Bank::new(),
            state: GameState::Setup,
            turn_no: 0,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{bank::*, board::*, game::*};
    #[test]
    fn test_init() {
        let g = Game::default();
        assert_eq!(
            g,
            Game {
                players: Vec::new(),
                board: Board::new(),
                bank: Bank::new(),
                state: GameState::Setup,
                turn_no: 0,
            }
        );
    }

    #[test]
    fn test_add_player() {
        let mut g = Game::default();
        assert_eq!(
            g,
            Game {
                players: Vec::new(),
                board: Board::default(),
                bank: Bank::new(),
                state: GameState::Setup,
                turn_no: 0,
            }
        );
        g.add_player(PlayerColour::Red);
        g.add_player(PlayerColour::Green);
        g.add_player(PlayerColour::Blue);
        g.add_player(PlayerColour::Purple);

        assert_eq!(
            g,
            Game {
                players: vec![
                    Player::new(PlayerColour::Red),
                    Player::new(PlayerColour::Green),
                    Player::new(PlayerColour::Blue),
                    Player::new(PlayerColour::Purple)
                ],
                board: Board::default(),
                bank: Bank::new(),
                state: GameState::Setup,
                turn_no: 0,
            }
        );
    }

    #[test]
    fn test_get_id() {
        let g = Game::new();
        let game_id = g.get_game_id();

        assert!(game_id.is_ok());
        let game_id = g.get_game_id().unwrap();
        assert!(Uuid::parse_str(&game_id.to_string()).is_ok());
    }

    #[test]
    fn test_roll_dice() {
        let (d1, d2) = Game::roll_dice();
        let roll = d1 + d2;

        assert!(roll > 0 && roll < 12);
    }

    #[test]
    fn test_get_player() {
        let mut g = Game::new();

        g.add_player(PlayerColour::Red);
        g.add_player(PlayerColour::Green);
        g.add_player(PlayerColour::Blue);
        g.add_player(PlayerColour::Purple);

        let r = g.get_player(&PlayerColour::Red);
        assert!(r.is_ok());
        assert_eq!(*r.unwrap().resources(), Resources::new());
    }

    #[test]
    fn test_trade() {
        let mut g = Game::new();

        g.add_player(PlayerColour::Red);
        g.add_player(PlayerColour::Green);
        g.add_player(PlayerColour::Blue);
        g.add_player(PlayerColour::Purple);

        {
            let red = g.get_player_mut(PlayerColour::Red).unwrap();
            *red.resources_mut() = Resources::new_explicit(0, 1, 1, 0, 0);
        }

        {
            let blue = g.get_player_mut(PlayerColour::Blue).unwrap();
            *blue.resources_mut() = Resources::new_explicit(2, 0, 0, 0, 0);
        }

        let b = g.get_bank_mut();
        let trade_id = b.propose_trade(
            PlayerColour::Red,
            Resources::new_explicit(0, 1, 1, 0, 0),
            Resources::new_explicit(2, 0, 0, 0, 0),
        );

        b.accept_trade(trade_id, PlayerColour::Blue)
            .expect("Could not find trade with that ID");
        b.finalize_trade(trade_id, PlayerColour::Blue)
            .expect("Could not find trade with that ID");
        println!("{:#?}", g.get_bank());
        g.finalize_trade(trade_id).unwrap();

        let red = g.get_player(&PlayerColour::Red).unwrap();
        assert_eq!(*red.resources(), Resources::new_explicit(2, 0, 0, 0, 0));
        let blue = g.get_player(&PlayerColour::Blue).unwrap();
        assert_eq!(*blue.resources(), Resources::new_explicit(0, 1, 1, 0, 0));
    }
}
