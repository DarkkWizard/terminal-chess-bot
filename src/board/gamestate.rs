use super::zobrist::ZobristRandomsHolder;
use crate::Sides;
pub struct Gamestate {
    pub side_to_play: usize,
    pub curr_zobrist_key: u64,
}
impl Gamestate {
    pub fn new() -> Gamestate {
        Gamestate {
            side_to_play: Sides::WHITE,
            curr_zobrist_key: 0,
        }
    }

    pub fn get_side_to_play(&self) -> usize {
        self.side_to_play
    }
}
