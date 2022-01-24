use crate::{
    analysis::BRANCHING_FACTOR,
    move_ordering::quiescence_move_ordering,
    piece::{KING_VALUE, PAWN_VALUE},
    Board, PlayableTeam,
};

const DELTA: i16 = 2 * PAWN_VALUE;

impl Board {
    #[must_use]
    pub fn quiesce(&self, mut alpha: i16, beta: i16) -> i16 {
        let stand_pat = self.static_evaluation();
        let stand_pat = if self.to_play == PlayableTeam::White {
            stand_pat
        } else {
            -stand_pat
        };

        if stand_pat > alpha {
            if stand_pat >= beta {
                return beta;
            }

            alpha = stand_pat;
        }

        let mut moves = Vec::with_capacity(BRANCHING_FACTOR);

        for (position, piece) in self
            .positions_pieces()
            .filter(|(_, p)| p.team() == self.to_play.into())
        {
            piece.quiescence_moves(position, self, &mut moves);
        }

        quiescence_move_ordering(self, &mut moves);

        if let Err(beta_cutoff) = moves.into_iter().try_for_each(|(from, to, _)| {
            if self[to].piece_value() == KING_VALUE {
                return Err(KING_VALUE);
            }

            if self.in_endgame() || stand_pat + DELTA + self[to].piece_value() > alpha {
                let possible_board = self.make_move(from, to).unwrap();

                let score = -possible_board.quiesce(-beta, -alpha);

                if score > alpha {
                    if score >= beta {
                        return Err(beta);
                    }
                    alpha = score;
                }
            }

            Ok(())
        }) {
            return beta_cutoff;
        };

        alpha
    }
}
