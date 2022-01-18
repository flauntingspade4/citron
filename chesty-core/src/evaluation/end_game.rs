use crate::Board;

impl Board {
    #[must_use]
    pub const fn end_game_evaluation(&self) -> i16 {
        0
    }
}
