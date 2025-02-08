//! Module for poker hand evaluation and comparison.

use crate::card::{Card, Face, Number, Rank, Suit};
use derivative::Derivative;

/// Represents a high card hand, consisting of five unmatched cards.
#[derive(Debug, Clone, Copy, Derivative)]
#[derivative(PartialEq, Eq, PartialOrd, Ord)]
pub struct HighCard {
    /// The highest card in the hand.
    high_card: Card,
    /// The remaining four cards in descending order.
    kickers: [Card; 4],
}

/// Represents a hand containing one pair and three kickers.
#[derive(Debug, Clone, Copy, Derivative)]
#[derivative(PartialEq, Eq, PartialOrd, Ord)]
pub struct OnePair {
    /// The matched pair of cards.
    pair: Pair,
    /// The remaining three cards in descending order.
    kickers: [Card; 3],
}

/// Represents a hand containing two pairs and one kicker.
#[derive(Debug, Clone, Copy, Derivative)]
#[derivative(PartialEq, Eq, PartialOrd, Ord)]
pub struct TwoPair {
    /// The higher ranked pair.
    first_pair: Pair,
    /// The lower ranked pair.
    second_pair: Pair,
    /// The remaining unpaired card.
    kicker: Card,
}

/// Represents a hand containing three cards of the same rank and two kickers.
#[derive(Debug, Clone, Copy, Derivative)]
#[derivative(PartialEq, Eq, PartialOrd, Ord)]
pub struct ThreeOfAKind {
    /// The three matched cards.
    triple: Triple,
    /// The remaining two cards in descending order.
    kickers: [Card; 2],
}

/// Represents five consecutive cards of different suits.
#[derive(Debug, Clone, Copy, Derivative)]
#[derivative(PartialEq, Eq, PartialOrd, Ord)]
pub struct Straight {
    /// The highest rank in the straight (highest can be an Ace, lowest is a 5 for a wheel).
    high_card: Rank,
    /// The suits of the five cards in the straight.
    #[derivative(PartialEq = "ignore", PartialOrd = "ignore", Ord = "ignore")]
    suits: [Suit; 5],
}

/// Represents five cards of the same suit.
#[derive(Debug, Clone, Copy, Derivative)]
#[derivative(PartialEq, Eq, PartialOrd, Ord)]
pub struct Flush {
    /// The suit shared by all five cards.
    #[derivative(PartialEq = "ignore", PartialOrd = "ignore", Ord = "ignore")]
    suit: Suit,
    /// The ranks of the five cards in descending order.
    ranks: [Rank; 5],
}

/// Represents a hand containing three cards of one rank and two cards of another rank.
#[derive(Debug, Clone, Copy, Derivative)]
#[derivative(PartialEq, Eq, PartialOrd, Ord)]
pub struct FullHouse {
    /// The three matched cards.
    triple: Triple,
    /// The two matched cards.
    pair: Pair,
}

/// Represents a hand containing four cards of the same rank and one kicker.
#[derive(Debug, Clone, Copy, Derivative)]
#[derivative(PartialEq, Eq, PartialOrd, Ord)]
pub struct FourOfAKind {
    /// The rank shared by all four cards.
    quad: Rank,
    /// The remaining unpaired card.
    kicker: Card,
}

/// Represents five consecutive cards of the same suit.
#[derive(Debug, Clone, Copy, Derivative)]
#[derivative(PartialEq, Eq, PartialOrd, Ord)]
pub struct StraightFlush {
    /// The highest rank in the straight (highest can be an Ace, lowest is a 5 for a wheel).
    high_card: Rank,
    /// The suit shared by all five cards.
    #[derivative(PartialEq = "ignore", PartialOrd = "ignore", Ord = "ignore")]
    suit: Suit,
}

/// Represents a pair of cards of the same rank.
///
/// Note that this is different from [`OnePair`], which also includes kicker cards.
#[derive(Debug, Clone, Copy, Derivative)]
#[derivative(PartialEq, Eq, PartialOrd, Ord)]
struct Pair {
    /// The rank shared by both cards in a pair.
    rank: Rank,
    /// The suits of the two cards in a pair.
    #[derivative(PartialEq = "ignore", PartialOrd = "ignore", Ord = "ignore")]
    suits: [Suit; 2],
}

/// Represents three cards of the same rank.
///
/// Note that this is different from [`ThreeOfAKind`], which also includes kicker cards.
#[derive(Debug, Clone, Copy, Derivative)]
#[derivative(PartialEq, Eq, PartialOrd, Ord)]
struct Triple {
    /// The rank shared by all three cards.
    rank: Rank,
    /// The suits of the three cards.
    #[derivative(PartialEq = "ignore", PartialOrd = "ignore", Ord = "ignore")]
    suits: [Suit; 3],
}

/// Represents different poker hand rankings with their respective cards.
///
/// Each variant contains the relevant cards that make up the hand.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PokerHand {
    HighCard(HighCard),
    OnePair(OnePair),
    TwoPair(TwoPair),
    ThreeOfAKind(ThreeOfAKind),
    Straight(Straight),
    Flush(Flush),
    FullHouse(FullHouse),
    FourOfAKind(FourOfAKind),
    StraightFlush(StraightFlush),
}

impl PokerHand {
    /// Given 5 cards as input, creates a `PokerHand` with the correct ranking.
    ///
    /// Note that there is definitely a cleaner way to implement Poker hands, but since
    pub fn solve(hand: Hand) -> Self {
        // Check for a straight flush first (highest ranking hand).
        if let Some(straight_flush) = hand.get_straight_flush() {
            return PokerHand::StraightFlush(straight_flush);
        }

        // Check for a four of a kind.
        if let Some(four_of_a_kind) = hand.get_four_of_a_kind() {
            return PokerHand::FourOfAKind(four_of_a_kind);
        }

        // Check for a full house.
        if let Some(full_house) = hand.get_full_house() {
            return PokerHand::FullHouse(full_house);
        }

        // Check for a flush.
        if let Some(flush) = hand.get_flush() {
            return PokerHand::Flush(flush);
        }

        // Check for a straight.
        if let Some(straight) = hand.get_straight() {
            return PokerHand::Straight(straight);
        }

        // Check for a three of a kind.
        if let Some(three_of_a_kind) = hand.get_triple() {
            return PokerHand::ThreeOfAKind(three_of_a_kind);
        }

        // Check for a two pair.
        if let Some(two_pair) = hand.get_two_pair() {
            return PokerHand::TwoPair(two_pair);
        }

        // Check for a one pair.
        if let Some(one_pair) = hand.get_one_pair() {
            return PokerHand::OnePair(one_pair);
        }

        // If no other hand is found, it's a high card hand.
        PokerHand::HighCard(HighCard {
            high_card: hand.cards[0],
            kickers: [hand.cards[1], hand.cards[2], hand.cards[3], hand.cards[4]],
        })
    }
}

/// Represents a standard `Hand` of 5 playing [`Card`]s.
///
/// Note that the cards will always be stored in sorted descending order (only by [`Rank`]).
#[derive(Debug)]
pub struct Hand {
    /// The cards in the hand.
    cards: [Card; 5],
}

/// TODO docs.
pub struct UniqueError;

impl Hand {
    /// Creates a new `Hand` of 5 [`Card`]s.
    ///
    /// Stores the cards in reverse (descending) sorted order.
    ///
    /// Returns an error if any cards are duplicates.
    pub fn new(mut cards: [Card; 5]) -> Result<Self, UniqueError> {
        // Sort in reverse order.
        cards.sort_by(|a, b| b.cmp(a));

        // Check for any duplicate cards.
        for i in 0..4 {
            if cards[i] == cards[i + 1] && cards[i].rank() == cards[i + 1].rank() {
                return Err(UniqueError);
            }
        }

        Ok(Self { cards })
    }

    /// Returns a [`StraightFlush`] if the hand contains consecutive cards of the same suit,
    /// otherwise returns `None`.
    ///
    /// This function assumes that all higher-ranking hand checks have been called.
    pub fn get_straight_flush(&self) -> Option<StraightFlush> {
        // Check if we have both a straight and a flush.
        if let Some(straight) = self.get_straight() {
            if let Some(flush) = self.get_flush() {
                return Some(StraightFlush {
                    high_card: straight.high_card,
                    suit: flush.suit,
                });
            }
        }
        None
    }

    /// Returns a [`FourOfAKind`] if the hand contains four cards of the same rank, otherwise
    /// returns `None`.
    ///
    /// This function assumes that all higher-ranking hand checks have been called.
    pub fn get_four_of_a_kind(&self) -> Option<FourOfAKind> {
        // Check if the first four cards match.
        if self.cards[0].rank() == self.cards[3].rank() {
            return Some(FourOfAKind {
                quad: self.cards[0].rank(),
                kicker: self.cards[4],
            });
        }

        // Check if the last four cards match.
        if self.cards[1].rank() == self.cards[4].rank() {
            return Some(FourOfAKind {
                quad: self.cards[1].rank(),
                kicker: self.cards[0],
            });
        }

        None
    }

    /// Returns a [`FullHouse`] if the hand contains a triple and a pair, otherwise returns `None`.
    ///
    /// This function assumes that all higher-ranking hand checks have been called.
    pub fn get_full_house(&self) -> Option<FullHouse> {
        // First check if have a triple.
        if let Some(ThreeOfAKind { triple, kickers }) = self.get_triple() {
            // Check if the kickers form a pair.
            if kickers[0].rank() == kickers[1].rank() {
                return Some(FullHouse {
                    triple,
                    pair: Pair {
                        rank: kickers[0].rank(),
                        suits: [kickers[0].suit(), kickers[1].suit()],
                    },
                });
            }
        }

        None
    }

    /// Returns a [`Flush`] if all cards in the hand have the same suit, otherwise returns `None`.
    ///
    /// This function assumes that all higher-ranking hand checks have been called.
    pub fn get_flush(&self) -> Option<Flush> {
        let first_suit = self.cards[0].suit();

        for i in 1..5 {
            if self.cards[i].suit() != first_suit {
                return None;
            }
        }

        Some(Flush {
            suit: first_suit,
            ranks: [
                self.cards[0].rank(),
                self.cards[1].rank(),
                self.cards[2].rank(),
                self.cards[3].rank(),
                self.cards[4].rank(),
            ],
        })
    }

    /// Returns a [`Straight`] if the cards form a straight (consecutive ranks), otherwise returns
    /// `None`.
    ///
    /// This function can also handle wheels / Ace-low straights (Ace, 2, 3, 4, 5).
    ///
    /// This function assumes that all higher-ranking hand checks have been called.
    pub fn get_straight(&self) -> Option<Straight> {
        // Check for standard straight.
        let mut is_sequential = true;
        for i in 0..4 {
            if self.cards[i].rank().as_u8() != self.cards[i + 1].rank().as_u8() + 1 {
                is_sequential = false;
                break;
            }
        }

        if is_sequential {
            return Some(Straight {
                high_card: self.cards[0].rank(),
                suits: [
                    self.cards[0].suit(),
                    self.cards[1].suit(),
                    self.cards[2].suit(),
                    self.cards[3].suit(),
                    self.cards[4].suit(),
                ],
            });
        }

        // Do a final check for a wheel / Ace-low straight (Ace, 2, 3, 4, 5).
        if self.cards[0].rank() == Rank::Face(Face::Ace)
            && self.cards[1].rank() == Rank::Number(Number::Five)
            && self.cards[2].rank() == Rank::Number(Number::Four)
            && self.cards[3].rank() == Rank::Number(Number::Three)
            && self.cards[4].rank() == Rank::Number(Number::Two)
        {
            return Some(Straight {
                // In a wheel, 5 is the high card.
                high_card: Rank::Number(Number::Five),
                suits: [
                    self.cards[0].suit(),
                    self.cards[1].suit(),
                    self.cards[2].suit(),
                    self.cards[3].suit(),
                    self.cards[4].suit(),
                ],
            });
        }

        None
    }

    /// Returns a [`ThreeOfAKind`] if the hand contains three cards of the same rank, otherwise
    /// returns `None`.
    ///
    /// This function assumes that all higher-ranking hand checks have been called.
    pub fn get_triple(&self) -> Option<ThreeOfAKind> {
        // Check each possible position for three matching cards.
        if self.cards[0].rank() == self.cards[2].rank() {
            return Some(ThreeOfAKind {
                triple: Triple {
                    rank: self.cards[0].rank(),
                    suits: [
                        self.cards[0].suit(),
                        self.cards[1].suit(),
                        self.cards[2].suit(),
                    ],
                },
                kickers: [self.cards[3], self.cards[4]],
            });
        }

        if self.cards[1].rank() == self.cards[3].rank() {
            return Some(ThreeOfAKind {
                triple: Triple {
                    rank: self.cards[1].rank(),
                    suits: [
                        self.cards[1].suit(),
                        self.cards[2].suit(),
                        self.cards[3].suit(),
                    ],
                },
                kickers: [self.cards[0], self.cards[4]],
            });
        }

        if self.cards[2].rank() == self.cards[4].rank() {
            return Some(ThreeOfAKind {
                triple: Triple {
                    rank: self.cards[2].rank(),
                    suits: [
                        self.cards[2].suit(),
                        self.cards[3].suit(),
                        self.cards[4].suit(),
                    ],
                },
                kickers: [self.cards[0], self.cards[1]],
            });
        }

        None
    }

    /// Returns a [`TwoPair`] if the hand contains two pairs, otherwise `None`.
    ///
    /// This function assumes that all higher-ranking hand checks have been called.
    pub fn get_two_pair(&self) -> Option<TwoPair> {
        // First check for the first pair
        if let Some(OnePair {
            pair: first_pair,
            kickers,
        }) = self.get_one_pair()
        {
            // Look for second pair in the kickers
            for i in 0..2 {
                if kickers[i].rank() == kickers[i + 1].rank() {
                    return Some(TwoPair {
                        first_pair,
                        second_pair: Pair {
                            rank: kickers[i].rank(),
                            suits: [kickers[i].suit(), kickers[i + 1].suit()],
                        },
                        kicker: if i == 0 { kickers[2] } else { kickers[0] },
                    });
                }
            }
        }
        None
    }

    /// Returns a `OnePair` if the hand contains a pair of cards with matching rank, otherwise
    /// returns `None`.
    ///
    /// This function assumes that all higher-ranking hand checks have been called.
    pub fn get_one_pair(&self) -> Option<OnePair> {
        for i in 0..4 {
            if self.cards[i].rank() == self.cards[i + 1].rank() {
                let pair = Pair {
                    rank: self.cards[i].rank(),
                    suits: [self.cards[i].suit(), self.cards[i + 1].suit()],
                };

                let mut kickers = [self.cards[0], self.cards[0], self.cards[0]];
                let mut k = 0;

                for j in 0..5 {
                    if j != i && j != (i + 1) {
                        kickers[k] = self.cards[j];
                        k += 1;
                    }
                }

                return Some(OnePair { pair, kickers });
            }
        }
        None
    }
}
