use crate::{
    piece::{Piece, PieceKind},
    Board, Position, Team, TeamComparison,
};

const ROOK_DIRECTIONS: [(i8, i8); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
const BISHOP_DIRECTIONS: [(i8, i8); 4] = [(-1, 1), (1, 1), (-1, -1), (1, -1)];
const KNIGHT_MOVES: [(i8, i8); 8] = [
    (-1, -2),
    (-1, 2),
    (-2, -1),
    (-2, 1),
    (1, -2),
    (1, 2),
    (2, -1),
    (2, 1),
];
const MONARCH_DIRECTIONS: [(i8, i8); 8] = [
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
    (-1, 0),
    (-1, 1),
];

macro_rules! impl_next_move {
    ($move_list:ident, $method_name:ident) => {
        fn $method_name(
            self,
            position: Position,
            board: &Board,
            moves: &mut Vec<(Position, Position, u16)>,
        ) {
            for (x, y) in $move_list {
                if let Some(move_to) = position.checked_add_to(x, y) {
                    if board[move_to].team() != self.team() {
                        moves.push((position, move_to, 0));
                    }
                }
            }
        }
    };
}

macro_rules! impl_next_direction_move {
    ($move_list:ident, $method_name:ident) => {
        fn $method_name(
            self,
            position: Position,
            board: &Board,
            moves: &mut Vec<(Position, Position, u16)>,
        ) {
            for (x, y) in $move_list {
                let mut move_to = position;
                while let Some(possible_move) = move_to.checked_add_to(x, y) {
                    move_to = possible_move;

                    match self.team().compare(&board[possible_move].team()) {
                        TeamComparison::Same => break,
                        TeamComparison::Different => {
                            moves.push((position, possible_move, 0));
                            break;
                        }
                        TeamComparison::None => moves.push((position, possible_move, 0)),
                    }
                }
            }
        }
    };
}

macro_rules! quiescence_impl_next_move {
    ($move_list:ident, $method_name:ident) => {
        fn $method_name(
            self,
            position: Position,
            board: &Board,
            moves: &mut Vec<(Position, Position, u16)>,
        ) {
            for (x, y) in $move_list {
                if let Some(move_to) = position.checked_add_to(x, y) {
                    if board[move_to].team() == !(self.team()) {
                        moves.push((position, move_to, 0));
                    }
                }
            }
        }
    };
}

macro_rules! quiescence_impl_next_direction_move {
    ($move_list:ident, $method_name:ident) => {
        fn $method_name(
            self,
            position: Position,
            board: &Board,
            moves: &mut Vec<(Position, Position, u16)>,
        ) {
            for (x, y) in $move_list {
                let mut move_to = position;

                while let Some(possible_move) = move_to.checked_add_to(x, y) {
                    move_to = possible_move;
                    match self.team().compare(&board[possible_move].team()) {
                        TeamComparison::Same => break,
                        TeamComparison::Different => {
                            moves.push((position, possible_move, 0));
                            break;
                        }
                        TeamComparison::None => {}
                    }
                }
            }
        }
    };
}

impl Board {
    /// Checks if a piece on the team of `attacking_team` attacks the
    /// square at `position`
    #[must_use]
    pub fn attacked(&self, attacking_team: Team, position: Position) -> bool {
        for (x, y) in MONARCH_DIRECTIONS {
            let mut move_to = position;

            while let Some(possible_move) = move_to.checked_add_to(x, y) {
                move_to = possible_move;

                match self[possible_move].team().compare(&attacking_team) {
                    TeamComparison::Same => break,
                    TeamComparison::Different => return true,
                    TeamComparison::None => continue,
                }
            }
        }

        for (x, y) in KNIGHT_MOVES {
            if let Some(possible_move) = position.checked_add_to(x, y) {
                if self[possible_move].kind() == PieceKind::Knight
                    && self[possible_move].team() == attacking_team
                {
                    return true;
                }
            }
        }

        false
    }
}

impl Piece {
    /// Appends all the legal moves of the piece at
    /// `position` on the board to `moves`

    pub fn legal_moves(
        &self,
        position: Position,
        board: &Board,
        moves: &mut Vec<(Position, Position, u16)>,
    ) {
        match self.kind() {
            PieceKind::None => {}
            PieceKind::Pawn => self.pawn_moves(position, board, moves),
            PieceKind::Rook => self.rook_moves(position, board, moves),
            PieceKind::Knight => self.knight_moves(position, board, moves),
            PieceKind::Bishop => self.bishop_moves(position, board, moves),
            PieceKind::Queen => self.queen_moves(position, board, moves),
            PieceKind::King => {
                self.king_moves(position, board, moves);
                self.castling(position, board, moves);
            }
        }
    }
    pub fn quiescence_moves(
        &self,
        position: Position,
        board: &Board,
        moves: &mut Vec<(Position, Position, u16)>,
    ) {
        match self.kind() {
            PieceKind::None => {}
            PieceKind::Pawn => self.quiescence_pawn_moves(position, board, moves),
            PieceKind::Rook => self.quiescence_rook_moves(position, board, moves),
            PieceKind::Knight => self.quiescence_knight_moves(position, board, moves),
            PieceKind::Bishop => self.quiescence_bishop_moves(position, board, moves),
            PieceKind::Queen => self.quiescence_queen_moves(position, board, moves),
            PieceKind::King => self.quiescence_king_moves(position, board, moves),
        }
    }

    fn pawn_moves(
        self,
        position: Position,
        board: &Board,
        moves: &mut Vec<(Position, Position, u16)>,
    ) {
        let y = if self.team() == Team::White { 1 } else { -1 };

        let move_to = position
            .checked_add_to(0, y)
            .expect("this indicated pawn promotion failed");

        if board[move_to].is_empty() {
            moves.push((position, move_to, 0));

            if !self.has_moved() {
                let double_move_to = move_to.checked_add_to(0, y).unwrap();
                if board[double_move_to].is_empty() {
                    moves.push((position, double_move_to, 0));
                }
            }
        }

        for x in [-1, 1] {
            if let Some(take_position) = move_to.checked_add_to(x, 0) {
                if board[take_position].team() == !(self.team()) {
                    moves.push((position, take_position, 0));
                }
            }
        }
    }

    fn quiescence_pawn_moves(
        self,
        position: Position,
        board: &Board,
        moves: &mut Vec<(Position, Position, u16)>,
    ) {
        let y = if self.team() == Team::White { 1 } else { -1 };

        let move_to = position
            .checked_add_to(0, y)
            .expect("this indicated pawn promotion failed");

        for x in [-1, 1] {
            if let Some(take_position) = move_to.checked_add_to(x, 0) {
                if board[take_position].team() == !(self.team()) {
                    moves.push((position, take_position, 0));
                }
            }
        }
    }
    fn castling(
        self,
        position: Position,
        board: &Board,
        moves: &mut Vec<(Position, Position, u16)>,
    ) {
        if !self.has_moved()
            && position == self.castling_king_square()
            && !board.attacked(!self.team(), position)
        {
            let bounds = [(0, (1..4)), (7, (5..7))];
            for (final_x, bound) in bounds {
                let mut valid = true;
                for x in bound {
                    if board[Position::new(x, position.y())].is_piece()
                        || board.attacked(!self.team(), position)
                    {
                        valid = false;
                        break;
                    }
                }
                if valid {
                    let castling_rook_position = Position::new(final_x, position.y());
                    let castling_rook = board[castling_rook_position];

                    if castling_rook.kind() == PieceKind::Rook && !castling_rook.has_moved() {
                        moves.push((position, castling_rook_position, 0));
                    }
                }
            }
        }
    }
    fn castling_king_square(self) -> Position {
        match self.team() {
            Team::White => Position::new(4, 0),
            Team::Black => Position::new(4, 7),
            Team::Neither => unreachable!("this should not happen"),
        }
    }

    impl_next_move!(KNIGHT_MOVES, knight_moves);
    impl_next_move!(MONARCH_DIRECTIONS, king_moves);

    impl_next_direction_move!(ROOK_DIRECTIONS, rook_moves);
    impl_next_direction_move!(BISHOP_DIRECTIONS, bishop_moves);
    impl_next_direction_move!(MONARCH_DIRECTIONS, queen_moves);

    quiescence_impl_next_move!(KNIGHT_MOVES, quiescence_knight_moves);
    quiescence_impl_next_move!(MONARCH_DIRECTIONS, quiescence_king_moves);

    quiescence_impl_next_direction_move!(ROOK_DIRECTIONS, quiescence_rook_moves);
    quiescence_impl_next_direction_move!(BISHOP_DIRECTIONS, quiescence_bishop_moves);
    quiescence_impl_next_direction_move!(MONARCH_DIRECTIONS, quiescence_queen_moves);
}
