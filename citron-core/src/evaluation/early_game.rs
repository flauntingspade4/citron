use crate::{heatmap::HEATMAPS, piece::PieceKind, Board};

impl Board {
    #[must_use]
    pub fn early_game_evaluation(&self) -> i16 {
        PieceKind::kinds().into_iter().fold(0, |mut old, kind| {
            for (white_heatmap, multiplier) in HEATMAPS[kind as usize][0] {
                old += (white_heatmap & self.pieces[kind as usize][0]).count_ones() as i16
                    * multiplier;
            }
            for (black_heatmap, multiplier) in HEATMAPS[kind as usize][1] {
                old += (black_heatmap & self.pieces[kind as usize][1]).count_ones() as i16
                    * multiplier;
            }

            old
        })
    }
}
