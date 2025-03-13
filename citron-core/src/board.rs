use std::fmt::{Display, Formatter};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{
    move_gen::Move,
    piece::{Piece, PieceKind},
    transposition_table::ZOBRIST_KEYS,
    PlayableTeam, Position, Team,
};

/// The chess board itself. Most functionality of the engine is
/// implemented as methods on this struct
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Board {
    /// Two arrays of each piece's bitmaps
    pub(crate) pieces: [[u64; 6]; 2],
    /// Two bitmaps, one for each team
    all_pieces: [u64; 2],
    /// The team that's turn it is to play
    to_play: PlayableTeam,
    hash: u64,
}

impl Board {
    pub const EMPTY_BOARD: Self = Self {
        pieces: [[0; 6]; 2],
        all_pieces: [0; 2],
        to_play: PlayableTeam::White,
        hash: 0,
    };

    /// Creates a new board in the starting position
    #[must_use]
    pub fn new() -> Self {
        Self::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w - - 0 0").unwrap()
    }

    /// Returns which side is currently to play
    #[must_use]
    pub const fn to_play(&self) -> PlayableTeam {
        self.to_play
    }

    pub const fn pieces(&self) -> [[u64; 6]; 2] {
        self.pieces
    }

    pub const fn all_pieces(&self) -> [u64; 2] {
        self.all_pieces
    }

    pub fn result(&self) -> Option<PlayableTeam> {
        match (
            self.pieces[0][PieceKind::King as usize].count_ones(),
            self.pieces[1][PieceKind::King as usize].count_ones(),
        ) {
            (1, 0) => Some(PlayableTeam::White),
            (0, 1) => Some(PlayableTeam::Black),
            (1, 1) => None,
            _ => panic!(
                "How are there {} white kings and {} black kings???",
                self.pieces[0][PieceKind::King as usize].count_ones(),
                self.pieces[1][PieceKind::King as usize].count_ones()
            ),
        }
    }

    /// Plays a [`Move`], returning the new board with the move played and
    /// all variables updated
    pub fn make_move(&self, played_move: &Move) -> Self {
        let mut board = self.clone();

        if played_move.captured_piece_kind() != PieceKind::None {
            board.remove_piece(
                Piece::new(!board.to_play, played_move.captured_piece_kind()),
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

        board
    }

    pub fn move_piece(&mut self, kind: PieceKind, from: Position, to: Position) {
        let piece = Piece::new(self.to_play, kind);

        self.remove_piece(piece, from);
        self.add_piece(piece, to);
    }

    pub fn add_piece(&mut self, piece: Piece, position: Position) {
        let square = position.to_bitmap();

        self.pieces[piece.team() as usize][piece.kind() as usize] |= square;
        self.all_pieces[piece.team() as usize] |= square;

        self.hash ^= ZOBRIST_KEYS.0[position.index() as usize][piece as usize];
    }

    pub fn remove_piece(&mut self, piece: Piece, position: Position) {
        let square = position.to_bitmap();

        // Check there is a piece there
        if self.pieces[piece.team() as usize][piece.kind() as usize]
            & self.all_pieces[piece.team() as usize]
            & square
            == square
        {
            self.pieces[piece.team() as usize][piece.kind() as usize] ^= square;
            self.all_pieces[piece.team() as usize] ^= square;

            self.hash ^= ZOBRIST_KEYS.0[position.index() as usize][piece as usize];
        }
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
                    board.add_piece(Piece::BlackKing, pos);
                }
                'K' => {
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

        Some(board)
    }
    #[must_use]
    pub fn kind_at(&self, team: PlayableTeam, position: Position) -> PieceKind {
        let bitmap = position.to_bitmap();

        if self.all_pieces[team as usize] & bitmap == 0 {
            PieceKind::None
        } else {
            for (pieces, kind) in self.pieces[team as usize].iter().zip(PieceKind::kinds()) {
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
                for (pieces, kind) in self.pieces[team as usize].iter().zip(PieceKind::kinds()) {
                    if pieces & bitmap == bitmap {
                        return Piece::new(team, kind);
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

    pub const fn blockers(&self) -> u64 {
        self.all_pieces[0] | self.all_pieces[1]
    }

    pub fn not_blockers(&self) -> u64 {
        !self.blockers()
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
