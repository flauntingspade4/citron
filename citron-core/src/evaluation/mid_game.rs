use crate::{
    magic::{self, pop_lsb},
    piece::{Piece, PieceKind},
    Board, PlayableTeam, Position, Team,
};

impl Board {
    #[must_use]
    pub fn middle_game_evaluation(&self) -> i16 {
        10 * (self.positions_pieces().fold(0, |moves, (position, p)| {
            let blockers = self.all_pieces[0] | self.all_pieces[1];

            let mobility: i16 = match p.kind() {
                PieceKind::Rook => magic::rook_attacks(position, blockers),
                PieceKind::Knight => magic::knight_attacks(position),
                PieceKind::Bishop => magic::bishop_attacks(position, blockers),
                PieceKind::Queen => {
                    magic::rook_attacks(position, blockers)
                        | magic::bishop_attacks(position, blockers)
                }
                PieceKind::King => magic::king_attacks(position),
                _ => 0,
            }
            .count_ones()
            .try_into()
            .unwrap();

            if p.team() == Team::Black {
                moves - mobility
            } else {
                moves + mobility
            }
        }) >> 1)
        // - self[self.king_positions.0].virtual_mobility(self.king_positions.0, self) * 7
        // + self[self.king_positions.1].virtual_mobility(self.king_positions.1, self) * 7
    }

    fn positions_from_map(mut piece_map: u64) -> impl Iterator<Item = Position> {
        std::iter::from_fn(move || {
            (piece_map != 0).then(|| Position::from_bitmap(pop_lsb(&mut piece_map)))
        })
    }
    pub fn positions_pieces(&self) -> impl Iterator<Item = (Position, Piece)> + use<'_> {
        self.pieces
            .iter()
            .zip(PlayableTeam::teams())
            .map(|(pieces, team)| {
                pieces
                    .iter()
                    .zip(PieceKind::kinds())
                    .map(move |(&piece_map, kind)| {
                        let temp_map = piece_map;

                        Self::positions_from_map(temp_map)
                            .map(move |pos| (pos, Piece::new(team, kind)))
                    })
                    .flatten()
            })
            .flatten()
    }
}
