use crate::{Board, Team};

impl Board {
    pub fn middle_game_evaluation(&self) -> i16 {
        self.material
            + (self.positions_pieces().fold(0, |moves, (position, p)| {
                let mobility = p.mobility(position, self);
                if p.team() == Team::Black {
                    moves - mobility
                } else {
                    moves + mobility
                }
            }) >> 3)
    }
}
