use crate::defs::{Piece, Side};
use rand::random;

pub const EMPTY: u64 = 0;

pub struct ZobristRandomsHolder {
    pub zobrist_pieces: [[[u64; 64]; 7]; 2],
    pub zobrist_castling: [[u64; 2]; 2],
    pub zobrist_is_black_move: u64,
    pub zobrist_full_move_number: u64,
    pub zobrist_enpassant: [u64; 64],
}

impl ZobristRandomsHolder {
    pub fn new() -> ZobristRandomsHolder {
        let mut new = ZobristRandomsHolder {
            zobrist_pieces: [[[EMPTY; 64]; 7]; 2],
            zobrist_castling: [[EMPTY; 2]; 2],
            zobrist_is_black_move: EMPTY,
            zobrist_full_move_number: EMPTY,
            zobrist_enpassant: [EMPTY; 64],
        };

        new.zobrist_pieces.iter_mut().for_each(|side| {
            side.iter_mut().for_each(|piece| {
                piece.iter_mut().for_each(|square| {
                    *square = random();
                });
            })
        });

        new.zobrist_castling.iter_mut().for_each(|piece| {
            piece.iter_mut().for_each(|square| {
                *square = random();
            })
        });

        new.zobrist_is_black_move = random();

        new.zobrist_full_move_number = random();

        new.zobrist_enpassant.iter_mut().for_each(|square| {
            *square = random();
        });

        //println!(
        //    "{:?} {:?} {} {} {:?}",
        //    new.zobrist_pieces
        //        .iter()
        //        .map(|matrix| matrix
        //            .iter()
        //            .map(|row| row
        //                .iter()
        //                .map(|n| format!("{:064b}", n))
        //                .collect::<Vec<_>>())
        //            .collect::<Vec<_>>())
        //        .collect::<Vec<_>>(),
        //    new.zobrist_castling,
        //    new.zobrist_is_black_move,
        //    new.zobrist_full_move_number,
        //    new.zobrist_enpassant,
        //);
        new
    }

    pub fn find_piece(&self, piece: Piece, side: Side, square: usize) -> u64 {
        self.zobrist_pieces[side][piece][square]
    }
}
