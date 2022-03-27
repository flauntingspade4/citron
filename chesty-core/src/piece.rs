use core::fmt::Display;

use crate::Team;

pub const PAWN_VALUE: i16 = 100;
const ROOK_VALUE: i16 = 5 * PAWN_VALUE;
const KNIGHT_VALUE: i16 = 3 * PAWN_VALUE;
const BISHOP_VALUE: i16 = 3 * PAWN_VALUE + 25;
pub const QUEEN_VALUE: i16 = 9 * PAWN_VALUE;
pub const KING_VALUE: i16 = 50 * PAWN_VALUE;

/// An enum representing all the different possible pieces
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
    /// Create a piece based off it's [`Team`] and [`PieceKind`]
    #[must_use]
    pub const fn new(team: Team, kind: PieceKind) -> Self {
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
    /// Returns true if the variant is `Piece::Empty`
    #[must_use]
    pub fn is_empty(&self) -> bool {
        *self == Self::Empty
    }
    #[must_use]
    pub fn is_piece(&self) -> bool {
        !self.is_empty()
    }
    #[must_use]
    pub const fn kind(&self) -> PieceKind {
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
    #[must_use]
    pub const fn team(&self) -> Team {
        match self {
            Piece::WhitePawn
            | Piece::WhiteKnight
            | Piece::WhiteBishop
            | Piece::WhiteRook
            | Piece::WhiteQueen
            | Piece::WhiteKing => Team::White,
            Piece::BlackPawn
            | Piece::BlackKnight
            | Piece::BlackBishop
            | Piece::BlackRook
            | Piece::BlackQueen
            | Piece::BlackKing => Team::Black,
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
                Self::Empty => " ",
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
    /// Returns an array of different kind of piece, without the pawn
    #[must_use]
    pub const fn kinds_no_pawn() -> [Self; 5] {
        [
            Self::Rook,
            Self::Knight,
            Self::Bishop,
            Self::Queen,
            Self::King,
        ]
    }
    /// Returns an array of different kind of piece, without the pawn
    #[must_use]
    pub const fn kinds_no_king() -> [Self; 5] {
        [
            Self::Pawn,
            Self::Rook,
            Self::Knight,
            Self::Bishop,
            Self::Queen,
        ]
    }
    /// Returns an array of different kind of piece
    #[must_use]
    pub const fn kinds() -> [Self; 6] {
        [
            Self::Pawn,
            Self::Rook,
            Self::Knight,
            Self::Bishop,
            Self::Queen,
            Self::King,
        ]
    }
    /// Returns the value of the the variant
    #[must_use]
    pub const fn value(&self) -> i16 {
        match self {
            PieceKind::Pawn => PAWN_VALUE,
            PieceKind::Rook => ROOK_VALUE,
            PieceKind::Knight => KNIGHT_VALUE,
            PieceKind::Bishop => BISHOP_VALUE,
            PieceKind::Queen => QUEEN_VALUE,
            PieceKind::King => KING_VALUE,
            PieceKind::None => 0,
        }
    }
}

impl Default for PieceKind {
    fn default() -> Self {
        Self::None
    }
}
