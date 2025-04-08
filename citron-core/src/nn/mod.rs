use std::{cmp::Ordering, collections::HashMap};

use ndarray::{Array3, ArrayView, Axis};
use session_handle::BZSessionHandle;
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
        handle: &BZSessionHandle,
    ) -> (Ordering, Option<Tensor<f32>>) {
        match (self, other) {
            (EvaluateOutput::Latent(lhs), EvaluateOutput::Latent(rhs)) => {
                let (a, b) = handle.compare_encoded(lhs, rhs).unwrap();
                (a, Some(b))
            }
            (EvaluateOutput::Latent(_), EvaluateOutput::KingCapture(playable_team)) => (
                match playable_team {
                    PlayableTeam::White => Ordering::Less,
                    PlayableTeam::Black => Ordering::Greater,
                },
                None,
            ),
            (EvaluateOutput::KingCapture(playable_team), EvaluateOutput::Latent(_)) => (
                match playable_team {
                    PlayableTeam::White => Ordering::Greater,
                    PlayableTeam::Black => Ordering::Less,
                },
                None,
            ),
            (EvaluateOutput::KingCapture(lhs), EvaluateOutput::KingCapture(rhs)) => (
                match (lhs, rhs) {
                    (PlayableTeam::White, PlayableTeam::Black) => Ordering::Greater,
                    (PlayableTeam::Black, PlayableTeam::White) => Ordering::Less,
                    _ => Ordering::Equal,
                },
                None,
            ),
        }
    }
}

impl Game {
    pub fn nn_evaluate(
        &self,
        depth: u8,
        handle: &BZSessionHandle,
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
        handle: &BZSessionHandle,
    ) -> EvaluateOutput {
        // println!(
        //     "Depth = {}, to_play = {}\n{}",
        //     depth,
        //     self.board.to_play(),
        //     self.board
        // );
        if depth == 0 {
            println!(
                "At depth = 0, encoding for {}\n{}",
                !self.board.to_play(),
                self.board
            );
            let network_input =
                board_to_network_input(&self.board, PlayableTeam::White).insert_axis(Axis(0));
            return EvaluateOutput::Latent(handle.encode(network_input.into()).unwrap());
        }

        if let Some(found) = transposition_table.get(&self.board.hash()) {
            if found.depth >= depth {
                return found.evaluation.clone();
            }
        }

        let mut moves = MoveGen::new(&self.board).into_inner();
        // for m in &moves {
        // println!("{m}");
        // }
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
                let (ord, result) = evaluation.compare(&beta, &handle);
                // let (ord, result) = handle.compare_encoded(&evaluation.0, &beta.0).unwrap();
                if ord == Ordering::Greater {
                    println!("beta cutoff due to result = {:?}", result);
                    return Err((evaluation, possible_move));
                }
            }

            if alpha.is_none() {
                best_move = Some(possible_move);
                alpha = Some(evaluation);
                println!(
                    "depth = {}, alpha is not set so setting to\n{}",
                    depth, possible_board.board
                );
            } else {
                let (ord, result) = evaluation.compare(&alpha.as_ref().unwrap(), &handle);
                println!(
                    "depth = {}, result is {:?} against alpha\n{}",
                    depth, result, possible_board.board
                );
                if ord == Ordering::Greater {
                    println!("Setting alpha");

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
        handle: &BZSessionHandle,
    ) -> EvaluateOutput {
        // println!(
        //     "Depth = {}, to_play = {}\n{}",
        //     depth,
        //     self.board.to_play(),
        //     self.board
        // );
        if depth == 0 {
            println!(
                "At depth = 0, encoding for {}\n{}",
                !self.board.to_play(),
                self.board
            );
            let network_input =
                board_to_network_input(&self.board, PlayableTeam::White).insert_axis(Axis(0));
            return EvaluateOutput::Latent(handle.encode(network_input.into()).unwrap());
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
                let (ord, result) = evaluation.compare(alpha, &handle);
                if ord == Ordering::Less {
                    println!("alpha cutoff with result {:?} on depth {depth}", result);
                    return Err((evaluation, possible_move));
                }
            }

            if beta.is_none() {
                best_move = Some(possible_move);
                beta = Some(evaluation);
                println!(
                    "depth = {}, beta is not set so setting to {}\n{}",
                    depth,
                    possible_board.to_play(),
                    possible_board.board
                );
            } else {
                let (ord, result) = evaluation.compare(&beta.as_ref().unwrap(), &handle);
                // let (ord, result) = handle
                // .compare_encoded(&evaluation.0, &beta.as_ref().unwrap().0)
                // .unwrap();
                println!(
                    "depth = {}, result is unknown against beta\n{}",
                    depth, possible_board.board
                );
                if ord == Ordering::Less {
                    println!("Setting beta as result = {:?}", result);

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
    let handle = BZSessionHandle::load(None);
    let depth = 3;

    // let game = Game::new();
    let game = Game::from_fen("2k5/1p6/1p5p/p6p/2Pp1P2/3P2K1/PP1r2PP/1R1Br3 w - - 0 1").unwrap();

    let transposition_table = game.nn_evaluate(depth, &handle);

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
    let handle = BZSessionHandle::load(None);

    let position_1 =
        Game::from_fen("2k5/1p6/1p5p/p6p/2Pp1P2/3P2K1/PP1r2PP/1R1Br3 w - - 0 1").unwrap();
    let input_1 = board_to_network_input(&position_1.board, position_1.to_play())
        .insert_axis(Axis(0))
        .into();
    let position_2 = Game::from_fen("2k5/1p6/1p5p/p7/2Pp1Pp1/3P2K1/PP1r2PP/4R3 b - - 0 2").unwrap();
    let input_2 = board_to_network_input(&position_2.board, position_2.to_play())
        .insert_axis(Axis(0))
        .into();

    let output = handle.call(&input_1, &input_2).unwrap();

    println!("{:?}", output);

    let output = handle.call(&input_2, &input_1).unwrap();

    println!("{:?}", output);
}
