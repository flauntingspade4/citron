use crate::{
    magic::{self, pop_lsb, MASK_FILE, MASK_RANK},
    piece::PieceKind,
    Board, PlayableTeam, Position,
};

use std::fmt::Display;

pub const BRANCHING_FACTOR: usize = 35;

/// A move, containing information about where the piece moved from and
/// to, the piece's kind, and the captured piece's kind
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Move {
    from: Position,
    to: Position,
    ordering_value: u16,
    moved_piece_kind: PieceKind,
    captured_piece_kind: PieceKind,
    flags: MoveFlags,
}

impl Move {
    /// Create a new move, based off a given pair of positions, the moved
    /// piece, and the captured piece
    #[must_use]
    pub const fn new(
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
            flags: MoveFlags(0),
        }
    }
    /// Returns the position the move is from
    #[must_use]
    pub const fn from(&self) -> Position {
        self.from
    }
    /// Returns the position the move is to
    #[must_use]
    pub const fn to(&self) -> Position {
        self.to
    }
    /// Returns the position the move is from, and the move is to
    #[must_use]
    pub const fn from_to(&self) -> (Position, Position) {
        (self.from, self.to)
    }
    /// Returns the kind of piece that was moved
    #[must_use]
    pub const fn moved_piece_kind(&self) -> PieceKind {
        self.moved_piece_kind
    }
    /// Returns the kind of piece that was captured
    #[must_use]
    pub const fn captured_piece_kind(&self) -> PieceKind {
        self.captured_piece_kind
    }
    pub(crate) const fn ordering_value(&self) -> &u16 {
        &self.ordering_value
    }
    pub(crate) fn ordering_value_mut(&mut self) -> &mut u16 {
        &mut self.ordering_value
    }
    pub(crate) fn flags_mut(&mut self) -> &mut MoveFlags {
        &mut self.flags
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (from_x, from_y) = self.from.to_uci();
        let (to_x, to_y) = self.to.to_uci();
        write!(f, "({from_x}{from_y}) ({to_x}{to_y})")
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct MoveFlags(u8);

impl MoveFlags {
    pub const fn is_promotion(&self) -> bool {
        self.0 & 1 == 1
    }
    pub fn set_promotion(&mut self, promotion: bool) {
        self.0 |= u8::from(promotion)
    }
}

pub struct MoveGen {
    move_list: Vec<Move>,
}

impl MoveGen {
    /// Generate all the possible moves for a given board, for
    /// the side to play
    #[must_use]
    pub fn new(board: &Board) -> Self {
        let mut move_list = Vec::with_capacity(BRANCHING_FACTOR);

        match board.to_play {
            PlayableTeam::White => board.gen_white_moves(&mut move_list),
            PlayableTeam::Black => board.gen_black_moves(&mut move_list),
        }

        Self { move_list }
    }
    #[must_use]
    pub fn into_inner(self) -> Vec<Move> {
        self.move_list
    }
}

#[test]
fn original_position_moves() {
    let board = Board::new();

    let moves = MoveGen::new(&board).into_inner();

    // There should always be 20 legal moves in the starting position
    assert_eq!(20, moves.len());
}

#[test]
fn opening_position_moves() {
    let board = Board::from_fen("r1bqkbnr/pppp1ppp/2n5/4p3/4P3/5N2/PPPP1PPP/RNBQKB1R w KQkq - 2 3")
        .unwrap();

    let moves = MoveGen::new(&board).into_inner();

    // In this position, there are 26 legal moves
    assert_eq!(26, moves.len());
}

#[test]
fn middle_game_moves() {
    let board = Board::from_fen("r1b5/ppk3pp/2p5/8/4Nr2/4Rn2/PPP4P/1K3B1R b - - 7 26").unwrap();

    let moves = MoveGen::new(&board).into_inner();

    assert_eq!(34, moves.len());

    let board =
        Board::from_fen("r2q1rk1/1p3p1p/1b4p1/pPp5/3p2b1/P7/B1PQNn1P/R1B2RK1 w - - 0 32").unwrap();

    let moves = MoveGen::new(&board).into_inner();

    assert_eq!(34, moves.len());
}

impl Board {
    fn gen_white_moves(&self, move_list: &mut Vec<Move>) {
        self.gen_white_pawn_moves(move_list);

        let blockers = self.get_occupied();

        for kind in PieceKind::kinds_no_pawn() {
            let pieces = self.pieces[PlayableTeam::White as usize][kind as usize];

            self.gen_moves(PlayableTeam::White, kind, pieces, blockers, move_list);
        }
    }

    fn gen_white_pawn_moves(&self, move_list: &mut Vec<Move>) {
        self.gen_white_single_pawn_moves(move_list);
        self.gen_white_double_pawn_moves(move_list);
        self.gen_white_pawn_left(move_list);
        self.gen_white_pawn_right(move_list);
    }

    fn gen_white_single_pawn_moves(&self, move_list: &mut Vec<Move>) {
        let mut moved_pawns =
            self.pieces[PlayableTeam::White as usize][PieceKind::Pawn as usize] << 8;
        moved_pawns &= self.get_not_occupied();

        let mut promotions = moved_pawns & MASK_RANK[7];
        moved_pawns &= !MASK_RANK[7];

        while moved_pawns != 0 {
            let to = pop_lsb(&mut moved_pawns);
            move_list.push(Move::new(
                Position::from_u8((to - 8) as u8),
                Position::from_u8(to as u8),
                PieceKind::Pawn,
                PieceKind::None,
            ));
        }

        while promotions != 0 {
            let to = pop_lsb(&mut promotions);

            let mut new_move = Move::new(
                Position::from_u8((to - 8) as u8),
                Position::from_u8(to as u8),
                PieceKind::Pawn,
                PieceKind::None,
            );
            new_move.flags_mut().set_promotion(true);

            move_list.push(new_move);
        }
    }

    fn gen_white_double_pawn_moves(&self, move_list: &mut Vec<Move>) {
        let single_pushes = (self.pieces[PlayableTeam::White as usize][PieceKind::Pawn as usize]
            << 8)
            & self.get_not_occupied();
        let mut double_pushes = (single_pushes << 8) & self.get_not_occupied() & MASK_RANK[3];

        while double_pushes != 0 {
            let to = pop_lsb(&mut double_pushes);
            move_list.push(Move::new(
                Position::from_u8((to - 16) as u8),
                Position::from_u8(to as u8),
                PieceKind::Pawn,
                PieceKind::None,
            ));
        }
    }

    fn gen_white_pawn_left(&self, move_list: &mut Vec<Move>) {
        let mut left_attacks =
            (self.pieces[PlayableTeam::White as usize][PieceKind::Pawn as usize] << 7)
                & self.all_pieces[PlayableTeam::Black as usize]
                & !MASK_FILE[7];

        let mut left_promotion_attacks = left_attacks & MASK_RANK[7];
        left_attacks &= !MASK_RANK[7];

        while left_attacks != 0 {
            let to = pop_lsb(&mut left_attacks);

            move_list.push(Move::new(
                Position::from_u8((to - 7) as u8),
                Position::from_u8(to as u8),
                PieceKind::Pawn,
                self.kind_at(PlayableTeam::Black, Position::from_u8(to as u8)),
            ));
        }

        while left_promotion_attacks != 0 {
            let to = pop_lsb(&mut left_promotion_attacks);

            let mut new_move = Move::new(
                Position::from_u8((to - 7) as u8),
                Position::from_u8(to as u8),
                PieceKind::Pawn,
                self.kind_at(PlayableTeam::Black, Position::from_u8(to as u8)),
            );
            new_move.flags_mut().set_promotion(true);

            move_list.push(new_move);
        }
    }

    fn gen_white_pawn_right(&self, move_list: &mut Vec<Move>) {
        let mut right_attacks =
            (self.pieces[PlayableTeam::White as usize][PieceKind::Pawn as usize] << 9)
                & self.all_pieces[PlayableTeam::Black as usize]
                & !MASK_FILE[0];

        let mut right_promotion_attacks = right_attacks & MASK_RANK[7];
        right_attacks &= !MASK_RANK[7];

        while right_attacks != 0 {
            let to = pop_lsb(&mut right_attacks);

            move_list.push(Move::new(
                Position::from_u8((to - 9) as u8),
                Position::from_u8(to as u8),
                PieceKind::Pawn,
                self.kind_at(PlayableTeam::Black, Position::from_u8(to as u8)),
            ));
        }

        while right_promotion_attacks != 0 {
            let to = pop_lsb(&mut right_promotion_attacks);

            let mut new_move = Move::new(
                Position::from_u8((to - 9) as u8),
                Position::from_u8(to as u8),
                PieceKind::Pawn,
                self.kind_at(PlayableTeam::Black, Position::from_u8(to as u8)),
            );
            new_move.flags_mut().set_promotion(true);

            move_list.push(new_move);
        }
    }

    fn gen_black_moves(&self, move_list: &mut Vec<Move>) {
        self.gen_black_pawn_moves(move_list);

        let blockers = self.get_occupied();

        for kind in PieceKind::kinds_no_pawn() {
            let pieces = self.pieces[PlayableTeam::Black as usize][kind as usize];

            self.gen_moves(PlayableTeam::Black, kind, pieces, blockers, move_list);
        }
    }

    fn gen_black_pawn_moves(&self, move_list: &mut Vec<Move>) {
        self.gen_black_single_pawn_moves(move_list);
        self.gen_black_double_pawn_moves(move_list);
        self.gen_black_pawn_left(move_list);
        self.gen_black_pawn_right(move_list);
    }

    fn gen_black_single_pawn_moves(&self, move_list: &mut Vec<Move>) {
        let mut moved_pawns =
            self.pieces[PlayableTeam::Black as usize][PieceKind::Pawn as usize] >> 8;
        moved_pawns &= self.get_not_occupied();

        let mut promotions = moved_pawns & MASK_RANK[0];
        moved_pawns &= !MASK_RANK[0];

        while moved_pawns != 0 {
            let to = pop_lsb(&mut moved_pawns);
            move_list.push(Move::new(
                Position::from_u8((to + 8) as u8),
                Position::from_u8(to as u8),
                PieceKind::Pawn,
                PieceKind::None,
            ));
        }

        while promotions != 0 {
            let to = pop_lsb(&mut promotions);

            let mut new_move = Move::new(
                Position::from_u8((to + 8) as u8),
                Position::from_u8(to as u8),
                PieceKind::Pawn,
                PieceKind::None,
            );
            new_move.flags_mut().set_promotion(true);

            move_list.push(new_move);
        }
    }

    fn gen_black_double_pawn_moves(&self, move_list: &mut Vec<Move>) {
        let single_pushes = (self.pieces[PlayableTeam::Black as usize][PieceKind::Pawn as usize]
            >> 8)
            & self.get_not_occupied();
        let mut double_pushes = (single_pushes >> 8) & self.get_not_occupied() & MASK_RANK[4];

        while double_pushes != 0 {
            let to = pop_lsb(&mut double_pushes);
            move_list.push(Move::new(
                Position::from_u8((to + 16) as u8),
                Position::from_u8(to as u8),
                PieceKind::Pawn,
                PieceKind::None,
            ));
        }
    }

    fn gen_black_pawn_left(&self, move_list: &mut Vec<Move>) {
        let mut left_attacks =
            (self.pieces[PlayableTeam::Black as usize][PieceKind::Pawn as usize] >> 7)
                & self.all_pieces[PlayableTeam::White as usize]
                & !MASK_FILE[0];

        let mut left_promotion_attacks = left_attacks & MASK_RANK[0];
        left_attacks &= !MASK_RANK[0];

        while left_attacks != 0 {
            let to = pop_lsb(&mut left_attacks);

            move_list.push(Move::new(
                Position::from_u8((to + 7) as u8),
                Position::from_u8(to as u8),
                PieceKind::Pawn,
                self.kind_at(PlayableTeam::White, Position::from_u8(to as u8)),
            ));
        }

        while left_promotion_attacks != 0 {
            let to = pop_lsb(&mut left_promotion_attacks);

            let mut new_move = Move::new(
                Position::from_u8((to + 7) as u8),
                Position::from_u8(to as u8),
                PieceKind::Pawn,
                self.kind_at(PlayableTeam::White, Position::from_u8(to as u8)),
            );
            new_move.flags_mut().set_promotion(true);

            move_list.push(new_move);
        }
    }

    fn gen_black_pawn_right(&self, move_list: &mut Vec<Move>) {
        let mut right_attacks =
            (self.pieces[PlayableTeam::Black as usize][PieceKind::Pawn as usize] >> 9)
                & self.all_pieces[PlayableTeam::White as usize]
                & !MASK_FILE[0];

        let mut right_promotion_attacks = right_attacks & MASK_RANK[0];
        right_attacks &= !MASK_RANK[0];

        while right_attacks != 0 {
            let to = pop_lsb(&mut right_attacks);

            move_list.push(Move::new(
                Position::from_u8((to + 9) as u8),
                Position::from_u8(to as u8),
                PieceKind::Pawn,
                self.kind_at(PlayableTeam::White, Position::from_u8(to as u8)),
            ));
        }

        while right_promotion_attacks != 0 {
            let to = pop_lsb(&mut right_promotion_attacks);

            let mut new_move = Move::new(
                Position::from_u8((to + 9) as u8),
                Position::from_u8(to as u8),
                PieceKind::Pawn,
                self.kind_at(PlayableTeam::White, Position::from_u8(to as u8)),
            );
            new_move.flags_mut().set_promotion(true);

            move_list.push(new_move);
        }
    }
}

impl Board {
    fn gen_moves(
        &self,
        team: PlayableTeam,
        kind: PieceKind,
        mut pieces: u64,
        blockers: u64,
        move_list: &mut Vec<Move>,
    ) {
        while pieces != 0 {
            let from = Position::from_u8(magic::pop_lsb(&mut pieces) as u8);

            let mut moves = Self::get_attacks_for_square(kind, from, blockers);

            while moves != 0 {
                let to = Position::from_u8(magic::pop_lsb(&mut moves) as u8);

                if self.team_at(to) != self.to_play.into() {
                    move_list.push(Move::new(from, to, kind, self.kind_at(!team, to)));
                }
            }
        }
    }
    fn get_attacks_for_square(kind: PieceKind, position: Position, blockers: u64) -> u64 {
        match kind {
            PieceKind::Pawn => todo!(),
            PieceKind::Rook => magic::rook_attacks(position, blockers),
            PieceKind::Knight => magic::knight_attacks(position),
            PieceKind::Bishop => magic::bishop_attacks(position, blockers),
            PieceKind::Queen => {
                magic::rook_attacks(position, blockers) | magic::bishop_attacks(position, blockers)
            }
            PieceKind::King => magic::king_attacks(position),
            PieceKind::None => 0,
        }
    }
}
