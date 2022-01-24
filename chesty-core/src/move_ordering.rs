use crate::{killer::KillerMoves, transposition_table::TranspositionTable, Board, Position};

const PREVIOUS_BEST_BONUS: u16 = 10_000;

const TOOK_PIECE_MULTIPLIER: u16 = 100;
const TAKING_PIECE_MULTIPLIER: u16 = 1;

pub fn move_ordering(
    board: &Board,
    ply: u8,
    moves: &mut [(Position, Position, u16)],
    (transposition_table, killer_table): (&mut TranspositionTable, &mut [KillerMoves]),
    hash: u64,
) {
    if let Some(best) = transposition_table.get(&hash) {
        for possible_move in moves.iter_mut() {
            if (possible_move.0, possible_move.1) == best.best_move {
                possible_move.2 += PREVIOUS_BEST_BONUS;
                break;
            }
        }
    }

    for possible_move in moves.iter_mut() {
        let take_value = board[possible_move.1].piece_value();
        if take_value == 0 {
            possible_move.2 += u16::from(
                killer_table[ply as usize].contains_move(possible_move.0, possible_move.1),
            ) * 250;
        } else {
            possible_move.2 += ((take_value as u16).saturating_mul(TOOK_PIECE_MULTIPLIER))
                - ((board[possible_move.0].piece_value() as u16)
                    .saturating_mul(TAKING_PIECE_MULTIPLIER));
        }
    }

    moves.sort_unstable_by(|a, b| b.2.cmp(&a.2));
}

pub fn quiescence_move_ordering(board: &Board, moves: &mut [(Position, Position, u16)]) {
    for possible_move in moves.iter_mut() {
        let take_value = board[possible_move.1].piece_value();
        if take_value != 0 {
            possible_move.2 += ((take_value as u16).saturating_mul(TOOK_PIECE_MULTIPLIER))
                - ((board[possible_move.0].piece_value() as u16)
                    .saturating_mul(TAKING_PIECE_MULTIPLIER));
        }
    }

    moves.sort_unstable_by(|a, b| b.2.cmp(&a.2));
}
