mod gamestate;
mod zobrist;

use self::{gamestate::Gamestate, zobrist::ZobristRandomsHolder};
use crate::defs::{next, Bitboard, Piece, Pieces, Side, Sides, Square};

//edit: BOARD
#[derive(Debug)]
pub struct Borat {
    pub bitboard_pieces: [[Bitboard; 7]; 3],
    pub bitboard_sides: [Bitboard; 3],
    pub game_state: Gamestate,
    pub piece_list: [(Piece, Side); 64],
    pub zobrist_randoms_holder: ZobristRandomsHolder,
}

impl Borat {
    pub fn new_empty() -> Borat {
        let mut board = Borat {
            bitboard_pieces: [[0; 7]; 3],
            bitboard_sides: [0; 3],
            game_state: Gamestate::new(),
            piece_list: [(Pieces::EMPTY, Sides::BOTH); 64],
            zobrist_randoms_holder: ZobristRandomsHolder::new(),
        };

        board.init();
        board
    }

    pub fn get_piece_board(&self, piece: Piece, side: Side) -> Bitboard {
        self.bitboard_pieces[side][piece]
    }

    pub fn get_side_board(&self, side: Side) -> Bitboard {
        self.bitboard_sides[side]
    }

    pub fn init(&mut self) {
        self.init_piece_bitboards_starting_pos();
        self.bitboard_sides = self.init_side_bitboards();
        self.game_state.curr_zobrist_key = self.init_zobrist_hash();
        self.piece_list = self.init_piece_list();
    }

    pub fn move_piece(
        &mut self,
        starting_square: Square,
        destination_square: Square,
        side: Side,
        piece: Piece,
    ) {
        self.remove_piece(starting_square, side, piece);
        self.place_piece(destination_square, side, piece);

        //change the piece list too
        self.update_piece_list(starting_square, destination_square, side, piece);

        //other things that i need to do here, cleanup and change which side is to move
        self.game_state.side_to_play ^= 1;
    }
}

impl Borat {
    fn remove_piece(&mut self, square: Square, side: Side, piece: Piece) {
        // need to take it out of the pieces bitboard, the sides bitboard, and the zobrist state
        let pow: u64 = 1;
        let mask = pow << square;

        //pieces bitboard
        self.bitboard_pieces[side][piece] ^= mask;

        // sides bitboard
        self.bitboard_sides[side] ^= mask;

        // zobrist state
        self.game_state.curr_zobrist_key ^=
            self.zobrist_randoms_holder.find_piece(piece, side, square);
    }

    fn place_piece(&mut self, square: Square, side: usize, piece: Piece) {
        // need to place it on the piece bitboard, the side bitboard, and the zobrist state
        let pow: u64 = 1;
        let mask = pow << square;

        // pieces bitboard
        self.bitboard_pieces[side][piece] |= mask;

        // sides bitboard
        self.bitboard_sides[side] |= mask;

        // zobrist state
        self.game_state.curr_zobrist_key ^=
            self.zobrist_randoms_holder.find_piece(piece, side, square);
    }

    fn update_piece_list(&mut self, start: Square, end: Square, piece: Piece, side: Square) {
        self.piece_list[end] = (piece, side);
        self.piece_list[start] = (Pieces::EMPTY, Sides::BOTH);
    }

    fn init_side_bitboards(&self) -> [Bitboard; 3] {
        let mut white_bb: Bitboard = 0;
        let mut black_bb: Bitboard = 0;
        let both_bb: Bitboard = self.bitboard_pieces[Sides::BOTH][Pieces::EMPTY];

        for (bb_white, bb_black) in self.bitboard_pieces[Sides::WHITE]
            .iter()
            .zip(self.bitboard_pieces[Sides::BLACK].iter())
        {
            white_bb |= *bb_white;
            black_bb |= *bb_black;
        }

        [white_bb, black_bb, both_bb]
    }

    fn init_zobrist_hash(&mut self) -> u64 {
        let mut key: u64 = 0;

        let white_pieces = self.bitboard_pieces[Sides::WHITE];
        let black_pieces = self.bitboard_pieces[Sides::BLACK];

        for (piece, (w, b)) in white_pieces.iter().zip(black_pieces.iter()).enumerate() {
            let mut num_white_pieces = *w;
            let mut num_black_pieces = *b;

            while num_white_pieces > 0 {
                let pos = next(&mut num_white_pieces);
                key ^= self
                    .zobrist_randoms_holder
                    .find_piece(piece, Sides::WHITE, pos);
            }
            while num_black_pieces > 0 {
                let pos = next(&mut num_black_pieces);
                key ^= self
                    .zobrist_randoms_holder
                    .find_piece(piece, Sides::BLACK, pos);
            }
        }
        key
    }

    fn init_piece_list(&self) -> [(Piece, Side); 64] {
        let mut piece_list: [Piece; 64] = [Pieces::EMPTY; 64];
        let mut side_list: [Side; 64] = [0; 64];

        for side in 0..3 {
            for piece in 0..7 {
                let mut bitboard = self.bitboard_pieces[side][piece];
                while bitboard != 0 {
                    let square = next(&mut bitboard);
                    piece_list[square] = piece;
                }
            }
        }
        for side in 0..3 {
            let mut bitboard = self.bitboard_sides[side];
            while bitboard != 0 {
                let square = next(&mut bitboard);
                side_list[square] = side;
            }
        }
        std::array::from_fn(|i| (piece_list[i], side_list[i]))
    }

    fn init_piece_bitboards_starting_pos(&mut self) {
        // TODO fix this and make a FEN reader
        // white
        self.bitboard_pieces[Sides::BLACK][Pieces::ROOK] |=
            0b10000001_00000000_00000000_00000000_00000000_00000000_00000000_00000000;
        self.bitboard_pieces[Sides::BLACK][Pieces::KNIGHT] |=
            0b01000010_00000000_00000000_00000000_00000000_00000000_00000000_00000000;
        self.bitboard_pieces[Sides::BLACK][Pieces::BISHOP] |=
            0b00100100_00000000_00000000_00000000_00000000_00000000_00000000_00000000;
        self.bitboard_pieces[Sides::BLACK][Pieces::QUEEN] |=
            0b00010000_00000000_00000000_00000000_00000000_00000000_00000000_00000000;
        self.bitboard_pieces[Sides::BLACK][Pieces::KING] |=
            0b00001000_00000000_00000000_00000000_00000000_00000000_00000000_00000000;
        self.bitboard_pieces[Sides::BLACK][Pieces::PAWN] |=
            0b00000000_11111111_00000000_00000000_00000000_00000000_00000000_00000000;
        // black
        self.bitboard_pieces[Sides::WHITE][Pieces::ROOK] |=
            0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_10000001;
        self.bitboard_pieces[Sides::WHITE][Pieces::KNIGHT] |=
            0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_01000010;
        self.bitboard_pieces[Sides::WHITE][Pieces::BISHOP] |=
            0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00100100;
        self.bitboard_pieces[Sides::WHITE][Pieces::QUEEN] |=
            0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00010000;
        self.bitboard_pieces[Sides::WHITE][Pieces::KING] |=
            0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00001000;
        self.bitboard_pieces[Sides::WHITE][Pieces::PAWN] |=
            0b00000000_00000000_00000000_00000000_00000000_00000000_11111111_00000000;

        // both
        self.bitboard_pieces[Sides::BOTH][Pieces::EMPTY] |=
            0b00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000;
    }
}
