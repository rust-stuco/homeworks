use crate::card::Card;

/// The size of a typical poker combination.
const POKER_COMBINATION_SIZE: usize = 5;

/// Represents a 5-card poker hand.
pub struct PokerHand {
    cards: [Card; POKER_COMBINATION_SIZE],
}

impl PokerHand {
    /// Creates a new `Hand` from 5 cards.
    pub fn new(mut cards: [Card; POKER_COMBINATION_SIZE]) -> Self {
        // Sorts the cards. Note that you technically don't need to do this, but it will be more
        // useful in a future homework.
        cards.sort();

        // Check for duplicates (equal cards appearing next to each other in the sorted hand)
        for i in 1.. {
            if cards[i - 1] == cards[i] {
                panic!("duplicate card found in hand");
            }
        }

        PokerHand { cards }
    }

    /// Returns the highest card in the hand based on rank and suit.
    pub fn highest_card(&self) -> &Card {
        let mut highest = &self.cards[0];

        for card in &self.cards[1..] {
            if card > highest {
                highest = card;
            }
        }

        highest
    }
}
