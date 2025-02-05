//! Module for poker hand evaluation and comparison.

use crate::card::{Card, Rank, Suit};
use derivative::Derivative;

/// Represents different poker hand rankings with their respective cards.
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

/// Represents a pair of cards of the same rank.
#[derive(Debug, Clone, Copy, Derivative)]
#[derivative(PartialEq, Eq, PartialOrd, Ord)]
struct Pair {
    /// The rank shared by both cards in the pair.
    rank: Rank,
    /// The suits of the two cards in the pair.
    #[derivative(PartialEq = "ignore", PartialOrd = "ignore", Ord = "ignore")]
    suits: [Suit; 2],
}

/// Represents three cards of the same rank.
#[derive(Debug, Clone, Copy, Derivative)]
#[derivative(PartialEq, Eq, PartialOrd, Ord)]
struct Triple {
    /// The rank shared by all three cards.
    rank: Rank,
    /// The suits of the three cards.
    #[derivative(PartialEq = "ignore", PartialOrd = "ignore", Ord = "ignore")]
    suits: [Suit; 3],
}

/// Represents a high card hand, consisting of five unmatched cards.
#[derive(Debug, Clone, Copy, Derivative)]
#[derivative(PartialEq, Eq, PartialOrd, Ord)]
pub struct HighCard {
    /// The highest card in the hand.
    high_card: Card,
    /// The remaining four cards in descending order.
    kickers: [Card; 4],
}

/// Represents a hand containing one pair and three kickers.
#[derive(Debug, Clone, Copy, Derivative)]
#[derivative(PartialEq, Eq, PartialOrd, Ord)]
pub struct OnePair {
    /// The matched pair of cards.
    pair: Pair,
    /// The remaining three cards in descending order.
    kickers: [Card; 3],
}

/// Represents a hand containing two pairs and one kicker.
#[derive(Debug, Clone, Copy, Derivative)]
#[derivative(PartialEq, Eq, PartialOrd, Ord)]
pub struct TwoPair {
    /// The higher ranked pair.
    first_pair: Pair,
    /// The lower ranked pair.
    second_pair: Pair,
    /// The remaining unpaired card.
    kicker: Card,
}

/// Represents a hand containing three cards of the same rank and two kickers.
#[derive(Debug, Clone, Copy, Derivative)]
#[derivative(PartialEq, Eq, PartialOrd, Ord)]
pub struct ThreeOfAKind {
    /// The three matched cards.
    triple: Triple,
    /// The remaining two cards in descending order.
    kickers: [Card; 2],
}

/// Represents five consecutive cards of different suits.
#[derive(Debug, Clone, Copy, Derivative)]
#[derivative(PartialEq, Eq, PartialOrd, Ord)]
pub struct Straight {
    /// The highest rank in the straight (highest can be an Ace, lowest is a 5 for a wheel).
    high_card: Rank,
    /// The suits of the five cards in the straight.
    #[derivative(PartialEq = "ignore", PartialOrd = "ignore", Ord = "ignore")]
    suits: [Suit; 5],
}

/// Represents five cards of the same suit.
#[derive(Debug, Clone, Copy, Derivative)]
#[derivative(PartialEq, Eq, PartialOrd, Ord)]
pub struct Flush {
    /// The suit shared by all five cards.
    #[derivative(PartialEq = "ignore", PartialOrd = "ignore", Ord = "ignore")]
    suit: Suit,
    /// The ranks of the five cards in descending order.
    ranks: [Rank; 5],
}

/// Represents a hand containing three cards of one rank and two cards of another rank.
#[derive(Debug, Clone, Copy, Derivative)]
#[derivative(PartialEq, Eq, PartialOrd, Ord)]
pub struct FullHouse {
    /// The three matched cards.
    triple: Triple,
    /// The two matched cards.
    pair: Pair,
}

/// Represents a hand containing four cards of the same rank and one kicker.
#[derive(Debug, Clone, Copy, Derivative)]
#[derivative(PartialEq, Eq, PartialOrd, Ord)]
pub struct FourOfAKind {
    /// The rank shared by all four cards.
    quad: Rank,
    /// The remaining unpaired card.
    kicker: Card,
}

/// Represents five consecutive cards of the same suit.
#[derive(Debug, Clone, Copy, Derivative)]
#[derivative(PartialEq, Eq, PartialOrd, Ord)]
pub struct StraightFlush {
    /// The ranks of the five cards in descending order.
    ranks: [Rank; 5],
    /// The suit shared by all five cards.
    #[derivative(PartialEq = "ignore", PartialOrd = "ignore", Ord = "ignore")]
    suit: Suit,
}

impl PokerHand {
    /// Given 5 cards as input, creates a `PokerHand` with the correct ranking.
    pub fn solve(mut cards: [Card; 5]) -> Self {
        cards.sort();

        todo!()
    }
}
