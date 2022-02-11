use crate::Board;

impl Board {
    #[must_use]
    /// A heuristic for the endgame evaluation
    pub const fn end_game_evaluation(&self) -> i16 {
        0
    }
}
