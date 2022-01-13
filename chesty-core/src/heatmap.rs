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
    0, 0, 0, 0, 0, 0, 3, 4, // a file
    0, 0, 0, 0, 0, 0, 3, 4, // b file
    0, 0, 0, 0, 0, 0, 3, 4, // c file
    0, 1, 1, 2, 2, 3, 3, 4, // d file
    0, 1, 1, 2, 2, 3, 3, 4, // e file
    0, 0, 0, 0, 0, 0, 3, 4, // f file
    0, 0, 0, 0, 0, 0, 3, 4, // g file
    0, 0, 0, 0, 0, 0, 3, 4, // h file
];

pub const BLACK_PAWN_HEATMAP: [i16; 64] = [
    -4, -3, 0, 0, 0, 0, 0, 0, // a file
    -4, -3, 0, 0, 0, 0, 0, 0, // b file
    -4, -3, 0, 0, 0, 0, 0, 0, // c file
    -4, -3, -3, -2, -2, -1, -1, 0, // d file
    -4, -3, -3, -2, -2, -1, -1, 0, // e file
    -4, -3, 0, 0, 0, 0, 0, 0, // f file
    -4, -3, 0, 0, 0, 0, 0, 0, // g file
    -4, -3, 0, 0, 0, 0, 0, 0, // h file
];

pub const WHITE_ROOK_HEATMAP: [i16; 64] = [
    1, 0, 0, 0, 0, 0, 0, 0, // a file
    0, 0, 0, 0, 0, 0, 0, 0, // b file
    0, 0, 0, 0, 0, 0, 0, 0, // c file
    2, 0, 0, 0, 0, 0, 0, 0, // d file
    2, 0, 0, 0, 0, 0, 0, 0, // e file
    0, 0, 0, 0, 0, 0, 0, 0, // f file
    0, 0, 0, 0, 0, 0, 0, 0, // g file
    1, 0, 0, 0, 0, 0, 0, 0, // h file
];

pub const BLACK_ROOK_HEATMAP: [i16; 64] = [
    0, 0, 0, 0, 0, 0, 0, -1, // a file
    0, 0, 0, 0, 0, 0, 0, 0, // b file
    0, 0, 0, 0, 0, 0, 0, 0, // c file
    0, 0, 0, 0, 0, 0, 0, -2, // d file
    0, 0, 0, 0, 0, 0, 0, -2, // e file
    0, 0, 0, 0, 0, 0, 0, 0, // f file
    0, 0, 0, 0, 0, 0, 0, 0, // g file
    0, 0, 0, 0, 0, 0, 0, -1, // h file
];

pub const WHITE_KNIGHT_HEATMAP: [i16; 64] = [
    -3, -2, -2, -2, -2, -2, -2, -3, // a file
    -1, 0, 0, 0, 0, 0, 0, 0, // b file
    0, 0, 0, 0, 0, 0, 0, 0, // c file
    0, 0, 0, 0, 0, 0, 0, 0, // d file
    0, 0, 0, 0, 0, 0, 0, 0, // e file
    0, 0, 0, 0, 0, 0, 0, 0, // f file
    -1, 0, 0, 0, 0, 0, 0, 0, // g file
    -3, -2, -2, -2, -2, -2, -2, -3, // h file
];

pub const BLACK_KNIGHT_HEATMAP: [i16; 64] = [
    3, 2, 2, 2, 2, 2, 2, 3, // a file
    0, 0, 0, 0, 0, 0, 0, 1, // b file
    0, 0, 0, 0, 0, 0, 0, 0, // c file
    0, 0, 0, 0, 0, 0, 0, 0, // d file
    0, 0, 0, 0, 0, 0, 0, 0, // e file
    0, 0, 0, 0, 0, 0, 0, 0, // f file
    0, 0, 0, 0, 0, 0, 0, 1, // g file
    3, 2, 2, 2, 2, 2, 2, 3, // h file
];
