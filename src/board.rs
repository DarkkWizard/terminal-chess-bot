#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum PieceColor {
    White,
    Black,
    All,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
    All,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct Bitboard {
    mnum: u64,
    mcolor: Option<PieceColor>,
    mtype: Option<PieceType>,
}

impl Bitboard {
    fn new(num: u64, color: Option<PieceColor>, heirarchy: Option<PieceType>) -> Bitboard {
        Bitboard {
            mnum: num,
            mcolor: color,
            mtype: heirarchy,
        }
    }
}
