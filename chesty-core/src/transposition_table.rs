use std::{collections::HashMap, lazy::SyncLazy};

use crate::{Board, PlayableTeam, Position};

use rand::{Fill, RngCore};

pub type TranspositionTable = HashMap<u64, TranspositionEntry>;

pub(crate) static ZOBRIST_KEYS: SyncLazy<([[u64; 12]; 64], u64)> = SyncLazy::new(|| {
    let mut initial: [u64; 12 * 64] = [0; 12 * 64];

    let mut rng = rand::thread_rng();

    initial.try_fill(&mut rng).unwrap();

    (unsafe { core::mem::transmute(initial) }, rng.next_u64())
});

#[derive(Debug)]
pub struct TranspositionEntry {
    pub depth: u8,
    // pub evaluation: Node,
    pub best_move: (Position, Position),
}

impl TranspositionEntry {
    pub const fn new(depth: u8, best_move: (Position, Position)) -> Self {
        Self {
            depth,
            // evaluation,
            best_move,
        }
    }
}
