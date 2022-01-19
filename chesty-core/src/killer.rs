use std::collections::HashSet;

use crate::{position::position_to_u16, Position};

#[derive(Clone, Debug, Default)]
pub struct KillerMoves {
    moves: HashSet<u16>,
}

impl KillerMoves {
    pub fn contains_move(&self, from: Position, to: Position) -> bool {
        self.moves.contains(&position_to_u16((from, to)))
    }
    pub fn add_move(&mut self, from: Position, to: Position) {
        self.moves.insert(position_to_u16((from, to)));
    }
}
