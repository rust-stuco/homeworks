/// Finds the sum of squared odd numbers, where the largest squared odd number
/// is less than or equal to `largest_square`.
///
/// Example:
///
/// The first 4 odd squares are 1, 9, 25, and 49.
/// If we call `sum_of_squared_odd_numbers_bad(30)`, this will add together
/// `1 + 9 + 25` and return 35.
///
/// ```
/// # use iterlab_ref::hofs::sum_squares::sum_of_squared_odd_numbers_bad;
/// assert_eq!(35, sum_of_squared_odd_numbers_bad(30));
/// ```
///
/// Note that we yanked this
/// [example](https://doc.rust-lang.org/rust-by-example/fn/hof.html)
/// from Rust by Example.
///
/// You _could_ just click this link and copy and paste the answer...
/// but try and do it on your own so you get practice thinking for yourself!
pub fn sum_of_squared_odd_numbers_bad(largest_square: usize) -> usize {
    // Imperative approach
    // Declare accumulator variable
    let mut acc = 0;
    // Iterate: 0, 1, 2, ... to infinity
    for n in 0.. {
        // Square the number
        let n_squared = n * n;

        if n_squared > largest_square {
            // Break loop if exceeded the upper limit
            break;
        } else if n_squared % 2 == 1 {
            // Accumulate value, if it's odd
            acc += n_squared;
        }
    }

    acc
}

/// Implement me to do the same thing as [`sum_of_squared_odd_numbers_bad`],
/// but without any for loops, if statements, or mutable variables!
///
/// Example:
///
/// ```
/// # use iterlab_ref::hofs::sum_squares::sum_of_squared_odd_numbers;
/// assert_eq!(35, sum_of_squared_odd_numbers(30));
/// ```
///
/// A reminder that the answer is in the docs for
/// [`sum_of_squared_odd_numbers_bad`], but we encourage you not to look at it!
pub fn sum_of_squared_odd_numbers(largest_square: usize) -> usize {
    // Functional approach
    (0..)
        .map(|n| n * n) // All natural numbers squared
        .take_while(|&n_squared| n_squared < largest_square) // Below limit
        .filter(|&n_squared| n_squared % 2 == 1) // That are odd
        .sum() // Sum them
}
