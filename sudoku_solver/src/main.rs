mod types;
mod sudoku;
mod sudoku_refsol;

use types::{Board, Number};

use sudoku_refsol::solve;

fn main() -> std::io::Result<()> {
    // 1. Load sudoku board from filename on command line
    let args: Vec<String> = std::env::args().collect();
    let board_filename = args.get(1).map(String::to_owned).unwrap_or(String::from("board1.sudoku"));

    let board_bytes = std::fs::read(board_filename)?;
    let mut board_numbers = [[Option::<Number>::None; 9]; 9];
    let mut index = 0usize;
    for r in 0..9 {
        for c in 0..9 {
            while !board_bytes[index].is_ascii_digit() {
                index += 1;
            }
            board_numbers[r][c] = Number::from_byte(board_bytes[index]);
            index += 1;
        }
    }

    // 2. Create actual board object
    let mut board = Board::from_array(board_numbers).unwrap();
    println!("{}", board);

    // 3. Attempt to solve the board
    let now = std::time::Instant::now();
    let solvable = solve(&mut board);
    let elapsed_time = now.elapsed();

    // 4. Print out the board
    println!("{}", board);
    if solvable {
        assert!(board.is_filled());
    }
    println!("Took {:?}", elapsed_time);

    Ok(())
}
