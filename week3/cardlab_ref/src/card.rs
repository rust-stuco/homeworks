/// Represents a standard playing card with a suit and a rank.
///
/// Each card consists of one of the four suits (Diamond, Club, Heart, Spade), as well as a rank
/// that can be either a number card (2-10) or a face card (Ace, King, Queen, Jack).
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Card {
    suit: Suit,
    rank: Rank,
}

/// Represents the four possible suits in a standard deck of playing cards.
///
/// The suits are ordered in the traditional manner:
/// - Diamonds (lowest)
/// - Clubs
/// - Hearts
/// - Spades (highest)
#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Suit {
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
#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Rank {
    /// A number card (2-10).
    Number(Number),
    /// A face card (Ace, King, Queen, Jack).
    Face(Face),
}

/// Represents the possible numbers for cards in a deck.
#[derive(PartialEq, Eq, PartialOrd, Ord)]
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
#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Face {
    /// The Jack card, traditionally representing a royal servant or knight.
    Jack,
    /// The Queen card, traditionally representing a female royal figure.
    Queen,
    /// The King card, traditionally representing a male royal figure.
    King,
    /// The Ace card, which often has special rules in many card games.
    Ace,
}

impl Card {
    /// Creates a new Card instance from a suit name and numeric rank.
    ///
    /// # Parameters
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
    /// * If rank is not in range [2, 14].
    ///
    /// # Examples
    ///
    /// ```
    /// # use cardlab_ref::card::Card;
    /// #
    /// let ace_of_spades = Card::new("spade", 14);
    /// let two_of_hearts = Card::new("heart", 2);
    /// let three_of_clubs = Card::new("club", 3);
    /// ```
    pub fn new(suit: &str, rank: u8) -> Self {
        // Convert the input `suit` integer into a `Suit` enum.
        // Note that this new `suit` "shadows" the input `suit`, becoming a new type.
        let suit = match suit {
            "diamond" => Suit::Diamond,
            "club" => Suit::Club,
            "heart" => Suit::Heart,
            "spade" => Suit::Spade,
            s => panic!("invalid suit {s}"),
        };

        // Convert the input `rank` integer into a `Rank` enum.
        let rank = match rank {
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
            14 => Rank::Face(Face::Ace),
            n => panic!("invalid rank number {n}, expected a rank in range [2, 14]"),
        };

        Card { suit, rank }
    }
    
    /// Returns a string representation of this card's suit.
    ///
    /// # Examples
    ///
    /// ```
    /// # use cardlab_ref::card::Card;
    /// #
    /// let card = Card::new("club", 3);
    /// assert_eq!(card.suit_name(), "club");
    /// ```
    pub fn suit_name(&self) -> &'static str {
        match self.suit {
            Suit::Diamond => "diamond",
            Suit::Club => "club", 
            Suit::Heart => "heart",
            Suit::Spade => "spade",
        }
    }

    /// Returns the numeric rank of this card, where Ace=1, number cards=2-10, and Jack=11, Queen=12, King=13.
    ///
    /// # Examples
    ///
    /// ```
    /// # use cardlab_ref::card::Card;
    /// #
    /// let card = Card::new("club", 3);
    /// assert_eq!(card.rank_number(), 3);
    /// ```
    pub fn rank_number(&self) -> u8 {
        match &self.rank {
            Rank::Number(Number::Two) => 2,
            Rank::Number(Number::Three) => 3,
            Rank::Number(Number::Four) => 4,
            Rank::Number(Number::Five) => 5,
            Rank::Number(Number::Six) => 6,
            Rank::Number(Number::Seven) => 7,
            Rank::Number(Number::Eight) => 8,
            Rank::Number(Number::Nine) => 9,
            Rank::Number(Number::Ten) => 10,
            Rank::Face(Face::Jack) => 11,
            Rank::Face(Face::Queen) => 12,
            Rank::Face(Face::King) => 13,
            Rank::Face(Face::Ace) => 14,
        }
    }
}
