#![doc = include_str!("../README.md")]

mod card;
pub use card::Card;

mod poker_hand;
pub use poker_hand::PokerHand;

#[cfg(test)]
mod tests;
