use crate::iterators::fibonacci::Fibonacci;

/// Find the sum of the range of fibonacci numbers
/// defined by `start` and `end`.
///
/// # Example:
///
/// Recall that the fibonacci numbers from this sequence:
///
/// ```text
/// 0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233...
/// ```
///
/// If we want to sum the 4th to 8th fibonacci number, we would need to sum
/// `3 + 5 + 8 + 13`.
///
/// ```
/// # use iterlab_ref::hofs::fib_fun::sum_fib_range;
/// assert_eq!(29, sum_fib_range(4, 8));
/// ```
///
/// Try to not use any for loops, if statements, or mutable variables!
pub fn sum_fib_range(start: usize, end: usize) -> usize {
    Fibonacci::new(0, 1).skip(start).take(end - start).sum()
}

/// For every `i`th number from `0..n`, find the sum of the first `i`
/// fibonnaci numbers and square that sum.
///
/// Then, for each of those squared sums, keep only the ones that are
/// divisible by 2 or 3.
///
/// Let's say that there are now `m` remaining numbers. For every `j`th
/// number from `1..m`, integer divide the number by `j`.
/// Since 0 is going to end up being the first of the `m` numbers anyways,
/// don't do `0 / 0`, and just return 0 instead.
///
/// Return these numbers as a vector of `usize`.
///
/// # Example
///
/// Let's walk through a call of `read_the_docs(6)`.
///
/// The first 6 fibonacci numbers are:
/// ```text
/// 0, 1, 1, 2, 3, 5
/// ```
///
/// We want 6 summations:
/// ```text
/// 0, 1, 2, 4, 7, 12
/// ```
///
/// Then we square these:
/// ```text
/// 0, 1, 4, 16, 49, 144
/// ```
///
/// We keep only those divisible by 2 or 3 (here only even numbers):
/// ```text
/// 0, 4, 16, 144
/// ```
///
/// Let's attach the position of each number to the above:
/// ```text
/// value: 0, 4, 16, 144
/// index: 0  1  2   3
/// ```
///
/// Dividing each out:
/// ```text
/// 0, 4, 8, 48
/// ```
///
/// Let's see if this works:
/// ```
/// # use iterlab_ref::hofs::fib_fun::read_the_docs;
/// assert_eq!(vec![0, 4, 8, 48], read_the_docs(6));
/// ```
///
/// ---
///
/// This time we will not give you any constraints on how to
/// implement this function.
///
/// However, you will probably find it a lot easier
/// to use iterators and dot chaining than to use a bunch of `for` loops.
/// You may even want to reuse some of the code you
/// wrote for previous parts...
///
/// Note that our reference solution is 6 short lines
/// (which means we have at least 5 chained methods)!
///
/// ---
///
/// Note that if you decide to use [`take`](std::iter::Iterator::take),
/// the number you input is an _exclusive_ end.
///
/// ```
/// assert_eq!(vec![0, 1, 2, 3, 4], (0..).take(5).collect::<Vec<_>>());
/// ```
pub fn read_the_docs(n: usize) -> Vec<usize> {
    (0..n)
        .map(|i| sum_fib_range(0, i + 1).pow(2))
        .filter(|&s| s % 2 == 0 || s % 3 == 0)
        .enumerate()
        .map(|(j, x)| if j == 0 { 0 } else { x / j })
        .collect()
}
