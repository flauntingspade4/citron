use super::consts::MASK_FILE;

pub const PAWN_ATTACKS: [[u64; 64]; 2] = init_pawn_attacks();

const fn init_pawn_attacks() -> [[u64; 64]; 2] {
    let mut pawn_attacks = [[0; 64]; 2];

    let mut i = 0;

    while i < 64 {
        let start = 1 << i;

        pawn_attacks[0][i] = ((start << 9) & !MASK_FILE[0]) | ((start << 7) & !MASK_FILE[7]);
        pawn_attacks[1][i] = ((start >> 9) & !MASK_FILE[7]) | ((start >> 7) & !MASK_FILE[0]);

        i += 1;
    }

    pawn_attacks
}
