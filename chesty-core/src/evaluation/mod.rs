mod early_game;
mod end_game;
mod mid_game;

#[cfg(feature = "debug")]
use core::sync::atomic::{AtomicUsize, Ordering};

use crate::{piece::Piece, Board};

#[cfg(feature = "debug")]
pub static POSITIONS_CONSIDERED: AtomicUsize = AtomicUsize::new(0);

impl Board {
    #[must_use]
    pub fn static_evaluation(&self) -> i16 {
        #[cfg(feature = "debug")]
        POSITIONS_CONSIDERED.fetch_add(1, Ordering::SeqCst);

        if self.turn <= 30 {
            self.early_game_evaluation()
        } else if self.turn <= 70 {
            self.middle_game_evaluation()
        } else {
            self.end_game_evaluation()
        }
    }
    pub fn calculate_material(&mut self) {
        self.material = self.pieces().map(Piece::value).sum();
    }
}
