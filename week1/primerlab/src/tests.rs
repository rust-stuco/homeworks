use crate::functions::{add, fib, gcd, is_prime, nth_prime};

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
    let g = 7128; //    2**3 * 3**4 *               11**1
    assert_eq!(gcd(x, y), g);
}

#[test]
fn test_fib_basic() {
    assert_eq!(fib(0), 0);
    assert_eq!(fib(1), 1);
    assert_eq!(fib(9), 34);
    assert_eq!(fib(10), 55);
    assert_eq!(fib(11), 89);
    assert_eq!(fib(12), 144);
    assert_eq!(fib(18), 2584);
    assert_eq!(fib(25), 75025);
    assert_eq!(fib(50), 12586269025);
    assert_eq!(fib(60), 1548008755920);
}
