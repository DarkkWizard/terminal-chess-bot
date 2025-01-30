mod board;
mod gamestate;
use self::board::{Bitboard, Pieces};
use self::gamestate::Gamestate;
use crate::defs::Side;

//edit: BOARD
pub struct Borat {
    pub bitboard_pieces: [[Bitboard; 6]; 2],
    pub bitboard_sides: [Bitboard; 2],
    pub game_state: Gamestate,
}

impl Borat {
    pub fn new_empty() -> Borat {
        Borat {
            bitboard_pieces: [[Bitboard::new(0); 6]; 2],
            bitboard_sides: [Bitboard::new(0); 2],
            game_state: Gamestate::new(Side::WHITE.try_into().unwrap()),
        }
    }
}
