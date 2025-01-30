use super::gamestate;

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

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Bitboard {
    pub mnum: u64,
}

impl Bitboard {
    pub fn new(num: u64) -> Bitboard {
        Bitboard { mnum: num }
    }
}

pub struct Pieces;
impl Pieces {
    pub const KING: usize = 0;
    pub const QUEEN: usize = 1;
    pub const ROOK: usize = 2;
    pub const BISHOP: usize = 3;
    pub const KNIGHT: usize = 4;
    pub const PAWN: usize = 5;
    pub const EMPTY: usize = 6;
}
