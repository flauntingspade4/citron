#![warn(clippy::pedantic, clippy::nursery)]
#![feature(once_cell, mixed_integer_ops, const_mut_refs)]

use core::{
    fmt::{Debug, Display, Formatter},
    ops::Not,
};

pub mod analysis;
mod evaluation;
// mod heatmap;
mod killer;
pub mod magic;
mod move_gen;
mod move_ordering;
// pub mod pgn;
pub mod piece;
mod position;
// mod quiescence;
mod transposition_table;

// pub use analysis::explore_line;
pub use position::Position;
// pub use transposition_table::hash;

pub use move_gen::MoveGen;
use piece::{Piece, PAWN_VALUE};
use transposition_table::ZOBRIST_KEYS;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Board {
    // The array of pieces
    pieces: [[u64; 6]; 2],
    all_pieces: [u64; 2],
    attackable: [u64; 2],
    occupied: u64,
    to_play: PlayableTeam,
    turn: u16,
    material: i16,
    absolute_material: i16,
    king_positions: (Position, Position),
    hash: u64,
}

impl Board {
    const EMPTY_BOARD: Self = Self {
        pieces: [[0; 6]; 2],
        all_pieces: [0; 2],
        attackable: [0; 2],
        occupied: 0,
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
    /*pub fn pieces(&self) -> impl Iterator<Item = &Piece> {
        self.board.iter()
    }
    pub fn positions_pieces(&self) -> impl Iterator<Item = (Position, &Piece)> {
        self.positions_squares().filter(|(_, p)| p.is_piece())
    }
    pub fn positions_squares(&self) -> impl Iterator<Item = (Position, &Piece)> {
        Position::positions().zip(self.pieces())
    }*/
    #[must_use]
    pub const fn to_play(&self) -> PlayableTeam {
        self.to_play
    }
    #[must_use]
    pub fn make_move(&self, piece: Piece, from: Position, to: Position) -> Option<Self> {
        let mut board = self.clone();

        board.remove_piece(piece, from);
        board.add_piece(piece, to);

        board.to_play = !board.to_play;
        board.turn += 1;

        Some(board)
    }
    pub fn add_piece(&mut self, piece: Piece, position: Position) {
        let square_index = position.index() as u64;

        let square = 1 << square_index;

        self.pieces[piece.team() as usize][piece.kind() as usize] |= square;
        self.all_pieces[piece.team() as usize] |= square;

        self.occupied |= square;

        self.attackable[piece.team() as usize] ^= square;
        self.hash ^= ZOBRIST_KEYS.0[position.index() as usize][piece as usize];
    }
    fn remove_piece(&mut self, piece: Piece, position: Position) {
        let square = 1 << position.index() as u64;

        self.pieces[piece.team() as usize][piece.kind() as usize] ^= square;
        self.all_pieces[piece.team() as usize] ^= square;

        self.occupied ^= square;
        self.attackable[piece.team() as usize] ^= square;
        self.hash ^= ZOBRIST_KEYS.0[position.index() as usize][piece as usize];
    }
    pub fn make_null_move(&self) -> Self {
        let mut board = self.clone();

        board.to_play = !board.to_play;

        board
    }
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
                    board.add_piece(Piece::BlackKing, pos)
                }
                'K' => {
                    board.king_positions.0 = pos;
                    board.add_piece(Piece::WhiteKing, pos)
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
        // board.calculate_material();

        Some(board)
    }
    fn moved_king(&mut self, to: Position) {
        match self.to_play {
            PlayableTeam::White => self.king_positions.0 = to,
            PlayableTeam::Black => self.king_positions.1 = to,
        }
    }
    const fn in_endgame(&self) -> bool {
        self.absolute_material <= 24 * PAWN_VALUE
    }
}

/*impl Index<Position> for Board {
    type Output = Piece;

    fn index(&self, index: Position) -> &Self::Output {
        let index = index.index() as usize;

        &self.board[index]
    }
}

impl IndexMut<Position> for Board {
    fn index_mut(&mut self, index: Position) -> &mut Self::Output {
        let index = index.index() as usize;

        &mut self.board[index]
    }
}*/

/*impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        for y in 0..8 {
            for x in 0..8 {
                write!(
                    f,
                    "| {} |",
                    self.board[Position::new(x, 7 - y).index() as usize]
                )?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}*/

impl Debug for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Board")
            .field("to_play", &self.to_play)
            .field("turn", &self.turn)
            .field("material", &self.material)
            .finish()
    }
}

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
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PlayableTeam {
    White,
    Black,
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

pub enum TeamComparison {
    /// Both teams were the same
    Same,
    /// Both teams were different
    Different,
    /// Either team was `Team::Neither`
    None,
}
