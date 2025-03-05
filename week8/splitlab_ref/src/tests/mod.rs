use crate::split_pattern::SplitPattern;
use crate::split_str::Split;

/// A macro to run a test for both [`Split`] and [`SplitPattern`].
macro_rules! test_splitters {
    ($name:ident, $haystack:expr, $pattern:expr, $expected:expr) => {
        #[test]
        fn $name() {
            // Test with `Split`.
            let split_result: Vec<_> = Split::new($haystack, $pattern).collect();
            assert_eq!(split_result, $expected);

            // Test with `SplitPattern`.
            let pattern_result: Vec<_> = SplitPattern::new($haystack, $pattern).collect();
            assert_eq!(pattern_result, $expected);

            // Test against the real `split` method.
            assert_eq!(
                pattern_result,
                $haystack.split($pattern).collect::<Vec<&str>>()
            );
        }
    };
}

/// A macro that only tests [`SplitPattern`].
macro_rules! test_pattern {
    ($name:ident, $haystack:expr, $pattern:expr, $expected:expr) => {
        #[test]
        fn $name() {
            let pattern_result: Vec<_> = SplitPattern::new($haystack, $pattern).collect();
            assert_eq!(pattern_result, $expected);

            // Test against the real `split` method.
            assert_eq!(
                pattern_result,
                $haystack.split($pattern).collect::<Vec<&str>>()
            );
        }
    };
}

mod both;
mod pattern;
