//! This module contains the definition and implementation of [`Card`], which represents a card in
//! a standard deck of 52 cards. [`Card`]s are uniquely identified by their rank (which can be the
//! numbers 2-10 or Jack, Queen, King, or Ace) and their suit (which can be Diamond, Club, Heart,
//! or Spade).
//!
//! Note to students: You can copy and paste your [`Card`] implementation from Card Lab here!

/// Represents a standard playing card with a suit and a rank.
///
/// Each card consists of one of the four suits (Diamond, Club, Heart, Spade), as well as a rank
/// that can be either a number card (2-10) or a face card (Jack, Queen, King, Ace).
///
/// This type implements the traits [`PartialEq`], [`Eq`], [`PartialOrd`], and [`Ord`].
///
/// # Examples
///
/// ```
/// # use pokerlab_ref::card::{Card, Suit, Rank, Face, Number};
/// #
/// let ace_spades = Card::new(Suit::Spade, Rank::Face(Face::Ace));
/// let king_spades = Card::new(Suit::Spade, Rank::Face(Face::King));
/// let ten_spades = Card::new(Suit::Spade, Rank::Number(Number::Ten));
/// let five_spades = Card::new(Suit::Spade, Rank::Number(Number::Five));
/// let ace_hearts = Card::new(Suit::Heart, Rank::Face(Face::Ace));
/// let ace_clubs = Card::new(Suit::Club, Rank::Face(Face::Ace));
///
/// // Compare cards of same suit but different rank.
/// assert!(ace_spades > king_spades);
/// assert!(king_spades > ten_spades);
/// assert!(ten_spades > five_spades);
///
/// // Compare cards of same rank but different suit.
/// assert!(ace_spades > ace_hearts);
/// assert!(ace_hearts > ace_clubs);
///
/// // Test equality between identical cards.
/// assert!(ace_spades == Card::new(Suit::Spade, Rank::Face(Face::Ace)));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Card {
    rank: Rank,
    suit: Suit,
}

impl Card {
    /// Creates a new Card instance from a suit name and numeric rank.
    ///
    /// # Parameters
    ///
    /// * `suit` - A string representing the suit:
    ///   * `"diamond"`
    ///   * `"club"`
    ///   * `"heart"`
    ///   * `"spade"`
    /// * `rank` - A number from 1-13 representing the rank:
    ///   * 2-10: Number cards
    ///   * 11: Jack
    ///   * 12: Queen
    ///   * 13: King
    ///   * 14: Ace
    ///
    /// # Panics
    ///
    /// * If suit is not one of: `"spade"`, `"heart"`, `"club"`, `"diamond"`.
    /// * If rank is not in the range [2, 14].
    ///
    /// # Examples
    ///
    /// ```
    /// # use pokerlab_ref::card::{Card, Suit, Rank, Face, Number};
    /// #
    /// let ace_of_spades = Card::new(Suit::Spade, Rank::Face(Face::Ace));
    /// let two_of_hearts = Card::new(Suit::Heart, Rank::Number(Number::Two));
    /// let three_of_clubs = Card::new(Suit::Club, Rank::Number(Number::Three));
    /// ```
    pub fn new(suit: Suit, rank: Rank) -> Self {
        Card { suit, rank }
    }

    /// Returns a string representation of this card's suit.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pokerlab_ref::card::{Card, Suit, Rank, Number};
    /// #
    /// let card = Card::new(Suit::Club, Rank::Number(Number::Three));
    /// assert_eq!(&Suit::Club, card.suit());
    /// ```
    pub fn suit(&self) -> &Suit {
        &self.suit
    }

    /// Returns the numeric rank of this card, where number cards = 2-10, and Jack = 11, Queen = 12,
    /// King = 13, and Ace = 14.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pokerlab_ref::card::{Card, Suit, Rank, Number};
    /// #
    /// let card = Card::new(Suit::Club, Rank::Number(Number::Three));
    /// assert_eq!(&Rank::Number(Number::Three), card.rank());
    /// ```
    pub fn rank(&self) -> &Rank {
        &self.rank
    }
}

/// Represents the four possible suits in a standard deck of playing cards.
///
/// The suits are ordered in the traditional manner:
/// - Diamonds (lowest)
/// - Clubs
/// - Hearts
/// - Spades (highest)
///
/// # Examples
///
/// ```
/// # use pokerlab_ref::card::{Card, Suit, Rank, Number};
/// #
/// let spade_card = Card::new(Suit::Spade, Rank::Number(Number::Two));
/// let diamond_card = Card::new(Suit::Diamond, Rank::Number(Number::Two));
/// assert!(spade_card > diamond_card);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Suit {
    /// The Diamond suit, typically represented by a red ♦ symbol.
    Diamond,
    /// The Club suit, typically represented by a black ♣ symbol.
    Club,
    /// The Heart suit, typically represented by a red ♥ symbol.
    Heart,
    /// The Spade suit, typically represented by a black ♠ symbol.
    Spade,
}

/// Represents the rank of a playing card, which can be either a number card (2-10) or a face card
/// (Ace, King, Queen, Jack).
///
/// # Examples
///
/// ```
/// # use pokerlab_ref::card::{Card, Suit, Rank, Face, Number};
/// #
/// let five_spades = Card::new(Suit::Spade, Rank::Number(Number::Five));
/// let seven_spades = Card::new(Suit::Spade, Rank::Number(Number::Seven));
/// assert!(seven_spades > five_spades);
///
/// let queen_hearts = Card::new(Suit::Heart, Rank::Face(Face::Queen));
/// let seven_clubs = Card::new(Suit::Club, Rank::Number(Number::Seven));
/// assert!(queen_hearts > seven_clubs);
///
/// let king_diamonds = Card::new(Suit::Diamond, Rank::Face(Face::King));
/// let ace_clubs = Card::new(Suit::Club, Rank::Face(Face::Ace));
/// assert!(ace_clubs > king_diamonds);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Rank {
    /// A number card (2-10).
    Number(Number),
    /// A face card (Ace, King, Queen, Jack).
    Face(Face),
}

impl Rank {
    pub fn as_u8(&self) -> u8 {
        match self {
            Rank::Number(number) => *number as u8,
            Rank::Face(face) => *face as u8,
        }
    }
}

/// Represents the possible numbers for cards in a deck.
///
/// # Examples
///
/// ```
/// # use pokerlab_ref::card::{Card, Suit, Rank, Number};
/// #
/// let two_hearts = Card::new(Suit::Heart, Rank::Number(Number::Two));
/// let seven_diamonds = Card::new(Suit::Diamond, Rank::Number(Number::Seven));
/// assert!(two_hearts < seven_diamonds);
/// ```
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Number {
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
}

/// Represents the face cards in a deck, including Ace.
///
/// Note that even though Ace could be considered a number card in some games, we're grouping it
/// with the face cards for this assignment.
///
/// # Examples
///
/// ```
/// # use pokerlab_ref::card::{Card, Suit, Rank, Face};
/// #
/// let queen_hearts = Card::new(Suit::Heart, Rank::Face(Face::Queen));
/// let jack_clubs = Card::new(Suit::Club, Rank::Face(Face::Jack));
/// assert!(queen_hearts > jack_clubs);
///
/// let ace_spades = Card::new(Suit::Spade, Rank::Face(Face::Ace));
/// let king_diamonds = Card::new(Suit::Diamond, Rank::Face(Face::King));
/// assert!(ace_spades > king_diamonds);
/// ```
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Face {
    /// The Jack card, traditionally representing a royal servant or knight.
    Jack = 11,
    /// The Queen card, traditionally representing a female royal figure.
    Queen = 12,
    /// The King card, traditionally representing a male royal figure.
    King = 13,
    /// The Ace card, which often has special rules in many card games.
    Ace = 14,
}
