use crate::{
    magic,
    piece::{Piece, PieceKind},
    Board, PlayableTeam, Position,
};

pub const BRANCHING_FACTOR: usize = 35;

pub struct MoveGen {
    move_list: Vec<(Position, Position, u16)>,
}

impl MoveGen {
    pub fn new(board: &Board) -> Self {
        let mut move_list = Vec::with_capacity(BRANCHING_FACTOR);

        match board.to_play {
            PlayableTeam::White => board.gen_white_moves(&mut move_list),
            PlayableTeam::Black => board.gen_black_moves(&mut move_list),
        }

        Self { move_list }
    }
}

impl Board {
    fn gen_white_moves(&self, move_list: &mut Vec<(Position, Position, u16)>) {
        self.gen_white_pawn_moves(move_list);

        let own = self.all_pieces[PlayableTeam::White as usize];

        for kind in PieceKind::kinds() {
            let pieces = self.pieces[PlayableTeam::White as usize][kind as usize];

            self.gen_moves(pieces, own, move_list);
        }
    }

    fn gen_white_pawn_moves(&self, move_list: &mut Vec<(Position, Position, u16)>) {}
}

impl Board {
    fn gen_black_moves(&self, move_list: &mut Vec<(Position, Position, u16)>) {
        self.gen_black_pawn_moves(move_list);

        let own = self.all_pieces[PlayableTeam::Black as usize];

        for kind in PieceKind::kinds() {
            let pieces = self.pieces[PlayableTeam::Black as usize][kind as usize];

            self.gen_moves(pieces, own, move_list);
        }
    }

    fn gen_black_pawn_moves(&self, move_list: &mut Vec<(Position, Position, u16)>) {}
}

impl Board {
    fn gen_moves(&self, mut pieces: u64, own: u64, move_list: &mut Vec<(Position, Position, u16)>) {
        while pieces != 0 {
            let from = magic::pop_lsb(&mut pieces);

            let mut moves =
                self.get_attacks_for_square(Piece::WhiteRook, Position::from_u8(from as u8), own);

            while moves != 0 {
                let to = magic::pop_lsb(&mut moves);

                move_list.push((
                    Position::from_u8(from as u8),
                    Position::from_u8(to as u8),
                    0,
                ));
            }
        }
    }
    fn get_attacks_for_square(&self, piece: Piece, position: Position, blockers: u64) -> u64 {
        match piece.kind() {
            PieceKind::Pawn => todo!(),
            PieceKind::Rook => magic::rook_attacks(position, blockers),
            PieceKind::Knight => magic::knight_attacks(position, blockers),
            PieceKind::Bishop => magic::bishop_attacks(position, blockers),
            PieceKind::Queen => {
                magic::rook_attacks(position, blockers) | magic::bishop_attacks(position, blockers)
            }
            PieceKind::King => magic::king_attacks(position, blockers),
            PieceKind::None => 0,
        }
    }
}
