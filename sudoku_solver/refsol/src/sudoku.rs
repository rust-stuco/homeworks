use crate::types::*;

/// Returns true if this board can be solved, modifying the board to a solved state if so, and
/// false if the board cannot be solved, with the board in an undefined state
pub fn solve(b: &mut Board) -> bool {
    let mut solvable = false;
    println!(
        "{}",
        solve_cps(
            b,
            Index {
                row: Number::One,
                col: Number::One,
            },
            Box::new(|_, was_solved| {
                if was_solved {
                    solvable = true;
                    "Board is solvable!"
                } else {
                    "Board is not solvable :("
                }
            }),
        )
    );
    solvable
}

/// Calls cc(board, true) iff the board can be solved by changing every non-Fixed square from (row, col) and
/// after (according to `next_index`). Otherwise, calls cc(board, false)
///
/// EXTRA CHALLENGE: try re-writing this to use a separate success continuation and failure
/// continuation. Where do things start to go wrong?
/// EXTRA CHALLENGE: try re-writing this with FnMut instead of FnOnce. Where do things start going
/// wrong?
/*
fn solve_cps<'a, T, F>(board: &'a mut Board, index: Index, cc: F) -> T
where
    F: FnOnce(&'a mut Board, bool) -> T,
*/
fn solve_cps<'a, 'b, T>(board: &'a mut Board, index: Index, cc: Box<dyn FnOnce(&'a mut Board, bool) -> T + 'b>) -> T
    where T: 'b
{
    use Square::*;

    if let Fixed(_) = board[index] {
        return match index.next() {
            Some(next_index) => {
                solve_cps(board, next_index, cc)
            }
            None => cc(board, true),
        };
    }

    let next_index = match index.next() {
        Some(i) => i,
        None => {
            for n in Number::iter() {
                if board.place(index, n) {
                    return cc(board, true);
                }
            }
            return cc(board, false);
        }
    };

    fn try_next_number<'a, 'b, T, F>(
        board: &'a mut Board,
        index: Index,
        next_index: Index,
        mut number: Number,
        cc: F,
    ) -> T
    where
        F: FnOnce(&'a mut Board, bool) -> T + 'b,
        T: 'b
    {
        while !board.place(index, number) {
            number = match number.next() {
                Some(n) => n,
                None => {
                    board.unplace(index);
                    return cc(board, false);
                }
            };
        }

        solve_cps(
            board,
            next_index,
            Box::new(move |board, was_solved| {
                if was_solved {
                    cc(board, true)
                } else {
                    let next_number = match number.next() {
                        Some(n) => n,
                        None => {
                            board.unplace(index);
                            return cc(board, false);
                        }
                    };
                    try_next_number(board, index, next_index, next_number, cc)
                }
            }) as Box<dyn FnOnce(&'a mut Board, bool) -> T>,
        )
    }

    try_next_number(board, index, next_index, Number::One, cc)
}
