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

pub const WHITE_PAWN_HEATMAP: [i16; 64] = [
    0, 0, 0, 0, 0, 0, 30, 40, // a file
    0, 0, 0, 0, 0, 0, 30, 40, // b file
    0, 0, 0, 0, 0, 0, 30, 40, // c file
    0, 0, 10, 20, 20, 30, 30, 40, // d file
    0, 0, 10, 20, 20, 30, 30, 40, // e file
    0, 0, 0, 0, 0, 0, 30, 40, // f file
    0, 0, 0, 0, 0, 0, 30, 40, // g file
    0, 0, 0, 0, 0, 0, 30, 40, // h file
];

pub const BLACK_PAWN_HEATMAP: [i16; 64] = [
    -40, -30, 0, 0, 0, 0, 0, 0, // a file
    -40, -30, 0, 0, 0, 0, 0, 0, // b file
    -40, -30, 0, 0, 0, 0, 0, 0, // c file
    -40, -30, -30, -20, -20, -10, 0, 0, // d file
    -40, -30, -30, -20, -20, -10, 0, 0, // e file
    -40, -30, 0, 0, 0, 0, 0, 0, // f file
    -40, -30, 0, 0, 0, 0, 0, 0, // g file
    -40, -30, 0, 0, 0, 0, 0, 0, // h file
];

pub const WHITE_ROOK_HEATMAP: [i16; 64] = [
    10, 0, 0, 0, 0, 0, 0, 0, // a file
    0, 0, 0, 0, 0, 0, 0, 0, // b file
    0, 0, 0, 0, 0, 0, 0, 0, // c file
    20, 0, 0, 0, 0, 0, 0, 0, // d file
    20, 0, 0, 0, 0, 0, 0, 0, // e file
    0, 0, 0, 0, 0, 0, 0, 0, // f file
    0, 0, 0, 0, 0, 0, 0, 0, // g file
    10, 0, 0, 0, 0, 0, 0, 0, // h file
];

pub const BLACK_ROOK_HEATMAP: [i16; 64] = [
    0, 0, 0, 0, 0, 0, 0, -10, // a file
    0, 0, 0, 0, 0, 0, 0, 0, // b file
    0, 0, 0, 0, 0, 0, 0, 0, // c file
    0, 0, 0, 0, 0, 0, 0, -20, // d file
    0, 0, 0, 0, 0, 0, 0, -20, // e file
    0, 0, 0, 0, 0, 0, 0, 0, // f file
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
