//! This module contains the definition and implementation of the [`Deck`] type.

use crate::card::{Card, Face, Number, Rank, Suit};
use rand::seq::SliceRandom;

/// The number of cards in a standard playing card deck.
pub const DECK_SIZE: usize = 52;

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

        // For each suit, create every card rank.
        for suit in [Suit::Diamond, Suit::Club, Suit::Heart, Suit::Spade] {
            // Note that you can avoid a lot of this boilerplate code by using an external crate:
            // https://docs.rs/enum-iterator/latest/enum_iterator/
            for number in [
                Number::Two,
                Number::Three,
                Number::Four,
                Number::Five,
                Number::Six,
                Number::Seven,
                Number::Eight,
                Number::Nine,
                Number::Ten,
            ] {
                cards.push(Card::new(suit, Rank::Number(number)));
            }
            for face in [Face::Jack, Face::Queen, Face::King, Face::Ace] {
                cards.push(Card::new(suit, Rank::Face(face)));
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
