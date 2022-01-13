use crate::{killer::KillerMoves, transposition_table::TranspositionTable, Board, Position};

const PREVIOUS_BEST_BONUS: u16 = 10_000;

const TOOK_PIECE_MULTIPLIER: u16 = 100;
const TAKING_PIECE_MULTIPLIER: u16 = 1;

pub fn move_ordering(
    board: &Board,
    ply: u8,
    moves: &mut [(Position, Position, u16)],
    transposition_table: &TranspositionTable,
    killer_table: &[KillerMoves],
    hash: u64,
) {
    if let Some(best) = transposition_table.get(&hash) {
        for possible_move in moves.iter_mut() {
            if (possible_move.0, possible_move.1) == best.value().best_move {
                possible_move.2 += PREVIOUS_BEST_BONUS;
                break;
            }
        }
    }

    for possible_move in moves.iter_mut() {
        let take_value = board[possible_move.1].piece_value();
        if take_value != 0 {
            possible_move.2 += ((take_value as u16).saturating_mul(TOOK_PIECE_MULTIPLIER))
                - ((board[possible_move.0].piece_value() as u16)
                    .saturating_mul(TAKING_PIECE_MULTIPLIER));
        } /*else if let Some(table) = killer_table.get(&ply) {
              possible_move.2 += match board.to_play() {
                  crate::PlayableTeam::White => {
                      table.contains_white_move(possible_move.0, possible_move.1) as u16 * 10
                  }
                  crate::PlayableTeam::Black => {
                      table.contains_black_move(possible_move.0, possible_move.1) as u16 * 10
                  }
              }
          }*/
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
