use crate::{move_gen::Move, piece::PieceKind, PlayableTeam};

pub struct Pgn {
    index: usize,
    moves: String,
    to_play: PlayableTeam,
}

impl Pgn {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            index: 1,
            moves: String::new(),
            to_play: PlayableTeam::White,
        }
    }
    pub fn add_move(&mut self, played_move: &Move) {
        match self.to_play {
            PlayableTeam::White => {
                self.moves = format!("{} {}. {}", self.moves, self.index, played_move.write());
                self.to_play = !self.to_play;
            }
            PlayableTeam::Black => {
                self.moves = format!("{} {}", self.moves, played_move.write());
                self.to_play = !self.to_play;
                self.index += 1;
            }
        }
    }
    #[must_use]
    pub fn finish(self) -> String {
        self.moves
    }
}

impl Move {
    fn write(&self) -> String {
        let (x, y) = self.to().to_uci();
        let (from_x, _) = self.from().to_uci();

        let uci = match piece_to_uci(self.moved_piece_kind()) {
            Ok(t) => t,
            Err(_) => panic!(),
        };

        if self.moved_piece_kind() == PieceKind::None {
            uci.map_or_else(
                || format!("{}{}", x, y),
                |piece_identifier| format!("{}{}{}{}", piece_identifier, from_x, x, y),
            )
        } else {
            uci.map_or_else(
                || format!("{}x{}{}", from_x, x, y),
                |piece_identifier| format!("{}{}x{}{}", piece_identifier, from_x, x, y),
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

/*
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
*/
