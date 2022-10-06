mod automaton;
mod parsing;

use std::ops::Range;

use automaton::Automaton;
use chumsky::{error::Simple, Parser};

/// A compiled regular expression for matching ASCII text.
pub struct Matcher {
    conf: MacherConfig,
    nfa: Automaton,
}

struct MacherConfig {
    /// `true` iff the pattern is allowed to match only a prefix of the target.
    /// Conceptually the same as if it ended in `.*`, but allows for a more efficent implementation.
    prefix_okay: bool,
}

impl Matcher {
    // Feel free to true us up with POSIX ERE as an exercise :). Mostly involves parsing changes.
    // Other possible extensions include:
    //  - submatch extraction (we only support match extraction right now---you could implement
    //    this with the State::Save construct)
    //  - adding nongreedy operations --- mostly just parsing, we have the plumbing for it.
    /// Complies a given regular expression. Once compiled, this can be used to search multiple
    /// times.
    ///
    /// The syntax for regular expressions is the same as [POSIX ERE], with the following exceptions.
    ///
    // Adding these are mostly parsing / lowering in Automaton construction.
    /// 1. Only a subset of bracket expressions are supported, namely:
    // These three would probably be pretty tedious and not super interesting (locale handling,
    // just making tables of chars, etc..)
    ///    1. There is no support for character class expressions (e.g., `[[:alpha:]]`);
    ///    2. There is no support for equivalence class expressions (e.g., `[=a=]`);
    ///    3. There is no support for collating symbols (e.g., `[[.ch.]]`); and
    // Wouldn't be too bad to add---if you want to play around with the parser you could do this
    // one!
    ///    4. There is no support for character ranges (e.g., `[a-zA-Z]`).
    // Similar to 1.4
    /// 2. Single character duplication is not supported (e.g. `a{N,M}`)
    // This one is kinda annoying and probably not fun
    /// 3. A closing parenthesis (`')'`) not corresponding to an opening parentheses are treated as
    ///    a syntax error instead of a literal closing parenthesis.
    // I wouldn't change this one though---the POSIX rules are a bit of a mess,
    // implementation-wise.
    /// 4. The matching precedences dictated by POSIX are not followed. Instead, we follow the Perl
    ///    conventions (i.e., instead of "longest leftmost", we take the leftmost match subject to
    ///    a total ordering of possible results where a result arising from exploration of the left
    ///    side of any binary operation or greedy performance of a repeating operation is
    ///    favoured).
    ///
    /// [POSIX ERE]: <https://pubs.opengroup.org/onlinepubs/9699919799/basedefs/V1_chap09.html#tag_09_04>
    pub fn new(regex: &'_ str) -> Result<Self, Vec<Simple<u8>>> {
        let (pat, conf) = parsing::pattern_parser().parse(regex.as_bytes())?;
        Ok(Self {
            conf,
            nfa: Automaton::new_from_pattern(pat),
        })
    }

    /// Returns `true` if and only if there is a match for the regex in the given string.
    pub fn matches(&self, text: &'_ str) -> bool {
        self.nfa.is_match(text.as_bytes(), self.conf.prefix_okay)
    }

    pub fn find<'t>(&self, text: &'t str) -> Option<Match<'t>> {
        // Implement this as an exercise if you want to report the exact ranges matched (e.g., with
        // text highlighting). Most of the plumbing already exists as
        // `Automaton::is_match_with_location`, so this shouldn't be too daunting!
        todo!(
            "Tried to find in {}, but you haven't implemented this!",
            text
        )
    }
}

#[cfg(not(refsol))]
pub struct Match<'t> {
    // ... and if you're implementing `Matcher::find`, you'll probably want to modfiy this
    /// I'm just here to supress a warning about unused lifetime parameters (hint: you probably
    /// want to use the lifetime parameter).
    _delete_me: std::marker::PhantomData<&'t ()>,
}
#[cfg(refsol)]
pub struct Match<'t> {
    start: usize,
    text: &'t str,
}

impl<'t> Match<'t> {
    /// Returns the range corresponding to the matched text
    pub fn range(&self) -> Range<usize> {
        todo!()
    }

    /// Returns the substring corresponding to the matched text
    pub fn as_str(&self) -> &'t str {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert!(Matcher::new("abc").unwrap().matches("abc"));
    }

    #[test]
    fn alternation() {
        let m = Matcher::new("abc|def").unwrap();
        assert!(m.matches("abc"));
        assert!(!m.matches("abd"));
        assert!(m.matches("def"));
        assert!(m.matches("defensible"));
        assert!(m.matches("fooabc"));
        assert!(m.matches("fooabcbar"));
    }

    #[test]
    fn repetition() {
        let m = Matcher::new("a+b*").unwrap();
        assert!(m.matches("abbbb"));
        assert!(!m.matches("bbbb"));
        assert!(m.matches("a"));
        assert!(m.matches("aaaa"));
        assert!(m.matches("abbba"));
        assert!(m.matches("ac"));
        assert!(m.matches("ac"));
    }

    #[test]
    fn anchored() {
        let m = Matcher::new("^abc|def$").unwrap();
        assert!(m.matches("abc"));
        assert!(!m.matches("abd"));
        assert!(m.matches("def"));
        assert!(!m.matches("defensible"));
        assert!(!m.matches("fooabc"));
        assert!(!m.matches("fooabcbar"));
    }
}
