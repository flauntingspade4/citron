use crate::{killer::KillerMoves, move_gen::Move, transposition_table::TranspositionTable};

const PREVIOUS_BEST_BONUS: u32 = 10_000;

const TOOK_PIECE_MULTIPLIER: u32 = 10;
const TAKING_PIECE_MULTIPLIER: u32 = 1;

pub fn move_ordering(
    ply: u8,
    moves: &mut [Move],
    (transposition_table, killer_table): (&mut TranspositionTable, &mut [KillerMoves]),
    hash: u64,
) {
    if let Some(best) = transposition_table.get(&hash) {
        for possible_move in moves.iter_mut() {
            if possible_move.from_to() == best.best_move.from_to() {
                *possible_move.ordering_value_mut() += PREVIOUS_BEST_BONUS;
                break;
            }
        }
    }

    for possible_move in moves.iter_mut() {
        let take_value = possible_move.captured_piece_kind().value();

        if take_value == 0 {
            *possible_move.ordering_value_mut() +=
                u32::from(killer_table[ply as usize].contains_move(possible_move.from_to())) * 250;
        } else {
            *possible_move.ordering_value_mut() +=
                ((take_value as u32).saturating_mul(TOOK_PIECE_MULTIPLIER)).saturating_sub(
                    (possible_move.moved_piece_kind().value() as u32)
                        .saturating_mul(TAKING_PIECE_MULTIPLIER),
                );
        }
    }

    moves.sort_unstable_by(|a, b| b.ordering_value().cmp(a.ordering_value()));
}

pub fn quiescence_move_ordering(moves: &mut [Move]) {
    for possible_move in moves.iter_mut() {
        let take_value = possible_move.captured_piece_kind().value();

        if take_value != 0 {
            *possible_move.ordering_value_mut() +=
                ((take_value as u32).saturating_mul(TOOK_PIECE_MULTIPLIER)).saturating_sub(
                    (possible_move.moved_piece_kind().value() as u32)
                        .saturating_mul(TAKING_PIECE_MULTIPLIER),
                );
        }
    }

    moves.sort_unstable_by(|a, b| b.ordering_value().cmp(a.ordering_value()));
}
