use crate::Card;
use rand::seq::SliceRandom;

/// The number of cards in a standard playing card deck.
const DECK_SIZE: usize = 52;

/// A deck of [`DECK_SIZE`] (52) playing [`Card`]s.
pub struct Deck {
    /// The cards of the deck. We treat the beginning of the vector as the bottom of the deck and
    /// the end of the vector as the top in order to support fast removal from the top of the deck.
    cards: Vec<Card>,
}

impl Deck {
    /// Creates a new, sorted deck of [`DECK_SIZE`] (52) playing cards.
    pub fn new() -> Self {
        let mut cards = Vec::with_capacity(DECK_SIZE);

        // For each suit, create cards with ranks 2-14.
        for suit in ["diamond", "club", "heart", "spade"] {
            for rank in 2..=14 {
                cards.push(Card::new(suit, rank));
            }
        }
        
        Self { cards }
    }

    /// Shuffle the deck of cards.
    pub fn shuffle(&mut self) {
        self.cards.shuffle(&mut rand::thread_rng());
    }

    /// Returns a reference to the top card of the deck without removing it.
    pub fn peek(&self) -> Option<&Card> {
        self.cards.last()
    }
    
    /// Takes and returns the top card from the deck.
    pub fn take_top_card(&mut self) -> Option<Card> {
        self.cards.pop()
    }
}

impl Default for Deck {
    fn default() -> Self {
        Self::new()
    }
}
