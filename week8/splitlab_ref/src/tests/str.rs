use super::*;

test_str!(
    test_crust_of_rust_it_works,
    "a b c d e",
    " ",
    vec!["a", "b", "c", "d", "e"]
);

test_str!(
    test_crust_of_rust_tail,
    "a b c d ",
    " ",
    vec!["a", "b", "c", "d", ""]
);

test_str!(test_empty_haystack, "", " ", vec![""]);

test_str!(
    test_starting_delimiter,
    " a b c",
    " ",
    vec!["", "a", "b", "c"]
);

test_str!(test_long_delimiter, "a---b---c", "---", vec!["a", "b", "c"]);

test_str!(
    test_multiple_consecutive_delimiters,
    "a  b   c",
    " ",
    vec!["a", "", "b", "", "", "c"]
);

test_str!(test_no_delimiter_in_haystack, "abcde", "x", vec!["abcde"]);

test_str!(test_unicode_delimiter, "a😊b😊c", "😊", vec!["a", "b", "c"]);

test_str!(test_only_delimiters, ";;;", ";", vec!["", "", "", ""]);
