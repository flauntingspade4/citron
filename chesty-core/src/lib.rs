#![warn(clippy::pedantic, clippy::nursery)]
#![feature(once_cell, mixed_integer_ops)]

use core::{
    fmt::{Debug, Display, Formatter},
    ops::{Index, IndexMut, Not},
};

mod analysis;
mod evaluation;
mod heatmap;
mod killer;
mod move_gen;
mod move_ordering;
pub mod pgn;
pub mod piece;
mod position;
mod quiescence;
mod transposition_table;

pub use analysis::explore_line;
pub use position::Position;
pub use transposition_table::hash;

use piece::{Piece, PieceKind, PAWN_VALUE, QUEEN_VALUE};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Board {
    // The array of pieces
    board: [Piece; 64],
    to_play: PlayableTeam,
    turn: u16,
    material: i16,
    absolute_material: i16,
    king_positions: (Position, Position),
}

impl Board {
    const EMPTY_BOARD: Self = Self {
        board: [Piece::EMPTY; 64],
        to_play: PlayableTeam::White,
        turn: 0,
        material: 0,
        absolute_material: 0,
        king_positions: (Position::new(0, 0), Position::new(0, 0)),
    };
    /// Creates a new board, with a default configuration
    #[must_use]
    pub fn new() -> Self {
        Self::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w - - 0 0").unwrap()
    }
    pub fn pieces(&self) -> impl Iterator<Item = &Piece> {
        self.board.iter()
    }
    pub fn positions_pieces(&self) -> impl Iterator<Item = (Position, &Piece)> {
        self.positions_squares().filter(|(_, p)| p.is_piece())
    }
    pub fn positions_squares(&self) -> impl Iterator<Item = (Position, &Piece)> {
        Position::positions().zip(self.pieces())
    }
    #[must_use]
    pub const fn to_play(&self) -> PlayableTeam {
        self.to_play
    }
    #[must_use]
    pub fn make_move(&self, from: Position, to: Position) -> Self {
        let mut board = self.clone();

        let mut piece = core::mem::replace(&mut board[from], Piece::EMPTY);

        piece.make_move();

        let mut castled = false;
        // Pawn promotion
        if piece.kind() == PieceKind::Pawn {
            let y = to.y();
            if y == 0 {
                piece.promote();
                board.material -= QUEEN_VALUE + PAWN_VALUE;
            } else if y == 7 {
                piece.promote();
                board.material += QUEEN_VALUE - PAWN_VALUE;
            }
        } else {
            // Castling
            if piece.kind() == PieceKind::King && from.x() == 4 {
                if to.x() == 0 {
                    let castling_rook = core::mem::take(&mut board[to]);

                    board[Position::new(2, to.y())] = piece;
                    board.moved_king(Position::new(2, to.y()));
                    board[Position::new(3, to.y())] = castling_rook;
                    castled = true;
                } else if to.x() == 7 {
                    let castling_rook = core::mem::take(&mut board[to]);

                    board[Position::new(6, to.y())] = piece;
                    board.moved_king(Position::new(6, to.y()));
                    board[Position::new(5, to.y())] = castling_rook;
                    castled = true;
                }
            }
        }

        if !castled {
            board.material -= board[to].value();
            board[to] = piece;
            if board[to].kind() == PieceKind::King {
                board.moved_king(to);
            }
        };

        board.to_play = !board.to_play;
        board.turn += 1;

        board
    }
    pub fn make_null_move(&self) -> Self {
        let mut board = self.clone();

        board.to_play = !board.to_play;

        board
    }
    #[must_use]
    pub fn from_fen(fen: &str) -> Option<Self> {
        use PlayableTeam::{Black, White};

        let mut board = Self::EMPTY_BOARD;

        let mut fen_parts = fen.split(' ');

        let mut x = 0;
        let mut y = 7;

        for c in fen_parts.next()?.chars() {
            let pos = Position::new(x, y);
            match c {
                'p' => {
                    board[pos] = Piece::new(PieceKind::Pawn, Black);
                    if y != 6 {
                        board[pos].make_move();
                    }
                }
                'P' => {
                    board[pos] = Piece::new(PieceKind::Pawn, White);
                    if y != 1 {
                        board[pos].make_move();
                    }
                }
                'r' => board[pos] = Piece::new(PieceKind::Rook, Black),
                'R' => board[pos] = Piece::new(PieceKind::Rook, White),
                'n' => board[pos] = Piece::new(PieceKind::Knight, Black),
                'N' => board[pos] = Piece::new(PieceKind::Knight, White),
                'b' => board[pos] = Piece::new(PieceKind::Bishop, Black),
                'B' => board[pos] = Piece::new(PieceKind::Bishop, White),
                'q' => board[pos] = Piece::new(PieceKind::Queen, Black),
                'Q' => board[pos] = Piece::new(PieceKind::Queen, White),
                'k' => {
                    board.king_positions.1 = pos;
                    board[pos] = Piece::new(PieceKind::King, Black);
                }
                'K' => {
                    board.king_positions.0 = pos;
                    board[pos] = Piece::new(PieceKind::King, White);
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
    fn moved_king(&mut self, to: Position) {
        match self.to_play {
            PlayableTeam::White => self.king_positions.0 = to,
            PlayableTeam::Black => self.king_positions.1 = to,
        }
    }
}

impl Index<Position> for Board {
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
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        for y in 0..8 {
            for x in 0..8 {
                write!(f, "| {} |", self[Position::new(x, 7 - y)])?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl Debug for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Board")
            .field("to_play", &self.to_play)
            .field("turn", &self.turn)
            .field("material", &self.material)
            .finish()
    }
}

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
