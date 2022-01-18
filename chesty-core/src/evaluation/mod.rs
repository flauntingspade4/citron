mod early_game;
mod end_game;
mod mid_game;

#[cfg(feature = "debug")]
use core::sync::atomic::{AtomicUsize, Ordering};

use crate::{
    piece::{KING_VALUE, PAWN_VALUE},
    Board,
};

#[cfg(feature = "debug")]
pub static POSITIONS_CONSIDERED: AtomicUsize = AtomicUsize::new(0);

const DEFAULT_MAXIMUM_ABSOLUTE_MATERIAL: i16 = 78 * PAWN_VALUE;

impl Board {
    #[must_use]
    pub fn static_evaluation(&self) -> i16 {
        #[cfg(feature = "debug")]
        POSITIONS_CONSIDERED.fetch_add(1, Ordering::SeqCst);

        self.material
            + trade_bonus(self.material, self.absolute_material)
            + if self.turn <= 30 {
                self.early_game_evaluation()
            } else if self.turn <= 70 {
                self.middle_game_evaluation()
            } else {
                self.end_game_evaluation()
            }
    }
    pub fn calculate_material(&mut self) {
        self.material = self.pieces().map(|p| p.value()).sum();
        self.material = self.pieces().map(|p| p.piece_value()).sum::<i16>() - 2 * KING_VALUE;
    }
}

/// If a side is up material, they wish to get the
/// remaining amount of material on the board as low as
/// possible
fn trade_bonus(material: i16, absolute_material: i16) -> i16 {
    if material > 0 {
        (DEFAULT_MAXIMUM_ABSOLUTE_MATERIAL - absolute_material) >> 5
    } else if material < 0 {
        -((DEFAULT_MAXIMUM_ABSOLUTE_MATERIAL - absolute_material) >> 5)
    } else {
        0
    }
}
