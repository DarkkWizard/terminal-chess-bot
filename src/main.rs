use std::io;

fn main() {
    // generate the board
    let mut board: Vec<Vec<char>> = vec![vec!['A'; 8]; 8];

    // starting position FEN string
    let fen = String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");
    fen_changer(&fen, &mut board);
    print_board_state(&board);

    let mut game_state = true;
    // prompt user for move input
    while game_state {
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        if input.trim() == "quit" {
            game_state = false;
            break;
        }
        make_it_move(&mut board, &input);
        game_state = false;
        break;
    }

    print_board_state(&board);
}

// takes in a FEN string and creates a board state to match it
fn fen_changer(fen: &str, board: &mut [Vec<char>]) {
    let mut row = 0;
    let mut col = 0;

    for c in fen.chars() {
        if c == '/' {
            row += 1;
            col = 0;
        } else if let Some(digit) = c.to_digit(10) {
            col += digit as usize;
        } else if c == 'p'
            || c == 'P'
            || c == 'r'
            || c == 'R'
            || c == 'n'
            || c == 'N'
            || c == 'b'
            || c == 'B'
            || c == 'q'
            || c == 'Q'
            || c == 'k'
            || c == 'K'
        {
            board[row][col] = c;
            col += 1;
        } else {
            panic!("Invalid character in FEN: {}", c);
        }
    }
}

// prints the current board state
fn print_board_state(board: &[Vec<char>]) {
    println!("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
    for row in board {
        print!("   ");
        for &square in row {
            match &square {
                'A' => print!("* "),
                _ => print!("{} ", square),
            }
        }
        println!();
    }
    println!("\n\n");
}

fn chess_notation_finder(spot: &str) -> (usize, usize) {
    let file = spot.chars().next().unwrap() as usize - 'a' as usize;
    let rank = (8 - spot.chars().nth(1).unwrap().to_digit(10).unwrap()) as usize;
    (rank, file)
}

fn make_it_move(board: &mut [Vec<char>], input: &str) {
    let start_square = &input[0..2];
    let end_square = &input[2..4];

    let (start_row, start_col) = chess_notation_finder(start_square);
    let (end_row, end_col) = chess_notation_finder(end_square);

    board[end_row][end_col] = board[start_row][start_col];
    board[start_row][start_col] = 'A';
}
