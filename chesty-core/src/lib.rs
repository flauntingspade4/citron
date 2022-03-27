#![warn(clippy::pedantic, clippy::nursery)]
#![feature(once_cell, mixed_integer_ops, const_mut_refs, array_zip)]

use core::{
    fmt::{Debug, Display, Formatter},
    ops::Not,
};

pub mod analysis;
mod evaluation;
mod heatmap;
mod killer;
pub mod magic;
pub mod move_gen;
mod move_ordering;
pub mod pgn;
pub mod piece;
mod position;
mod quiescence;
mod transposition_table;

use move_gen::Move;
pub use position::Position;

pub use move_gen::MoveGen;
use piece::{Piece, PieceKind, PAWN_VALUE};
use transposition_table::ZOBRIST_KEYS;

/// The chess board itself. Most functionality of the engine is
/// implemented as methods on this struct
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Board {
    /// Two arrays of each piece's bitmaps
    pieces: [[u64; 6]; 2],
    /// Two bitmaps, one for each team
    all_pieces: [u64; 2],
    /// The team that's turn it is to play
    to_play: PlayableTeam,
    turn: u16,
    /// The material count. A negative count indicates it's in black's favour,
    /// and a positive in white's
    pub material: i16,
    /// The amount of material remaining on the board, excluding any material
    /// kings may be worth
    pub absolute_material: i16,
    /// The position of each side's king
    king_positions: (Position, Position),
    /// The hash of the current board
    hash: u64,
}

impl Board {
    const EMPTY_BOARD: Self = Self {
        pieces: [[0; 6]; 2],
        all_pieces: [0; 2],
        to_play: PlayableTeam::White,
        turn: 0,
        material: 0,
        absolute_material: 0,
        king_positions: (Position::new(0, 0), Position::new(0, 0)),
        hash: 0,
    };
    /// Creates a new board, with a default configuration
    #[must_use]
    pub fn new() -> Self {
        Self::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w - - 0 0").unwrap()
    }
    /// Returns which side is currently to play
    #[must_use]
    pub const fn to_play(&self) -> PlayableTeam {
        self.to_play
    }
    /// Makes a [`Move`]
    #[must_use]
    pub fn make_move(&self, played_move: &Move) -> Option<Self> {
        let mut board = self.clone();

        if played_move.captured_piece_kind() != PieceKind::None {
            if board.to_play == PlayableTeam::White {
                board.material += played_move.captured_piece_kind().value();
            } else {
                board.material -= played_move.captured_piece_kind().value();
            }
            board.absolute_material -= played_move.captured_piece_kind().value();
            board.remove_piece(
                Piece::new((!self.to_play).into(), played_move.captured_piece_kind()),
                played_move.to(),
            );
        }

        board.move_piece(
            played_move.moved_piece_kind(),
            played_move.from(),
            played_move.to(),
        );

        board.to_play = !board.to_play;

        if board.to_play == PlayableTeam::Black {
            board.hash ^= ZOBRIST_KEYS.1;
        }

        Some(board)
    }
    fn move_piece(&mut self, kind: PieceKind, from: Position, to: Position) {
        let piece = Piece::new(self.to_play.into(), kind);

        self.remove_piece(piece, from);
        self.add_piece(piece, to);
    }
    fn add_piece(&mut self, piece: Piece, position: Position) {
        let square = 1 << position.index();

        self.pieces[piece.team() as usize][piece.kind() as usize] |= square;
        self.all_pieces[piece.team() as usize] |= square;

        self.hash ^= ZOBRIST_KEYS.0[position.index() as usize][piece as usize];
    }
    fn remove_piece(&mut self, piece: Piece, position: Position) {
        let square = 1 << position.index();

        self.pieces[piece.team() as usize][piece.kind() as usize] ^= square;
        self.all_pieces[piece.team() as usize] ^= square;

        self.hash ^= ZOBRIST_KEYS.0[position.index() as usize][piece as usize];
    }
    /// Makes a null move (Effectively just switching who it is to move)
    #[must_use]
    pub fn make_null_move(&self) -> Self {
        let mut board = self.clone();

        board.to_play = !board.to_play;

        board
    }
    /// Creates a board from a given FEN
    #[must_use]
    pub fn from_fen(fen: &str) -> Option<Self> {
        let mut board = Self::EMPTY_BOARD;

        let mut fen_parts = fen.split(' ');

        let mut x = 0;
        let mut y = 7;

        for c in fen_parts.next()?.chars() {
            let pos = Position::new(x, y);
            match c {
                'p' => board.add_piece(Piece::BlackPawn, pos),
                'P' => board.add_piece(Piece::WhitePawn, pos),
                'r' => board.add_piece(Piece::BlackRook, pos),
                'R' => board.add_piece(Piece::WhiteRook, pos),
                'n' => board.add_piece(Piece::BlackKnight, pos),
                'N' => board.add_piece(Piece::WhiteKnight, pos),
                'b' => board.add_piece(Piece::BlackBishop, pos),
                'B' => board.add_piece(Piece::WhiteBishop, pos),
                'q' => board.add_piece(Piece::BlackQueen, pos),
                'Q' => board.add_piece(Piece::WhiteQueen, pos),
                'k' => {
                    board.king_positions.1 = pos;
                    board.add_piece(Piece::BlackKing, pos);
                }
                'K' => {
                    board.king_positions.0 = pos;
                    board.add_piece(Piece::WhiteKing, pos);
                }
                '/' => {
                    if x == 8 {
                        x = 0;
                        y -= 1;
                    } else {
                        return None;
                    }
                }
                a if a.is_numeric() => {
                    let a = a.to_digit(10)?;
                    x += (a - 1) as u8;
                }
                _ => return None,
            }
            if c != '/' {
                x += 1;
            }
        }

        if let Some(to_play) = fen_parts.next() {
            if to_play.trim() == "b" {
                board.to_play = PlayableTeam::Black;
            }
        }

        // Castling rights
        fen_parts.next()?;
        fen_parts.next()?;

        // En passant
        // fen_parts.next()?;

        // Half move clock
        fen_parts.next()?;

        let turn = fen_parts.next()?.parse().ok()?;

        board.turn = turn;
        board.calculate_material();

        Some(board)
    }
    /*fn moved_king(&mut self, to: Position) {
        match self.to_play {
            PlayableTeam::White => self.king_positions.0 = to,
            PlayableTeam::Black => self.king_positions.1 = to,
        }
    }*/
    const fn in_endgame(&self) -> bool {
        self.absolute_material <= 24 * PAWN_VALUE
    }
    #[must_use]
    pub fn kind_at(&self, team: PlayableTeam, position: Position) -> PieceKind {
        let bitmap = position.to_bitmap();

        if self.all_pieces[team as usize] & bitmap == 0 {
            PieceKind::None
        } else {
            for (pieces, kind) in self.pieces[team as usize].zip(PieceKind::kinds()) {
                if pieces & bitmap == bitmap {
                    return kind;
                }
            }

            PieceKind::None
        }
    }
    #[must_use]
    pub fn team_at(&self, position: Position) -> Team {
        let bitmap = position.to_bitmap();

        for team in PlayableTeam::teams() {
            if self.all_pieces[team as usize] & bitmap == bitmap {
                return team.into();
            }
        }

        Team::Neither
    }
    #[must_use]
    pub fn piece_at(&self, position: Position) -> Piece {
        let bitmap = position.to_bitmap();

        for team in PlayableTeam::teams() {
            if self.all_pieces[team as usize] & bitmap != 0 {
                for (pieces, kind) in self.pieces[team as usize].zip(PieceKind::kinds()) {
                    if pieces & bitmap == bitmap {
                        return Piece::new(team.into(), kind);
                    }
                }
            }
        }

        Piece::Empty
    }
    /// Returns the hash of the current board
    #[must_use]
    pub const fn hash(&self) -> u64 {
        self.hash
    }
    fn get_occupied(&self) -> u64 {
        self.all_pieces[0] | self.all_pieces[1]
    }
    fn get_not_occupied(&self) -> u64 {
        !self.get_occupied()
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        for y in 0..8 {
            for x in 0..8 {
                write!(f, "| {} |", self.piece_at(Position::new(x, 7 - y)))?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
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
pub enum PlayableTeam {
    White,
    Black,
}

impl PlayableTeam {
    #[must_use]
    pub const fn teams() -> [Self; 2] {
        [Self::White, Self::Black]
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
