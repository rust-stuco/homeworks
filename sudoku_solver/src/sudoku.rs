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
            || {
                solvable = true;
                println!("Success Continuation called!");
                "Board is solvable!"
            },
            || {
                println!("Failure Continuation called!");
                "Board is not solvable :("
            },
        )
    );
    if solvable {
        assert!(b.is_filled());
    }
    solvable
}

/// Calls sc iff the board can be solved by changing every non-Fixed square from (row, col) and
/// after (according to `next_index`). If sc is not called, fc is called instead.
fn solve_cps<T>(
    board: &mut Board,
    index: Index,
    mut sc: impl FnMut() -> T,
    mut fc: impl FnMut() -> T,
) -> T {
    use Square::*;

    let next_index = match index.next() {
        Some(i) => i,
        None => {
            for n in Number::iter() {
                if board.place(index, n) {
                    return sc();
                }
            }
            return fc();
        }
    };

    if let Fixed(_) = board[index] {
        return match index.next() {
            Some(next_index) => solve_cps(board, next_index, sc, fc),
            None => sc(),
        };
    }

    fn try_next_number<T>(
        board: &mut Board,
        index: Index,
        next_index: Index,
        number: Number,
        mut sc: impl FnMut() -> T,
        mut fc: impl FnMut() -> T,
    ) -> T {
        if board.place(index, number) {
            return sc();
        }

        let next_number = match number.next() {
            Some(n) => n,
            None => return fc(),
        };

        solve_cps(board, next_index, sc, move || {
            try_next_number(board, index, next_index, next_number, sc, fc)
        })
    }

    try_next_number(board, index, next_index, Number::One, sc, fc)
}
