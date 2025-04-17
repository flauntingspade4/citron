use std::{collections::hash_map::Entry, ops::Range};

use rand::Rng;

pub mod population;

use crate::{
    analysis::Node,
    killer::KillerMoves,
    move_ordering::{move_ordering, quiescence_move_ordering},
    piece::{PieceKind, KING_VALUE},
    transposition_table::{TranspositionEntry, TranspositionTable},
    Game, MoveGen, PlayableTeam,
};

const MULTICUT_M_BOUNDS: Range<usize> = 0..15;
const MULTICUT_C_BOUNDS: Range<u8> = 1..5;
const MULTICUT_R_BOUNDS: Range<u8> = 1..5;
const NULL_REDUCTION_BOUNDS: Range<u8> = 1..5;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Config {
    multicut_m: usize,
    multicut_c: u8,
    multicut_r: u8,
    multicut_use: bool,
    null_reduction: u8,
    null_use: bool,
    visit_count: usize,
}

impl Config {
    pub fn new_random(rng: &mut impl Rng) -> Self {
        Self {
            multicut_m: rng.random_range(MULTICUT_M_BOUNDS),
            multicut_c: rng.random_range(MULTICUT_C_BOUNDS),
            multicut_r: rng.random_range(MULTICUT_R_BOUNDS),
            multicut_use: rng.random_bool(0.5),
            null_reduction: rng.random_range(NULL_REDUCTION_BOUNDS),
            null_use: rng.random_bool(0.5),
            visit_count: 0,
        }
    }

    pub fn mutate(&mut self, rng: &mut impl Rng) {
        let change_index = rng.random_range(0..6);
        let add = rng.random_bool(0.5);

        match change_index {
            0 => {
                self.multicut_m = if add || self.multicut_m == 0 {
                    (self.multicut_m + 1).min(MULTICUT_M_BOUNDS.end)
                } else {
                    self.multicut_m - 1
                }
            }
            1 => {
                self.multicut_c = if add || self.multicut_c == 0 {
                    (self.multicut_c + 1).min(MULTICUT_C_BOUNDS.end)
                } else {
                    self.multicut_c - 1
                }
            }
            2 => {
                self.multicut_r = if add || self.multicut_r == 0 {
                    (self.multicut_r + 1).min(MULTICUT_R_BOUNDS.end)
                } else {
                    self.multicut_r - 1
                }
            }
            3 => self.multicut_use = !self.multicut_use,
            4 => {
                self.null_reduction = if add || self.null_reduction == 0 {
                    (self.null_reduction + 1).min(NULL_REDUCTION_BOUNDS.end)
                } else {
                    self.null_reduction - 1
                }
            }
            5 => self.null_use = !self.null_use,
            _ => unreachable!(),
        }
    }

    pub fn crossover(&self, other: &Self, rng: &mut impl Rng) -> (Self, Self) {
        let crossover_point = rng.random_range(0..7);

        let (mut lhs, mut rhs) = (self.clone(), other.clone());

        if crossover_point <= 0 {
            let temp = rhs.multicut_m;
            rhs.multicut_m = lhs.multicut_m;
            lhs.multicut_m = temp;
        }
        if crossover_point <= 1 {
            let temp = rhs.multicut_c;
            rhs.multicut_c = lhs.multicut_c;
            lhs.multicut_c = temp;
        }
        if crossover_point <= 2 {
            let temp = rhs.multicut_r;
            rhs.multicut_r = lhs.multicut_r;
            lhs.multicut_r = temp;
        }
        if crossover_point <= 3 {
            let temp = rhs.multicut_use;
            rhs.multicut_use = lhs.multicut_use;
            lhs.multicut_use = temp;
        }
        if crossover_point <= 4 {
            let temp = rhs.null_reduction;
            rhs.null_reduction = lhs.null_reduction;
            lhs.null_reduction = temp;
        }
        if crossover_point <= 5 {
            let temp = rhs.null_use;
            rhs.null_use = lhs.null_use;
            lhs.null_use = temp;
        }

        (lhs, rhs)
    }

    pub fn evaluate(&mut self, depth: u8, positions: &[Game]) -> usize {
        self.visit_count = 0;

        for p in positions {
            let _ = p.iterative_deepening_ply_ga(depth, self);
        }

        self.visit_count
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            multicut_m: 5,
            multicut_c: 2,
            multicut_r: 3,
            multicut_use: true,
            null_reduction: 3,
            null_use: true,
            visit_count: 0,
        }
    }
}

impl Config {
    pub fn visit_count(&self) -> usize {
        self.visit_count
    }

    pub fn reset_count(&mut self) {
        self.visit_count = 0;
    }

    pub fn increment_count(&mut self) {
        self.visit_count += 1;
    }
}

impl Game {
    #[must_use]
    pub fn iterative_deepening_ga(&self, depth: u8, config: &mut Config) -> TranspositionTable {
        self.iterative_deepening_ply_ga(depth * 2, config)
    }
    #[must_use]
    pub fn iterative_deepening_ply_ga(&self, depth: u8, config: &mut Config) -> TranspositionTable {
        let mut transposition_table = TranspositionTable::new();
        let mut killer_table = Vec::with_capacity(depth as usize);

        killer_table.resize_with(depth as usize, KillerMoves::default);

        let _ = self.evaluate_private_ga(
            depth,
            0,
            std::i16::MIN,
            std::i16::MAX,
            (&mut transposition_table, killer_table.as_mut_slice()),
            config,
        );

        transposition_table
    }

    fn evaluate_private_ga(
        &self,
        depth: u8,
        ply: u8,
        mut alpha: i16,
        beta: i16,
        (transposition_table, killer_table): (&mut TranspositionTable, &mut [KillerMoves]),
        config: &mut Config,
    ) -> i16 {
        // println!("Search with ply {ply}");'
        config.visit_count += 1;

        if depth == 0 {
            return self.quiesce_ga(alpha, beta, config);
        }

        if let Some(t) = transposition_table.get(&self.board.hash()) {
            if t.depth > depth {
                if let Node::PvNode(evaluation) = t.evaluation {
                    return evaluation;
                }
            }
        }

        let mut best_move = None;
        let mut pv_search = true;

        let mut moves = MoveGen::new(&self.board).into_inner();
        move_ordering(
            ply,
            &mut moves,
            (transposition_table, killer_table),
            self.board.hash(),
        );

        // Null move
        if config.null_use && self.null_move_condition(&moves) {
            let null_board = self.make_null_move();
            let null_score = -null_board.evaluate_private_ga(
                depth.saturating_sub(config.null_reduction),
                ply + 1,
                -beta,
                -(beta - 1),
                (transposition_table, killer_table),
                config,
            );
            if null_score >= beta {
                return null_score;
            }
        }

        // Multi-cut
        if config.multicut_use && depth >= config.multicut_r {
            let mut c = 0;

            if let Err(multi_cut) =
                moves
                    .iter()
                    .take(config.multicut_m)
                    .try_for_each(|possible_move| {
                        let possible_board = self.make_move(possible_move);

                        let eval = -possible_board.evaluate_private_ga(
                            depth.saturating_sub(config.multicut_r),
                            ply + 1,
                            -beta,
                            -(beta - 1),
                            (transposition_table, killer_table),
                            config,
                        );

                        if eval >= beta {
                            c += 1;
                            if c >= config.multicut_c {
                                return Err(beta);
                            }
                        }

                        Ok(())
                    })
            {
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
                                self.board.hash(),
                                TranspositionEntry::new(
                                    depth,
                                    Node::PvNode(KING_VALUE),
                                    possible_move.clone(),
                                ),
                            );
                        }
                        return Err((KING_VALUE, possible_move));
                    }

                    let possible_board = self.make_move(&possible_move);

                    let score = if index > 3 && depth >= 3 && best_move.is_none() {
                        let eval = -possible_board.evaluate_private_ga(
                            depth - 3,
                            ply + 1,
                            -beta,
                            -alpha,
                            (transposition_table, killer_table),
                            config,
                        );
                        if eval > alpha {
                            -possible_board.evaluate_private_ga(
                                depth - 1,
                                ply + 1,
                                -beta,
                                -alpha,
                                (transposition_table, killer_table),
                                config,
                            )
                        } else {
                            eval
                        }
                    } else if pv_search {
                        -possible_board.evaluate_private_ga(
                            depth - 1,
                            ply + 1,
                            -beta,
                            -alpha,
                            (transposition_table, killer_table),
                            config,
                        )
                    } else {
                        let score = -possible_board.evaluate_private_ga(
                            depth - 1,
                            ply + 1,
                            -(alpha + 1),
                            -alpha,
                            (transposition_table, killer_table),
                            config,
                        );

                        if score > alpha {
                            -possible_board.evaluate_private_ga(
                                depth - 1,
                                ply + 1,
                                -beta,
                                -alpha,
                                (transposition_table, killer_table),
                                config,
                            )
                        } else {
                            score
                        }
                    };

                    if score > alpha {
                        if score >= beta {
                            // if possible_move.captured_piece_kind() == PieceKind::None {
                            //     killer_table[ply as usize].add_move(possible_move.from_to());
                            // }

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

            match transposition_table.entry(self.board.hash()) {
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

            match transposition_table.entry(self.board.hash()) {
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

    #[must_use]
    pub fn quiesce_ga(&self, mut alpha: i16, beta: i16, config: &mut Config) -> i16 {
        const DELTA: i16 = 250;

        let stand_pat = if self.to_play() == PlayableTeam::White {
            self.static_evaluation()
        } else {
            -self.static_evaluation()
        };

        let mut best_value = stand_pat;

        if stand_pat >= beta {
            return beta;
        }
        if stand_pat > alpha {
            alpha = stand_pat;
        }

        let mut moves = MoveGen::new(&self.board).into_inner();

        quiescence_move_ordering(&mut moves);

        if let Err(beta_cutoff) = moves
            .into_iter()
            .filter(|possible_move| possible_move.captured_piece_kind() != PieceKind::None)
            .try_for_each(|possible_move| {
                if possible_move.captured_piece_kind() == PieceKind::King {
                    return Err(KING_VALUE);
                }

                if self.in_endgame_ga()
                    || stand_pat + DELTA + possible_move.captured_piece_kind().value() > alpha
                {
                    let possible_board = self.make_move(&possible_move);

                    let score = -possible_board.quiesce_ga(-beta, -alpha, config);

                    if score >= beta {
                        return Err(score);
                    }
                    if score > best_value {
                        best_value = score;
                    }
                    if score > alpha {
                        alpha = score;
                    }
                }

                Ok(())
            })
        {
            return beta_cutoff;
        };

        best_value
    }

    fn in_endgame_ga(&self) -> bool {
        self.turn > 50
    }
}
