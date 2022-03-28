use std::lazy::SyncLazy;

use super::{
    bitscan_backward, bitscan_forward,
    consts::{BISHOP_INDEX_BITS, BISHOP_MAGICS, MASK_FILE, MASK_RANK},
    get_blockers_from_index, Dir, RAYS,
};

pub const BISHOP_MASKS: [u64; 64] = init_bishop_masks();

pub static BISHOP_ATTACKS: SyncLazy<Vec<[u64; 1024]>> = SyncLazy::new(|| {
    let mut attacks = vec![[0; 1024]; 64];

    for (attack_square, square) in attacks.iter_mut().zip(0..64) {
        *attack_square = init_bishop_attacks(square);
    }

    attacks
});

const fn init_bishop_masks() -> [u64; 64] {
    let mut masks = [0; 64];
    let edge_squares = MASK_FILE[0] | MASK_FILE[7] | MASK_RANK[0] | MASK_RANK[7];

    let mut square = 0;

    while square < 64 {
        masks[square] = (RAYS[square][Dir::NorthEast as usize]
            | RAYS[square][Dir::NorthWest as usize]
            | RAYS[square][Dir::SouthWest as usize]
            | RAYS[square][Dir::SouthEast as usize])
            & !edge_squares;

        square += 1;
    }

    masks
}

const fn get_bishop_attacks_slow(square: u64, blockers: u64) -> u64 {
    let mut attacks = 0;
    let square = square as usize;

    attacks |= RAYS[square][Dir::NorthWest as usize];
    if RAYS[square][Dir::NorthWest as usize] & blockers != 0 {
        attacks &= !RAYS
            [bitscan_forward(RAYS[square][Dir::NorthWest as usize] & blockers) as usize]
            [Dir::NorthWest as usize];
    }

    attacks |= RAYS[square][Dir::NorthEast as usize];
    if RAYS[square][Dir::NorthEast as usize] & blockers != 0 {
        attacks &= !RAYS
            [bitscan_backward(RAYS[square][Dir::NorthEast as usize] & blockers) as usize]
            [Dir::NorthEast as usize];
    }

    attacks |= RAYS[square][Dir::SouthEast as usize];
    if RAYS[square][Dir::SouthEast as usize] & blockers != 0 {
        attacks &= !RAYS
            [bitscan_forward(RAYS[square][Dir::SouthEast as usize] & blockers) as usize]
            [Dir::SouthEast as usize];
    }

    attacks |= RAYS[square][Dir::SouthWest as usize];
    if RAYS[square][Dir::SouthWest as usize] & blockers != 0 {
        attacks &= !RAYS
            [bitscan_backward(RAYS[square][Dir::SouthWest as usize] & blockers) as usize]
            [Dir::SouthWest as usize];
    }

    attacks
}

const fn init_bishop_attacks(square: usize) -> [u64; 1024] {
    let mut attacks = [0; 1024];

    let mut blocker_index = 0;

    while blocker_index < (1 << BISHOP_INDEX_BITS[square]) {
        let blockers = get_blockers_from_index(blocker_index, BISHOP_MASKS[square]);

        attacks[((blockers.wrapping_mul(BISHOP_MAGICS[square])) >> (64 - BISHOP_INDEX_BITS[square]))
            as usize] = get_bishop_attacks_slow(square as u64, blockers);

        blocker_index += 1;
    }

    attacks
}
