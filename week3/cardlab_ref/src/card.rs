/// Represents a standard playing card with a suit and a rank.
///
/// Each card consists of one of the four suits (Spade, Heart, Club, Diamond), as well as a rank
/// that can be either a number card (2-10) or a face card (Ace, King, Queen, Jack).
pub struct Card {
    suit: Suit,
    rank: Rank,
}

/// Represents the four possible suits in a standard deck of playing cards.
///
/// The suits are ordered in the traditional manner:
/// - Spades (highest)
/// - Hearts
/// - Clubs
/// - Diamonds (lowest)
enum Suit {
    /// The Spade suit, typically represented by a black ♠ symbol.
    Spade,
    /// The Heart suit, typically represented by a red ♥ symbol.
    Heart,
    /// The Club suit, typically represented by a black ♣ symbol.
    Club,
    /// The Diamond suit, typically represented by a red ♦ symbol.
    Diamond,
}

/// Represents the rank of a playing card, which can be either a number card (2-10) or a face card
/// (Ace, King, Queen, Jack).
enum Rank {
    /// A number card (2-10).
    Number(Number),
    /// A face card (Ace, King, Queen, Jack).
    Face(Face),
}

/// Represents the possible numbers for cards in a deck.
enum Number {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
}

/// Represents the face cards in a deck, including Ace.
///
/// Note that even though Ace could be considered a number card in some games, we're grouping it
/// with the face cards for this assignment.
enum Face {
    /// The Ace card, which often has special rules in many card games.
    Ace,
    /// The King card, traditionally representing a male royal figure.
    King,
    /// The Queen card, traditionally representing a female royal figure.
    Queen,
    /// The Jack card, traditionally representing a royal servant or knight.
    Jack,
}

impl Card {
    /// Creates a new Card instance from a suit name and numeric rank.
    ///
    /// # Parameters
    /// * `suit` - A string representing the suit:
    ///   * "spade"
    ///   * "heart"
    ///   * "club"
    ///   * "diamond"
    /// * `rank` - A number from 1-13 representing the rank:
    ///   * 1: Ace
    ///   * 2-10: Number cards
    ///   * 11: Jack
    ///   * 12: Queen
    ///   * 13: King
    ///
    /// # Panics
    ///
    /// * If suit is not one of: "spade", "heart", "club", "diamond"
    /// * If rank is not in range [1, 13]
    ///
    /// # Examples
    ///
    /// ```
    /// let ace_of_spades = Card::new("spade", 1);
    /// let two_of_hearts = Card::new("heart", 2);
    /// ```
    pub fn new(suit: u8, rank: u8) -> Self {
        // Convert the input `suit` integer into a `Suit` enum.
        // Note that this new `suit` "shadows" the input `suit`, becoming a new type.
        let suit = match suit {
            0 => Suit::Spade,
            1 => Suit::Heart,
            2 => Suit::Club,
            3 => Suit::Diamond,
            n => panic!("invalid suit number {n}, expected a suit in range [0, 3]"),
        };

        // Convert the input `rank` integer into a `Rank` enum
        let rank = match rank {
            1 => Rank::Face(Face::Ace),
            2 => Rank::Number(Number::Two),
            3 => Rank::Number(Number::Three),
            4 => Rank::Number(Number::Four),
            5 => Rank::Number(Number::Five),
            6 => Rank::Number(Number::Six),
            7 => Rank::Number(Number::Seven),
            8 => Rank::Number(Number::Eight),
            9 => Rank::Number(Number::Nine),
            10 => Rank::Number(Number::Ten),
            11 => Rank::Face(Face::Jack),
            12 => Rank::Face(Face::Queen),
            13 => Rank::Face(Face::King),
            n => panic!("invalid rank number {n}, expected a rank in range [1, 13]"),
        };

        // Create and return the new Card
        Card { suit, rank }
    }
}
