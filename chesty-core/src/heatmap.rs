use crate::{
    piece::{Piece, PieceKind},
    Position, Team,
};

impl Piece {
    #[must_use]
    pub fn positional_value(&self, position: Position) -> i16 {
        let index = position.index() as usize;

        match (self.kind(), self.team()) {
            (PieceKind::Pawn, Team::White) => WHITE_PAWN_HEATMAP[index],
            (PieceKind::Pawn, Team::Black) => BLACK_PAWN_HEATMAP[index],
            (PieceKind::Rook, Team::White) => WHITE_ROOK_HEATMAP[index],
            (PieceKind::Rook, Team::Black) => BLACK_ROOK_HEATMAP[index],
            (PieceKind::Knight, Team::White) => WHITE_KNIGHT_HEATMAP[index],
            (PieceKind::Knight, Team::Black) => BLACK_KNIGHT_HEATMAP[index],
            _ => 0,
        }
    }
}

pub const HEATMAPS: [[&[(u64, i16)]; 2]; 6] = [
    [WHITE_PAWN_HEATMAPS, BLACK_PAWN_HEATMAPS],
    [&[], &[]],
    [&[], &[]],
    [&[], &[]],
    [&[], &[]],
    [&[], &[]],
];

pub const WHITE_PAWN_HEATMAPS: &[(u64, i16)] = &[
    (
        0b00000000_00000000_00000000_00011000_00011000_00000000_00000000_00000000,
        20,
    ),
    (
        0b00000100_00000100_00000100_00000100_00000100_00000100_00000100_00000100,
        30,
    ),
];

pub const BLACK_PAWN_HEATMAPS: &[(u64, i16)] = &[
    (
        0b00000000_00000000_00000000_00001100_00001100_00000000_00000000_00000000,
        -20,
    ),
    (
        0b00100000_00100000_00100000_00100000_00100000_00100000_00100000_00100000,
        -30,
    ),
];

pub const WHITE_PAWN_HEATMAP: [i16; 64] = [
    0, 0, 0, 0, 0, 0, 30, 40, // a file
    0, 0, 0, 0, 0, 0, 30, 40, // b file
    0, 0, 0, 0, 0, 0, 30, 40, // c file
    0, 0, 0, 20, 20, 30, 30, 40, // d file
    0, 0, 0, 20, 20, 30, 30, 40, // e file
    0, 0, 0, 0, 0, 0, 30, 40, // f file
    0, 0, 0, 0, 0, 0, 30, 40, // g file
    0, 0, 0, 0, 0, 0, 30, 40, // h file
];

pub const BLACK_PAWN_HEATMAP: [i16; 64] = [
    -40, -30, 0, 0, 0, 0, 0, 0, // a file
    -40, -30, 0, 0, 0, 0, 0, 0, // b file
    -40, -30, 0, 0, 0, 0, 0, 0, // c file
    -40, -30, -30, -20, -20, 0, 0, 0, // d file
    -40, -30, -30, -20, -20, 0, 0, 0, // e file
    -40, -30, 0, 0, 0, 0, 0, 0, // f file
    -40, -30, 0, 0, 0, 0, 0, 0, // g file
    -40, -30, 0, 0, 0, 0, 0, 0, // h file
];

pub const WHITE_ROOK_HEATMAP: [i16; 64] = [
    10, 0, 0, 0, 0, 0, 0, 0, // a file
    0, 0, 0, 0, 0, 0, 0, 0, // b file
    0, 0, 0, 0, 0, 0, 0, 0, // c file
    35, 0, 0, 0, 0, 0, 0, 0, // d file
    0, 0, 0, 0, 0, 0, 0, 0, // e file
    35, 0, 0, 0, 0, 0, 0, 0, // f file
    0, 0, 0, 0, 0, 0, 0, 0, // g file
    10, 0, 0, 0, 0, 0, 0, 0, // h file
];

pub const BLACK_ROOK_HEATMAP: [i16; 64] = [
    0, 0, 0, 0, 0, 0, 0, -10, // a file
    0, 0, 0, 0, 0, 0, 0, 0, // b file
    0, 0, 0, 0, 0, 0, 0, 0, // c file
    0, 0, 0, 0, 0, 0, 0, -35, // d file
    0, 0, 0, 0, 0, 0, 0, 0, // e file
    0, 0, 0, 0, 0, 0, 0, -35, // f file
    0, 0, 0, 0, 0, 0, 0, 0, // g file
    0, 0, 0, 0, 0, 0, 0, -10, // h file
];

pub const WHITE_KNIGHT_HEATMAP: [i16; 64] = [
    -30, -20, -20, -20, -20, -20, -20, -30, // a file
    -10, 0, 0, 0, 0, 0, 0, 0, // b file
    0, 0, 0, 0, 0, 0, 0, 0, // c file
    0, -10, 0, 0, 0, 0, 0, 0, // d file
    0, -10, 0, 0, 0, 0, 0, 0, // e file
    0, 0, 0, 0, 0, 0, 0, 0, // f file
    -10, 0, 0, 0, 0, 0, 0, 0, // g file
    -30, -20, -20, -20, -20, -20, -20, -30, // h file
];

pub const BLACK_KNIGHT_HEATMAP: [i16; 64] = [
    30, 20, 20, 20, 20, 20, 20, 30, // a file
    0, 0, 0, 0, 0, 0, 0, 10, // b file
    0, 0, 0, 0, 0, 0, 0, 0, // c file
    0, 0, 0, 0, 0, 0, 10, 0, // d file
    0, 0, 0, 0, 0, 0, 10, 0, // e file
    0, 0, 0, 0, 0, 0, 0, 0, // f file
    0, 0, 0, 0, 0, 0, 0, 10, // g file
    30, 20, 20, 20, 20, 20, 20, 30, // h file
];

#[test]
fn positional_value_test() {
    assert_eq!(
        Piece::new(Team::White, PieceKind::Knight)
            .positional_value(Position::from_uci("a8").unwrap()),
        WHITE_KNIGHT_HEATMAP[Position::from_uci("a8").unwrap().index() as usize]
    );
    assert_eq!(
        Piece::new(Team::White, PieceKind::King)
            .positional_value(Position::from_uci("e1").unwrap()),
        0
    );
    assert_eq!(
        Piece::new(Team::Black, PieceKind::King)
            .positional_value(Position::from_uci("e8").unwrap()),
        0
    );
}
