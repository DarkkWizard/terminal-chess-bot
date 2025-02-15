use crate::board::Borat;
pub type Piece = usize;
pub type Side = usize;
pub type Square = usize;

pub struct Sides;
impl Sides {
    pub const WHITE: Side = 0;
    pub const BLACK: Side = 1;
    pub const BOTH: Side = 2;
}

// SMALLEST IS THE LEAST SIGNIFICANT
// Bitboard Square Mapping:
//
//   56  57  58  59  60  61  62  63     a8  b8  c8  d8  e8  f8  g8  h8
//   48  49  50  51  52  53  54  55     a7  b7  c7  d7  e7  f7  g7  h7
//   40  41  42  43  44  45  46  47     a6  b6  c6  d6  e6  f6  g6  h6
//   32  33  34  35  36  37  38  39     a5  b5  c5  d5  e5  f5  g5  h5
//   24  25  26  27  28  29  30  31     a4  b4  c4  d4  e4  f4  g4  h4
//   16  17  18  19  20  21  22  23     a3  b3  c3  d3  e3  f3  g3  h3
//    8   9  10  11  12  13  14  15     a2  b2  c2  d2  e2  f2  g2  h2
//    0   1   2   3   4   5   6   7     a1  b1  c1  d1  e1  f1  g1  h1
//
pub type Bitboard = u64;

pub struct Pieces;
impl Pieces {
    pub const KING: Piece = 0;
    pub const QUEEN: Piece = 1;
    pub const ROOK: Piece = 2;
    pub const BISHOP: Piece = 3;
    pub const KNIGHT: Piece = 4;
    pub const PAWN: Piece = 5;
    pub const EMPTY: Piece = 6;
}

// unsets the bit closest to the right and returns the position of the bit that it unset
pub fn next(bitboard: &mut Bitboard) -> usize {
    let square = bitboard.trailing_zeros() as usize;
    *bitboard ^= 1u64 << square;
    square
}

pub fn print_bitboard(bitboard: &u64) {
    const LAST_BIT: u64 = 63;
    for rank in 0..8 {
        for file in (0..8).rev() {
            let mask = 1u64 << (LAST_BIT - (rank * 8) - file);
            let char = if bitboard & mask != 0 { '@' } else { '+' };
            print!("{char} ");
        }
        println!();
    }
}

pub fn print_board(board: &Borat) {
    let mut buffer: String = String::new();
    // Piece, Side
    for item in board.piece_list {
        match item.0 {
            Pieces::KING => {
                if item.1 == Sides::WHITE {
                    buffer.push('K');
                    buffer.push(' ');
                } else {
                    buffer.push('k');
                    buffer.push(' ');
                }
            }
            Pieces::QUEEN => {
                if item.1 == Sides::WHITE {
                    buffer.push('Q');
                    buffer.push(' ');
                } else {
                    buffer.push('q');
                    buffer.push(' ');
                }
            }
            Pieces::ROOK => {
                if item.1 == Sides::WHITE {
                    buffer.push('R');
                    buffer.push(' ');
                } else {
                    buffer.push('r');
                    buffer.push(' ');
                }
            }
            Pieces::BISHOP => {
                if item.1 == Sides::WHITE {
                    buffer.push('B');
                    buffer.push(' ');
                } else {
                    buffer.push('b');
                    buffer.push(' ');
                }
            }
            Pieces::KNIGHT => {
                if item.1 == Sides::WHITE {
                    buffer.push('N');
                    buffer.push(' ');
                } else {
                    buffer.push('n');
                    buffer.push(' ');
                }
            }
            Pieces::PAWN => {
                if item.1 == Sides::WHITE {
                    buffer.push('P');
                    buffer.push(' ');
                } else {
                    buffer.push('p');
                    buffer.push(' ');
                }
            }
            Pieces::EMPTY => buffer.push_str("+ "),
            _ => (),
        }
    }
    let newbuff = [
        &buffer[112..127],
        &buffer[96..111],
        &buffer[80..95],
        &buffer[64..79],
        &buffer[48..63],
        &buffer[32..47],
        &buffer[16..31],
        &buffer[0..15],
    ];
    for line in newbuff {
        println!("{}", line);
    }
}
