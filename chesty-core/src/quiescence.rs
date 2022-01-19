use core::sync::atomic::{AtomicI16, Ordering};

use crate::{
    analysis::BRANCHING_FACTOR,
    move_ordering::quiescence_move_ordering,
    piece::{KING_VALUE, PAWN_VALUE},
    Board, Team,
};

const DELTA: i16 = 2 * PAWN_VALUE;

impl Board {
    #[must_use]
    pub fn quiesce_white(&self, mut alpha: i16, beta: i16) -> i16 {
        let stand_pat = self.static_evaluation();

        if stand_pat >= beta {
            return beta;
        }

        if stand_pat > alpha {
            alpha = stand_pat;
        }

        let alpha = AtomicI16::new(alpha);

        let mut moves = Vec::with_capacity(BRANCHING_FACTOR);

        for (position, piece) in self
            .positions_pieces()
            .filter(|(_, p)| p.team() == Team::White)
        {
            piece.quiescence_moves(position, self, &mut moves);
        }

        quiescence_move_ordering(self, &mut moves);

        if let Err(beta_cutoff) = moves.into_iter().try_for_each(|(from, to, _)| {
            if self[to].piece_value() == KING_VALUE {
                return Err(KING_VALUE);
            }

            // println!("{} {}", stand_pat + DELTA - self[to].piece_value(), alpha.load(Ordering::SeqCst));

            if stand_pat + DELTA + self[to].piece_value() > alpha.load(Ordering::SeqCst) {
                let possible_board = self.make_move(from, to);

                let score = possible_board.quiesce_black(alpha.load(Ordering::SeqCst), beta);

                if score > alpha.load(Ordering::SeqCst) {
                    if score >= beta {
                        return Err(beta);
                    }
                    alpha.store(score, Ordering::SeqCst);
                }
            }

            Ok(())
        }) {
            return beta_cutoff;
        };

        alpha.into_inner()
    }
    #[must_use]
    pub fn quiesce_black(&self, alpha: i16, mut beta: i16) -> i16 {
        let stand_pat = self.static_evaluation();

        if stand_pat <= alpha {
            return alpha;
        }

        if stand_pat < beta {
            beta = stand_pat;
        }

        let beta = AtomicI16::new(beta);

        let mut moves = Vec::with_capacity(BRANCHING_FACTOR);

        for (position, piece) in self
            .positions_pieces()
            .filter(|(_, p)| p.team() == Team::Black)
        {
            piece.quiescence_moves(position, self, &mut moves);
        }

        quiescence_move_ordering(self, &mut moves);

        if let Err(alpha_cutoff) = moves.into_iter().try_for_each(|(from, to, _)| {
            if self[to].piece_value() == KING_VALUE {
                return Err(-KING_VALUE);
            }

            if stand_pat - DELTA - self[to].piece_value() < beta.load(Ordering::SeqCst) {
                let possible_board = self.make_move(from, to);

                let score = possible_board.quiesce_white(alpha, beta.load(Ordering::SeqCst));

                if score < beta.load(Ordering::SeqCst) {
                    if score <= alpha {
                        return Err(alpha);
                    }
                    beta.store(score, Ordering::SeqCst);
                }
            }

            Ok(())
        }) {
            return alpha_cutoff;
        }

        beta.into_inner()
    }
}
