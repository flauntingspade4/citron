#![warn(clippy::pedantic, clippy::nursery)]
#![feature(once_cell, mixed_integer_ops)]

use core::{
    fmt::{Debug, Display, Formatter},
    ops::Not,
};

pub mod analysis;
mod evaluation;
mod heatmap;
mod killer;
pub mod magic;
// mod move_gen;
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
    teams: [u64; 2],
    pieces: [u64; 6],
    to_play: PlayableTeam,
    turn: u16,
    material: i16,
    absolute_material: i16,
    king_positions: (Position, Position),
}

impl Board {
    const EMPTY_BOARD: Self = Self {
        teams: [0; 2],
        pieces: [0; 6],
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
    pub fn make_move(&self, from: Position, to: Position) -> Option<Self> {
        let mut board = self.clone();

        let mut piece = core::mem::replace(&mut board[from], Piece::EMPTY);

        piece.is_piece().then(|| {
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
        })
    }
    pub fn make_null_move(&self) -> Self {
        let mut board = self.clone();

        board.to_play = !board.to_play;

        board
    }
    fn flip_piece(&mut self, position: Position, piece: Piece) {
        let index = 1 << position.index() as u64;

        let team_index = match piece.team() {
            Team::White => 1,
            Team::Black => 0,
            Team::Neither => panic!(),
        };

        self.teams[team_index] ^= index;

        let piece_index = match piece.kind() {
            PieceKind::None => panic!(),
            PieceKind::Pawn => 0,
            PieceKind::Rook => 1,
            PieceKind::Knight => 2,
            PieceKind::Bishop => 3,
            PieceKind::Queen => 4,
            PieceKind::King => 5,
        };

        self.pieces[piece_index] ^= index;
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
                'p' => board.flip_piece(pos, Piece::new(PieceKind::Pawn, Black)),
                'P' => board.flip_piece(pos, Piece::new(PieceKind::Pawn, White)),
                'r' => board.flip_piece(pos, Piece::new(PieceKind::Rook, Black)),
                'R' => board.flip_piece(pos, Piece::new(PieceKind::Rook, White)),
                'n' => board.flip_piece(pos, Piece::new(PieceKind::Knight, Black)),
                'N' => board.flip_piece(pos, Piece::new(PieceKind::Knight, White)),
                'b' => board.flip_piece(pos, Piece::new(PieceKind::Bishop, Black)),
                'B' => board.flip_piece(pos, Piece::new(PieceKind::Bishop, White)),
                'q' => board.flip_piece(pos, Piece::new(PieceKind::Queen, Black)),
                'Q' => board.flip_piece(pos, Piece::new(PieceKind::Queen, White)),
                'k' => {
                    board.king_positions.1 = pos;
                    board.flip_piece(pos, Piece::new(PieceKind::King, Black))
                }
                'K' => {
                    board.king_positions.0 = pos;
                    board.flip_piece(pos, Piece::new(PieceKind::King, White))
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
    const fn in_endgame(&self) -> bool {
        self.absolute_material <= 24 * PAWN_VALUE
    }
    pub fn team_at(&self, position: Position) -> Team {
        let index = 1 << position.index() as u64;

        if self.teams[0] & index == index {
            Team::White
        } else if self.teams[1] & index == index {
            Team::Black
        } else {
            Team::Neither
        }
    }
    pub fn piece_at(&self, position: Position) -> PieceKind {
        let index = 1 << position.index() as u64;

        if self.pieces[0] & index == index {
            PieceKind::Pawn
        } else if self.pieces[1] & index == index {
            PieceKind::Rook
        } else if self.pieces[2] & index == index {
            PieceKind::Knight
        } else if self.pieces[3] & index == index {
            PieceKind::Bishop
        } else if self.pieces[4] & index == index {
            PieceKind::Queen
        } else if self.pieces[5] & index == index {
            PieceKind::King
        } else {
            PieceKind::None
        }
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
