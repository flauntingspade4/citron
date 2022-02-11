use crate::{piece::PieceKind, Board, PlayableTeam, Position};

/// A way of representing the current move list
pub struct Pgn {
    index: usize,
    moves: String,
    to_play: PlayableTeam,
}

impl Pgn {
    #[must_use]
    /// Creates a new list of moves
    pub const fn new() -> Self {
        Self {
            index: 1,
            moves: String::new(),
            to_play: PlayableTeam::White,
        }
    }
    /// Adds a move to the list
    pub fn add_move(&mut self, played_move: (Position, Position), board: &Board) {
        let played_move = Move {
            from: played_move.0,
            to: played_move.1,
        };

        match self.to_play {
            PlayableTeam::White => {
                self.moves = format!(
                    "{} {}. {}",
                    self.moves,
                    self.index,
                    played_move.write(board)
                );
                self.to_play = !self.to_play;
            }
            PlayableTeam::Black => {
                self.moves = format!("{} {}", self.moves, played_move.write(board));
                self.to_play = !self.to_play;
                self.index += 1;
            }
        }
    }
    #[must_use]
    /// Returns the current move list
    pub fn finish(self) -> String {
        self.moves
    }
}

struct Move {
    from: Position,
    to: Position,
}

impl Move {
    pub fn write(&self, board: &Board) -> String {
        let (x, y) = position_to_uci(self.to);
        let (from_x, _) = position_to_uci(self.from);

        let uci = match piece_to_uci(board[self.from].kind()) {
            Ok(t) => t,
            Err(_) => panic!("{}\n({}) ({})", board, self.from, self.to),
        };

        if board[self.to].is_piece() {
            uci.map_or_else(
                || format!("{}x{}{}", from_x, x, y),
                |piece_identifier| format!("{}{}x{}{}", piece_identifier, from_x, x, y),
            )
        } else {
            uci.map_or_else(
                || format!("{}{}", x, y),
                |piece_identifier| format!("{}{}{}{}", piece_identifier, from_x, x, y),
            )
        }
    }
}

const fn piece_to_uci(kind: PieceKind) -> Result<Option<char>, ()> {
    Ok(match kind {
        PieceKind::None => return Err(()),
        PieceKind::Pawn => None,
        PieceKind::Rook => Some('R'),
        PieceKind::Knight => Some('N'),
        PieceKind::Bishop => Some('B'),
        PieceKind::Queen => Some('Q'),
        PieceKind::King => Some('K'),
    })
}

fn position_to_uci(position: Position) -> (char, char) {
    (
        match position.x() {
            0 => 'a',
            1 => 'b',
            2 => 'c',
            3 => 'd',
            4 => 'e',
            5 => 'f',
            6 => 'g',
            7 => 'h',
            _ => panic!(),
        },
        match position.y() {
            0 => '1',
            1 => '2',
            2 => '3',
            3 => '4',
            4 => '5',
            5 => '6',
            6 => '7',
            7 => '8',
            _ => panic!(),
        },
    )
}

#[test]
fn pgn_gen() {
    let mut board = Board::new();

    let mut pgn = Pgn::new();

    let played_move = (
        Position::from_uci("e2").unwrap(),
        Position::from_uci("e4").unwrap(),
    );

    pgn.add_move(played_move, &board);

    board = board.make_move(played_move.0, played_move.1).unwrap();

    let played_move = (
        Position::from_uci("e7").unwrap(),
        Position::from_uci("e5").unwrap(),
    );

    pgn.add_move(played_move, &board);

    println!("{}", pgn.finish());
}
