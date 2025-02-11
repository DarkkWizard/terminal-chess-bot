// use std::io;
use crate::board::Borat;
use crate::defs::{print_bitboard, Pieces, Sides};
//use std::io::Write;
//use termcolor::{BufferWriter, Color, ColorChoice, ColorSpec, WriteColor};
mod board;
mod defs;

fn main() {
    let board: Borat = Borat::new_empty();
    // print_bitboard(&board.bitboard_sides[Sides::WHITE]);
    // print_bitboard(&board.bitboard_sides[Sides::BLACK]);
    print_bitboard(&board.bitboard_pieces[Sides::BLACK][Pieces::QUEEN]);
}
