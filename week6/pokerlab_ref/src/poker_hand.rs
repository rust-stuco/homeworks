//! TODO Add docs.

use crate::card::Card;

/// TODO docs.
pub enum PokerHand {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
}

impl PokerHand {
    pub fn new(_cards: [Card; 5]) -> Self {
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
