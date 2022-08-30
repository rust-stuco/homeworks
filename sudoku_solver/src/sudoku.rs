use crate::types::*;

/// Returns true if this board can be solved, modifying the board to a solved state if so, and
/// false if the board cannot be solved, with the board in an undefined state
pub fn solve(b: &mut Board) -> bool {
    let mut solvable = false;
    solve_cps(b, Index { row: Number::One, col: Number::One }, Box::new(|| solvable = true), Box::new(|| {}));
    if solvable {
        assert!(b.is_filled());
    }
    solvable
}

/// Calls sc iff the board can be solved by changing every non-Fixed square from (row, col) and
/// after (according to `next_index`). If sc is not called, fc is called instead.
fn solve_cps<'a>(board: &mut Board, index: Index, sc: Box<dyn FnMut() + 'a>, fc: Box<dyn FnMut() + 'a>) {
    todo!()
}
