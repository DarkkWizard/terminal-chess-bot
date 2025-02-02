// use std::io;
use crate::board::Borat;
use crate::defs::{Bitboard, Pieces, Sides};
//use std::io::Write;
//use termcolor::{BufferWriter, Color, ColorChoice, ColorSpec, WriteColor};
mod board;
mod defs;

fn main() {
    let board: Borat = Borat::new_empty();
    let var: Bitboard = board.get_piece(Pieces::PAWN, Sides::WHITE);
    print_bitboard(&var.mnum);
}

fn print_bitboard(bitboard: &u64) {
    const LAST_BIT: u64 = 63;
    for rank in 0..8 {
        for file in (0..8).rev() {
            let mask = 1u64 << (LAST_BIT - (rank * 8) - file);
            let char = if bitboard & mask != 0 { '1' } else { '*' };
            print!("{char} ");
        }
        println!();
    }
}
