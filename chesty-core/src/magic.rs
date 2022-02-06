// code heavily inspired by https://github.com/GunshipPenguin/shallow-blue

use std::lazy::SyncLazy;

use crate::Position;

pub const KING_ATTACKS: [u64; 64] = [
    0x302,
    0x705,
    0xe0a,
    0x1c14,
    0x3828,
    0x7050,
    0xe0a0,
    0xc040,
    0x30203,
    0x70507,
    0xe0a0e,
    0x1c141c,
    0x382838,
    0x705070,
    0xe0a0e0,
    0xc040c0,
    0x3020300,
    0x7050700,
    0xe0a0e00,
    0x1c141c00,
    0x38283800,
    0x70507000,
    0xe0a0e000,
    0xc040c000,
    0x302030000,
    0x705070000,
    0xe0a0e0000,
    0x1c141c0000,
    0x3828380000,
    0x7050700000,
    0xe0a0e00000,
    0xc040c00000,
    0x30203000000,
    0x70507000000,
    0xe0a0e000000,
    0x1c141c000000,
    0x382838000000,
    0x705070000000,
    0xe0a0e0000000,
    0xc040c0000000,
    0x3020300000000,
    0x7050700000000,
    0xe0a0e00000000,
    0x1c141c00000000,
    0x38283800000000,
    0x70507000000000,
    0xe0a0e000000000,
    0xc040c000000000,
    0x302030000000000,
    0x705070000000000,
    0xe0a0e0000000000,
    0x1c141c0000000000,
    0x3828380000000000,
    0x7050700000000000,
    0xe0a0e00000000000,
    0xc040c00000000000,
    0x203000000000000,
    0x507000000000000,
    0xa0e000000000000,
    0x141c000000000000,
    0x2838000000000000,
    0x5070000000000000,
    0xa0e0000000000000,
    0x40c0000000000000,
];

pub const KNIGHT_ATTACKS: [u64; 64] = [
    0x20400,
    0x50800,
    0xa1100,
    0x142200,
    0x284400,
    0x508800,
    0xa01000,
    0x402000,
    0x2040004,
    0x5080008,
    0xa110011,
    0x14220022,
    0x28440044,
    0x50880088,
    0xa0100010,
    0x40200020,
    0x204000402,
    0x508000805,
    0xa1100110a,
    0x1422002214,
    0x2844004428,
    0x5088008850,
    0xa0100010a0,
    0x4020002040,
    0x20400040200,
    0x50800080500,
    0xa1100110a00,
    0x142200221400,
    0x284400442800,
    0x508800885000,
    0xa0100010a000,
    0x402000204000,
    0x2040004020000,
    0x5080008050000,
    0xa1100110a0000,
    0x14220022140000,
    0x28440044280000,
    0x50880088500000,
    0xa0100010a00000,
    0x40200020400000,
    0x204000402000000,
    0x508000805000000,
    0xa1100110a000000,
    0x1422002214000000,
    0x2844004428000000,
    0x5088008850000000,
    0xa0100010a0000000,
    0x4020002040000000,
    0x400040200000000,
    0x800080500000000,
    0x1100110a00000000,
    0x2200221400000000,
    0x4400442800000000,
    0x8800885000000000,
    0x100010a000000000,
    0x2000204000000000,
    0x4020000000000,
    0x8050000000000,
    0x110a0000000000,
    0x22140000000000,
    0x44280000000000,
    0x0088500000000000,
    0x0010a00000000000,
    0x20400000000000,
];

pub const SQUARE_BB: [u64; 65] = [
    0x1,
    0x2,
    0x4,
    0x8,
    0x10,
    0x20,
    0x40,
    0x80,
    0x100,
    0x200,
    0x400,
    0x800,
    0x1000,
    0x2000,
    0x4000,
    0x8000,
    0x10000,
    0x20000,
    0x40000,
    0x80000,
    0x100000,
    0x200000,
    0x400000,
    0x800000,
    0x1000000,
    0x2000000,
    0x4000000,
    0x8000000,
    0x10000000,
    0x20000000,
    0x40000000,
    0x80000000,
    0x100000000,
    0x200000000,
    0x400000000,
    0x800000000,
    0x1000000000,
    0x2000000000,
    0x4000000000,
    0x8000000000,
    0x10000000000,
    0x20000000000,
    0x40000000000,
    0x80000000000,
    0x100000000000,
    0x200000000000,
    0x400000000000,
    0x800000000000,
    0x1000000000000,
    0x2000000000000,
    0x4000000000000,
    0x8000000000000,
    0x10000000000000,
    0x20000000000000,
    0x40000000000000,
    0x80000000000000,
    0x100000000000000,
    0x200000000000000,
    0x400000000000000,
    0x800000000000000,
    0x1000000000000000,
    0x2000000000000000,
    0x4000000000000000,
    0x8000000000000000,
    0x0,
];

const MASK_FILE: [u64; 8] = [
    0x101010101010101,
    0x202020202020202,
    0x404040404040404,
    0x808080808080808,
    0x1010101010101010,
    0x2020202020202020,
    0x4040404040404040,
    0x8080808080808080,
];

const MASK_RANK: [u64; 8] = [
    0xff,
    0xff00,
    0xff0000,
    0xff000000,
    0xff00000000,
    0xff0000000000,
    0xff000000000000,
    0xff00000000000000,
];

const ROOK_MAGICS: [u64; 64] = [
    0xa8002c000108020,
    0x6c00049b0002001,
    0x100200010090040,
    0x2480041000800801,
    0x280028004000800,
    0x900410008040022,
    0x280020001001080,
    0x2880002041000080,
    0xa000800080400034,
    0x4808020004000,
    0x2290802004801000,
    0x411000d00100020,
    0x402800800040080,
    0xb000401004208,
    0x2409000100040200,
    0x1002100004082,
    0x22878001e24000,
    0x1090810021004010,
    0x801030040200012,
    0x500808008001000,
    0xa08018014000880,
    0x8000808004000200,
    0x201008080010200,
    0x801020000441091,
    0x800080204005,
    0x1040200040100048,
    0x120200402082,
    0xd14880480100080,
    0x12040280080080,
    0x100040080020080,
    0x9020010080800200,
    0x813241200148449,
    0x491604001800080,
    0x100401000402001,
    0x4820010021001040,
    0x400402202000812,
    0x209009005000802,
    0x810800601800400,
    0x4301083214000150,
    0x204026458e001401,
    0x40204000808000,
    0x8001008040010020,
    0x8410820820420010,
    0x1003001000090020,
    0x804040008008080,
    0x12000810020004,
    0x1000100200040208,
    0x430000a044020001,
    0x280009023410300,
    0xe0100040002240,
    0x200100401700,
    0x2244100408008080,
    0x8000400801980,
    0x2000810040200,
    0x8010100228810400,
    0x2000009044210200,
    0x4080008040102101,
    0x40002080411d01,
    0x2005524060000901,
    0x502001008400422,
    0x489a000810200402,
    0x1004400080a13,
    0x4000011008020084,
    0x26002114058042,
];

const BISHOP_MAGICS: [u64; 64] = [
    0x89a1121896040240,
    0x2004844802002010,
    0x2068080051921000,
    0x62880a0220200808,
    0x4042004000000,
    0x100822020200011,
    0xc00444222012000a,
    0x28808801216001,
    0x400492088408100,
    0x201c401040c0084,
    0x840800910a0010,
    0x82080240060,
    0x2000840504006000,
    0x30010c4108405004,
    0x1008005410080802,
    0x8144042209100900,
    0x208081020014400,
    0x4800201208ca00,
    0xf18140408012008,
    0x1004002802102001,
    0x841000820080811,
    0x40200200a42008,
    0x800054042000,
    0x88010400410c9000,
    0x520040470104290,
    0x1004040051500081,
    0x2002081833080021,
    0x400c00c010142,
    0x941408200c002000,
    0x658810000806011,
    0x188071040440a00,
    0x4800404002011c00,
    0x104442040404200,
    0x511080202091021,
    0x4022401120400,
    0x80c0040400080120,
    0x8040010040820802,
    0x480810700020090,
    0x102008e00040242,
    0x809005202050100,
    0x8002024220104080,
    0x431008804142000,
    0x19001802081400,
    0x200014208040080,
    0x3308082008200100,
    0x41010500040c020,
    0x4012020c04210308,
    0x208220a202004080,
    0x111040120082000,
    0x6803040141280a00,
    0x2101004202410000,
    0x8200000041108022,
    0x21082088000,
    0x2410204010040,
    0x40100400809000,
    0x822088220820214,
    0x40808090012004,
    0x910224040218c9,
    0x402814422015008,
    0x90014004842410,
    0x1000042304105,
    0x10008830412a00,
    0x2520081090008908,
    0x40102000a0a60140,
];

const ROOK_MASKS: [u64; 64] = init_rook_masks();
const BISHOP_MASKS: [u64; 64] = init_bishop_masks();

static ROOK_ATTACKS: SyncLazy<Vec<[u64; 4096]>> = SyncLazy::new(|| {
    let mut attacks = vec![[0; 4096]; 64];

    for (attack_square, square) in attacks.iter_mut().zip(0..64) {
        *attack_square = init_rook_attacks(square);
    }

    attacks
});

static BISHOP_ATTACKS: SyncLazy<Vec<[u64; 1024]>> = SyncLazy::new(|| {
    let mut attacks = vec![[0; 1024]; 64];

    for (attack_square, square) in attacks.iter_mut().zip(0..64) {
        *attack_square = init_bishop_attacks(square);
    }

    attacks
});

const ROOK_INDEX_BITS: [u64; 64] = [
    12, 11, 11, 11, 11, 11, 11, 12, 11, 10, 10, 10, 10, 10, 10, 11, 11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11, 11, 10, 10, 10, 10, 10, 10, 11, 11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11, 12, 11, 11, 11, 11, 11, 11, 12,
];

const BISHOP_INDEX_BITS: [u64; 64] = [
    6, 5, 5, 5, 5, 5, 5, 6, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 7, 7, 7, 7, 5, 5, 5, 5, 7, 9, 9, 7, 5, 5,
    5, 5, 7, 9, 9, 7, 5, 5, 5, 5, 7, 7, 7, 7, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 6, 5, 5, 5, 5, 5, 5, 6,
];

const fn pop_lsb(mask: &mut u64) -> u64 {
    let index = bitscan_forward(*mask);

    *mask ^= 1 << index;

    index as u64
}

const fn bitscan_forward(board: u64) -> u64 {
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
    return INDEX64[((board.wrapping_mul(DEBRUIJN64)) >> 58) as usize];
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

pub fn rook_attacks(position: Position, mut blockers: u64) -> u64 {
    let square = position.index() as usize;

    blockers &= ROOK_MASKS[square];
    ROOK_ATTACKS[square]
        [((blockers * ROOK_MAGICS[square]) >> (64 - ROOK_INDEX_BITS[square])) as usize]
}

pub fn bishop_attacks(position: Position, mut blockers: u64) -> u64 {
    let square = position.index() as usize;

    blockers &= BISHOP_MASKS[square];
    BISHOP_ATTACKS[square]
        [((blockers * BISHOP_MAGICS[square]) >> (64 - BISHOP_INDEX_BITS[square])) as usize]
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

const fn north_west(board: u64, n: u64) -> u64 {
    let mut new_board = board;
    let mut i = 0;

    while i < n {
        new_board = (new_board >> 1) & (!MASK_FILE[7]);

        i += 1;
    }

    new_board
}

const fn north_east(board: u64, n: u64) -> u64 {
    let mut new_board = board;
    let mut i = 0;

    while i < n {
        new_board = (new_board >> 1) & (!MASK_FILE[0]);

        i += 1;
    }

    new_board
}

const fn row(square: usize) -> u64 {
    (square >> 3) as u64
}

const fn col(square: usize) -> u64 {
    (square % 8) as u64
}
