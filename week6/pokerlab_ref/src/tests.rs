use crate::card::{Card, Face, Number, Rank, Suit};

#[test]
fn test_simple_card_comparisons() {
    let ace_spades = Card::new(Suit::Spade, Rank::Face(Face::Ace));
    let king_spades = Card::new(Suit::Spade, Rank::Face(Face::King));
    let two_spades = Card::new(Suit::Spade, Rank::Number(Number::Two));

    assert!(ace_spades > king_spades);
    assert!(king_spades > two_spades);
    assert!(ace_spades > two_spades);
}

#[test]
fn test_suit_comparisons() {
    let ace_spades = Card::new(Suit::Spade, Rank::Face(Face::Ace));
    let ace_hearts = Card::new(Suit::Heart, Rank::Face(Face::Ace));
    let ace_clubs = Card::new(Suit::Club, Rank::Face(Face::Ace));
    let ace_diamonds = Card::new(Suit::Diamond, Rank::Face(Face::Ace));

    assert!(ace_spades > ace_hearts);
    assert!(ace_hearts > ace_clubs);
    assert!(ace_clubs > ace_diamonds);
}

#[test]
fn test_equality() {
    let ace_spades1 = Card::new(Suit::Spade, Rank::Face(Face::Ace));
    let ace_spades2 = Card::new(Suit::Spade, Rank::Face(Face::Ace));
    let king_spades = Card::new(Suit::Spade, Rank::Face(Face::King));

    assert_eq!(ace_spades1, ace_spades2);
    assert_ne!(ace_spades1, king_spades);
}

#[test]
fn test_deck_sorting_easy() {
    let mut cards = [
        Card::new(Suit::Diamond, Rank::Number(Number::Two)),
        Card::new(Suit::Spade, Rank::Face(Face::Ace)),
        Card::new(Suit::Heart, Rank::Number(Number::Seven)),
        Card::new(Suit::Club, Rank::Number(Number::Ten)),
        Card::new(Suit::Diamond, Rank::Face(Face::Ace)),
        Card::new(Suit::Spade, Rank::Number(Number::Two)),
        Card::new(Suit::Heart, Rank::Face(Face::Ace)),
        Card::new(Suit::Club, Rank::Face(Face::Ace)),
    ];

    cards.sort();

    let expected = [
        Card::new(Suit::Diamond, Rank::Number(Number::Two)),
        Card::new(Suit::Spade, Rank::Number(Number::Two)),
        Card::new(Suit::Heart, Rank::Number(Number::Seven)),
        Card::new(Suit::Club, Rank::Number(Number::Ten)),
        Card::new(Suit::Diamond, Rank::Face(Face::Ace)),
        Card::new(Suit::Club, Rank::Face(Face::Ace)),
        Card::new(Suit::Heart, Rank::Face(Face::Ace)),
        Card::new(Suit::Spade, Rank::Face(Face::Ace)),
    ];

    assert!(cards == expected);
}

#[test]
fn test_deck_sorting_medium() {
    let mut cards = [
        Card::new(Suit::Diamond, Rank::Number(Number::Two)),
        Card::new(Suit::Spade, Rank::Face(Face::Ace)),
        Card::new(Suit::Heart, Rank::Number(Number::Seven)),
        Card::new(Suit::Club, Rank::Number(Number::Ten)),
        Card::new(Suit::Diamond, Rank::Face(Face::Ace)),
        Card::new(Suit::Spade, Rank::Number(Number::Two)),
        Card::new(Suit::Heart, Rank::Face(Face::Ace)),
        Card::new(Suit::Club, Rank::Face(Face::Ace)),
        Card::new(Suit::Diamond, Rank::Number(Number::Eight)),
        Card::new(Suit::Heart, Rank::Number(Number::Two)),
        Card::new(Suit::Club, Rank::Number(Number::Three)),
        Card::new(Suit::Spade, Rank::Face(Face::Jack)),
        Card::new(Suit::Diamond, Rank::Face(Face::King)),
        Card::new(Suit::Heart, Rank::Face(Face::King)),
        Card::new(Suit::Club, Rank::Face(Face::King)),
        Card::new(Suit::Spade, Rank::Face(Face::King)),
    ];

    cards.sort();

    let expected = [
        Card::new(Suit::Diamond, Rank::Number(Number::Two)),
        Card::new(Suit::Heart, Rank::Number(Number::Two)),
        Card::new(Suit::Spade, Rank::Number(Number::Two)),
        Card::new(Suit::Club, Rank::Number(Number::Three)),
        Card::new(Suit::Heart, Rank::Number(Number::Seven)),
        Card::new(Suit::Diamond, Rank::Number(Number::Eight)),
        Card::new(Suit::Club, Rank::Number(Number::Ten)),
        Card::new(Suit::Spade, Rank::Face(Face::Jack)),
        Card::new(Suit::Diamond, Rank::Face(Face::King)),
        Card::new(Suit::Club, Rank::Face(Face::King)),
        Card::new(Suit::Heart, Rank::Face(Face::King)),
        Card::new(Suit::Spade, Rank::Face(Face::King)),
        Card::new(Suit::Diamond, Rank::Face(Face::Ace)),
        Card::new(Suit::Club, Rank::Face(Face::Ace)),
        Card::new(Suit::Heart, Rank::Face(Face::Ace)),
        Card::new(Suit::Spade, Rank::Face(Face::Ace)),
    ];

    assert!(cards == expected);
}

#[test]
fn test_full_deck_sorting() {
    let mut cards = Vec::new();
    let suits = [Suit::Diamond, Suit::Club, Suit::Heart, Suit::Spade];
    let ranks = [
        Rank::Number(Number::Two),
        Rank::Number(Number::Three),
        Rank::Number(Number::Four),
        Rank::Number(Number::Five),
        Rank::Number(Number::Six),
        Rank::Number(Number::Seven),
        Rank::Number(Number::Eight),
        Rank::Number(Number::Nine),
        Rank::Number(Number::Ten),
        Rank::Face(Face::Jack),
        Rank::Face(Face::Queen),
        Rank::Face(Face::King),
        Rank::Face(Face::Ace),
    ];

    // Create full deck of 52 cards.
    for &suit in suits.iter() {
        for &rank in ranks.iter() {
            cards.push(Card::new(suit, rank));
        }
    }

    // Verify initial deck size.
    assert_eq!(cards.len(), 52);

    // Randomly shuffle the cards.
    use rand::seq::SliceRandom;
    let mut rng = rand::thread_rng();
    cards.shuffle(&mut rng);

    // Sort the cards.
    cards.sort();

    // Verify that the cards are properly sorted.
    let mut prev_card = &cards[0];
    assert!(prev_card == &Card::new(Suit::Diamond, Rank::Number(Number::Two)));
    assert!(cards[51] == Card::new(Suit::Spade, Rank::Face(Face::Ace)));

    for card in cards.iter().skip(1) {
        assert!(card >= prev_card);

        if *card.suit() == Suit::Club {
            assert_eq!(prev_card.suit(), &Suit::Diamond);
            assert_eq!(prev_card.rank(), card.rank());
        } else if *card.suit() == Suit::Heart {
            assert_eq!(prev_card.suit(), &Suit::Club);
            assert_eq!(prev_card.rank(), card.rank());
        } else if *card.suit() == Suit::Spade {
            assert_eq!(prev_card.suit(), &Suit::Heart);
            assert_eq!(prev_card.rank(), card.rank());
        } else if *card.suit() == Suit::Diamond {
            assert_eq!(prev_card.suit(), &Suit::Spade);
            assert!(prev_card.rank().as_u8() == card.rank().as_u8() - 1);
        }

        prev_card = card;
    }
}
