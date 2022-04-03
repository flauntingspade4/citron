use crate::{PlayableTeam, Position};

mod bishop;
mod consts;
mod pawn;
mod rook;

use bishop::{BISHOP_ATTACKS, BISHOP_MASKS};
use consts::{
    BISHOP_INDEX_BITS, BISHOP_MAGICS, KING_ATTACKS, KNIGHT_ATTACKS, ROOK_INDEX_BITS, ROOK_MAGICS,
};
pub use consts::{MASK_FILE, MASK_RANK};
use pawn::PAWN_ATTACKS;
use rook::{ROOK_ATTACKS, ROOK_MASKS};

#[must_use]
pub const fn pawn_attacks(position: Position, team: PlayableTeam) -> u64 {
    let square = position.index() as usize;

    PAWN_ATTACKS[team as usize][square]
}

#[must_use]
pub fn bishop_attacks(position: Position, mut blockers: u64) -> u64 {
    let square = position.index() as usize;

    blockers &= BISHOP_MASKS[square];

    BISHOP_ATTACKS[square][((blockers.wrapping_mul(BISHOP_MAGICS[square]))
        >> (64 - BISHOP_INDEX_BITS[square])) as usize]
}

#[must_use]
pub fn rook_attacks(position: Position, mut blockers: u64) -> u64 {
    let square = position.index() as usize;

    blockers &= ROOK_MASKS[square];

    ROOK_ATTACKS[square]
        [((blockers.wrapping_mul(ROOK_MAGICS[square])) >> (64 - ROOK_INDEX_BITS[square])) as usize]
}

#[must_use]
pub const fn knight_attacks(position: Position) -> u64 {
    let square = position.index() as usize;

    KNIGHT_ATTACKS[square]
}

#[must_use]
pub const fn king_attacks(position: Position) -> u64 {
    let square = position.index() as usize;

    KING_ATTACKS[square]
}

pub const fn pop_lsb(mask: &mut u64) -> u64 {
    let index = bitscan_forward(*mask);

    *mask ^= 1 << index;

    index as u64
}

#[must_use]
pub const fn bitscan_forward(board: u64) -> u64 {
    const INDEX64: [u64; 64] = [
        0, 47, 1, 56, 48, 27, 2, 60, 57, 49, 41, 37, 28, 16, 3, 61, 54, 58, 35, 52, 50, 42, 21, 44,
        38, 32, 29, 23, 17, 11, 4, 62, 46, 55, 26, 59, 40, 36, 15, 53, 34, 51, 20, 43, 31, 22, 10,
        45, 25, 39, 14, 33, 19, 30, 9, 24, 13, 18, 8, 12, 7, 6, 5, 63,
    ];
    const DEBRUIJN64: u64 = 0x03f79d71b4cb0a89;

    INDEX64[(((board ^ (board - 1)).wrapping_mul(DEBRUIJN64)) >> 58) as usize]
}

const fn bitscan_backward(mut board: u64) -> u64 {
    const INDEX64: [u64; 64] = [
        0, 47, 1, 56, 48, 27, 2, 60, 57, 49, 41, 37, 28, 16, 3, 61, 54, 58, 35, 52, 50, 42, 21, 44,
        38, 32, 29, 23, 17, 11, 4, 62, 46, 55, 26, 59, 40, 36, 15, 53, 34, 51, 20, 43, 31, 22, 10,
        45, 25, 39, 14, 33, 19, 30, 9, 24, 13, 18, 8, 12, 7, 6, 5, 63,
    ];

    const DEBRUIJN64: u64 = 0x03f79d71b4cb0a89;

    board |= board >> 1;
    board |= board >> 2;
    board |= board >> 4;
    board |= board >> 8;
    board |= board >> 16;
    board |= board >> 32;
    INDEX64[((board.wrapping_mul(DEBRUIJN64)) >> 58) as usize]
}

const fn get_blockers_from_index(index: u64, mut mask: u64) -> u64 {
    let mut blockers = 0;
    let mut i = 0;

    let bits = mask.count_ones();

    while i < bits {
        let bit_pos = pop_lsb(&mut mask);

        if (index & (1 << i)) != 0 {
            blockers |= 1 << bit_pos;
        }

        i += 1;
    }

    blockers
}

#[repr(u8)]
enum Dir {
    North,
    South,
    East,
    West,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}

const RAYS: [[u64; 8]; 64] = init_rays();

const fn init_rays() -> [[u64; 8]; 64] {
    let mut rays = [[0; 8]; 64];

    let mut square = 0;

    while square < 64 {
        rays[square][Dir::North as usize] = 0x0101010101010100 << square;

        rays[square][Dir::South as usize] = 0x0080808080808080 >> (63 - square);

        rays[square][Dir::East as usize] = 2 * ((1 << (square | 7)) - (1 << square));

        rays[square][Dir::West as usize] = (1 << square) - (1 << (square & 56));

        rays[square][Dir::NorthWest as usize] =
            north_west(0x102040810204000, 7 - col(square)) << (row(square) * 8);

        rays[square][Dir::NorthEast as usize] =
            north_east(0x8040201008040200, col(square)) << (row(square) * 8);

        rays[square][Dir::SouthWest as usize] =
            north_west(0x40201008040201, 7 - col(square)) >> ((7 - row(square)) * 8);

        rays[square][Dir::SouthEast as usize] =
            north_east(0x2040810204080, col(square)) >> ((7 - row(square)) * 8);

        square += 1;
    }

    rays
}

const fn north_east(mut board: u64, n: u64) -> u64 {
    let mut i = 0;

    while i < n {
        board = (board << 1) & (!MASK_FILE[0]);

        i += 1;
    }

    board
}

const fn north_west(mut board: u64, n: u64) -> u64 {
    let mut i = 0;

    while i < n {
        board = (board >> 1) & (!MASK_FILE[7]);

        i += 1;
    }

    board
}

const fn row(square: usize) -> u64 {
    (square >> 3) as u64
}

const fn col(square: usize) -> u64 {
    (square & 7) as u64
}
