//! Module for poker hand evaluation and comparison.

use crate::card::{Card, Rank, Suit};
use derivative::Derivative;

/// Represents different poker hand rankings with their respective cars.
/// Each variant contains the relevant cards that make up the hand.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PokerHand {
    HighCard(HighCard),
    OnePair(OnePair),
    TwoPair(TwoPair),
    ThreeOfAKind(ThreeOfAKind),
    Straight(Straight),
    Flush(Flush),
    FullHouse(FullHouse),
    FourOfAKind(FourOfAKind),
    StraightFlush(StraightFlush),
}

#[derive(Debug, Clone, Copy, Derivative)]
#[derivative(PartialEq, Eq, PartialOrd, Ord)]
struct Pair {
    rank: Rank,
    #[derivative(PartialEq = "ignore", PartialOrd = "ignore", Ord = "ignore")]
    suits: [Suit; 2],
}

#[derive(Debug, Clone, Copy, Derivative)]
#[derivative(PartialEq, Eq, PartialOrd, Ord)]
struct Triple {
    rank: Rank,
    #[derivative(PartialEq = "ignore", PartialOrd = "ignore", Ord = "ignore")]
    suits: [Suit; 3],
}

#[derive(Debug, Clone, Copy, Derivative)]
#[derivative(PartialEq, Eq, PartialOrd, Ord)]
pub struct HighCard {
    high_card: Card,
    kickers: [Card; 4],
}

#[derive(Debug, Clone, Copy, Derivative)]
#[derivative(PartialEq, Eq, PartialOrd, Ord)]
pub struct OnePair {
    pair: Pair,
    kickers: [Card; 3],
}

#[derive(Debug, Clone, Copy, Derivative)]
#[derivative(PartialEq, Eq, PartialOrd, Ord)]
pub struct TwoPair {
    first_pair: Pair,
    second_pair: Pair,
    kicker: Card,
}

#[derive(Debug, Clone, Copy, Derivative)]
#[derivative(PartialEq, Eq, PartialOrd, Ord)]
pub struct ThreeOfAKind {
    triple: Triple,
    kickers: [Card; 2],
}

#[derive(Debug, Clone, Copy, Derivative)]
#[derivative(PartialEq, Eq, PartialOrd, Ord)]
pub struct Straight {
    /// The highest rank in the straight (highest can be an Ace, lowest is a 5 for a wheel).
    high_card: Rank,
    #[derivative(PartialEq = "ignore", PartialOrd = "ignore", Ord = "ignore")]
    suits: [Suit; 5],
}

#[derive(Debug, Clone, Copy, Derivative)]
#[derivative(PartialEq, Eq, PartialOrd, Ord)]
pub struct Flush {
    #[derivative(PartialEq = "ignore", PartialOrd = "ignore", Ord = "ignore")]
    suit: Suit,
    ranks: [Rank; 5],
}

#[derive(Debug, Clone, Copy, Derivative)]
#[derivative(PartialEq, Eq, PartialOrd, Ord)]
pub struct FullHouse {
    triple: Triple,
    pair: Pair,
}

#[derive(Debug, Clone, Copy, Derivative)]
#[derivative(PartialEq, Eq, PartialOrd, Ord)]
pub struct FourOfAKind {
    quad: Rank,
    kicker: Card,
}

#[derive(Debug, Clone, Copy, Derivative)]
#[derivative(PartialEq, Eq, PartialOrd, Ord)]
pub struct StraightFlush {
    ranks: [Rank; 5],
    #[derivative(PartialEq = "ignore", PartialOrd = "ignore", Ord = "ignore")]
    suit: Suit,
}

impl PokerHand {
    /// Given 5 cards as input, creates a `PokerHand` with the correct ranking.
    pub fn solve(_cards: [Card; 5]) -> Self {
        todo!()
    }
}
