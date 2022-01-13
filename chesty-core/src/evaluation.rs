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

        let heatmap_eval = if self.turn <= 30 {
            self.positions_squares()
                .map(|(position, piece)| piece.positional_value(position))
                .sum()
        } else {
            0
        };

        heatmap_eval + self.material
    }
    pub fn calculate_material(&mut self) {
        self.material = self.pieces().map(Piece::value).sum();
    }
}
