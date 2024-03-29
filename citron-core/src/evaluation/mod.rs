use core::cmp::Ordering;

mod early_game;
mod end_game;
mod mid_game;

#[cfg(feature = "debug")]
use core::sync::atomic::{AtomicUsize, Ordering as AtomicOrdering};

use crate::{
    piece::{PieceKind, PAWN_VALUE},
    Board, PlayableTeam,
};

#[cfg(feature = "debug")]
pub static POSITIONS_CONSIDERED: AtomicUsize = AtomicUsize::new(0);

const DEFAULT_MAXIMUM_ABSOLUTE_MATERIAL: i16 = 78 * PAWN_VALUE + 100;

impl Board {
    #[must_use]
    pub fn static_evaluation(&self) -> i16 {
        #[cfg(feature = "debug")]
        POSITIONS_CONSIDERED.fetch_add(1, AtomicOrdering::SeqCst);

        self.material
            + self.trade_bonus()
            + if self.turn <= 30 {
                self.early_game_evaluation()
            } else if self.turn <= 70 {
                self.middle_game_evaluation()
            } else {
                self.end_game_evaluation()
            }
    }
    pub fn calculate_material(&mut self) {
        for kind in PieceKind::kinds_no_king() {
            self.material += self.pieces[PlayableTeam::White as usize][kind as usize].count_ones()
                as i16
                * kind.value();
            self.absolute_material += self.pieces[PlayableTeam::White as usize][kind as usize]
                .count_ones() as i16
                * kind.value();

            self.material -= self.pieces[PlayableTeam::Black as usize][kind as usize].count_ones()
                as i16
                * kind.value();
            self.absolute_material += self.pieces[PlayableTeam::Black as usize][kind as usize]
                .count_ones() as i16
                * kind.value();
        }
    }
    /// If a side is up material, they wish to get the
    /// recrateing amount of material on the board as low as
    /// possible
    fn trade_bonus(&self) -> i16 {
        match self.material.cmp(&0) {
            Ordering::Less => -((DEFAULT_MAXIMUM_ABSOLUTE_MATERIAL - self.absolute_material) >> 7),
            Ordering::Equal => 0,
            Ordering::Greater => (DEFAULT_MAXIMUM_ABSOLUTE_MATERIAL - self.absolute_material) >> 7,
        }
    }
}
