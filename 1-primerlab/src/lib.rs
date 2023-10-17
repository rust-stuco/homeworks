mod exercises;

/// Make sure this works!
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

// Solutions and some test cases adapted from 15-112:
// https://www.kosbie.net/cmu/spring-22/15-112/notes/notes-loops.html#nthPrime

/// Given a number n, return true if it is a prime number, and false otherwise.
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
/// Please do not use the "return" keyword.
/// Please write this function using recursion.
pub fn gcd(x: usize, y: usize) -> usize {
    if y == 0 {
        x
    } else {
        gcd(y, x % y)
    }
}

/// Returns the nth fibonnaci number.
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

// Test cases are located in test.rs!
