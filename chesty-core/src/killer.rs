use std::collections::HashSet;

use crate::Position;

/// A [`HashSet`] of [Position]s, existing for the killer moves optimization
#[derive(Clone, Debug, Default)]
pub struct KillerMoves {
    moves: HashSet<(Position, Position)>,
}

impl KillerMoves {
    pub fn contains_move(&self, possible_move: (Position, Position)) -> bool {
        self.moves.contains(&possible_move)
    }
    pub fn add_move(&mut self, possible_move: (Position, Position)) {
        self.moves.insert(possible_move);
    }
}
