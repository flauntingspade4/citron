use std::collections::hash_map::Entry;

use crate::{
    killer::KillerMoves,
    move_ordering::move_ordering,
    piece::KING_VALUE,
    position::{position_to_u16, u16_to_position},
    transposition_table::{hash, TranspositionEntry, TranspositionTable},
    Board, PlayableTeam, Team,
};

pub const BRANCHING_FACTOR: usize = 35;
const ASPIRATION_WINDOW: i16 = 2;

#[derive(Debug, Clone, Copy)]
pub enum Node {
    PvNode(i16),
    AllNode(i16),
}

impl Node {
    pub fn into_inner(self) -> i16 {
        match self {
            Node::PvNode(s) => s,
            Node::AllNode(s) => s,
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
            starting_board = starting_board.make_move(from, to);
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
        let mut beta = KING_VALUE;
        let mut alpha = -KING_VALUE;

        let mut transposition_table = TranspositionTable::new();
        let mut killer_table = Vec::new();
        killer_table.resize_with(depth as usize, KillerMoves::default);

        #[cfg(feature = "debug")]
        let (mut time_taken, mut start) = (
            Vec::with_capacity(depth as usize),
            std::time::Instant::now(),
        );

        match self.to_play {
            PlayableTeam::White => {
                for i in 0..=depth {
                    let eval = self.evaluate_private_white(
                        i,
                        0,
                        alpha,
                        beta,
                        (&mut transposition_table, killer_table.as_mut_slice()),
                        false,
                    );
                    if eval <= alpha || eval >= beta {
                        beta = KING_VALUE;
                        alpha = -KING_VALUE;
                        self.evaluate_private_white(
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

                    #[cfg(feature = "debug")]
                    {
                        time_taken.push(start.elapsed().as_nanos());
                        start = std::time::Instant::now();
                    }
                }
            }
            PlayableTeam::Black => {
                for i in 0..=depth {
                    let eval = self.evaluate_private_black(
                        i,
                        0,
                        alpha,
                        beta,
                        (&mut transposition_table, killer_table.as_mut_slice()),
                        false,
                    );
                    if eval <= alpha || eval >= beta {
                        beta = KING_VALUE;
                        alpha = -KING_VALUE;
                        self.evaluate_private_black(
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

                    #[cfg(feature = "debug")]
                    {
                        time_taken.push(start.elapsed().as_nanos());
                        start = std::time::Instant::now();
                    }
                }
            }
        }

        #[cfg(feature = "debug")]
        println!("{:?}", time_taken);

        transposition_table
    }
    #[must_use]
    pub fn evaluate(&self, depth: u8) -> TranspositionTable {
        self.evaluate_ply(depth * 2)
    }
    #[must_use]
    pub fn evaluate_ply(&self, depth: u8) -> TranspositionTable {
        let beta = KING_VALUE;
        let alpha = -KING_VALUE;

        let mut transposition_table = TranspositionTable::new();
        let mut killer_table = Vec::new();
        killer_table.resize_with(depth as usize, KillerMoves::default);

        let tables = (&mut transposition_table, killer_table.as_mut_slice());

        match self.to_play {
            PlayableTeam::White => {
                self.evaluate_private_white(depth, 0, alpha, beta, tables, false);
            }
            PlayableTeam::Black => {
                self.evaluate_private_black(depth, 0, alpha, beta, tables, false);
            }
        }

        transposition_table
    }
    fn evaluate_private_white(
        &self,
        depth: u8,
        ply: u8,
        mut alpha: i16,
        beta: i16,
        (transposition_table, killer_table): (&mut TranspositionTable, &mut [KillerMoves]),
        previous_null: bool,
    ) -> i16 {
        if depth == 0 {
            return self.quiesce_white(alpha, beta);
        }

        let hash = hash(self);

        if let Some(t) = transposition_table.get(&hash) {
            if t.depth > depth {
                if let Node::PvNode(evaluation) = t.evaluation {
                    return evaluation;
                }
            }
        }

        // Null move
        if depth >= 3 && !previous_null {
            let board = self.make_null_move();

            let value = board.evaluate_private_black(
                depth - 3,
                ply,
                beta - 1,
                beta,
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

        let mut moves = Vec::with_capacity(BRANCHING_FACTOR);

        for (position, piece) in self
            .positions_pieces()
            .filter(|(_, p)| p.team() == Team::White)
        {
            piece.legal_moves(position, self, &mut moves);
        }

        move_ordering(
            self,
            ply,
            &mut moves,
            transposition_table,
            killer_table,
            hash,
        );

        if let Err(beta_cutoff) =
            moves
                .into_iter()
                .enumerate()
                .try_for_each(|(index, (from, to, _))| {
                    if self[to].piece_value() == KING_VALUE {
                        return Err(KING_VALUE);
                    }

                    let possible_board = self.make_move(from, to);

                    /*let score = possible_board.evaluate_private_black(
                        depth - 1,
                        alpha,
                        beta,
                        transposition_table,
                        false,
                    );*/

                    let score = if index > 3 && depth >= 2 && best_move == 0 {
                        let eval = possible_board.evaluate_private_black(
                            depth.saturating_sub(2),
                            ply + 1,
                            alpha,
                            beta,
                            (transposition_table, killer_table),
                            false,
                        );
                        if eval > alpha {
                            possible_board.evaluate_private_black(
                                depth - 1,
                                ply + 1,
                                alpha,
                                beta,
                                (transposition_table, killer_table),
                                false,
                            )
                        } else {
                            eval
                        }
                    } else {
                        possible_board.evaluate_private_black(
                            depth - 1,
                            ply + 1,
                            alpha,
                            beta,
                            (transposition_table, killer_table),
                            false,
                        )
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
    fn evaluate_private_black(
        &self,
        depth: u8,
        ply: u8,
        alpha: i16,
        mut beta: i16,
        (transposition_table, killer_table): (&mut TranspositionTable, &mut [KillerMoves]),
        previous_null: bool,
    ) -> i16 {
        if depth == 0 {
            return self.quiesce_black(alpha, beta);
        }

        let hash = hash(self);

        if let Some(t) = transposition_table.get(&hash) {
            if t.depth > depth {
                if let Node::PvNode(evaluation) = t.evaluation {
                    return evaluation;
                }
            }
        }

        // Null move
        if depth >= 3 && !previous_null {
            let board = self.make_null_move();

            let value = board.evaluate_private_white(
                depth - 3,
                ply,
                alpha,
                alpha + 1,
                (transposition_table, killer_table),
                true,
            );

            if value <= alpha {
                return value;
            }
        }

        // 0 to represent two empty `Position`s
        let mut best_move = 0;
        let mut best_evaluation = KING_VALUE;

        let mut moves = Vec::with_capacity(BRANCHING_FACTOR);

        for (position, piece) in self
            .positions_pieces()
            .filter(|(_, p)| p.team() == Team::Black)
        {
            piece.legal_moves(position, self, &mut moves);
        }

        move_ordering(
            self,
            ply,
            &mut moves,
            transposition_table,
            killer_table,
            hash,
        );

        if let Err(alpha_cutoff) =
            moves
                .into_iter()
                .enumerate()
                .try_for_each(|(index, (from, to, _))| {
                    if self[to].piece_value() == KING_VALUE {
                        return Err(-KING_VALUE);
                    }

                    let possible_board = self.make_move(from, to);

                    let score = if index > 3 && depth >= 2 && best_move == 0 {
                        let eval = possible_board.evaluate_private_white(
                            depth.saturating_sub(2),
                            ply + 1,
                            alpha,
                            beta,
                            (transposition_table, killer_table),
                            false,
                        );
                        if eval < beta {
                            possible_board.evaluate_private_white(
                                depth - 1,
                                ply + 1,
                                alpha,
                                beta,
                                (transposition_table, killer_table),
                                false,
                            )
                        } else {
                            eval
                        }
                    } else {
                        possible_board.evaluate_private_white(
                            depth - 1,
                            ply + 1,
                            alpha,
                            beta,
                            (transposition_table, killer_table),
                            false,
                        )
                    };

                    if score < beta {
                        if score <= alpha {
                            if self[to].value() == 0 {
                                killer_table[ply as usize].add_move(from, to);
                            }

                            return Err(alpha);
                        }
                        beta = score;
                        best_evaluation = score;
                        best_move = position_to_u16((from, to));
                    }

                    Ok(())
                })
        {
            return alpha_cutoff;
        }

        let transposition_entry = TranspositionEntry::new(
            depth,
            if best_move == 0 {
                Node::AllNode(best_evaluation)
            } else {
                Node::PvNode(beta)
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

        beta
    }
}

#[test]
fn good_test() {
    let board = Board::from_fen("3rk2r/1p4pp/p1p1bp2/2Bn4/B7/P4P2/3K3P/5R2 b k - 3 22").unwrap();

    println!("{}", board);

    let start = std::time::Instant::now();

    let table = board.iterative_deepening(4);

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

    let starting_board = board.make_move(from, to);

    // explore_line(starting_board, &table);

    #[cfg(feature = "debug")]
    println!(
        "{} nodes/s",
        crate::evaluation::POSITIONS_CONSIDERED * 1000 / elapsed as usize
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
        crate::evaluation::POSITIONS_CONSIDERED * 1000 / elapsed as usize
    );
}

#[test]
fn fight_self() {
    use crate::pgn::Pgn;

    let mut fast_to_play = PlayableTeam::White;
    let mut fast_won = 0;
    let mut slow_won = 0;

    let depth = 4;

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

                    board = board.make_move(from, to);

                    let table = board.iterative_deepening(depth);

                    let best = table.get(&hash(&board)).unwrap();
                    let (from, to) = best.best_move;

                    if best.evaluation.into_inner() == KING_VALUE {
                        slow_won += 1;
                        break;
                    }

                    pgn.add_move((from, to), &board);

                    board = board.make_move(from, to);

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

                    board = board.make_move(from, to);

                    let table = board.iterative_deepening(depth);

                    let best = table.get(&hash(&board)).unwrap();
                    let (from, to) = best.best_move;

                    if best.evaluation.into_inner() == KING_VALUE {
                        fast_won += 1;
                        break;
                    }

                    pgn.add_move((from, to), &board);

                    board = board.make_move(from, to);
                }
            }
        }

        println!("{}", pgn.finish());

        println!("{} {}", fast_won, slow_won);

        fast_to_play = !fast_to_play;
    }
}
