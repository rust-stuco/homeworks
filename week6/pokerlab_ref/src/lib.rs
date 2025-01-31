#![doc = include_str!("../README.md")]

mod card;
pub use card::Card;

mod deck;
pub use deck::Deck;

mod poker_hand;
pub use poker_hand::PokerHand;

#[cfg(test)]
mod tests;
