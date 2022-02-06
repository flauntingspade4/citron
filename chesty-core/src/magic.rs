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

const ROOK_TABLE: [SMagic; 64] = [
    SMagic::new(0x101010101017e, 0x80001020400080, 52),
    SMagic::new(0x202020202027c, 0x40001000200040, 53),
    SMagic::new(0x404040404047a, 0x80081000200080, 53),
    SMagic::new(0x8080808080876, 0x80040800100080, 53),
    SMagic::new(0x1010101010106e, 0x80020400080080, 53),
    SMagic::new(0x2020202020205e, 0x80010200040080, 53),
    SMagic::new(0x4040404040403e, 0x80008001000200, 53),
    SMagic::new(0x8080808080807e, 0x80002040800100, 52),
    SMagic::new(0x1010101017e00, 0x800020400080, 53),
    SMagic::new(0x2020202027c00, 0x400020005000, 54),
    SMagic::new(0x4040404047a00, 0x801000200080, 54),
    SMagic::new(0x8080808087600, 0x800800100080, 54),
    SMagic::new(0x10101010106e00, 0x800400080080, 54),
    SMagic::new(0x20202020205e00, 0x800200040080, 54),
    SMagic::new(0x40404040403e00, 0x800100020080, 54),
    SMagic::new(0x80808080807e00, 0x800040800100, 53),
    SMagic::new(0x10101017e0100, 0x208000400080, 53),
    SMagic::new(0x20202027c0200, 0x404000201000, 54),
    SMagic::new(0x40404047a0400, 0x808010002000, 54),
    SMagic::new(0x8080808760800, 0x808008001000, 54),
    SMagic::new(0x101010106e1000, 0x808004000800, 54),
    SMagic::new(0x202020205e2000, 0x808002000400, 54),
    SMagic::new(0x404040403e4000, 0x10100020004, 54),
    SMagic::new(0x808080807e8000, 0x20000408104, 53),
    SMagic::new(0x101017e010100, 0x208080004000, 53),
    SMagic::new(0x202027c020200, 0x200040005000, 54),
    SMagic::new(0x404047a040400, 0x100080200080, 54),
    SMagic::new(0x8080876080800, 0x80080100080, 54),
    SMagic::new(0x1010106e101000, 0x40080080080, 54),
    SMagic::new(0x2020205e202000, 0x20080040080, 54),
    SMagic::new(0x4040403e404000, 0x10080800200, 54),
    SMagic::new(0x8080807e808000, 0x800080004100, 53),
    SMagic::new(0x1017e01010100, 0x204000800080, 53),
    SMagic::new(0x2027c02020200, 0x200040401000, 54),
    SMagic::new(0x4047a04040400, 0x100080802000, 54),
    SMagic::new(0x8087608080800, 0x80080801000, 54),
    SMagic::new(0x10106e10101000, 0x40080800800, 54),
    SMagic::new(0x20205e20202000, 0x20080800400, 54),
    SMagic::new(0x40403e40404000, 0x20001010004, 54),
    SMagic::new(0x80807e80808000, 0x800040800100, 53),
    SMagic::new(0x17e0101010100, 0x204000808000, 53),
    SMagic::new(0x27c0202020200, 0x200040008080, 54),
    SMagic::new(0x47a0404040400, 0x100020008080, 54),
    SMagic::new(0x8760808080800, 0x80010008080, 54),
    SMagic::new(0x106e1010101000, 0x40008008080, 54),
    SMagic::new(0x205e2020202000, 0x20004008080, 54),
    SMagic::new(0x403e4040404000, 0x10002008080, 54),
    SMagic::new(0x807e8080808000, 0x4081020004, 53),
    SMagic::new(0x7e010101010100, 0x204000800080, 53),
    SMagic::new(0x7c020202020200, 0x200040008080, 54),
    SMagic::new(0x7a040404040400, 0x100020008080, 54),
    SMagic::new(0x76080808080800, 0x80010008080, 54),
    SMagic::new(0x6e101010101000, 0x40008008080, 54),
    SMagic::new(0x5e202020202000, 0x20004008080, 54),
    SMagic::new(0x3e404040404000, 0x800100020080, 54),
    SMagic::new(0x7e808080808000, 0x800041000080, 53),
    SMagic::new(0x7e01010101010100, 0xfffcddfced714a, 53),
    SMagic::new(0x7c02020202020200, 0x7ffcddfced714a, 54),
    SMagic::new(0x7a04040404040400, 0x3fffcdffd88096, 54),
    SMagic::new(0x7608080808080800, 0x40810002101, 53),
    SMagic::new(0x6e10101010101000, 0x1000204080011, 53),
    SMagic::new(0x5e20202020202000, 0x1000204000801, 53),
    SMagic::new(0x3e40404040404000, 0x1000082000401, 53),
    SMagic::new(0x7e80808080808000, 0x1fffaabfad1a2, 53),
];

const BISHOP_TABLE: [SMagic; 64] = [
    SMagic::new(0x40201008040200, 0x2020202020200, 58),
    SMagic::new(0x402010080400, 0x2020202020000, 59),
    SMagic::new(0x4020100a00, 0x4010202000000, 59),
    SMagic::new(0x40221400, 0x4040080000000, 59),
    SMagic::new(0x2442800, 0x1104000000000, 59),
    SMagic::new(0x204085000, 0x821040000000, 59),
    SMagic::new(0x20408102000, 0x410410400000, 59),
    SMagic::new(0x2040810204000, 0x104104104000, 58),
    SMagic::new(0x20100804020000, 0x40404040400, 59),
    SMagic::new(0x40201008040000, 0x20202020200, 59),
    SMagic::new(0x4020100a0000, 0x40102020000, 59),
    SMagic::new(0x4022140000, 0x40400800000, 59),
    SMagic::new(0x244280000, 0x11040000000, 59),
    SMagic::new(0x20408500000, 0x8210400000, 59),
    SMagic::new(0x2040810200000, 0x4104104000, 59),
    SMagic::new(0x4081020400000, 0x2082082000, 59),
    SMagic::new(0x10080402000200, 0x4000808080800, 59),
    SMagic::new(0x20100804000400, 0x2000404040400, 59),
    SMagic::new(0x4020100a000a00, 0x1000202020200, 57),
    SMagic::new(0x402214001400, 0x800802004000, 57),
    SMagic::new(0x24428002800, 0x800400a00000, 57),
    SMagic::new(0x2040850005000, 0x200100884000, 57),
    SMagic::new(0x4081020002000, 0x400082082000, 59),
    SMagic::new(0x8102040004000, 0x200041041000, 59),
    SMagic::new(0x8040200020400, 0x2080010101000, 59),
    SMagic::new(0x10080400040800, 0x1040008080800, 59),
    SMagic::new(0x20100a000a1000, 0x208004010400, 57),
    SMagic::new(0x40221400142200, 0x404004010200, 55),
    SMagic::new(0x2442800284400, 0x840000802000, 55),
    SMagic::new(0x4085000500800, 0x404002011000, 57),
    SMagic::new(0x8102000201000, 0x808001041000, 59),
    SMagic::new(0x10204000402000, 0x404000820800, 59),
    SMagic::new(0x4020002040800, 0x1041000202000, 59),
    SMagic::new(0x8040004081000, 0x820800101000, 59),
    SMagic::new(0x100a000a102000, 0x104400080800, 57),
    SMagic::new(0x22140014224000, 0x20080080080, 55),
    SMagic::new(0x44280028440200, 0x404040040100, 55),
    SMagic::new(0x8500050080400, 0x808100020100, 57),
    SMagic::new(0x10200020100800, 0x1010100020800, 59),
    SMagic::new(0x20400040201000, 0x808080010400, 59),
    SMagic::new(0x2000204081000, 0x820820004000, 59),
    SMagic::new(0x4000408102000, 0x410410002000, 59),
    SMagic::new(0xa000a10204000, 0x82088001000, 57),
    SMagic::new(0x14001422400000, 0x2011000800, 57),
    SMagic::new(0x28002844020000, 0x80100400400, 57),
    SMagic::new(0x50005008040200, 0x1010101000200, 57),
    SMagic::new(0x20002010080400, 0x2020202000400, 59),
    SMagic::new(0x40004020100800, 0x1010101000200, 59),
    SMagic::new(0x20408102000, 0x410410400000, 59),
    SMagic::new(0x40810204000, 0x208208200000, 59),
    SMagic::new(0xa1020400000, 0x2084100000, 59),
    SMagic::new(0x142240000000, 0x20880000, 59),
    SMagic::new(0x284402000000, 0x1002020000, 59),
    SMagic::new(0x500804020000, 0x40408020000, 59),
    SMagic::new(0x201008040200, 0x4040404040000, 59),
    SMagic::new(0x402010080400, 0x2020202020000, 59),
    SMagic::new(0x2040810204000, 0x104104104000, 58),
    SMagic::new(0x4081020400000, 0x2082082000, 59),
    SMagic::new(0xa102040000000, 0x20841000, 59),
    SMagic::new(0x14224000000000, 0x208800, 59),
    SMagic::new(0x28440200000000, 0x10020200, 59),
    SMagic::new(0x50080402000000, 0x404080200, 59),
    SMagic::new(0x20100804020000, 0x40404040400, 59),
    SMagic::new(0x40201008040200, 0x2020202020200, 58),
];

/*const MASK_DIAGONAL: [u64; 15] = [
    0x80,
    0x8040,
    0x804020,
    0x80402010,
    0x8040201008,
    0x804020100804,
    0x80402010080402,
    0x8040201008040201,
    0x4020100804020100,
    0x2010080402010000,
    0x1008040201000000,
    0x804020100000000,
    0x402010000000000,
    0x201000000000000,
    0x100000000000000,
];

//Precomputed anti-diagonal masks
const MASK_ANTI_DIAGONAL: [u64; 15] = [
    0x1,
    0x102,
    0x10204,
    0x1020408,
    0x102040810,
    0x10204081020,
    0x1020408102040,
    0x102040810204080,
    0x204081020408000,
    0x408102040800000,
    0x810204080000000,
    0x1020408000000000,
    0x2040800000000000,
    0x4080000000000000,
    0x8000000000000000,
];*/

const ROOK_ATTACKS: [[u64; 64]; 4096] = init_rook_attacks();
const BISHOP_ATTACKS: [[u64; 64]; 1024] = init_bishop_attacks();

/*const fn diagonal_of(position: Position) -> usize {
    7 + position.y() as usize - position.x() as usize
}
const fn anti_diagonal_of(position: Position) -> usize {
    (position.y() + position.x()) as usize
}

const fn sliding_attacks(position: Position, subset: u64, mask: u64) -> u64 {
    (((mask & subset).saturating_sub(SQUARE_BB[position.index() as usize] * 2))
        ^ ((mask & subset)
            .reverse_bits()
            .saturating_sub(SQUARE_BB[position.index() as usize].reverse_bits())
            * 2))
        .reverse_bits()
        & mask
}

const fn get_rook_attacks_for_init(position: Position, subset: u64) -> u64 {
    sliding_attacks(position, subset, MASK_FILE[position.x() as usize])
        | sliding_attacks(position, subset, MASK_RANK[position.y() as usize])
}

const fn init_rook_attacks() -> [[u64; 64]; 4096] {
    let mut attacks = [[0; 64]; 4096];

    let mut index;

    let mut square: usize = 0;

    while square < 64 {
        let mut subset = 0;

        loop {
            index = subset;
            index *= ROOK_TABLE[square].magic;
            index >>= ROOK_TABLE[square].shift;
            attacks[square][index as usize] =
                get_rook_attacks_for_init(Position::from_u8(square as u8), subset);
            subset = (subset.saturating_sub(ROOK_TABLE[square].mask)) & ROOK_TABLE[square].mask;

            if subset == 0 {
                break;
            }
        }

        square += 1;
    }

    attacks
}

const fn get_bishop_attacks_for_init(position: Position, subset: u64) -> u64 {
    sliding_attacks(position, subset, MASK_DIAGONAL[diagonal_of(position)])
        | sliding_attacks(
            position,
            subset,
            MASK_ANTI_DIAGONAL[anti_diagonal_of(position)],
        )
}

const fn init_bishop_attacks() -> [[u64; 64]; 512] {
    let mut attacks = [[0; 64]; 512];

    let mut index;

    let mut square: usize = 0;

    while square < 64 {
        let mut subset = 0;

        loop {
            index = subset;
            index *= BISHOP_TABLE[square].magic;
            index >>= BISHOP_TABLE[square].shift;
            attacks[square][index as usize] =
                get_bishop_attacks_for_init(Position::from_u8(square as u8), subset);
            subset = (subset.saturating_sub(BISHOP_TABLE[square].mask)) & BISHOP_TABLE[square].mask;

            if subset == 0 {
                break;
            }
        }

        square += 1;
    }

    attacks
}*/

const ROOK_INDEX_BITS: [u64; 64] = [
    12, 11, 11, 11, 11, 11, 11, 12, 11, 10, 10, 10, 10, 10, 10, 11, 11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11, 11, 10, 10, 10, 10, 10, 10, 11, 11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11, 12, 11, 11, 11, 11, 11, 11, 12,
];

const BISHOP_INDEX_BITS: [u64; 64] = [
    6, 5, 5, 5, 5, 5, 5, 6, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 7, 7, 7, 7, 5, 5, 5, 5, 7, 9, 9, 7, 5, 5,
    5, 5, 7, 9, 9, 7, 5, 5, 5, 5, 7, 7, 7, 7, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 6, 5, 5, 5, 5, 5, 5, 6,
];

const LSB_64_TABLE: [u8; 154] = [
    22, 0, 0, 0, 30, 0, 0, 38, 18, 0, 16, 15, 17, 0, 46, 9, 19, 8, 7, 10, 0, 63, 1, 56, 55, 57, 2,
    11, 0, 58, 0, 0, 20, 0, 3, 0, 0, 59, 0, 0, 0, 0, 0, 12, 0, 0, 0, 0, 0, 0, 4, 0, 0, 60, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 21, 0, 0, 0, 29, 0, 0, 37, 0, 0, 0, 13, 0, 0, 45, 0, 0, 0, 5, 0, 0, 61, 0,
    0, 0, 53, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 28, 0, 0, 36, 0, 0, 0, 0, 0, 0, 44, 0, 0, 0, 0, 0, 27,
    0, 0, 35, 0, 52, 0, 0, 26, 0, 43, 34, 25, 23, 24, 33, 31, 32, 42, 39, 40, 51, 41, 14, 0, 49,
    47, 48, 0, 50, 6, 0, 0, 62, 0, 0, 0, 54,
];

const fn pop_lsb(mask: &mut u64) -> u64 {
    let index = bitscan_forward(*mask);

    *mask ^= 1 << index;

    index as u64
}

const fn bitscan_forward(mut board: u64) -> u64 {
    let mut t32;
    board ^= board - 1;
    t32 = board ^ (board >> 32);
    t32 ^= 0x01C5FC81;
    t32 += t32 >> 16;
    t32 -= (t32 >> 8) + 51;
    LSB_64_TABLE[(t32 & 255) as usize] as u64
}

const fn bitscan_backward(board: u64) -> u64 {
    64 - bitscan_forward(board.reverse_bits())
}

const fn get_rook_attacks_slow(square: u64, blockers: u64) -> u64 {
    let mut attacks = 0;
    let square = square as usize;

    // North
    attacks |= RAYS[square][Dir::North as usize];
    if RAYS[square][Dir::North as usize] & blockers != 0 {
        attacks &= !RAYS[bitscan_forward(RAYS[square][Dir::North as usize] & blockers) as usize]
            [Dir::North as usize];
    }

    // South
    attacks |= RAYS[square][Dir::South as usize];
    if RAYS[square][Dir::South as usize] & blockers != 0 {
        attacks &= !RAYS[bitscan_backward(RAYS[square][Dir::South as usize] & blockers) as usize]
            [Dir::South as usize];
    }

    // East
    attacks |= RAYS[square][Dir::East as usize];
    if RAYS[square][Dir::East as usize] & blockers != 0 {
        attacks &= !RAYS[bitscan_forward(RAYS[square][Dir::East as usize] & blockers) as usize]
            [Dir::East as usize];
    }

    // West
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

    // North
    attacks |= RAYS[Dir::NorthWest as usize][square];
    if RAYS[Dir::NorthWest as usize][square] & blockers != 0 {
        attacks &= !RAYS[Dir::NorthWest as usize]
            [bitscan_forward(RAYS[Dir::NorthWest as usize][square] & blockers) as usize];
    }

    // South
    attacks |= RAYS[Dir::NorthEast as usize][square];
    if RAYS[Dir::NorthEast as usize][square] & blockers != 0 {
        attacks &= !RAYS[Dir::NorthEast as usize]
            [bitscan_backward(RAYS[Dir::NorthEast as usize][square] & blockers) as usize];
    }

    // East
    attacks |= RAYS[Dir::SouthEast as usize][square];
    if RAYS[Dir::SouthEast as usize][square] & blockers != 0 {
        attacks &= !RAYS[Dir::SouthEast as usize]
            [bitscan_forward(RAYS[Dir::SouthEast as usize][square] & blockers) as usize];
    }

    // West
    attacks |= RAYS[Dir::SouthWest as usize][square];
    if RAYS[Dir::SouthWest as usize][square] & blockers != 0 {
        attacks &= !RAYS[Dir::SouthWest as usize]
            [bitscan_backward(RAYS[Dir::SouthWest as usize][square] & blockers) as usize];
    }

    attacks
}

const fn get_blockers_from_index(index: u64, mut mask: u64) -> u64 {
    let mut blockers = 0;
    let bits = mask.count_ones();

    let mut i = 0;

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

const fn init_rook_attacks() -> [[u64; 64]; 4096] {
    let mut attacks = [[0; 64]; 4096];

    let mut square = 0;

    while square < 64 {
        let mut blocker_index = 0;

        while blocker_index < (1 << ROOK_INDEX_BITS[square]) {
            let blockers = get_blockers_from_index(blocker_index, ROOK_MASKS[square]);
            attacks[((blockers * ROOK_MAGICS[square]) >> (64 - ROOK_INDEX_BITS[square]))
                as usize][square] = get_rook_attacks_slow(square as u64, blockers);

            blocker_index += 1;
        }

        square += 1;
    }

    attacks
}

const fn init_bishop_attacks() -> [[u64; 64]; 1024] {
    let mut attacks = [[0; 64]; 1024];

    let mut square = 0;

    while square < 64 {
        let mut blocker_index = 0;

        while blocker_index < (1 << BISHOP_INDEX_BITS[square]) {
            let blockers = get_blockers_from_index(blocker_index, BISHOP_MASKS[square]);
            attacks[((blockers * BISHOP_MAGICS[square]) >> (64 - BISHOP_INDEX_BITS[square]))
                as usize][square] = get_bishop_attacks_slow(square as u64, blockers);

            blocker_index += 1;
        }

        square += 1;
    }

    attacks
}

pub const fn rook_attacks(position: Position, board: u64) -> u64 {
    ROOK_ATTACKS[position.index() as usize][(((board & ROOK_TABLE[position.index() as usize].mask)
        * ROOK_TABLE[position.index() as usize].magic)
        >> ROOK_TABLE[position.index() as usize].shift)
        as usize]
}

pub const fn bishop_attacks(position: Position, board: u64) -> u64 {
    BISHOP_ATTACKS[position.index() as usize][(((board
        & BISHOP_TABLE[position.index() as usize].mask)
        * BISHOP_TABLE[position.index() as usize].magic)
        >> BISHOP_TABLE[position.index() as usize].shift)
        as usize]
}

pub struct SMagic {
    mask: u64,
    magic: u64,
    shift: u64,
}

impl SMagic {
    pub const fn new(mask: u64, magic: u64, shift: u64) -> Self {
        Self { mask, magic, shift }
    }
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
        // North
        rays[square][Dir::North as usize] = 0x0101010101010100 << square;

        // South
        rays[square][Dir::South as usize] = 0x0080808080808080 >> (63 - square);

        // East
        rays[square][Dir::East as usize] = 2 * ((1 << (square | 7)) - (1 << square));

        // West
        rays[square][Dir::West as usize] = (1 << square) - (1 << (square & 56));

        // North West
        rays[square][Dir::NorthWest as usize] =
            north_west(0x102040810204000, 7 - col(square)) << (row(square) * 8);

        // North East
        rays[square][Dir::NorthEast as usize] =
            north_east(0x8040201008040200, col(square)) << (row(square) * 8);

        // South West
        rays[square][Dir::SouthWest as usize] =
            north_west(0x40201008040201, 7 - col(square)) >> ((7 - row(square)) * 8);

        // South East
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