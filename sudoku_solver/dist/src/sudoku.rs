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
            |_, was_solved| {
                if was_solved {
                    solvable = true;
                    "Board is solvable!"
                } else {
                    "Board is not solvable :("
                }
            },
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
fn solve_cps<'a, T, F>(board: &'a mut Board, index: Index, cc: F) -> T
where
    F: FnOnce(&'a mut Board, bool) -> T,
{
    // 1. If the current square is fixed, don't try to change it and go to the next

    // 2. If we are at the final index, just try all numbers, calling the success continuation if
    // we succeed in placing one

    fn try_next_number<'a, T, F>(
        board: &'a mut Board,
        index: Index,
        next_index: Index,
        number: Number,
        cc: F,
    ) -> T
    where
        F: FnOnce(&'a mut Board, bool) -> T,
    {
        // 3. Try placing each value from `number` onwards. If this all fails, call the failure
        // continuation

        solve_cps(
            board,
            next_index,
            // 4. You figure out what to do here!
            // Hint for the "type recursion limit exceeded" error: try `Box`-ing your closure
            // See https://doc.rust-lang.org/book/ch17-02-trait-objects.html
            |_, _| { todo!() }
        )
    }

    let next_index = index.next().unwrap();
    try_next_number(board, index, next_index, Number::One, cc)
}
