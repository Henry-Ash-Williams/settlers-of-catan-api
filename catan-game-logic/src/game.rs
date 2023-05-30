use crate::Player;
use crate::board::Board; 
use crate::bank::Bank;

pub struct Game<'a> {
    players: &'a [Player],
    board: Board,
    bank: Bank,
}
