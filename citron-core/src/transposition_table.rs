use std::{collections::HashMap, lazy::SyncLazy};

use crate::{analysis::Node, move_gen::Move};

use rand::{Fill, RngCore};

pub static ZOBRIST_KEYS: SyncLazy<([[u64; 12]; 64], u64)> = SyncLazy::new(|| {
    let mut initial: [u64; 12 * 64] = [0; 12 * 64];

    let mut rng = rand::thread_rng();

    initial.try_fill(&mut rng).unwrap();

    // SAFETY: Transmuting to define array boundaries is always safe
    (unsafe { core::mem::transmute(initial) }, rng.next_u64())
});

pub type TranspositionTable = HashMap<u64, TranspositionEntry>;

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
