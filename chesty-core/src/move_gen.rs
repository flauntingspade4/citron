use crate::{magic, piece::PieceKind, Board, PlayableTeam, Position};

pub const BRANCHING_FACTOR: usize = 35;

#[derive(Debug)]
pub struct Move {
    from: Position,
    to: Position,
    pub ordering_value: u16,
    moved_piece_kind: PieceKind,
    captured_piece_kind: PieceKind,
}

impl Move {
    pub fn new(
        from: Position,
        to: Position,
        moved_piece_kind: PieceKind,
        captured_piece_kind: PieceKind,
    ) -> Self {
        Self {
            from,
            to,
            ordering_value: 0,
            moved_piece_kind,
            captured_piece_kind,
        }
    }
    pub fn from(&self) -> Position {
        self.from
    }
    pub fn to(&self) -> Position {
        self.to
    }
    pub fn from_to(&self) -> (Position, Position) {
        (self.from, self.to)
    }
    pub fn moved_piece_kind(&self) -> PieceKind {
        self.moved_piece_kind
    }
    pub fn captured_piece_kind(&self) -> PieceKind {
        self.captured_piece_kind
    }
}

pub struct MoveGen {
    move_list: Vec<Move>,
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
    pub fn into_inner(self) -> Vec<Move> {
        self.move_list
    }
}

impl Board {
    fn gen_white_moves(&self, move_list: &mut Vec<Move>) {
        self.gen_white_pawn_moves(move_list);

        let own = self.all_pieces[PlayableTeam::White as usize];

        for kind in PieceKind::kinds() {
            let pieces = self.pieces[PlayableTeam::White as usize][kind as usize];

            self.gen_moves(pieces, own, move_list, kind);
        }
    }

    fn gen_white_pawn_moves(&self, move_list: &mut Vec<Move>) {}
}

impl Board {
    fn gen_black_moves(&self, move_list: &mut Vec<Move>) {
        self.gen_black_pawn_moves(move_list);

        let own = self.all_pieces[PlayableTeam::Black as usize];

        for kind in PieceKind::kinds() {
            let pieces = self.pieces[PlayableTeam::Black as usize][kind as usize];

            self.gen_moves(pieces, own, move_list, kind);
        }
    }

    fn gen_black_pawn_moves(&self, move_list: &mut Vec<Move>) {}
}

impl Board {
    fn gen_moves(&self, mut pieces: u64, own: u64, move_list: &mut Vec<Move>, kind: PieceKind) {
        while pieces != 0 {
            let from = magic::pop_lsb(&mut pieces);

            let mut moves = self.get_attacks_for_square(kind, Position::from_u8(from as u8), own);

            while moves != 0 {
                let to = magic::pop_lsb(&mut moves);

                move_list.push(Move::new(
                    Position::from_u8(from as u8),
                    Position::from_u8(to as u8),
                    kind,
                    todo!("captured piece kind"),
                ));
            }
        }
    }
    fn get_attacks_for_square(&self, kind: PieceKind, position: Position, blockers: u64) -> u64 {
        match kind {
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
