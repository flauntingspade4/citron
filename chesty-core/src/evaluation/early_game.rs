use crate::Board;

impl Board {
    #[must_use]
    /// A heuristic for the earlygame evaluation
    pub fn early_game_evaluation(&self) -> i16 {
        // The sum of the positional value of all the pieces
        self.positions_squares()
            .map(|(position, piece)| piece.positional_value(position))
            .sum::<i16>()
    }
}
