use std::collections::hash_map::Entry;

use crate::{
    killer::KillerMoves,
    move_ordering::move_ordering,
    piece::KING_VALUE,
    position::{position_to_u16, u16_to_position},
    transposition_table::{hash, TranspositionEntry, TranspositionTable},
    Board,
};

const INF: i16 = i16::MAX;

/// The average number of legal moves in any given position
pub const BRANCHING_FACTOR: usize = 35;
const ASPIRATION_WINDOW: i16 = 2;

const MULTICUT_M: usize = 5;
const MULTICUT_C: usize = 2;

#[derive(Debug, Clone, Copy)]
/// The possible different kinds of nodes that can be saved
pub enum Node {
    /// A principle variation node, one that is acceptable for
    /// both players
    PvNode(i16),
    /// An all node, where no move was good enough to improve alpha
    AllNode(i16),
}

impl Node {
    /// Returns the alpha score of the given node
    pub const fn into_inner(self) -> i16 {
        match self {
            Self::PvNode(s) | Self::AllNode(s) => s,
        }
    }
}

/// Prints information about the best moves calculated from a
/// given position, and what the best calculated response is
pub fn explore_line(mut starting_board: Board, transposition_table: &TranspositionTable) {
    for _ in 0..10 {
        if let Some(best) = transposition_table.get(&hash(&starting_board)) {
            let (from, to) = best.best_move;
            println!(
                "Best move in position: ({}) ({}) {:?}",
                from, to, best.evaluation
            );
            starting_board = starting_board.make_move(from, to).unwrap();
            println!("{}", starting_board);
            println!("{:?}", starting_board);
            println!("{:?}", *best);
        } else {
            break;
        }
    }
}

impl Board {
    #[must_use]
    /// Evaluate using iterative deepening, to a given depth.
    /// Just calls [`Board::iterative_deepening_ply`] with `depth * 2`
    pub fn iterative_deepening(&self, depth: u8) -> TranspositionTable {
        self.iterative_deepening_ply(depth * 2)
    }
    #[must_use]
    /// Evaluate using iterative deepening, to a given depth ply
    pub fn iterative_deepening_ply(&self, depth: u8) -> TranspositionTable {
        let mut beta = INF;
        let mut alpha = -INF;

        let mut transposition_table = TranspositionTable::new();
        let mut killer_table = Vec::new();
        killer_table.resize_with(depth as usize, KillerMoves::default);

        for i in 0..=depth {
            let eval = self.evaluate_private(
                i,
                0,
                alpha,
                beta,
                (&mut transposition_table, killer_table.as_mut_slice()),
                false,
            );
            if eval <= alpha || eval >= beta {
                beta = INF;
                alpha = -INF;
                transposition_table.clear();
                self.evaluate_private(
                    i,
                    0,
                    alpha,
                    beta,
                    (&mut transposition_table, killer_table.as_mut_slice()),
                    false,
                );
            } else {
                alpha = eval - ASPIRATION_WINDOW;
                beta = eval + ASPIRATION_WINDOW;
            }
        }

        transposition_table
    }
    /// The private evaluation function, that actually evaluates
    /// a position
    fn evaluate_private(
        &self,
        depth: u8,
        ply: u8,
        mut alpha: i16,
        beta: i16,
        (transposition_table, killer_table): (&mut TranspositionTable, &mut [KillerMoves]),
        previous_null: bool,
    ) -> i16 {
        if depth == 0 {
            return self.quiesce(alpha, beta);
        }

        let hash = hash(self);

        if let Some(t) = transposition_table.get(&hash) {
            if t.depth >= depth {
                if let Node::PvNode(evaluation) = t.evaluation {
                    return evaluation;
                }
            }
        }

        // Null move
        if !self.in_endgame() && depth >= 3 && !previous_null {
            let board = self.make_null_move();

            let value = -board.evaluate_private(
                depth.saturating_sub(4),
                ply,
                -beta,
                -(beta - 1),
                (transposition_table, killer_table),
                true,
            );

            if value >= beta {
                return value;
            }
        }

        // 0 to represent two empty `Position`s
        let mut best_move = 0;
        let mut best_evaluation = -KING_VALUE;
        let mut pv_search = true;

        let mut moves = Vec::with_capacity(BRANCHING_FACTOR);

        for (position, piece) in self
            .positions_pieces()
            .filter(|(_, p)| p.team() == self.to_play.into())
        {
            piece.legal_moves(position, self, &mut moves);
        }

        move_ordering(
            self,
            ply,
            &mut moves,
            (transposition_table, killer_table),
            hash,
        );

        // Multi-cut
        if depth >= 3 {
            let mut c = 0;

            if let Err(multi_cut) = moves.iter().take(MULTICUT_M).try_for_each(|(from, to, _)| {
                let possible_board = self.make_move(*from, *to).unwrap();

                let eval = -possible_board.evaluate_private(
                    depth.saturating_sub(3),
                    ply + 1,
                    -beta,
                    -(beta - 1),
                    (transposition_table, killer_table),
                    false,
                );

                if eval >= beta {
                    c += 1;
                    if c == MULTICUT_C {
                        return Err(beta);
                    }
                }

                Ok(())
            }) {
                return multi_cut;
            }
        }

        if let Err(beta_cutoff) =
            moves
                .into_iter()
                .enumerate()
                .try_for_each(|(index, (from, to, _))| {
                    if self[to].piece_value() == KING_VALUE {
                        return Err(KING_VALUE);
                    }

                    let possible_board = self.make_move(from, to).unwrap();

                    let score = if index > 3 && depth >= 2 && best_move == 0 {
                        let eval = -possible_board.evaluate_private(
                            depth - 2,
                            ply + 1,
                            -beta,
                            -alpha,
                            (transposition_table, killer_table),
                            false,
                        );
                        if eval > alpha {
                            -possible_board.evaluate_private(
                                depth - 1,
                                ply + 1,
                                -beta,
                                -alpha,
                                (transposition_table, killer_table),
                                false,
                            )
                        } else {
                            eval
                        }
                    } else if pv_search {
                        -possible_board.evaluate_private(
                            depth - 1,
                            ply + 1,
                            -beta,
                            -alpha,
                            (transposition_table, killer_table),
                            false,
                        )
                    } else {
                        let score = -possible_board.evaluate_private(
                            depth - 1,
                            ply + 1,
                            -(alpha + 1),
                            -alpha,
                            (transposition_table, killer_table),
                            false,
                        );

                        if score > alpha {
                            -possible_board.evaluate_private(
                                depth - 1,
                                ply + 1,
                                -beta,
                                -alpha,
                                (transposition_table, killer_table),
                                false,
                            )
                        } else {
                            score
                        }
                    };

                    if score > alpha {
                        if score >= beta {
                            if self[to].value() == 0 {
                                killer_table[ply as usize].add_move(from, to);
                            }

                            return Err(beta);
                        }
                        alpha = score;
                        best_evaluation = score;
                        best_move = position_to_u16((from, to));
                        pv_search = false;
                    }

                    Ok(())
                })
        {
            return beta_cutoff;
        };

        let transposition_entry = TranspositionEntry::new(
            depth,
            if best_move == 0 {
                Node::AllNode(best_evaluation)
            } else {
                Node::PvNode(alpha)
            },
            u16_to_position(best_move),
        );

        match transposition_table.entry(hash) {
            Entry::Occupied(mut entry) => {
                if entry.get().depth <= depth {
                    entry.insert(transposition_entry);
                }
            }
            Entry::Vacant(entry) => {
                entry.insert(transposition_entry);
            }
        }

        alpha
    }
}
