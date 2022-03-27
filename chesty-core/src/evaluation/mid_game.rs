use crate::Board;

impl Board {
    #[must_use]
    pub fn middle_game_evaluation(&self) -> i16 {
        /*10 * (self.positions_pieces().fold(0, |moves, (position, p)| {
            let mobility = p.mobility(position, self);
            if p.team() == Team::Black {
                moves - mobility
            } else {
                moves + mobility
            }
        }) >> 1)*/
        // - self[self.king_positions.0].virtual_mobility(self.king_positions.0, self) * 7
        // + self[self.king_positions.1].virtual_mobility(self.king_positions.1, self) * 7
        0
    }
}
