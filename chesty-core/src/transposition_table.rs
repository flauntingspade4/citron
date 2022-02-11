use std::{lazy::SyncLazy, collections::HashMap};

use crate::{analysis::Node, piece::Piece, Board, PlayableTeam, Position};

use rand::{Fill, RngCore};

pub type TranspositionTable = HashMap<u64, TranspositionEntry>;

static ZOBRIST_KEYS: SyncLazy<([[u64; 12]; 64], u64)> = SyncLazy::new(|| {
    let mut initial: [u64; 12 * 64] = [0; 12 * 64];

    let mut rng = rand::thread_rng();

    initial.try_fill(&mut rng).unwrap();

    (unsafe { core::mem::transmute(initial) }, rng.next_u64())
});

fn piece_index(piece: Piece) -> usize {
    match piece.inner() & 0b0001_1111 {
        0b0001_1001 => 0,
        0b0001_1010 => 1,
        0b0001_1011 => 2,
        0b0001_1100 => 3,
        0b0001_1101 => 4,
        0b0001_1110 => 5,
        0b0000_1001 => 6,
        0b0000_1010 => 7,
        0b0000_1011 => 8,
        0b0000_1100 => 9,
        0b0000_1101 => 10,
        0b0000_1110 => 11,
        _ => panic!("unrecognised piece {}", piece.inner()),
    }
}

#[must_use]
/// Hashes a given board
pub fn hash(board: &Board) -> u64 {
    let mut hash = 0;
    for (position, piece) in board.positions_pieces() {
        let index = position.index() as usize;

        hash ^= ZOBRIST_KEYS.0[index][piece_index(*piece)];
        if board.to_play == PlayableTeam::Black {
            hash ^= ZOBRIST_KEYS.1;
        }
    }
    hash
}

#[derive(Debug)]
pub struct TranspositionEntry {
    pub depth: u8,
    pub evaluation: Node,
    pub best_move: (Position, Position),
}

impl TranspositionEntry {
    pub const fn new(depth: u8, evaluation: Node, best_move: (Position, Position)) -> Self {
        Self {
            depth,
            evaluation,
            best_move,
        }
    }
}
