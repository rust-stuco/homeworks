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
#[cfg(test)]
fn nth_prime(n: usize) -> usize {
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
/// You are NOT allowed to use the "return" keyword.
/// Your solution MUST be recursive.
#[cfg(test)]
fn gcd(x: usize, y: usize) -> usize {
    if y == 0 {
        x
    } else {
        gcd(y, x % y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Run these tests with `cargo test`.
    /// Some of these tests might be slow though. In that case, run `cargo test --release`!

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_is_prime_basic() {
        assert!(!is_prime(1));
        assert!(is_prime(2));
        assert!(is_prime(3));
        assert!(!is_prime(4));
        assert!(is_prime(5));
        assert!(!is_prime(6));
        assert!(is_prime(7));
        assert!(!is_prime(8));
        assert!(!is_prime(9));
        assert!(!is_prime(10));
        assert!(is_prime(11));
        assert!(!is_prime(12));
    }

    #[test]
    fn test_is_prime_evens() {
        for i in (4..100_000_000).step_by(2) {
            assert!(!is_prime(i));
        }
    }

    #[test]
    fn test_random_primes() {
        for p in [
            3499501457,
            8264009899,
            4989623741,
            5165673577,
            2594204321,
            847402035575327,
        ] {
            assert!(is_prime(p));
        }
    }

    #[test]
    fn test_nth_prime_basic() {
        assert_eq!(nth_prime(0), 2);
        assert_eq!(nth_prime(1), 3);
        assert_eq!(nth_prime(2), 5);
        assert_eq!(nth_prime(3), 7);
        assert_eq!(nth_prime(4), 11);
        assert_eq!(nth_prime(5), 13);
        assert_eq!(nth_prime(6), 17);
    }

    #[test]
    fn test_nth_prime_larger() {
        assert_eq!(nth_prime(10), 31);
        assert_eq!(nth_prime(20), 73);
        assert_eq!(nth_prime(30), 127);
        assert_eq!(nth_prime(40), 179);
        assert_eq!(nth_prime(50), 233);
        assert_eq!(nth_prime(60), 283);
        assert_eq!(nth_prime(70), 353);
        assert_eq!(nth_prime(80), 419);
        assert_eq!(nth_prime(90), 467);
        assert_eq!(nth_prime(100), 547);
        assert_eq!(nth_prime(100_000), 1299721);
    }

    #[test]
    fn test_nth_prime_very_large() {
        // This might take a long time.
        assert_eq!(nth_prime(1_000_000), 15485867);
    }

    #[test]
    fn test_gcd_basic() {
        assert_eq!(gcd(4, 6), 2);
        assert_eq!(gcd(3, 3), 3);
        let a = 3usize.pow(6);
        let b = 2usize.pow(6);
        assert_eq!(gcd(a, a), a);
        assert_eq!(gcd(a, b), 1);
        assert_eq!(gcd(2 * 3 * 4 * 5, 3 * 5), 15);
        let x = 1568160; // 2**5 * 3**4 * 5**1 *        11**2
        let y = 3143448; // 2**3 * 3**6 *        7**2 * 11**1
        let g = 7128; // 2**3 * 3**4 *               11**1
        assert_eq!(gcd(x, y), g);
    }
}