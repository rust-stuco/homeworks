pub mod sum_squares {
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
}

pub mod fib_fun {
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
    /// This time we will not give you any constraints on how to
    /// implement this function.
    ///
    /// However, you will probably find it a lot easier
    /// to use iterators and dot chaining than to use a bunch of `for` loops.
    /// Our reference solution is 9 short lines,
    /// and it can definitely be made shorter!
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
            .map(|i| {
                let total: usize = Fibonacci::new(0, 1).take(i + 1).sum();
                total * total
            })
            .filter(|&s| s % 2 == 0 || s % 3 == 0)
            .enumerate()
            .map(|(j, x)| if j == 0 { 0 } else { x / j })
            .collect()
    }
}
