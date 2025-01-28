// use std::io;
use board::*;
mod board;

fn main() {}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Square {
    // nothing in it
    Empty,
    // a piece is in it defined by type piece
    Filled(Piece),
    // it's a boundry square
    Boundry,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Piece {
    pub color: PieceColor,
    pub kind: PieceType,
}

impl Piece {
    fn new(color: PieceColor, kind: PieceType) -> Self {
        Piece { color, kind }
    }

    fn to_character(self) -> &'static str {
        match (self.color, self.kind) {
            (PieceColor::White, PieceType::Pawn) => "P",
            (PieceColor::White, PieceType::Rook) => "R",
            (PieceColor::White, PieceType::Knight) => "N",
            (PieceColor::White, PieceType::Bishop) => "B",
            (PieceColor::White, PieceType::Queen) => "Q",
            (PieceColor::White, PieceType::King) => "K",
            (PieceColor::Black, PieceType::Pawn) => "p",
            (PieceColor::Black, PieceType::Rook) => "r",
            (PieceColor::Black, PieceType::Knight) => "n",
            (PieceColor::Black, PieceType::Bishop) => "b",
            (PieceColor::Black, PieceType::Queen) => "q",
            (PieceColor::Black, PieceType::King) => "k",
        }
    }
}

pub struct Board {}

// takes in a FEN string and creates a board state to match it
fn chess_notation_finder(spot: &str) -> (usize, usize) {
    let file = spot.chars().next().unwrap() as usize - 'a' as usize;
    let rank = (8 - spot.chars().nth(1).unwrap().to_digit(10).unwrap()) as usize;
    (rank, file)
}
