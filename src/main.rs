// use std::io;
use crate::board::Borat;
use crate::defs::{print_bitboard, Pieces, Sides};
//use std::io::Write;
//use termcolor::{BufferWriter, Color, ColorChoice, ColorSpec, WriteColor};
mod board;
mod defs;

fn main() {
    let mut board: Borat = Borat::new_empty();
    // print_bitboard(&board.bitboard_sides[Sides::WHITE]);
    // print_bitboard(&board.bitboard_sides[Sides::BLACK]);
    println!("\n\n");
    print_bitboard(&board.bitboard_pieces[Sides::BLACK][Pieces::BISHOP]);
    println!("\n\n");
    board.move_piece(61, 22, Sides::BLACK, Pieces::BISHOP);
    print_bitboard(&board.bitboard_pieces[Sides::BLACK][Pieces::BISHOP]);
    println!("\n\n");
}
