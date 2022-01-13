use crate::Board;

impl Board {
    pub fn end_game_evaluation(&self) -> i16 {
        self.material
    }
}
