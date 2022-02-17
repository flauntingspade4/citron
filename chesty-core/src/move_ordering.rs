use crate::{killer::KillerMoves, move_gen::Move, transposition_table::TranspositionTable, Board};

const PREVIOUS_BEST_BONUS: u16 = 10_000;

const TOOK_PIECE_MULTIPLIER: u16 = 100;
const TAKING_PIECE_MULTIPLIER: u16 = 1;

pub fn move_ordering(
    board: &Board,
    ply: u8,
    moves: &mut [Move],
    (transposition_table, killer_table): (&mut TranspositionTable, &mut [KillerMoves]),
    hash: u64,
) {
    if let Some(best) = transposition_table.get(&hash) {
        for possible_move in moves.iter_mut() {
            if possible_move.from_to() == best.best_move.from_to() {
                possible_move.ordering_value += PREVIOUS_BEST_BONUS;
                break;
            }
        }
    }

    for possible_move in moves.iter_mut() {
        if let Some(take_value) = possible_move.captured_piece_kind() {
            possible_move.ordering_value += ((take_value as u16)
                .saturating_mul(TOOK_PIECE_MULTIPLIER))
                - ((possible_move.moved_piece_kind().value() as u16)
                    .saturating_mul(TAKING_PIECE_MULTIPLIER));
        } else {
            possible_move.ordering_value +=
                u16::from(killer_table[ply as usize].contains_move(possible_move.from_to())) * 250;
        }
    }

    moves.sort_unstable_by(|a, b| b.ordering_value.cmp(&a.ordering_value));
}

pub fn quiescence_move_ordering(board: &Board, moves: &mut [Move]) {
    for possible_move in moves.iter_mut() {
        if let Some(take_value) = possible_move.captured_piece_kind() {
            possible_move.ordering_value += ((take_value as u16)
                .saturating_mul(TOOK_PIECE_MULTIPLIER))
                - ((possible_move.moved_piece_kind().value() as u16)
                    .saturating_mul(TAKING_PIECE_MULTIPLIER));
        }
    }

    moves.sort_unstable_by(|a, b| b.ordering_value.cmp(&a.ordering_value));
}
