//! Tests for Poker hand rank comparisons.
//!
//! Disclaimer: many of the poker hand tests were generated with AI, since a lot of it consists of
//! boilerplate code. However, we were still the ones coming up with the specific test scenarios.

use pokerlab_ref::card::{Card, Rank::*, Suit::*};
use pokerlab_ref::hand::{Hand, PokerHand};

fn assert_tie(hand1: Hand, hand2: Hand) {
    let solved_hand1 = PokerHand::solve(hand1);
    let solved_hand2 = PokerHand::solve(hand2);

    assert_eq!(solved_hand1, solved_hand2);
}

fn assert_winner(winner: Hand, loser: Hand) {
    let solved_winner = PokerHand::solve(winner);
    let solved_loser = PokerHand::solve(loser);

    assert!(solved_winner > solved_loser);
}

#[test]
fn test_high_card_vs_high_card() {
    // Higher high card hand: (A♠ K♠ Q♠ J♦ 9♣).
    let higher_hand = Hand::new([
        Card::new(Spade, Ace),
        Card::new(Spade, King),
        Card::new(Spade, Queen),
        Card::new(Diamond, Jack),
        Card::new(Club, Nine),
    ])
    .unwrap();

    // Lower high card hand: (K♠ Q♠ J♠ 9♦ 8♣).
    let lower_hand = Hand::new([
        Card::new(Spade, King),
        Card::new(Spade, Queen),
        Card::new(Spade, Jack),
        Card::new(Diamond, Nine),
        Card::new(Club, Eight),
    ])
    .unwrap();

    assert_winner(higher_hand, lower_hand);
}

#[test]
fn test_high_card_tie_different_kickers() {
    // Higher kickers: (A♠ Q♠ J♠ 9♦ 8♣).
    let higher_hand = Hand::new([
        Card::new(Spade, Ace),
        Card::new(Spade, Queen),
        Card::new(Spade, Jack),
        Card::new(Diamond, Nine),
        Card::new(Club, Eight),
    ])
    .unwrap();

    // Lower kickers: (A♠ J♠ 9♠ 8♦ 7♣).
    let lower_hand = Hand::new([
        Card::new(Spade, Ace),
        Card::new(Spade, Jack),
        Card::new(Spade, Nine),
        Card::new(Diamond, Eight),
        Card::new(Club, Seven),
    ])
    .unwrap();

    assert_winner(higher_hand, lower_hand);
}

#[test]
fn test_high_card_identical_ranks() {
    // First high card hand: (A♠ K♠ Q♠ J♦ 9♣).
    let first_hand = Hand::new([
        Card::new(Spade, Ace),
        Card::new(Spade, King),
        Card::new(Spade, Queen),
        Card::new(Diamond, Jack),
        Card::new(Club, Nine),
    ])
    .unwrap();

    // Second high card hand: (A♣ K♣ Q♣ J♥ 9♦).
    let second_hand = Hand::new([
        Card::new(Club, Ace),
        Card::new(Club, King),
        Card::new(Club, Queen),
        Card::new(Heart, Jack),
        Card::new(Diamond, Nine),
    ])
    .unwrap();

    assert_tie(first_hand, second_hand);
}

#[test]
fn test_one_pair_vs_high_card() {
    // One pair hand: (Q♠ Q♣ A♦ K♥ J♣).
    let pair_hand = Hand::new([
        Card::new(Spade, Queen),
        Card::new(Club, Queen),
        Card::new(Diamond, Ace),
        Card::new(Heart, King),
        Card::new(Club, Jack),
    ])
    .unwrap();

    // High card hand: (A♠ K♠ Q♠ J♦ 9♣).
    let high_card_hand = Hand::new([
        Card::new(Spade, Ace),
        Card::new(Spade, King),
        Card::new(Spade, Queen),
        Card::new(Diamond, Jack),
        Card::new(Club, Nine),
    ])
    .unwrap();

    assert_winner(pair_hand, high_card_hand);
}

#[test]
fn test_one_pair_vs_one_pair_different_pairs() {
    // Higher pair: (Q♠ Q♣ A♦ K♥ J♣).
    let higher_pair_hand = Hand::new([
        Card::new(Spade, Queen),
        Card::new(Club, Queen),
        Card::new(Diamond, Ace),
        Card::new(Heart, King),
        Card::new(Club, Jack),
    ])
    .unwrap();

    // Lower pair: (5♠ 5♣ A♦ K♥ Q♣).
    let lower_pair_hand = Hand::new([
        Card::new(Spade, Five),
        Card::new(Club, Five),
        Card::new(Diamond, Ace),
        Card::new(Heart, King),
        Card::new(Club, Queen),
    ])
    .unwrap();

    assert_winner(higher_pair_hand, lower_pair_hand);
}

#[test]
fn test_two_pair_vs_one_pair() {
    // Two pair hand: (K♠ K♣ Q♦ Q♥ 2♣).
    let two_pair_hand = Hand::new([
        Card::new(Spade, King),
        Card::new(Club, King),
        Card::new(Diamond, Queen),
        Card::new(Heart, Queen),
        Card::new(Club, Two),
    ])
    .unwrap();

    // One pair hand: (A♠ A♣ K♦ Q♥ J♣).
    let one_pair_hand = Hand::new([
        Card::new(Spade, Ace),
        Card::new(Club, Ace),
        Card::new(Diamond, King),
        Card::new(Heart, Queen),
        Card::new(Club, Jack),
    ])
    .unwrap();

    assert_winner(two_pair_hand, one_pair_hand);
}

#[test]
fn test_three_of_a_kind_vs_one_pair() {
    // Three of a kind hand: (Q♠ Q♣ Q♦ K♥ J♣).
    let three_of_a_kind_hand = Hand::new([
        Card::new(Spade, Queen),
        Card::new(Club, Queen),
        Card::new(Diamond, Queen),
        Card::new(Heart, King),
        Card::new(Club, Jack),
    ])
    .unwrap();

    // One pair hand: (A♠ A♣ K♦ Q♥ J♣).
    let one_pair_hand = Hand::new([
        Card::new(Spade, Ace),
        Card::new(Club, Ace),
        Card::new(Diamond, King),
        Card::new(Heart, Queen),
        Card::new(Club, Jack),
    ])
    .unwrap();

    assert_winner(three_of_a_kind_hand, one_pair_hand);
}
