use core::fmt::Display;

use crate::{PlayableTeam, Team};

pub const PAWN_VALUE: i16 = 100;
const ROOK_VALUE: i16 = 5 * PAWN_VALUE;
const KNIGHT_VALUE: i16 = 3 * PAWN_VALUE;
const BISHOP_VALUE: i16 = 3 * PAWN_VALUE + 25;
pub const QUEEN_VALUE: i16 = 9 * PAWN_VALUE;
pub const KING_VALUE: i16 = 50 * PAWN_VALUE;

/// A board piece - can be of type `PieceKind::Empty`
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Piece {
    // Information about the piece
    inner: u8,
}

impl Piece {
    pub const EMPTY: Self = Self { inner: 0 };
    #[must_use]
    pub const fn new(kind: PieceKind, team: PlayableTeam) -> Self {
        // The first 4 bits are the piece's kind, the next
        // 2 are for it's team
        let mut inner = match team {
            PlayableTeam::White => 0b0001_1000,
            PlayableTeam::Black => 0b0000_1000,
        };

        let kind = match kind {
            PieceKind::None => 0b0000,
            PieceKind::Pawn => 0b0001,
            PieceKind::Rook => 0b0010,
            PieceKind::Knight => 0b0011,
            PieceKind::Bishop => 0b0100,
            PieceKind::Queen => 0b0101,
            PieceKind::King => 0b0110,
        };

        inner |= kind;

        Self { inner }
    }
    pub(crate) const fn inner(self) -> u8 {
        self.inner
    }
    /// Returns `self`s [`PieceKind`]
    #[must_use]
    pub fn kind(&self) -> PieceKind {
        match self.inner & 0b0111 {
            0b0000 => PieceKind::None,
            0b0001 => PieceKind::Pawn,
            0b0010 => PieceKind::Rook,
            0b0011 => PieceKind::Knight,
            0b0100 => PieceKind::Bishop,
            0b0101 => PieceKind::Queen,
            0b0110 => PieceKind::King,
            _ => unreachable!("this should not happen"),
        }
    }
    /// Returns `self`s team
    #[must_use]
    pub fn team(&self) -> Team {
        match self.inner & 0b0001_1000 {
            0b0001_1000 => Team::White,
            0b0000_1000 => Team::Black,
            0b0000_0000 => Team::Neither,
            _ => unreachable!("this should not happen"),
        }
    }
    /// Checks whether `self` has moved
    #[must_use]
    pub const fn has_moved(&self) -> bool {
        self.inner & 0b1000_0000 == 0b1000_0000
    }
    pub(crate) fn make_move(&mut self) {
        self.inner |= 0b1000_0000;
    }
    /// Returns whether the piece is none
    #[must_use]
    pub fn is_empty(&self) -> bool {
        *self == Self::EMPTY
    }
    /// Returns whether the piece isn't none
    #[must_use]
    pub fn is_piece(&self) -> bool {
        !self.is_empty()
    }
    /// Returns positive [`Piece::piece_value`] if
    /// the piece is white, and negative if black
    #[must_use]
    pub fn value(self) -> i16 {
        let value = self.piece_value();
        if self.team() == Team::White {
            value
        } else {
            -value
        }
    }
    /// Turns `self`s kind to `PieceKind::Queen`
    pub fn promote(&mut self) {
        *self = Self::new(PieceKind::Queen, self.team().try_into().unwrap());
    }
    /// Returns the value of `self`s kind
    #[must_use]
    pub fn piece_value(self) -> i16 {
        match self.kind() {
            PieceKind::None => 0,
            PieceKind::Pawn => PAWN_VALUE,
            PieceKind::Rook => ROOK_VALUE,
            PieceKind::Knight => KNIGHT_VALUE,
            PieceKind::Bishop => BISHOP_VALUE,
            PieceKind::Queen => QUEEN_VALUE,
            PieceKind::King => KING_VALUE,
        }
    }
}

impl Display for Piece {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}",
            match self.team() {
                Team::White => match self.kind() {
                    PieceKind::None => unreachable!("this should not happen"),
                    PieceKind::Pawn => "♟",
                    PieceKind::Rook => "♜",
                    PieceKind::Knight => "♞",
                    PieceKind::Bishop => "♝",
                    PieceKind::Queen => "♛",
                    PieceKind::King => "♚",
                },
                Team::Black => match self.kind() {
                    PieceKind::None => unreachable!("this should not happen"),
                    PieceKind::Pawn => "♙",
                    PieceKind::Rook => "♖",
                    PieceKind::Knight => "♘",
                    PieceKind::Bishop => "♗",
                    PieceKind::Queen => "♕",
                    PieceKind::King => "♔",
                },
                Team::Neither => " ",
            }
        )
    }
}

/// The possible different kinds of piece
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PieceKind {
    None,
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

impl Default for PieceKind {
    fn default() -> Self {
        Self::None
    }
}
