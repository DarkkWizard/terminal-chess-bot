// use std::io;
use crate::board::Borat;
use crate::defs::{print_bitboard, print_board, Pieces, Sides};
use crate::movegen::MoveGen; //use std::io::Write;
//use termcolor::{BufferWriter, Color, ColorChoice, ColorSpec, WriteColor};
mod board;
mod defs;
mod movegen;

fn main() {
    let mut board: Borat = Borat::new_empty();
    let generator: MoveGen = MoveGen::init();
    println!("\n\n");
    println!("\n\n");
    println!("\n\n");
    print_board(&board);
    println!("\n\n");
    println!("\n\n");
    board.move_piece(61, 22, Sides::BLACK, Pieces::BISHOP);
    println!("\n\n");
    print_board(&board);
}




// this is a good printer thing that i need to remember how to use
//generator.king_attacks.iter().for_each(|&number| print_bitboard(&number));
