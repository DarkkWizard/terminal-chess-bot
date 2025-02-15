use crate::defs::{Bitboard};

#[derive(Debug)]
pub struct MoveGen {
    pub king_attacks: [Bitboard; 64],
    pub queen_attacks: [Bitboard; 64],
    pub bishop_attacks: [Bitboard; 64],
    pub rook_attacks: [Bitboard; 64],
    pub pawn_attacks: [Bitboard; 64],
    pub knight_attacks: [Bitboard; 64],
}

impl MoveGen {
    pub fn new_empty() -> MoveGen {
        MoveGen{
            king_attacks: [0; 64],
            queen_attacks: [0; 64],
            bishop_attacks: [0; 64],
            rook_attacks: [0; 64],
            pawn_attacks: [0; 64],
            knight_attacks: [0; 64],
        }
    }

    pub fn init() -> MoveGen {
        let mut generator = MoveGen::new_empty();
        generator.king_attacks = Self::king_init();
        generator.queen_attacks = Self::queen_init();
        generator.bishop_attacks = Self::bishop_init();
        generator.rook_attacks = Self::rook_init();
        generator.pawn_attacks = Self::pawn_init();
        generator.knight_attacks = Self::knight_init();
        generator
    }

    pub fn king_init() -> [Bitboard; 64] {
        let mut ret_list: [Bitboard; 64] = [0; 64];
        for (square, bb_ref) in ret_list.iter_mut().enumerate() {
            let mut bb: Bitboard = 0; 
            let rank = square / 8;
            let file = square % 8;

            for r in (rank.max(1) - 1)..=(rank.min(6) + 1) {
                for f in (file.max(1) - 1)..=(file.min(6) + 1) {
                    if r == rank && f == file {
                        continue;
                    }
                    let attack_square = r * 8 + f;
                    bb |= 1 << attack_square;
                }
            }
            *bb_ref= bb;
        }
        ret_list
    }

    
    pub fn queen_init() -> [Bitboard; 64] {
        [0; 64]
    }

    pub fn bishop_init() -> [Bitboard; 64] {
        [0; 64]
    }

    pub fn rook_init() -> [Bitboard; 64] {
        [0; 64]
    }

    pub fn pawn_init() -> [Bitboard; 64] {
        [0; 64]
    }

    pub fn knight_init() -> [Bitboard; 64] {
        [0; 64]
    }
}

impl MoveGen {
}
