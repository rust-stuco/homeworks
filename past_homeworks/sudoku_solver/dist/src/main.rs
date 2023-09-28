mod sudoku;
mod types;

use types::{Board, Number};

use sudoku::solve;

fn read_board_from_bytes(board_bytes: &[u8]) -> Board {
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

    Board::from_array(board_numbers).unwrap()
}

fn main() -> std::io::Result<()> {
    // 1. Load sudoku board from filename on command line
    let args: Vec<String> = std::env::args().collect();
    let board_filename = args
        .get(1)
        .map(String::to_owned)
        .unwrap_or(String::from("board1.sudoku"));

    let board_bytes = std::fs::read(board_filename)?;

    // 2. Create actual board object
    let mut board = read_board_from_bytes(&board_bytes);
    println!("{}", board);

    // 3. Attempt to solve the board
    // This needs to be on a separate stack because the main thread's stack size is **NOT**
    // configurable for whatever reason, and we're doing a lot of deep recursion
    let mut solvable = false;
    let now = std::time::Instant::now();
    std::thread::scope(|s| {
        std::thread::Builder::new()
            .stack_size(1024 * 1024 * 1024) // lol. lmao.
            .spawn_scoped(s, || {
                solvable = solve(&mut board);
            })
            .expect("Could not create child thread to run solver")
            .join()
            .expect("Could not join with child thread running solver");
    });
    let elapsed_time = now.elapsed();

    // 4. Print out the board
    println!("{}", board);
    if solvable {
        assert!(board.is_filled());
    } else {
        println!("Board not solvable");
    }
    println!("Took {:?}", elapsed_time);

    Ok(())
}

/// Testbench module
#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! testcase {
        ($board_filename:ident, $solvable:expr) => {
            paste::paste! {
                #[test]
                fn [<test_ $board_filename>]() {
                    let mut board = read_board_from_bytes(include_bytes!(concat!("../", stringify!($board_filename), ".sudoku")));
                    let mut solvable = Option::<bool>::None;
                    std::thread::scope(|s| {
                        std::thread::Builder::new()
                            .stack_size(1024 * 1024 * 1024) // lol. lmao.
                            .spawn_scoped(s, || {
                                solvable = Some(solve(&mut board));
                            })
                            .expect("Could not create child thread to run solver")
                            .join()
                            .expect("Could not join with child thread running solver");
                    });
                    assert_eq!(Some($solvable), solvable);
                }
            }
        }
    }

    testcase!(board1, true);
    testcase!(board2, true);
    testcase!(board3, true);
    testcase!(board4, true);
    testcase!(allzeros, true);
    testcase!(invalid, false);
}
