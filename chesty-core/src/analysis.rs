use core::sync::atomic::{AtomicI16, AtomicU16, Ordering};

use crate::{
    killer::KillerMoves,
    move_ordering::move_ordering,
    piece::{KING_VALUE, PAWN_VALUE},
    position::{position_to_u16, u16_to_position},
    transposition_table::{hash, TranspositionEntry, TranspositionTable},
    Board, PlayableTeam, Team,
};

use dashmap::mapref::entry::Entry;
use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};

pub const BRANCHING_FACTOR: usize = 35;
const ASPIRATION_WINDOW: i16 = PAWN_VALUE >> 2;

pub fn explore_line(mut starting_board: Board, transposition_table: &TranspositionTable) {
    for _ in 0..10 {
        if let Some(best) = transposition_table.get(&hash(&starting_board)) {
            let (from, to) = best.value().best_move;
            println!(
                "Best move in position: ({}) ({}) {}",
                from,
                to,
                best.value().evaluation
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
        let mut beta = i16::MAX;
        let mut alpha = -beta;

        let transposition_table = TranspositionTable::new();
        let mut killer_table = Vec::new();
        killer_table.resize_with(depth as usize, KillerMoves::default);

        let tables = (&transposition_table, killer_table.as_slice());

        match self.to_play {
            PlayableTeam::White => {
                for i in 0..depth {
                    let eval = self.evaluate_private_white(i, 0, alpha, beta, tables, true);
                    if eval <= alpha || eval >= beta {
                        beta = i16::MAX;
                        alpha = -beta;
                        self.evaluate_private_white(i, 0, alpha, beta, tables, true);
                    } else {
                        alpha = eval - ASPIRATION_WINDOW;
                        beta = eval + ASPIRATION_WINDOW;
                    }
                }
                self.evaluate_private_white(depth, 0, alpha, beta, tables, true);
            }
            PlayableTeam::Black => {
                for i in 0..depth {
                    let eval = self.evaluate_private_black(i, 0, alpha, beta, tables, true);
                    if eval <= alpha || eval >= beta {
                        beta = i16::MAX;
                        alpha = -beta;
                        self.evaluate_private_black(i, 0, alpha, beta, tables, true);
                    } else {
                        alpha = eval - ASPIRATION_WINDOW;
                        beta = eval + ASPIRATION_WINDOW;
                    }
                }
                self.evaluate_private_black(depth, 0, alpha, beta, tables, true);
            }
        }

        transposition_table
    }
    #[must_use]
    pub fn evaluate(&self, depth: u8) -> TranspositionTable {
        self.evaluate_ply(depth * 2)
    }
    #[must_use]
    pub fn evaluate_ply(&self, depth: u8) -> TranspositionTable {
        let beta = i16::MAX;
        let alpha = -beta;

        let transposition_table = TranspositionTable::new();
        let mut killer_table = Vec::new();
        killer_table.resize_with(depth as usize, KillerMoves::default);

        let tables = (&transposition_table, killer_table.as_slice());

        match self.to_play {
            PlayableTeam::White => {
                self.evaluate_private_white(depth, 0, alpha, beta, tables, true);
            }
            PlayableTeam::Black => {
                self.evaluate_private_black(depth, 0, alpha, beta, tables, true);
            }
        }

        transposition_table
    }
    fn evaluate_private_white(
        &self,
        depth: u8,
        ply: u8,
        alpha: i16,
        beta: i16,
        (transposition_table, killer_table): (&TranspositionTable, &[KillerMoves]),
        pv_child: bool,
    ) -> i16 {
        if depth == 0 {
            return self.quiesce_white(alpha, beta);
        }

        let hash = hash(self);

        if let Some(t) = transposition_table.get(&hash) {
            if t.value().depth > depth {
                return t.value().evaluation;
            }
        }

        // 0 to represent two empty `Position`s
        let best_move = AtomicU16::new(0);
        let alpha = AtomicI16::new(alpha);

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
                .into_par_iter()
                .enumerate()
                .try_for_each(|(index, (from, to, _))| {
                    if self[to].piece_value() == KING_VALUE {
                        return Err(KING_VALUE);
                    }

                    let possible_board = self.make_move(from, to);

                    /*let score = possible_board.evaluate_private_black(
                        depth - 1,
                        alpha.load(Ordering::SeqCst),
                        beta,
                        transposition_table,
                        false,
                    );*/

                    let score = if index > 4
                        && !pv_child
                        && depth >= 2
                        && best_move.load(Ordering::SeqCst) == 0
                    {
                        let eval = possible_board.evaluate_private_black(
                            depth.saturating_sub(2),
                            ply + 1,
                            alpha.load(Ordering::SeqCst),
                            beta,
                            (transposition_table, killer_table),
                            false,
                        );
                        if eval > alpha.load(Ordering::SeqCst) {
                            possible_board.evaluate_private_black(
                                depth - 1,
                                ply + 1,
                                alpha.load(Ordering::SeqCst),
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
                            alpha.load(Ordering::SeqCst),
                            beta,
                            (transposition_table, killer_table),
                            best_move.load(Ordering::SeqCst) != 0,
                        )
                    };

                    if score > alpha.load(Ordering::SeqCst) {
                        if score >= beta {
                            if self[to].value() == 0 {
                                killer_table[ply as usize].add_move(from, to);
                            }

                            return Err(beta);
                        }
                        alpha.store(score, Ordering::SeqCst);
                        best_move.store(position_to_u16((from, to)), Ordering::SeqCst);
                    }

                    Ok(())
                })
        {
            return beta_cutoff;
        };

        let alpha = alpha.into_inner();
        let best_move = best_move.into_inner();

        let best_move = u16_to_position(best_move);

        let transposition_entry = TranspositionEntry::new(depth, alpha, best_move);

        match transposition_table.entry(hash) {
            Entry::Occupied(mut entry) => {
                if entry.get().depth < depth {
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
        beta: i16,
        (transposition_table, killer_table): (&TranspositionTable, &[KillerMoves]),
        pv_child: bool,
    ) -> i16 {
        if depth == 0 {
            return self.quiesce_black(alpha, beta);
        }

        let hash = hash(self);

        if let Some(t) = transposition_table.get(&hash) {
            if t.value().depth > depth {
                return t.value().evaluation;
            }
        }

        // 0 to represent two empty `Position`s
        let best_move = AtomicU16::new(0);
        let beta = AtomicI16::new(beta);

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
                .into_par_iter()
                .enumerate()
                .try_for_each(|(index, (from, to, _))| {
                    if self[to].piece_value() == KING_VALUE {
                        return Err(-KING_VALUE);
                    }

                    let possible_board = self.make_move(from, to);

                    let score = if index > 4
                        && !pv_child
                        && depth >= 2
                        && best_move.load(Ordering::SeqCst) == 0
                    {
                        let eval = possible_board.evaluate_private_white(
                            depth.saturating_sub(2),
                            ply + 1,
                            alpha,
                            beta.load(Ordering::SeqCst),
                            (transposition_table, killer_table),
                            false,
                        );
                        if eval < beta.load(Ordering::SeqCst) {
                            possible_board.evaluate_private_white(
                                depth - 1,
                                ply + 1,
                                alpha,
                                beta.load(Ordering::SeqCst),
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
                            beta.load(Ordering::SeqCst),
                            (transposition_table, killer_table),
                            best_move.load(Ordering::SeqCst) != 0,
                        )
                    };

                    if score < beta.load(Ordering::SeqCst) {
                        if score <= alpha {
                            if self[to].value() == 0 {
                                killer_table[ply as usize].add_move(from, to);
                            }

                            return Err(alpha);
                        }
                        beta.store(score, Ordering::SeqCst);
                        best_move.store(position_to_u16((from, to)), Ordering::SeqCst);
                    }

                    Ok(())
                })
        {
            return alpha_cutoff;
        }

        let beta = beta.into_inner();
        let best_move = best_move.into_inner();

        let best_move = u16_to_position(best_move);

        let transposition_entry = TranspositionEntry::new(depth, beta, best_move);

        match transposition_table.entry(hash) {
            Entry::Occupied(mut entry) => {
                if entry.get().depth < depth {
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
    let board = Board::from_fen("5r2/p4p1p/2p2p2/2pn3P/5PPk/P2bP1R1/1r1P1K2/R7 w - - 0 1").unwrap();
    // Board::from_fen("r1bqkb1r/pp5p/4pPp1/1Npp4/3n4/3Q4/PPPP1PPP/R1B1KB1R b KQkq - 1 11").unwrap();

    println!("{}", board);

    let start = std::time::Instant::now();

    let table = board.iterative_deepening(4);

    let elapsed = start.elapsed().as_millis();

    let best = table.get(&hash(&board)).unwrap();
    let (from, to) = best.value().best_move;

    println!(
        "{}ms ({}) ({}) {}",
        elapsed,
        from,
        to,
        best.value().evaluation as f64 / 10.,
    );

    // explore_line(board, &table);

    #[cfg(feature = "debug")]
    println!(
        "{}",
        crate::evaluation::POSITIONS_CONSIDERED.load(Ordering::SeqCst) * 1000 / elapsed as usize
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
    let (from, to) = best.value().best_move;

    let elapsed = start.elapsed().as_millis();

    println!(
        "{}ms ({}) ({}) {}",
        elapsed,
        from,
        to,
        best.value().evaluation
    );

    #[cfg(feature = "debug")]
    println!(
        "{} nodes/s",
        crate::evaluation::POSITIONS_CONSIDERED.load(Ordering::SeqCst) * 1000 / elapsed as usize
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
                    let (from, to) = best.value().best_move;

                    if best.value().evaluation == -KING_VALUE {
                        fast_won += 1;
                        break;
                    }

                    pgn.add_move((from, to), &board);

                    board = board.make_move(from, to);

                    let table = board.iterative_deepening(depth);

                    let best = table.get(&hash(&board)).unwrap();
                    let (from, to) = best.value().best_move;

                    if best.value().evaluation == KING_VALUE {
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
                    let (from, to) = best.value().best_move;

                    if best.value().evaluation == -KING_VALUE {
                        slow_won += 1;
                        break;
                    }

                    pgn.add_move((from, to), &board);

                    board = board.make_move(from, to);

                    let table = board.iterative_deepening(depth);

                    let best = table.get(&hash(&board)).unwrap();
                    let (from, to) = best.value().best_move;

                    if best.value().evaluation == KING_VALUE {
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
