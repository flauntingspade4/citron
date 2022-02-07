use core::fmt::Display;

use crate::{PlayableTeam, Team};

pub const PAWN_VALUE: i16 = 100;
const ROOK_VALUE: i16 = 5 * PAWN_VALUE;
const KNIGHT_VALUE: i16 = 3 * PAWN_VALUE;
const BISHOP_VALUE: i16 = 3 * PAWN_VALUE + 25;
pub const QUEEN_VALUE: i16 = 9 * PAWN_VALUE;
pub const KING_VALUE: i16 = 50 * PAWN_VALUE;

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Piece {
    WhitePawn,
    WhiteKnight,
    WhiteBishop,
    WhiteRook,
    WhiteQueen,
    WhiteKing,
    BlackPawn,
    BlackKnight,
    BlackBishop,
    BlackRook,
    BlackQueen,
    BlackKing,
    Empty,
}

impl Piece {
    pub fn new(team: Team, kind: PieceKind) -> Self {
        match (team, kind) {
            (Team::White, PieceKind::Pawn) => Self::WhitePawn,
            (Team::White, PieceKind::Rook) => Self::WhiteRook,
            (Team::White, PieceKind::Knight) => Self::WhiteKnight,
            (Team::White, PieceKind::Bishop) => Self::WhiteBishop,
            (Team::White, PieceKind::Queen) => Self::WhiteQueen,
            (Team::White, PieceKind::King) => Self::WhiteKing,
            (Team::Black, PieceKind::Pawn) => Self::BlackPawn,
            (Team::Black, PieceKind::Rook) => Self::BlackRook,
            (Team::Black, PieceKind::Knight) => Self::BlackKnight,
            (Team::Black, PieceKind::Bishop) => Self::BlackBishop,
            (Team::Black, PieceKind::Queen) => Self::BlackQueen,
            (Team::Black, PieceKind::King) => Self::BlackKing,
            _ => Self::Empty,
        }
    }
    pub fn is_empty(&self) -> bool {
        *self == Self::Empty
    }
    pub fn is_piece(&self) -> bool {
        !self.is_empty()
    }
    pub fn kind(&self) -> PieceKind {
        match self {
            Self::WhitePawn | Self::BlackPawn => PieceKind::Pawn,
            Self::WhiteKnight | Self::BlackKnight => PieceKind::Knight,
            Self::WhiteBishop | Self::BlackBishop => PieceKind::Bishop,
            Self::WhiteRook | Self::BlackRook => PieceKind::Rook,
            Self::WhiteQueen | Self::BlackQueen => PieceKind::Queen,
            Self::WhiteKing | Self::BlackKing => PieceKind::King,
            Self::Empty => PieceKind::None,
        }
    }
    pub fn team(&self) -> Team {
        match self {
            Piece::WhitePawn => Team::White,
            Piece::WhiteKnight => Team::White,
            Piece::WhiteBishop => Team::White,
            Piece::WhiteRook => Team::White,
            Piece::WhiteQueen => Team::White,
            Piece::WhiteKing => Team::White,
            Piece::BlackPawn => Team::Black,
            Piece::BlackKnight => Team::Black,
            Piece::BlackBishop => Team::Black,
            Piece::BlackRook => Team::Black,
            Piece::BlackQueen => Team::Black,
            Piece::BlackKing => Team::Black,
            Piece::Empty => Team::Neither,
        }
    }
}

impl Default for Piece {
    fn default() -> Self {
        Self::Empty
    }
}

impl Display for Piece {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::WhitePawn => "♟",
                Self::WhiteRook => "♜",
                Self::WhiteKnight => "♞",
                Self::WhiteBishop => "♝",
                Self::WhiteQueen => "♛",
                Self::WhiteKing => "♚",
                Self::BlackPawn => "♙",
                Self::BlackRook => "♖",
                Self::BlackKnight => "♘",
                Self::BlackBishop => "♗",
                Self::BlackQueen => "♕",
                Self::BlackKing => "♔",
                Self::Empty => panic!("this should not happen"),
            }
        )
    }
}

/// The possible different kinds of piece
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PieceKind {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
    None,
}

impl PieceKind {
    pub fn kinds() -> [Self; 5] {
        [
            Self::Rook,
            Self::Knight,
            Self::Bishop,
            Self::Queen,
            Self::King,
        ]
    }
}

impl Default for PieceKind {
    fn default() -> Self {
        Self::None
    }
}
