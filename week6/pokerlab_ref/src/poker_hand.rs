//! TODO Add docs.

use crate::card::Card;

/// TODO docs.
/// Should allow students to modify this how they wish.
pub enum PokerHand {
    HighCard {
        high_card: Card,
        kickers: [Card; 4],
    },
    OnePair {
        pair: (Card, Card),
        kickers: [Card; 3],
    },
    TwoPair {
        first_pair: (Card, Card),
        second_pair: (Card, Card),
    },
    ThreeOfAKind {
        triplet: (Card, Card, Card),
        kickers: [Card; 2],
    },
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
}

impl PokerHand {
    /// Given 5 cards as input, creates a `PokerHand` with the correct ranking.
    pub fn solve(_cards: [Card; 5]) -> Self {
        todo!("implement me")
    }
}

impl PartialEq for PokerHand {
    fn eq(&self, _other: &Self) -> bool {
        todo!("implement me")
    }
}

impl Eq for PokerHand {}

/// Provides partial ordering for a `PokerHand` by delegating to the total ordering implementation.
impl PartialOrd for PokerHand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PokerHand {
    fn cmp(&self, _other: &Self) -> std::cmp::Ordering {
        todo!()
    }
}
