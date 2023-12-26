//! This module contains the [`Charmander`] type, as well as its method implementations.
//!
//! The [`Charmander`] struct will model the pokemon of the same name.
//! We want to design the struct in such a way that we can easily create a [`Charmander`] and
//! modify its attributes like health, level, and name.
//!
//! [`Charmander`] should have the following attributes:
//!  - `name: String`
//!  - `level: usize`
//!  - `health: usize`
//!  - `attack: usize`
//!  - `defense: usize`
//!
//! All of these fields should be _private_ (not accessible outside of the struct).
//!
//! ---
//!
//! Once you've added the fields to the struct, implement the following methods:
//! - [`new`](Charmander::new): This method will take in a name and create a new [`Charmander`]
//! struct with the following default values:
//!     - `level: 0`
//!     - `health: 100`
//!     - `attack: 42`
//!     - `defense: 33`
//! - [`level_up`](Charmander::level_up): This method will increase the level of the
//! [`Charmander`] struct by the input `levels`.
//! - [`get_health`](Charmander::get_health): This method will return the health value of the
//! [`Charmander`] struct.
//! - [`get_attack`](Charmander::get_attack): This method will return the attack value of the
//! [`Charmander`] struct.
//! - [`get_defense`](Charmander::get_defense): This method will return the defense value of the
//! [`Charmander`] struct.
//!
//! ---
//!
//! We also want [`Charmander`]s to battle with each other.
//! We'll implement the following methods
//! [`attack`](Charmander::attack) and [`fight`](Charmander::fight).
//! Click on these hyperlinks or read the doc comments to see the specification for these methods!
//!
//! For some of these methods, you may have to do some extra work to determine the value to return.
//! **Make sure to read the specification in either the comments in the code or on this writeup!**
//!
//! _Note that this pokemon is **completely** separate from the_
//! _[`Eevee`](crate::pokemon::eevee::Eevee)_
//! _pokemon that you will implement in the next part, so make sure not to mix them up!_
//!
//! Once you have finished this section (and passed all the tests),
//! you can follow hyperlinks or click this [`eevee`](crate::pokemon::eevee) link
//! to get to part 2 of this homework!
pub struct Charmander {
    // Change me with the fields that Charmander needs!
    _placeholder: (),
}

/// Implement the Charmander struct here.
///
/// Once you have finished this section (and passed all the tests),
/// you can follow hyperlinks or click this [`eevee`](crate::pokemon::eevee) link
/// to get to part 2 of this homework!
impl Charmander {
    /// Takes in a `name` as a `String` and creates a new [`Charmander`]
    /// struct with the following default values:
    /// - `level: 0`
    /// - `health: 100`
    /// - `attack: 42`
    /// - `defense: 33`
    ///
    /// _Note: We 0-level because we're programmers_ 😎
    pub fn new(name: String) -> Self {
        todo!()
    }

    /// Increases the level of the [`Charmander`] by the input `levels`.
    pub fn level_up(&mut self, levels: usize) {
        todo!()
    }

    /// Return the health value of the [`Charmander`].
    ///
    /// The health value is calculated by the following formula: `(health + (5 * level))`.
    pub fn get_health(&self) -> usize {
        todo!()
    }

    /// Returns the attack value of the [`Charmander`].
    ///
    /// The attack value is calculated by the following formula: `(attack + (3 * level))`.
    pub fn get_attack(&self) -> usize {
        todo!()
    }
    /// Returns the defense value of the [`Charmander`].
    ///
    /// The defense value is calculated by the following formula: `(defense + (4 * level))`.
    pub fn get_defense(&self) -> usize {
        todo!()
    }

    /// Takes `damage` subtracted by [`Charmander`]'s `defense`.
    ///
    /// If the [`Charmander`] takes more damage than it has health,
    /// this function should panic with the message
    /// `"{name} fainted!"`, where `"{name}"` is the name of the [`Charmander`].
    /// In other words, we should panic if the [`Charmander`] hits 0 health.
    ///
    /// ```
    /// # use pokelab::pokemon::charmander::*;
    /// #
    /// let mut charmander = Charmander::new(String::from("Ben"));
    /// assert_eq!(charmander.get_health(), 100);
    /// assert_eq!(charmander.get_defense(), 33);
    ///
    /// charmander.take_damage(30);
    /// assert_eq!(charmander.get_health(), 100); // Not enough damage to overcome defense
    ///
    /// charmander.take_damage(66);
    /// assert_eq!(charmander.get_health(), 67); // 33 actual damage taken
    /// ```
    pub fn take_damage(&mut self, damage: usize) {
        todo!()
    }

    /// Attacks another [`Charmander`] struct and deals damage.
    ///
    /// [`attack`](Charmander::attack) will do damage to the `other` [`Charmander`]
    /// equal to its current attack. Will panic if the other [`Charmander`] faints.
    ///
    /// ```
    /// # use pokelab::pokemon::charmander::*;
    /// #
    /// let mut attacker = Charmander::new(String::from("David"));
    /// let mut defender = Charmander::new(String::from("Connor"));
    /// assert_eq!(attacker.get_attack(), 42);
    /// assert_eq!(defender.get_defense(), 33);
    ///
    /// attacker.attack(&mut defender);
    /// assert_eq!(defender.get_health(), 91); // 9 damage
    ///
    /// attacker.level_up(1);
    /// attacker.attack(&mut defender);
    /// assert_eq!(defender.get_health(), 79); // 12 damage
    /// ```
    pub fn attack(&self, other: &mut Self) {
        todo!()
    }

    /// Pits two [`Charmander`]s against each other!
    ///
    /// Note that this is an associated function, so you cannot write `charmander.fight(other)`.
    ///
    /// The [`Charmander`] with the higher level will go first.
    /// Will panic if a [`Charmander`] faints.
    ///
    /// ```
    /// # use pokelab::pokemon::charmander::*;
    /// #
    /// let mut charmander1 = Charmander::new(String::from("Ben"));
    /// let mut charmander2 = Charmander::new(String::from("Connor"));
    ///
    /// charmander1.level_up(5);
    /// assert_eq!(charmander1.get_health(), 125); // +25
    /// assert_eq!(charmander1.get_attack(), 57);  // +15
    /// assert_eq!(charmander1.get_defense(), 53); // +20
    ///
    /// Charmander::fight(&mut charmander1, &mut charmander2);
    /// assert_eq!(charmander1.get_health(), 125); // no damage taken
    /// assert_eq!(charmander2.get_health(), 76);  // 57 - 33 = 24 damage taken
    /// ```
    pub fn fight(charmander1: &mut Self, charmander2: &mut Self) {
        todo!()
    }
}
