// use std::io;
mod board;
mod defs;

fn main() {
    let sixtyfourbitnum =
        0b01001001_11001101_10010100_10011110_11110011_11000111_01000111_10100110_u64;
    println!("{:064b}", sixtyfourbitnum);
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
