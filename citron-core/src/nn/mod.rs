use std::{cmp::Ordering, collections::HashMap};

use ndarray::{Array3, ArrayView, Axis};
use session_handle::HBSessionHandle;
use tensorflow::Tensor;

use crate::{
    killer::KillerMoves,
    magic::pop_lsb,
    move_gen::Move,
    move_ordering::{move_ordering, MoveOrderingEntry},
    piece::PieceKind,
    Board, Game, MoveGen, PlayableTeam, Position,
};

pub mod session_handle;

#[derive(Clone, Debug)]
pub enum EvaluateOutput {
    Latent(Tensor<f32>),
    KingCapture(PlayableTeam),
}

impl EvaluateOutput {
    pub fn compare(
        &self,
        other: &Self,
        handle: &HBSessionHandle,
    ) -> (Ordering, Option<Tensor<f32>>) {
        match (self, other) {
            (EvaluateOutput::Latent(lhs), EvaluateOutput::Latent(rhs)) => {
                let (ord, b) = handle.compare_encoded(lhs, rhs).unwrap();
                (ord, Some(b))
            }
            (EvaluateOutput::Latent(_), EvaluateOutput::KingCapture(playable_team)) => (
                match playable_team {
                    PlayableTeam::White => Ordering::Greater,
                    PlayableTeam::Black => Ordering::Less,
                },
                None,
            ),
            (EvaluateOutput::KingCapture(playable_team), EvaluateOutput::Latent(_)) => (
                match playable_team {
                    PlayableTeam::White => Ordering::Less,
                    PlayableTeam::Black => Ordering::Greater,
                },
                None,
            ),
            (EvaluateOutput::KingCapture(lhs), EvaluateOutput::KingCapture(rhs)) => (
                match (lhs, rhs) {
                    (PlayableTeam::White, PlayableTeam::Black) => Ordering::Less,
                    (PlayableTeam::Black, PlayableTeam::White) => Ordering::Greater,
                    _ => Ordering::Equal,
                },
                None,
            ),
        }
    }
}

impl Game {
    pub fn nn_evaluate_iterative(
        &self,
        depth: u8,
        handle: &HBSessionHandle,
    ) -> HashMap<u64, NNTranspositionEntry> {
        let mut transposition_table = HashMap::new();
        let mut killer_table = Vec::with_capacity(depth as usize);
        killer_table.resize_with(depth as usize, KillerMoves::default);

        match self.to_play() {
            PlayableTeam::White => self.nn_evaluate_max(
                depth,
                0,
                None,
                None,
                (&mut transposition_table, &mut killer_table),
                handle,
            ),
            PlayableTeam::Black => self.nn_evaluate_min(
                depth,
                0,
                None,
                None,
                (&mut transposition_table, &mut killer_table),
                handle,
            ),
        };

        transposition_table
    }
    pub fn nn_evaluate_max(
        &self,
        depth: u8,
        ply: u8,
        mut alpha: Option<EvaluateOutput>,
        beta: Option<EvaluateOutput>,
        (transposition_table, killer_table): (
            &mut HashMap<u64, NNTranspositionEntry>,
            &mut [KillerMoves],
        ),
        handle: &HBSessionHandle,
    ) -> EvaluateOutput {
        if depth == 0 {
            let mut network_input = board_to_network_input(&self.board, PlayableTeam::White);
            add_move_information_to_nn_input(&self.board, &mut network_input);
            return EvaluateOutput::Latent(
                handle
                    .encode(network_input.insert_axis(Axis(0)).into())
                    .unwrap(),
            );
        }

        if let Some(found) = transposition_table.get(&self.board.hash()) {
            if found.depth >= depth {
                return found.evaluation.clone();
            }
        }

        let mut moves = MoveGen::new(&self.board).into_inner();
        move_ordering(
            ply,
            &mut moves,
            (transposition_table, killer_table),
            self.board.hash(),
        );

        let mut best_move = None;
        match moves.into_iter().try_for_each(|possible_move| {
            if possible_move.captured_piece_kind() == PieceKind::King {
                return Err((
                    EvaluateOutput::KingCapture(PlayableTeam::Black),
                    possible_move,
                ));
            }
            let possible_board = self.make_move(&possible_move);

            let evaluation = possible_board.nn_evaluate_max(
                depth - 1,
                ply + 1,
                alpha.clone(),
                beta.clone(),
                (transposition_table, killer_table),
                handle,
            );

            if let Some(beta) = &beta {
                let (ord, _) = evaluation.compare(&beta, &handle);
                if ord == Ordering::Greater {
                    return Err((evaluation, possible_move));
                }
            }

            if alpha.is_none() {
                best_move = Some(possible_move);
                alpha = Some(evaluation);
            } else {
                let (ord, _) = evaluation.compare(&alpha.as_ref().unwrap(), &handle);
                if ord == Ordering::Greater {
                    best_move = Some(possible_move);
                    alpha = Some(evaluation);
                }
            }

            Ok(())
        }) {
            Ok(_) => {
                if let Some(best_move) = best_move {
                    let transposition_entry =
                        NNTranspositionEntry::new(depth, alpha.clone().unwrap(), best_move);

                    transposition_table.insert(self.board.hash(), transposition_entry);
                }
                alpha.unwrap()
            }
            Err((beta_cutoff, possible_move)) => {
                if possible_move.captured_piece_kind() == PieceKind::None {
                    killer_table[ply as usize].add_move(possible_move.from_to());
                }

                let transposition_entry =
                    NNTranspositionEntry::new(depth, beta_cutoff.clone(), possible_move);

                transposition_table.insert(self.board.hash(), transposition_entry);

                beta_cutoff
            }
        }
    }

    pub fn nn_evaluate_min(
        &self,
        depth: u8,
        ply: u8,
        alpha: Option<EvaluateOutput>,
        mut beta: Option<EvaluateOutput>,
        (transposition_table, killer_table): (
            &mut HashMap<u64, NNTranspositionEntry>,
            &mut [KillerMoves],
        ),
        handle: &HBSessionHandle,
    ) -> EvaluateOutput {
        if depth == 0 {
            let mut network_input = board_to_network_input(&self.board, PlayableTeam::White);
            add_move_information_to_nn_input(&self.board, &mut network_input);
            return EvaluateOutput::Latent(
                handle
                    .encode(network_input.insert_axis(Axis(0)).into())
                    .unwrap(),
            );
        }

        if let Some(found) = transposition_table.get(&self.board.hash()) {
            if found.depth >= depth {
                return found.evaluation.clone();
            }
        }

        let mut moves = MoveGen::new(&self.board).into_inner();
        move_ordering(
            ply,
            &mut moves,
            (transposition_table, killer_table),
            self.board.hash(),
        );

        let mut best_move = None;
        match moves.into_iter().try_for_each(|possible_move| {
            if possible_move.captured_piece_kind() == PieceKind::King {
                return Err((
                    EvaluateOutput::KingCapture(PlayableTeam::White),
                    possible_move,
                ));
            }

            let possible_board = self.make_move(&possible_move);

            let evaluation = possible_board.nn_evaluate_max(
                depth - 1,
                ply + 1,
                alpha.clone(),
                beta.clone(),
                (transposition_table, killer_table),
                handle,
            );

            if let Some(alpha) = &alpha {
                let (ord, _) = evaluation.compare(alpha, &handle);
                if ord == Ordering::Less {
                    return Err((evaluation, possible_move));
                }
            }

            if beta.is_none() {
                best_move = Some(possible_move);
                beta = Some(evaluation);
            } else {
                let (ord, _) = evaluation.compare(&beta.as_ref().unwrap(), &handle);
                if ord == Ordering::Less {
                    best_move = Some(possible_move);
                    beta = Some(evaluation);
                }
            }

            Ok(())
        }) {
            Ok(_) => {
                if let Some(best_move) = best_move {
                    let transposition_entry =
                        NNTranspositionEntry::new(depth, beta.clone().unwrap(), best_move);

                    transposition_table.insert(self.board.hash(), transposition_entry);
                }
                beta.unwrap()
            }
            Err((alpha_cutoff, possible_move)) => {
                if possible_move.captured_piece_kind() == PieceKind::None {
                    killer_table[ply as usize].add_move(possible_move.from_to());
                }
                let transposition_entry =
                    NNTranspositionEntry::new(depth, alpha_cutoff.clone(), possible_move);
                transposition_table.insert(self.board.hash(), transposition_entry);

                alpha_cutoff
            }
        }
    }
}

/// Calculates an array from the board that can
/// then be used with [`BZSessionHandle`].
/// If it is black's turn to play the board will
/// be 'rotated', as if it were facing black
pub fn board_to_network_input(board: &Board, played_team: PlayableTeam) -> Array3<f32> {
    let mut array = Array3::zeros([8, 8, 12]);

    let mut piece_map = board.pieces();
    // Iterate over each team and piece kind
    for &team in PlayableTeam::teams().iter() {
        for &piece_type in PieceKind::kinds().iter() {
            // Add each piece for that team and kind to the array
            while piece_map[team as usize][piece_type as usize] != 0 {
                let bitmap = pop_lsb(&mut piece_map[team as usize][piece_type as usize]);
                let position = Position::from_bitmap(1 << bitmap);

                if played_team == PlayableTeam::White {
                    array[[
                        position.x() as usize,
                        position.y() as usize,
                        team as usize * 6 + piece_type as usize,
                    ]] = 1.;
                } else {
                    array[[
                        7 - position.x() as usize,
                        7 - position.y() as usize,
                        (!team) as usize * 6 + piece_type as usize,
                    ]] = 1.;
                }
            }
        }
    }

    array
}

pub fn add_move_information_to_nn_input(board: &Board, input: &mut Array3<f32>) {
    let first_turn_array = [if board.to_play() == PlayableTeam::White {
        1.
    } else {
        0.
    }; 64];
    let second_turn_array = [if board.to_play() == PlayableTeam::Black {
        1.
    } else {
        0.
    }; 64];
    input
        .append(
            Axis(2),
            ArrayView::from(&first_turn_array)
                .into_shape((8, 8, 1))
                .unwrap(),
        )
        .unwrap();
    input
        .append(
            Axis(2),
            ArrayView::from(&second_turn_array)
                .into_shape((8, 8, 1))
                .unwrap(),
        )
        .unwrap();
}

pub struct NNTranspositionEntry {
    pub depth: u8,
    pub evaluation: EvaluateOutput,
    pub best_move: Move,
}

impl NNTranspositionEntry {
    pub fn new(depth: u8, evaluation: EvaluateOutput, best_move: Move) -> Self {
        Self {
            depth,
            evaluation,
            best_move,
        }
    }
}

impl MoveOrderingEntry for NNTranspositionEntry {
    fn from_to_equals(&self, other: &Move) -> bool {
        self.best_move.from_to() == other.from_to()
    }
}

#[test]
fn nn_test() {
    let handle = HBSessionHandle::load(None);
    let max_depth = 4;

    // let game = Game::new();
    let game = Game::from_fen("r2qkb1r/pp2nppp/3p4/2pNN1B1/2BnP3/3P4/PPP2PPP/R2bK2R w KQkq - 1 0")
        .unwrap();

    let transposition_table = game.nn_evaluate_iterative(max_depth, &handle);

    let best = transposition_table
        .get(&game.board.hash())
        .unwrap()
        .best_move
        .clone();

    println!("Best move is {}", best);

    if let Some(true_best) = transposition_table.get(
        &Game::from_fen("2k5/1p6/1p5p/p7/2Pp1Pp1/3P2K1/PP1r2PP/4R3 b - - 0 2")
            .unwrap()
            .board
            .hash(),
    ) {
        println!("True best is {}", true_best.best_move);
    }
}

#[test]
fn position_comparison() {
    let handle = HBSessionHandle::load(None);

    let position_1 =
        Game::from_fen("rnbqkbnr/p2p1ppp/1pp1p3/8/2B1P3/5Q2/PPPP1PPP/RNB1K1NR w KQkq - 0 4")
            .unwrap();
    let mut input_1 = board_to_network_input(&position_1.board, position_1.to_play());
    add_move_information_to_nn_input(&position_1.board, &mut input_1);
    let input_1 = input_1.insert_axis(Axis(0)).into();
    let position_2 =
        Game::from_fen("rnbqkbnr/p2ppQpp/2p5/1p6/2B1P3/8/PPPP1PPP/RNB1K1NR b KQkq - 0 4").unwrap();
    let mut input_2 = board_to_network_input(&position_2.board, position_2.to_play());
    add_move_information_to_nn_input(&position_2.board, &mut input_2);
    let input_2 = input_2.insert_axis(Axis(0)).into();

    let output = handle.call(&input_1, &input_2).unwrap();

    println!("{:?}", output);

    let output = handle.call(&input_2, &input_1).unwrap();

    println!("{:?}", output);
}
