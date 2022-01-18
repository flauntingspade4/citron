use crate::Board;

impl Board {
    #[must_use]
    pub fn early_game_evaluation(&self) -> i16 {
        self.positions_squares()
            .map(|(position, piece)| piece.positional_value(position))
            .sum::<i16>()
    }
}
