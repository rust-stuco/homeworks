use super::*;

test_splitters!(
    test_crust_of_rust_it_works,
    "a b c d e",
    " ",
    vec!["a", "b", "c", "d", "e"]
);

test_splitters!(
    test_crust_of_rust_tail,
    "a b c d ",
    " ",
    vec!["a", "b", "c", "d", ""]
);

test_splitters!(test_empty_haystack, "", " ", vec![""]);

test_splitters!(
    test_starting_delimiter,
    " a b c",
    " ",
    vec!["", "a", "b", "c"]
);

test_splitters!(test_long_delimiter, "a---b---c", "---", vec!["a", "b", "c"]);

test_splitters!(
    test_multiple_consecutive_delimiters,
    "a  b   c",
    " ",
    vec!["a", "", "b", "", "", "c"]
);

test_splitters!(test_no_delimiter_in_haystack, "abcde", "x", vec!["abcde"]);

test_splitters!(test_unicode_delimiter, "a😊b😊c", "😊", vec!["a", "b", "c"]);

test_splitters!(test_only_delimiters, ";;;", ";", vec!["", "", "", ""]);
