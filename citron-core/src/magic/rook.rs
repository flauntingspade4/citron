use std::lazy::SyncLazy;

use super::{
    bitscan_backward, bitscan_forward,
    consts::{MASK_FILE, MASK_RANK, ROOK_INDEX_BITS, ROOK_MAGICS},
    get_blockers_from_index, Dir, RAYS,
};

pub const ROOK_MASKS: [u64; 64] = init_rook_masks();

pub static ROOK_ATTACKS: SyncLazy<Vec<[u64; 4096]>> = SyncLazy::new(|| {
    let mut attacks = vec![[0; 4096]; 64];

    for (attack_square, square) in attacks.iter_mut().zip(0..64) {
        *attack_square = init_rook_attacks(square);
    }

    attacks
});

const fn init_rook_masks() -> [u64; 64] {
    let mut masks = [0; 64];

    let mut square = 0;

    while square < 64 {
        masks[square] = (RAYS[square][Dir::North as usize] & !MASK_RANK[7])
            | (RAYS[square][Dir::South as usize] & !MASK_RANK[0])
            | (RAYS[square][Dir::East as usize] & !MASK_FILE[7])
            | (RAYS[square][Dir::West as usize] & !MASK_FILE[0]);
        square += 1;
    }

    masks
}

const fn get_rook_attacks_slow(square: u64, blockers: u64) -> u64 {
    let mut attacks = 0;
    let square = square as usize;

    attacks |= RAYS[square][Dir::North as usize];
    if RAYS[square][Dir::North as usize] & blockers != 0 {
        attacks &= !RAYS[bitscan_forward(RAYS[square][Dir::North as usize] & blockers) as usize]
            [Dir::North as usize];
    }

    attacks |= RAYS[square][Dir::South as usize];
    if RAYS[square][Dir::South as usize] & blockers != 0 {
        attacks &= !RAYS[bitscan_backward(RAYS[square][Dir::South as usize] & blockers) as usize]
            [Dir::South as usize];
    }

    attacks |= RAYS[square][Dir::East as usize];
    if RAYS[square][Dir::East as usize] & blockers != 0 {
        attacks &= !RAYS[bitscan_forward(RAYS[square][Dir::East as usize] & blockers) as usize]
            [Dir::East as usize];
    }

    attacks |= RAYS[square][Dir::West as usize];
    if RAYS[square][Dir::West as usize] & blockers != 0 {
        attacks &= !RAYS[bitscan_backward(RAYS[square][Dir::West as usize] & blockers) as usize]
            [Dir::West as usize];
    }

    attacks
}

const fn init_rook_attacks(square: usize) -> [u64; 4096] {
    let mut attacks = [0; 4096];

    let mut blocker_index = 0;

    while blocker_index < (1 << ROOK_INDEX_BITS[square]) {
        let blockers = get_blockers_from_index(blocker_index, ROOK_MASKS[square]);

        attacks[((blockers.wrapping_mul(ROOK_MAGICS[square])) >> (64 - ROOK_INDEX_BITS[square]))
            as usize] = get_rook_attacks_slow(square as u64, blockers);

        blocker_index += 1;
    }

    attacks
}
