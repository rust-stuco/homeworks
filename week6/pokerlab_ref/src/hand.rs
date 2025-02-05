//! Module for poker hand evaluation and comparison.

use crate::card::{Card, Face, Number, Rank, Suit};
use derivative::Derivative;

/// Represents a standard `Hand` of 5 playing [`Card`]s.
///
/// Note that the cards will always be stored in sorted descending order (only by [`Rank`]).
#[derive(Debug)]
pub struct Hand {
    /// The cards in the hand.
    cards: [Card; 5],
}

pub struct UniqueError;

impl Hand {
    /// Creates a new `Hand` of 5 [`Card`]s.
    ///
    /// Stores the cards in reverse (descending) sorted order.
    ///
    /// Returns an error if any cards are duplicates.
    pub fn new(mut cards: [Card; 5]) -> Result<Self, UniqueError> {
        // Sort in reverse order.
        cards.sort_by(|a, b| b.cmp(a));

        // Check for duplicate cards.
        for i in 0..4 {
            if cards[i] == cards[i + 1] && cards[i].rank() == cards[i + 1].rank() {
                return Err(UniqueError);
            }
        }

        Ok(Self { cards })
    }

    /// Checks if all cards in the hand have the same suit.
    pub fn is_flush(&self) -> bool {
        let first_suit = self.cards[0].suit();

        for i in 1..5 {
            if self.cards[i].suit() != first_suit {
                return false;
            }
        }

        true
    }

    /// Checks if the cards form a straight (consecutive ranks).
    pub fn is_straight(&self) -> bool {
        // Check for standard straight.
        let mut is_sequential = true;
        for i in 0..4 {
            if self.cards[i].rank().as_u8() != self.cards[i + 1].rank().as_u8() + 1 {
                is_sequential = false;
                break;
            }
        }

        if is_sequential {
            return true;
        }

        // Do a final check for a wheel / Ace-low straight (A, 2, 3, 4, 5).
        self.cards[0].rank() == Rank::Face(Face::Ace)
            && self.cards[1].rank() == Rank::Number(Number::Five)
            && self.cards[2].rank() == Rank::Number(Number::Four)
            && self.cards[3].rank() == Rank::Number(Number::Three)
            && self.cards[4].rank() == Rank::Number(Number::Two)
    }

    /// Checks if the hand contains four cards of the same rank
    pub fn is_four_of_a_kind(&self) -> bool {
        // Check that the first four cards or the last four cards are all equal (relies on order).
        (self.cards[0].rank() == self.cards[3].rank())
            || (self.cards[1].rank() == self.cards[4].rank())
    }

    /// Checks if the hand contains three cards of the same rank.
    pub fn has_triple(&self) -> bool {
        // Check each possible position for three matching cards (relies on order).
        (self.cards[0].rank() == self.cards[2].rank())
            || (self.cards[1].rank() == self.cards[3].rank())
            || (self.cards[2].rank() == self.cards[4].rank())
    }

    /// Checks if the hand contains at least one pair.
    pub fn has_pair(&self) -> bool {
        for i in 0..4 {
            if self.cards[i].rank() == self.cards[i + 1].rank() {
                return true;
            }
        }
        false
    }
}

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

impl PokerHand {
    /// Given 5 cards as input, creates a `PokerHand` with the correct ranking.
    /// 
    /// Note that there is definitely a cleaner way to implement Poker hands, but since 
    pub fn solve(hand: Hand) -> Self {
        let cards = hand.cards;

        todo!("Implement me!")
    }
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
