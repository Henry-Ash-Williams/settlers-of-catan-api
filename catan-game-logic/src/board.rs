use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct Board();

impl Board {
    pub fn new() -> Self {
        Board()
    }
}
