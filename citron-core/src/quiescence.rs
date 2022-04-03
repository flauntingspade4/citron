use crate::{
    move_ordering::quiescence_move_ordering,
    piece::{PieceKind, KING_VALUE, PAWN_VALUE},
    Board, MoveGen, PlayableTeam,
};

const DELTA: i16 = 2 * PAWN_VALUE;

impl Board {
    #[must_use]
    pub fn quiesce(&self, mut alpha: i16, beta: i16) -> i16 {
        let stand_pat = if self.to_play == PlayableTeam::White {
            self.static_evaluation()
        } else {
            -self.static_evaluation()
        };

        if stand_pat > alpha {
            if stand_pat >= beta {
                return beta;
            }

            alpha = stand_pat;
        }

        let mut moves = MoveGen::new(self).into_inner();

        quiescence_move_ordering(&mut moves);

        if let Err(beta_cutoff) = moves
            .into_iter()
            .filter(|possible_move| possible_move.captured_piece_kind() != PieceKind::None)
            .try_for_each(|possible_move| {
                if possible_move.captured_piece_kind() == PieceKind::King {
                    return Err(KING_VALUE);
                }

                if self.in_endgame()
                    || stand_pat + DELTA + possible_move.captured_piece_kind().value() > alpha
                {
                    let possible_board = self.make_move(&possible_move).unwrap();

                    let score = -possible_board.quiesce(-beta, -alpha);

                    if score > alpha {
                        if score >= beta {
                            return Err(beta);
                        }
                        alpha = score;
                    }
                }

                Ok(())
            })
        {
            return beta_cutoff;
        };

        alpha
    }
}
