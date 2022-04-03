use std::collections::hash_map::Entry;

use crate::{
    killer::KillerMoves,
    move_ordering::move_ordering,
    piece::{PieceKind, KING_VALUE},
    transposition_table::{TranspositionEntry, TranspositionTable},
    Board, MoveGen,
};

const ASPIRATION_WINDOW: i16 = 25;
const INF: i16 = std::i16::MAX;

const MULTICUT_M: usize = 5;
const MULTICUT_C: usize = 2;

#[derive(Debug, Clone, Copy)]
pub enum Node {
    PvNode(i16),
    AllNode(i16),
    CutNode(i16),
}

impl Node {
    #[must_use]
    pub const fn into_inner(self) -> i16 {
        match self {
            Self::PvNode(s) | Self::AllNode(s) | Self::CutNode(s) => s,
        }
    }
}

pub fn explore_line(mut starting_board: Board, transposition_table: &TranspositionTable) {
    for _ in 0..10 {
        if let Some(best) = transposition_table.get(&starting_board.hash) {
            let (from, to) = best.best_move.from_to();
            println!(
                "Best move in position: ({}) ({}) {:?}",
                from, to, best.evaluation
            );

            starting_board = starting_board.make_move(&best.best_move).unwrap();

            let (from, to) = best.best_move.from_to();
            let (fx, fy) = from.to_uci();
            let (tx, ty) = to.to_uci();
            println!("{fx}{fy} {tx}{ty}");
            println!("{}", starting_board);
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
        let mut beta = INF;
        let mut alpha = -INF;

        let mut transposition_table = TranspositionTable::new();
        let mut killer_table = Vec::with_capacity(depth as usize);

        killer_table.resize_with(depth as usize, KillerMoves::default);

        for depth in 0..=depth {
            for i in 0.. {
                let eval = self.evaluate_private(
                    depth,
                    0,
                    alpha,
                    beta,
                    (&mut transposition_table, killer_table.as_mut_slice()),
                );
                if eval <= alpha {
                    alpha -= ASPIRATION_WINDOW << (2 * i);
                } else if eval >= beta {
                    beta += ASPIRATION_WINDOW << (2 * i);
                } else {
                    alpha = eval - ASPIRATION_WINDOW;
                    beta = eval + ASPIRATION_WINDOW;
                    break;
                }
            }
        }

        transposition_table
    }
    fn evaluate_private(
        &self,
        depth: u8,
        ply: u8,
        mut alpha: i16,
        beta: i16,
        (transposition_table, killer_table): (&mut TranspositionTable, &mut [KillerMoves]),
    ) -> i16 {
        if depth == 0 {
            return self.static_evaluation();
        }

        if let Some(t) = transposition_table.get(&self.hash) {
            if t.depth > depth {
                if let Node::PvNode(evaluation) = t.evaluation {
                    return evaluation;
                }
            }
        }

        let mut best_move = None;
        let mut pv_search = true;

        let mut moves = MoveGen::new(self).into_inner();

        move_ordering(
            ply,
            &mut moves,
            (transposition_table, killer_table),
            self.hash,
        );

        // Multi-cut
        if depth >= 3 {
            let mut c = 0;

            if let Err(multi_cut) = moves.iter().take(MULTICUT_M).try_for_each(|possible_move| {
                let possible_board = self.make_move(possible_move).unwrap();

                let eval = -possible_board.evaluate_private(
                    depth - 3,
                    ply + 1,
                    -beta,
                    -(beta - 1),
                    (transposition_table, killer_table),
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

        if let Err((beta_cutoff, possible_move)) =
            moves
                .into_iter()
                .enumerate()
                .try_for_each(|(index, possible_move)| {
                    // If the move considered is the capture of a king
                    if possible_move.captured_piece_kind() == PieceKind::King {
                        if ply == 0 {
                            transposition_table.insert(
                                self.hash,
                                TranspositionEntry::new(
                                    depth,
                                    Node::PvNode(KING_VALUE),
                                    possible_move.clone(),
                                ),
                            );
                        }
                        return Err((KING_VALUE, possible_move));
                    }

                    let possible_board = self.make_move(&possible_move).unwrap();

                    let score = if index > 3 && depth >= 3 && best_move.is_none() {
                        let eval = -possible_board.evaluate_private(
                            depth - 3,
                            ply + 1,
                            -beta,
                            -alpha,
                            (transposition_table, killer_table),
                        );
                        if eval > alpha {
                            -possible_board.evaluate_private(
                                depth - 1,
                                ply + 1,
                                -beta,
                                -alpha,
                                (transposition_table, killer_table),
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
                        )
                    } else {
                        let score = -possible_board.evaluate_private(
                            depth - 1,
                            ply + 1,
                            -(alpha + 1),
                            -alpha,
                            (transposition_table, killer_table),
                        );

                        if score > alpha {
                            -possible_board.evaluate_private(
                                depth - 1,
                                ply + 1,
                                -beta,
                                -alpha,
                                (transposition_table, killer_table),
                            )
                        } else {
                            score
                        }
                    };

                    if score > alpha {
                        if score >= beta {
                            if possible_move.captured_piece_kind() == PieceKind::None {
                                killer_table[ply as usize].add_move(possible_move.from_to());
                            }

                            return Err((beta, possible_move));
                        }

                        alpha = score;
                        best_move = Some(possible_move);
                        pv_search = false;
                    }

                    Ok(())
                })
        {
            let transposition_entry =
                TranspositionEntry::new(depth, Node::CutNode(beta_cutoff), possible_move);

            match transposition_table.entry(self.hash) {
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

        if let Some(best_move) = best_move {
            let transposition_entry =
                TranspositionEntry::new(depth, Node::PvNode(alpha), best_move);

            match transposition_table.entry(self.hash) {
                Entry::Occupied(mut entry) => {
                    if entry.get().depth <= depth {
                        entry.insert(transposition_entry);
                    }
                }
                Entry::Vacant(entry) => {
                    entry.insert(transposition_entry);
                }
            }
        }

        alpha
    }
}

#[test]
fn good_test() {
    let board =
        Board::from_fen("r2q1rk1/1p3p1p/1b4p1/pPp2b2/3pn1P1/P2Q4/B1P1NP1P/R1B2RK1 b - - 0 30")
            .unwrap();

    println!("{}", board);

    let start = std::time::Instant::now();

    let table = board.iterative_deepening_ply(12);

    let elapsed = start.elapsed().as_millis();

    let best = table.get(&board.hash()).unwrap();
    let (from, to) = best.best_move.from_to();

    println!(
        "{}ms ({}) ({}) {}",
        elapsed,
        from,
        to,
        best.evaluation.into_inner() as f64 / 100.,
    );

    let starting_board = board.make_move(&best.best_move).unwrap();

    explore_line(starting_board, &table);

    #[cfg(feature = "debug")]
    println!(
        "{}k nodes/s",
        POSITIONS_CONSIDERED.load(Ordering::SeqCst) / elapsed as usize,
    );
}

/*
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
        "{}k nodes/s",
        POSITIONS_CONSIDERED.load(Ordering::SeqCst) / elapsed as usize
    );
}

#[test]
fn fight_self() {
    use crate::{pgn::Pgn, PlayableTeam};

    let mut fast_to_play = PlayableTeam::White;
    let mut fast_won = 0;
    let mut slow_won = 0;

    let depth = 4;

    for _ in 0..1 {
        let mut board = Board::new();

        let mut pgn = Pgn::new();

        match fast_to_play {
            PlayableTeam::White => {
                for i in 0..40 {
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
*/
