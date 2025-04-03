use std::{cmp::Ordering, collections::HashMap};

use ndarray::{Array3, Axis};
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
pub struct EvaluateOutput(Tensor<f32>);

impl Game {
    pub fn nn_evaluate(
        &self,
        depth: u8,
        ply: u8,
        mut alpha: Option<EvaluateOutput>,
        beta: Option<EvaluateOutput>,
        (transposition_table, killer_moves): (
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
                board_to_network_input(&self.board, !self.board.to_play()).insert_axis(Axis(0));
            return EvaluateOutput(handle.encode(network_input.into()).unwrap());
        }

        let mut moves = MoveGen::new(&self.board).into_inner();
        for m in &moves {
            println!("{m}");
        }
        // move_ordering(
        // ply,
        // &mut moves,
        // (transposition_table, killer_moves),
        // self.board.hash(),
        // );

        let mut best_move = None;
        if let Err((beta_cutoff, possible_move)) = moves.into_iter().try_for_each(|possible_move| {
            let possible_board = self.make_move(&possible_move);

            let evaluation = possible_board.nn_evaluate(
                depth - 1,
                ply + 1,
                beta.clone(),
                alpha.clone(),
                (transposition_table, killer_moves),
                handle,
            );

            if alpha.is_none() {
                best_move = Some(possible_move);
                alpha = Some(evaluation);
                println!(
                    "depth = {}, alpha is not set so setting to {}\n{}",
                    depth,
                    possible_board.to_play(),
                    possible_board.board
                );
            } else {
                let (ord, result) = handle
                    .compare_encoded(&evaluation.0, &alpha.as_ref().unwrap().0)
                    .unwrap();
                println!(
                    "depth = {}, result is [{}, {}]\n{}",
                    depth, result[0], result[1], possible_board.board
                );
                if ord != Ordering::Greater {
                    return Ok(());
                }
                println!("Setting alpha");
                if let Some(beta) = &beta {
                    let (ord, result) = handle.compare_encoded(&evaluation.0, &beta.0).unwrap();
                    if ord == Ordering::Greater {
                        println!("Setting beta");
                        return Err((evaluation.0, possible_move));
                    }
                }

                best_move = Some(possible_move);
                alpha = Some(evaluation);
            }

            Ok(())
        }) {
            let transposition_entry =
                NNTranspositionEntry::new(depth, beta_cutoff.clone(), possible_move);

            transposition_table.insert(self.board.hash(), transposition_entry);

            EvaluateOutput(beta_cutoff)
        } else {
            if let Some(best_move) = best_move {
                let transposition_entry =
                    NNTranspositionEntry::new(depth, alpha.clone().unwrap().0, best_move);

                transposition_table.insert(self.board.hash(), transposition_entry);
            }
            alpha.unwrap()
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

pub struct NNTranspositionEntry {
    pub depth: u8,
    pub latent: Tensor<f32>,
    pub best_move: Move,
}

impl NNTranspositionEntry {
    pub fn new(depth: u8, latent: Tensor<f32>, best_move: Move) -> Self {
        Self {
            depth,
            latent,
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
    let depth = 1;

    // let game = Game::new();
    let game = Game::from_fen("3Q4/6pk/1R5p/8/4p3/4P1P1/4NP2/5K2 w - - 0 44").unwrap();
    let mut transposition_table = HashMap::new();
    let mut killer_table = Vec::with_capacity(depth as usize);

    killer_table.resize_with(depth as usize, KillerMoves::default);

    game.nn_evaluate(
        depth,
        0,
        None,
        None,
        (&mut transposition_table, killer_table.as_mut_slice()),
        &handle,
    );

    let best = transposition_table
        .get(&game.board.hash())
        .unwrap()
        .best_move
        .clone();

    println!("Best move is {}", best);
}

#[test]
fn position_comparison() {
    let handle = BZSessionHandle::load(None);

    let position_1 =
        Game::from_fen("2q1r3/p4pk1/1pBQ1np1/2p4p/5N2/1P2P1PP/P3bP2/2RR2K1 b - - 2 23").unwrap();
    let input_1 = board_to_network_input(&position_1.board, position_1.to_play())
        .insert_axis(Axis(0))
        .into();
    let position_2 =
        Game::from_fen("2q1r3/p4pk1/bpBQ2p1/2p4p/4nN2/1P2P1PP/P4P2/2RR2K1 b - - 2 23").unwrap();
    let input_2 = board_to_network_input(&position_2.board, position_2.to_play())
        .insert_axis(Axis(0))
        .into();

    let output = handle.call(&input_1, &input_2).unwrap();

    println!("{:?}", output);

    let output = handle.call(&input_2, &input_1).unwrap();

    println!("{:?}", output);
}
