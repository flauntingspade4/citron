#![warn(clippy::pedantic, clippy::nursery)]

use core::{
    fmt::{Debug, Display, Formatter},
    ops::Not,
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub mod analysis;
mod board;
mod evaluation;
#[cfg(feature = "nn_evaluation")]
pub mod ga;
mod heatmap;
mod killer;
pub mod magic;
pub mod move_gen;
mod move_ordering;
#[cfg(feature = "nn_evaluation")]
pub mod nn;
pub mod pgn;
pub mod piece;
mod position;
mod quiescence;
mod transposition_table;

use move_gen::Move;
pub use position::Position;

pub use board::Board;
pub use move_gen::MoveGen;
use piece::{Piece, PieceKind};
pub use transposition_table::hash;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Game {
    pub board: Board,
    turn: u16,
    /// The material count. A negative count indicates it's in black's favour,
    /// and a positive in white's
    pub material: i16,
    /// The amount of material remaining on the board, the kings
    pub absolute_material: i16,
    /// The position of each side's king
    king_positions: (Position, Position),
}

impl Game {
    /// Creates a new board, with a default configuration
    #[must_use]
    pub fn new() -> Self {
        Self::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w - - 0 0").unwrap()
    }
    /// Returns which side is currently to play
    #[must_use]
    pub const fn to_play(&self) -> PlayableTeam {
        self.board.to_play()
    }
    /// Makes a [`Move`]
    // TODO Make this mutate the board instead and unmake the move
    pub fn make_move(&self, played_move: &Move) -> Self {
        let mut game = self.clone();

        if game.board.to_play() == PlayableTeam::White {
            game.material += played_move.captured_piece_kind().value();
        } else {
            game.material -= played_move.captured_piece_kind().value();
        }

        game.absolute_material -= played_move.captured_piece_kind().value();

        game.board = game.board.make_move(played_move);
        game.turn += 1;

        game
    }
    pub fn add_piece(&self, added_piece: Piece, position: Position) -> Self {
        let mut game = self.clone();

        if game.board.to_play() == PlayableTeam::White {
            game.material += added_piece.kind().value();
        } else {
            game.material -= added_piece.kind().value();
        }

        game.absolute_material += added_piece.kind().value();

        game.board.add_piece(added_piece, position);
        game.turn += 1;

        game
    }
    /// Makes a null move (Effectively just switching who it is to move)
    #[must_use]
    pub fn make_null_move(&self) -> Self {
        let mut game = self.clone();

        game.board = game.board.make_null_move();
        game.turn += 1;

        game
    }

    pub fn castle(&self, castling_side: CastlingSide) -> Self {
        let game = match self.to_play() {
            PlayableTeam::White => match castling_side {
                CastlingSide::QueenSide => {
                    let king_moved = self.make_move(&Move::new(
                        Position::new(4, 0),
                        Position::new(2, 0),
                        PieceKind::King,
                        PieceKind::None,
                    ));
                    king_moved.make_move(&Move::new(
                        Position::new(0, 0),
                        Position::new(3, 0),
                        PieceKind::Rook,
                        PieceKind::None,
                    ))
                }
                CastlingSide::KingSide => {
                    let king_moved = self.make_move(&Move::new(
                        Position::new(4, 0),
                        Position::new(6, 0),
                        PieceKind::King,
                        PieceKind::None,
                    ));
                    king_moved.make_move(&Move::new(
                        Position::new(0, 0),
                        Position::new(5, 0),
                        PieceKind::Rook,
                        PieceKind::None,
                    ))
                }
            },
            PlayableTeam::Black => match castling_side {
                CastlingSide::QueenSide => {
                    let king_moved = self.make_move(&Move::new(
                        Position::new(4, 7),
                        Position::new(2, 7),
                        PieceKind::King,
                        PieceKind::None,
                    ));
                    king_moved.make_move(&Move::new(
                        Position::new(0, 7),
                        Position::new(3, 7),
                        PieceKind::Rook,
                        PieceKind::None,
                    ))
                }
                CastlingSide::KingSide => {
                    let king_moved = self.make_move(&Move::new(
                        Position::new(4, 7),
                        Position::new(6, 7),
                        PieceKind::King,
                        PieceKind::None,
                    ));
                    king_moved.make_move(&Move::new(
                        Position::new(0, 7),
                        Position::new(5, 7),
                        PieceKind::Rook,
                        PieceKind::None,
                    ))
                }
            },
        };

        game
    }

    /// Creates a board from a given FEN
    #[must_use]
    pub fn from_fen(fen: &str) -> Option<Self> {
        let board = Board::from_fen(fen)?;
        let mut fen_parts = fen.split(' ');

        fen_parts.next()?;
        fen_parts.next()?;
        fen_parts.next()?;
        fen_parts.next()?;

        let turn = fen_parts.next()?.parse().ok()?;

        let mut game = Self {
            board,
            turn,
            material: 0,
            absolute_material: 0,
            king_positions: (Position::new(0, 0), Position::new(0, 0)),
        };

        game.calculate_material();
        Some(game)
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        for y in 0..8 {
            for x in 0..8 {
                write!(f, "| {} |", self.board.piece_at(Position::new(x, 7 - y)))?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

pub enum CastlingSide {
    QueenSide,
    KingSide,
}

/*
#[test]
fn king_position_test() {
    let board = Board::new();

    assert_eq!(
        board.king_positions,
        (
            Position::from_uci("e1").unwrap(),
            Position::from_uci("e8").unwrap()
        )
    );

    // Not a legal move
    let king_moved = board
        .make_move(
            Position::from_uci("e1").unwrap(),
            Position::from_uci("e2").unwrap(),
        )
        .unwrap();

    assert_eq!(
        king_moved.king_positions,
        (
            Position::from_uci("e2").unwrap(),
            Position::from_uci("e8").unwrap()
        )
    );
}*/

/// A playable team
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PlayableTeam {
    White,
    Black,
}

impl PlayableTeam {
    #[must_use]
    pub const fn teams() -> [Self; 2] {
        [Self::White, Self::Black]
    }

    pub fn compare(&self, other: &Self) -> TeamComparison {
        match (self, other) {
            (PlayableTeam::White, PlayableTeam::White)
            | (PlayableTeam::Black, PlayableTeam::Black) => TeamComparison::Same,
            _ => TeamComparison::Different,
        }
    }
}

impl Display for PlayableTeam {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::White => "white",
                Self::Black => "black",
            }
        )
    }
}

impl TryFrom<Team> for PlayableTeam {
    type Error = ();

    fn try_from(team: Team) -> Result<Self, Self::Error> {
        Ok(match team {
            Team::White => Self::White,
            Team::Black => Self::Black,
            Team::Neither => return Err(()),
        })
    }
}

impl Not for PlayableTeam {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Team {
    White,
    Black,
    Neither,
}

impl Team {
    #[must_use]
    pub const fn compare(&self, other: &Self) -> TeamComparison {
        match (self, other) {
            (Team::White, Team::White) | (Team::Black, Team::Black) => TeamComparison::Same,
            (Team::White, Team::Black) | (Team::Black, Team::White) => TeamComparison::Different,
            _ => TeamComparison::None,
        }
    }
    #[must_use]
    pub const fn teams() -> [Self; 3] {
        [Self::White, Self::Black, Self::Neither]
    }
}

impl From<PlayableTeam> for Team {
    fn from(team: PlayableTeam) -> Self {
        match team {
            PlayableTeam::White => Self::White,
            PlayableTeam::Black => Self::Black,
        }
    }
}

impl Not for Team {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
            Self::Neither => Self::Neither,
        }
    }
}

impl Display for Team {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::White => "white",
                Self::Black => "black",
                Self::Neither => "neither",
            }
        )
    }
}

/// A comparison between two different teams, returned from [`Team::compare`]
pub enum TeamComparison {
    /// Both teams were the same
    Same,
    /// Both teams were different
    Different,
    /// Either team was `Team::Neither`
    None,
}
