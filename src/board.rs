mod gamestate;
mod zobrist;
use self::{gamestate::Gamestate, zobrist::Zobrist};
use crate::defs::{Bitboard, Side, Sides};
use crate::defs::{Piece, Pieces};

//edit: BOARD
pub struct Borat {
    pub bitboard_pieces: [[Bitboard; 6]; 2],
    pub bitboard_sides: [Bitboard; 2],
    pub game_state: Gamestate,
    pub piece_list: [Piece; 64],
    pub zobrist: Zobrist,
}

impl Borat {
    pub fn new_empty() -> Borat {
        Borat {
            bitboard_pieces: [[Bitboard::new(0); 6]; 2],
            bitboard_sides: [Bitboard::new(0); 2],
            game_state: Gamestate::new(Sides::WHITE.try_into().unwrap()),
            piece_list: [Pieces::EMPTY; 64],
            zobrist: Zobrist::new(),
        }
    }

    pub fn get_piece(&self, piece: Piece, side: Side) -> Bitboard {
        self.bitboard_pieces[side][piece]
    }

    pub fn get_side(&self, side: Side) -> Bitboard {
        self.bitboard_sides[side]
    }

    //   pub fn new_from_fen(fen: String) -> Borat {
    //       let white_bb: Bitboard;
    //       let black_bb: Bitboard;
    //       let
    //   }
}
