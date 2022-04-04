use std::{collections::HashMap, lazy::SyncLazy};

use crate::{analysis::Node, move_gen::Move, Board, PlayableTeam, Position};

use rand::{Fill, RngCore};

pub static ZOBRIST_KEYS: SyncLazy<([[u64; 12]; 64], u64)> = SyncLazy::new(|| {
    let mut initial: [u64; 12 * 64] = [0; 12 * 64];

    let mut rng = rand::thread_rng();

    initial.try_fill(&mut rng).unwrap();

    // SAFETY: Transmuting to define array boundaries is always safe
    (unsafe { core::mem::transmute(initial) }, rng.next_u64())
});

pub type TranspositionTable = HashMap<u64, TranspositionEntry>;

#[must_use]
pub fn hash(board: &Board) -> u64 {
    let mut hash = 0;

    for position in Position::positions() {
        let piece = board.piece_at(position);

        if piece.is_piece() {
            hash ^= ZOBRIST_KEYS.0[position.index() as usize][piece as usize];
        }
    }

    if board.to_play == PlayableTeam::Black {
        hash ^= ZOBRIST_KEYS.1;
    }

    hash
}

#[derive(Debug)]
pub struct TranspositionEntry {
    pub depth: u8,
    pub evaluation: Node,
    pub best_move: Move,
}

impl TranspositionEntry {
    pub const fn new(depth: u8, evaluation: Node, best_move: Move) -> Self {
        Self {
            depth,
            evaluation,
            best_move,
        }
    }
}
