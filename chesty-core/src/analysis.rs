use std::collections::hash_map::Entry;

use crate::{
    killer::KillerMoves,
    move_ordering::move_ordering,
    piece::KING_VALUE,
    position::{position_to_u16, u16_to_position},
    transposition_table::{hash, TranspositionEntry, TranspositionTable},
    Board,
};

pub const BRANCHING_FACTOR: usize = 35;
const ASPIRATION_WINDOW: i16 = 25;

const MULTICUT_M: usize = 5;
const MULTICUT_C: usize = 2;

#[derive(Debug, Clone, Copy)]
pub enum Node {
    PvNode(i16),
    AllNode(i16),
    CutNode(i16),
}

impl Node {
    pub const fn into_inner(self) -> i16 {
        match self {
            Self::PvNode(s) | Self::AllNode(s) => s,
        }
    }
}

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
    pub fn iterative_deepening(&self, depth: u8) -> TranspositionTable {
        self.iterative_deepening_ply(depth * 2)
    }
    #[must_use]
    pub fn iterative_deepening_ply(&self, depth: u8) -> TranspositionTable {
        let mut beta = KING_VALUE + 1;
        let mut alpha = -KING_VALUE - 1;

        let mut transposition_table = TranspositionTable::new();
        let mut killer_table = Vec::new();
        killer_table.resize_with(depth as usize, KillerMoves::default);

        #[cfg(feature = "debug")]
        let bar = indicatif::ProgressBar::new(depth as u64);

        for depth in 0..=depth {
            for i in 0.. {
                let eval = self.evaluate_private(
                    depth,
                    0,
                    alpha,
                    beta,
                    (&mut transposition_table, killer_table.as_mut_slice()),
                    false,
                );
                if eval <= alpha {
                    alpha -= ASPIRATION_WINDOW << (3 * i);
                } else if eval >= beta {
                    beta += ASPIRATION_WINDOW << (3 * i);
                } else {
                    alpha = eval - ASPIRATION_WINDOW;
                    beta = eval + ASPIRATION_WINDOW;
                    break;
                }
            }
            #[cfg(feature = "debug")]
            bar.inc(1);
        }

        #[cfg(feature = "debug")]
        bar.finish();

        transposition_table
    }
    #[must_use]
    pub fn evaluate(&self, depth: u8) -> TranspositionTable {
        self.evaluate_ply(depth * 2)
    }
    #[must_use]
    pub fn evaluate_ply(&self, depth: u8) -> TranspositionTable {
        let beta = KING_VALUE + 1;
        let alpha = -KING_VALUE - 1;

        let mut transposition_table = TranspositionTable::new();
        let mut killer_table = Vec::new();
        killer_table.resize_with(depth as usize, KillerMoves::default);

        let tables = (&mut transposition_table, killer_table.as_mut_slice());

        self.evaluate_private(depth, 0, alpha, beta, tables, false);

        transposition_table
    }
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
            return self.quiesce(alpha, beta, ply);
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
        /*if !self.in_endgame() && depth >= 3 && !previous_null {
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
        }*/

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
                    depth - 3,
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

        if let Err((beta_cutoff, from, to)) =
            moves
                .into_iter()
                .enumerate()
                .try_for_each(|(index, (from, to, _))| {
                    if self[to].piece_value() == KING_VALUE {
                        // If the first move considered is a capture of a king
                        if ply == 0 {
                            transposition_table.insert(
                                hash,
                                TranspositionEntry::new(
                                    depth,
                                    Node::PvNode(self[to].value()),
                                    (from, to),
                                ),
                            );
                        }
                        return Err((KING_VALUE, from, to));
                    }

                    let possible_board = self.make_move(from, to).unwrap();

                    let score = if index > 3 && depth >= 3 && best_move == 0 {
                        let eval = -possible_board.evaluate_private(
                            depth - 3,
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

                            return Err((beta, from, to));
                        }
                        alpha = score;
                        best_evaluation = score;
                        best_move = position_to_u16((from, to));
                        pv_search = false;
                    }

                    Ok(())
                })
        {
            let transposition_entry =
                TranspositionEntry::new(depth, Node::CutNode(beta_cutoff), (from, to));

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

#[test]
fn good_test() {
    let board = Board::from_fen("3rk2r/1p4pp/p1p1bp2/2Bn4/B7/P4P2/3K3P/5R2 b k - 3 22").unwrap();

    println!("{}", board);

    let start = std::time::Instant::now();

    let table = board.iterative_deepening(5);

    let elapsed = start.elapsed().as_millis();

    let best = table.get(&hash(&board)).unwrap();
    let (from, to) = best.best_move;

    println!(
        "{}ms ({}) ({}) {}",
        elapsed,
        from,
        to,
        best.evaluation.into_inner() as f64 / 100.,
    );

    // let starting_board = board.make_move(from, to).unwrap();

    // explore_line(starting_board, &table);

    #[cfg(feature = "debug")]
    println!(
        "{} nodes/s",
        POSITIONS_CONSIDERED.load(Ordering::SeqCst) * 1000 / elapsed as usize,
    );
}

#[test]
fn horde() {
    let board = Board::from_fen(
        "rnbqkbnr/pppppppp/8/1PP2PP1/PPPPPPPP/PPPPPPPP/PPPPPPPP/PPPPPPPP w - - 0 0",
    )
    .unwrap();

    println!("{}", board);

    let start = std::time::Instant::now();

    let table = board.iterative_deepening(6);

    let best = table.get(&hash(&board)).unwrap();
    let (from, to) = best.best_move;

    let elapsed = start.elapsed().as_millis();

    println!(
        "{}ms ({}) ({}) {}",
        elapsed,
        from,
        to,
        best.evaluation.into_inner()
    );

    #[cfg(feature = "debug")]
    println!(
        "{} nodes/s",
        POSITIONS_CONSIDERED.load(Ordering::SeqCst) * 1000 / elapsed as usize
    );
}

#[test]
fn fight_self() {
    use crate::{pgn::Pgn, PlayableTeam};

    let mut fast_to_play = PlayableTeam::White;
    let mut fast_won = 0;
    let mut slow_won = 0;

    let depth = 6;

    for _ in 0..1 {
        let mut board = Board::new();

        let mut pgn = Pgn::new();

        match fast_to_play {
            PlayableTeam::White => {
                for i in 0..80 {
                    let table = board.iterative_deepening(depth);

                    let best = table.get(&hash(&board)).unwrap();
                    let (from, to) = best.best_move;

                    if best.evaluation.into_inner() == -KING_VALUE {
                        fast_won += 1;
                        break;
                    }

                    pgn.add_move((from, to), &board);

                    board = board.make_move(from, to).unwrap();

                    let table = board.iterative_deepening_ply(depth - 1);

                    let best = table.get(&hash(&board)).unwrap();
                    let (from, to) = best.best_move;

                    if best.evaluation.into_inner() == KING_VALUE {
                        slow_won += 1;
                        break;
                    }

                    pgn.add_move((from, to), &board);

                    board = board.make_move(from, to).unwrap();

                    if i % 5 == 0 {
                        println!("Done move {}", i);
                    }
                }
            }
            PlayableTeam::Black => {
                for _ in 0..40 {
                    let table = board.iterative_deepening(depth);

                    let best = table.get(&hash(&board)).unwrap();
                    let (from, to) = best.best_move;

                    if best.evaluation.into_inner() == -KING_VALUE {
                        slow_won += 1;
                        break;
                    }

                    pgn.add_move((from, to), &board);

                    board = board.make_move(from, to).unwrap();

                    let table = board.iterative_deepening(depth);

                    let best = table.get(&hash(&board)).unwrap();
                    let (from, to) = best.best_move;

                    if best.evaluation.into_inner() == KING_VALUE {
                        fast_won += 1;
                        break;
                    }

                    pgn.add_move((from, to), &board);

                    board = board.make_move(from, to).unwrap();
                }
            }
        }

        println!("{}", pgn.finish());

        println!("{} {}", fast_won, slow_won);

        fast_to_play = !fast_to_play;
    }
}
