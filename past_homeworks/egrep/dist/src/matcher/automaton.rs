use super::parsing::PatternNode;
use id_arena::{Arena, Id};

mod bit_set;
use bit_set::BitSet;

type StateId = Id<State>;

#[derive(Debug, Clone, PartialEq)]
/// A specific state in the FSM which the Automaton simulates.
enum State {
    /// Process a single byte and advance to the given state iff it is in the set.
    Arrow(BitSet, StateId),
    /// Process no input and advance to both following states. The left state has higher priority
    /// when it comes to resolving ambiguous submatching.
    Split(StateId, StateId),
    /// Process no input and advance. If input is processed in this state, the corresponding set of
    /// next states is the empty set.
    Accepting,
    /// Save the current position as the given saved position. The index for the saved position must
    /// be less than `MAX_SAVE_LOCS`.
    Save(u8, StateId),
}

#[derive(Debug, Clone)]
/// A NFA whose alphabet is bytes
pub(super) struct Automaton {
    /// The initial state
    start: StateId,
    /// The states comprising the NFA
    states: Arena<State>,
}

impl Automaton {
    /// Constructs a new Automaton to decide the pattern whose AST is given.
    pub(super) fn new(pat: PatternNode<u8>) -> Self {
        let mut states = Arena::new();
        let accept = states.alloc(State::Accepting);
        let start = Self::new_from_pattern_in(&mut states, pat, accept);
        Self { start, states }
    }

    /// Produces the corresponding states to the AST `pat` in the given, already-allocated arena.
    /// The new state will have a destination state of `dest`.
    ///
    /// Returns the entry point to the sub-FSM which matches `pat`.
    fn new_from_pattern_in(
        arena: &mut Arena<State>,
        pat: PatternNode<u8>,
        dest: StateId,
    ) -> StateId {
        match pat {
            PatternNode::Literal(b) => arena.alloc(State::Arrow(BitSet::singelton(b), dest)),
            PatternNode::Class { negated, class } => {
                let bitset = if negated {
                    let mut bitset = BitSet::with_all_true();
                    for elem in class {
                        bitset.remove(elem);
                    }
                    bitset
                } else {
                    let mut bitset = BitSet::new();
                    for elem in class {
                        bitset.insert(elem);
                    }
                    bitset
                };
                arena.alloc(State::Arrow(bitset, dest))
            }
            PatternNode::Concat(x, y) => {
                let second = Self::new_from_pattern_in(arena, *y, dest);
                Self::new_from_pattern_in(arena, *x, second)
            }
            PatternNode::Or(x, y) => {
                let left = Self::new_from_pattern_in(arena, *x, dest);
                let right = Self::new_from_pattern_in(arena, *y, dest);
                arena.alloc(State::Split(left, right))
            }
            PatternNode::Repeat(x) => {
                let i = arena.alloc(/* just some valid state */ State::Accepting);
                let j = Self::new_from_pattern_in(arena, *x, i);
                // Prefer the repeating construct over dest
                *arena.get_mut(i).expect("just inserted this") = State::Split(j, dest);
                i
            }
            PatternNode::NonGreedyRepeat(x) => {
                let i = arena.alloc(/* just some valid state */ State::Accepting);
                let j = Self::new_from_pattern_in(arena, *x, i);
                // Prefer dest over the repeating construct
                *arena.get_mut(i).expect("just inserted this") = State::Split(dest, j);
                i
            }
            PatternNode::Save(x) => {
                let end = arena.alloc(State::Save(1, dest));
                let saved = Self::new_from_pattern_in(arena, *x, end);
                arena.alloc(State::Save(0, saved))
            }
            PatternNode::Wild => arena.alloc(State::Arrow(BitSet::with_all_true(), dest)),
            PatternNode::Empty => dest,
        }
    }

    /// Attempts to recognize the given byte string with the automaton.
    /// If more than one match could occur, ties are broken by favouring the leftmost-starting
    /// match, and the left side of any alternation. Repetition from a pattern is treated greedily
    /// by default.
    ///
    /// Accepts a prefix of the byte string iff `prefix_okay`.
    pub(super) fn is_match(&self, bs: &[u8], prefix_okay: bool) -> bool {
        let mut curr_states = vec![];
        self.add_state(self.start, &mut curr_states);
        let mut next_states = vec![];
        let mut accepted_at = None;

        // Add an extra character to advance final-round accepting states.
        // This character isn't meaningfully processed, as any states advanced to by reading it are
        // discarded.
        for (i, &b) in bs.iter().chain(std::iter::once(&b'\0')).enumerate() {
            for state in curr_states.drain(..) {
                match self.states.get(state) {
                    Some(&State::Accepting) => {
                        if prefix_okay {
                            return true;
                        } else {
                            accepted_at = Some(i);
                        }
                    }
                    Some(&State::Arrow(ref p, x)) => {
                        if p.get(b) {
                            self.add_state(x, &mut next_states)
                        }
                    }
                    _ => (),
                }
            }
            std::mem::swap(&mut curr_states, &mut next_states);
        }

        accepted_at.map(|i| i == bs.len()).unwrap_or(false)
    }

    /// Adds the next state following from `state` to `next_states`. This comprises following any
    /// non-predicate arrows, e.g., as part of a split.
    fn add_state(&self, state: StateId, next_states: &mut Vec<StateId>) {
        match self.states.get(state) {
            Some(&State::Split(left, right)) => {
                // The first thread added has the highest priority, so add `left`
                // after `right`.
                self.add_state(left, next_states);
                self.add_state(right, next_states);
            }
            Some(&State::Save(_, next)) => {
                self.add_state(next, next_states);
            }
            _ => {
                next_states.push(state);
            }
        }
    }

    /// Attempts to recognize the given byte string with the automaton.
    /// If more than one match could occur, ties are broken by favouring the leftmost-starting
    /// match, and the left side of any alternation. Repetition from a pattern is treated greedily
    /// by default.
    ///
    /// Accepts a prefix of the byte string iff `prefix_okay`.
    pub(super) fn is_match_with_location(
        &self,
        bs: &[u8],
        prefix_okay: bool,
    ) -> Option<(usize, usize)> {
        let mut curr_states = vec![];
        self.add_thread(
            Thread {
                state: self.start,
                saved_locs: [0; 2],
            },
            0,
            &mut curr_states,
        );
        let mut next_states = vec![];
        let mut longest_match = None;

        // Add an extra character to advance final-round accepting states.
        // This character isn't meaningfully processed, as any states advanced to by reading it are
        // discarded.
        for (i, &b) in bs.iter().chain(std::iter::once(&b'\0')).enumerate() {
            // Don't need to clear next_states because it was just curr_states, which we just
            // drained.
            for t in curr_states.drain(..) {
                match self.states.get(t.state) {
                    Some(&State::Accepting) => {
                        // Due to overwriting multiple accepting states with lastest one,
                        // we treat the _last_ thread added as higher priority.
                        longest_match = Some((t.saved_locs[0], t.saved_locs[1]));
                    }
                    Some(&State::Arrow(ref p, next)) => {
                        if p.get(b) {
                            self.add_thread(
                                Thread::new(next, t.saved_locs),
                                i + 1,
                                &mut next_states,
                            )
                        }
                    }
                    _ => unreachable!(
                        "All other states should be advanced though in `add_thread`\
                        ---only Accepting and Arrow possibly consume data."
                    ),
                }
            }
            std::mem::swap(&mut curr_states, &mut next_states);
        }

        longest_match.and_then(|m| {
            if !prefix_okay && m.1 != bs.len() {
                None
            } else {
                Some(m)
            }
        })
    }

    /// Adds the next state following from `state` to `next_states`. This comprises following any
    /// non-predicate arrows, e.g., as part of a split.
    fn add_thread(&self, mut thread: Thread, pos: usize, next_states: &mut Vec<Thread>) {
        match self.states.get(thread.state) {
            Some(&State::Split(left, right)) => {
                // The last thread added has the highest priority, so add `left`
                // after `right`.
                self.add_thread(Thread::new(right, thread.saved_locs), pos, next_states);
                self.add_thread(Thread::new(left, thread.saved_locs), pos, next_states);
            }
            Some(&State::Save(loc_idx, next)) => {
                thread.saved_locs[usize::from(loc_idx)] = pos;
                self.add_thread(Thread::new(next, thread.saved_locs), pos, next_states);
            }
            _ => {
                next_states.push(thread);
            }
        }
    }
}

impl From<PatternNode<u8>> for Automaton {
    fn from(pat: PatternNode<u8>) -> Self {
        Self::new(pat)
    }
}

// Currently just support whole expression matching.
const MAX_SAVE_LOCS: usize = 2;

#[derive(Debug, Clone)]
struct Thread {
    state: StateId,
    saved_locs: [usize; MAX_SAVE_LOCS],
}

impl Thread {
    fn new(state: StateId, saved_locs: [usize; MAX_SAVE_LOCS]) -> Self {
        Self { state, saved_locs }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chumsky::Parser;

    type Result<T> = std::result::Result<T, Vec<chumsky::error::Simple<u8>>>;

    fn automaton_from_str(s: &'_ str) -> Result<Automaton> {
        Ok(Automaton::new(
            super::super::parsing::pattern_parser()
                .parse(s.as_bytes())?
                .0,
        ))
    }

    #[test]
    fn match_single() -> Result<()> {
        let a = automaton_from_str("a")?;
        assert!(a.is_match(b"a", false));
        assert!(!a.is_match(b"b", false));
        assert!(!a.is_match(b"", false));
        Ok(())
    }

    #[test]
    fn match_simple_concat() -> Result<()> {
        let abc = automaton_from_str("abc")?;
        assert!(abc.is_match(b"abc", false));
        assert!(!abc.is_match(b"ab", false));
        assert!(!abc.is_match(b"bc", false));
        assert!(!abc.is_match(b"abcd", false));
        assert!(abc.is_match(b"abcd", true));
        Ok(())
    }

    #[test]
    fn match_locs() -> Result<()> {
        let a = automaton_from_str("aa?")?;
        assert_eq!(a.is_match_with_location(b"a", true), Some((0, 1)));
        assert_eq!(a.is_match_with_location(b"ab", true), Some((0, 1)));
        assert_eq!(a.is_match_with_location(b"ab", false), None);
        assert_eq!(a.is_match_with_location(b"aa", true), Some((0, 2)));
        assert_eq!(a.is_match_with_location(b"bab", true), Some((1, 2)));
        assert_eq!(a.is_match_with_location(b"baab", true), Some((1, 3)));
        assert_eq!(a.is_match_with_location(b"bbbab", true), Some((3, 4)));
        assert_eq!(a.is_match_with_location(b"bbbaab", true), Some((3, 5)));
        assert_eq!(a.is_match_with_location(b"b", false), None);
        assert_eq!(a.is_match_with_location(b"", false), None);
        Ok(())
    }
}
