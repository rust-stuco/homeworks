#![allow(unused_variables)]

/// Solutions and some test cases adapted from [15-112](
/// https://www.kosbie.net/cmu/spring-22/15-112/notes/notes-loops.html#nthPrime).
/// Test cases are located in tests.rs!
/// Make sure this works!
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

/// Given a number n, return true if it is a prime number, and false otherwise.
///
/// # Example
///
/// ```
/// use primerlab_refsol::functions::is_prime;
///
/// let test_prime = 2;
/// assert!(is_prime(test_prime));
///
/// let test_not_prime = 42;
/// assert!(!is_prime(test_not_prime));
/// ```
pub fn is_prime(n: usize) -> bool {
    if n == 0 || n == 1 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    let max_factor = f64::sqrt(n as f64) as usize + 1;
    for i in (3..max_factor).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

/// Given a number n, return the nth prime. Refer to the test cases below for more details.
///
/// For example, the 0th prime is 2, and the 1st prime is 3, then the 2nd prime is 5, etc.
///
/// # Example
///
/// ```
/// use primerlab_refsol::functions::nth_prime;
///
/// let n = 4;
/// assert_eq!(nth_prime(n), 11);
///
/// let n = 20;
/// assert_eq!(nth_prime(n), 73);
/// ```
///
/// ### Suggestions
/// You can look [here](https://en.wikipedia.org/wiki/List_of_prime_numbers) for more primes,
/// but note that Wikipedia 1-indexes them.
pub fn nth_prime(n: usize) -> usize {
    let mut found = 0;
    let mut guess = 0;
    while found <= n {
        guess += 1;
        if is_prime(guess) {
            found += 1
        }
    }
    guess
}

/// Returns the Greatest Common Divisor (gcd) of two numbers x and y.
///
/// # Example
///
/// ```
/// use primerlab_refsol::functions::gcd;
///
/// let first_num = 60;
/// let second_num = 24;
///
/// assert_eq!(gcd(first_num, second_num), 12);
/// ```
///
/// # **Restrictions**
/// **Please do not use the "return" keyword.**
/// **Please write this function using recursion.**
pub fn gcd(x: usize, y: usize) -> usize {
    if y == 0 {
        x
    } else {
        gcd(y, x % y)
    }
}

/// Returns the nth fibonacci number.
///
/// We consider the 0th fibonacci number to be 0, and the first to be 1.
///
/// # Example
///
/// ```
/// use primerlab_refsol::functions::fib;
///
/// assert_eq!(fib(2), 1);
/// assert_eq!(fib(4), 3);
/// assert_eq!(fib(7), 13);
/// ```
pub fn fib(n: usize) -> usize {
    let init = (0, 1);

    fn fib_helper(from: (usize, usize), n: usize) -> usize {
        if n == 0 {
            from.0
        } else {
            fib_helper((from.1, from.0 + from.1), n - 1)
        }
    }

    fib_helper(init, n)
}
