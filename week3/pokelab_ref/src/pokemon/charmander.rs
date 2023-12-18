//! This module contains the [`Charmander`] type, as well as its method implementations.
//!
//! The [`Charmander`] struct will model the pokemon of the same name.
//! We want to design the struct such that it is easy to create a new fire type pokemon.
//! As well as modify the pokemon's attributes such as its health, level, and name.
//!
//! [`Charmander`] has the following attributes:
//!  - `name: String`
//!  - `level: usize`
//!  - `health: usize`
//!  - `attack: usize`
//!  - `defense: usize`
//! All of these fields should be _private_.
//!
//! Once you've designed the struct, implement the following methods:
//! - [`new`](Charmander::new): This method will take in a name and create a new [`Charmander`]
//! struct with the following default values:
//!     - `level: 1`
//!     - `health: 100`
//!     - `attack: 42`
//!     - `defense: 33`
//! - [`level_up`](Charmander::level_up): This method will increase the level of the
//! [`Charmander`] struct by the input `l`.
//! - [`get_health`](Charmander::get_health): This method will return the health value of the
//! [`Charmander`] struct.
//! - [`get_attack`](Charmander::get_attack): This method will return the attack value of the
//! [`Charmander`] struct.
//! - [`get_defense`](Charmander::get_defense): This method will return the defense value of the
//! [`Charmander`] struct.
//!
//! For some of these methods, you may have to do some extra work to determine the value to return.
//! Make sure to read the specification in either the comments in the code or on this writeup!
//!
//! We also want [`Charmander`]s to battle with each other.
//! We'll implement the following methods
//! [`attack`](Charmander::attack) and [`fight`](Charmander::fight).
//! Click on these hyperlinks or read the doc comments to see the specification for these methods!
//!
//! **Note that this pokemon is _completely_ different from the**
//! **[`Eevee`](crate::pokemon::eevee::Eevee)**
//! **pokemon that you will implement in the next part, so make sure not to mix them up!**
pub struct Charmander {
    name: String,
    level: usize,
    health: usize,
    attack: usize,
    defense: usize,
}

/// Implement the Charmander struct here.
impl Charmander {
    /// Takes in a `name` as a `String` and creates a new [`Charmander`]
    /// struct with the following default values:
    /// - `level: 0`
    /// - `health: 100`
    /// - `attack: 42`
    /// - `defense: 33`
    pub fn new(name: String) -> Self {
        Self {
            name,
            level: 0,
            health: 100,
            attack: 42,
            defense: 33,
        }
    }

    /// Increases the level of the [`Charmander`] by the input `l`.
    pub fn level_up(&mut self, l: usize) {
        self.level += l;
    }

    /// Return the health value of the [`Charmander`].
    ///
    /// The health value is calculated by the following formula: `(health + (5 * level))`.
    pub fn get_health(&self) -> usize {
        self.health + (5 * self.level)
    }

    /// Returns the attack value of the [`Charmander`].
    ///
    /// The attack value is calculated by the following formula: `(attack + (3 * level))`.
    pub fn get_attack(&self) -> usize {
        self.attack + (3 * self.level)
    }
    /// Returns the defense value of the [`Charmander`].
    ///
    /// The defense value is calculated by the following formula: `(defense + (4 * level))`.
    pub fn get_defense(&self) -> usize {
        self.defense + (4 * self.level)
    }

    /// Takes `damage` subtracted by [`Charmander`]'s `defense`.
    ///
    /// ```
    /// # use pokelab_ref::pokemon::charmander::*;
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
        if damage < self.get_defense() {
            return;
        }
        let dmg_taken = damage - self.get_defense();

        if dmg_taken >= self.health {
            panic!("{} fainted!", self.name);
        }

        self.health -= dmg_taken;
    }

    /// Attacks another [`Charmander`] struct and deals damage.
    ///
    /// [`attack`](Charmander::attack) will do damage to the `other` [`Charmander`]
    /// equal to its current attack.
    ///
    /// ```
    /// # use pokelab_ref::pokemon::charmander::*;
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
        other.take_damage(self.get_attack());
    }

    /// Pits two [`Charmander`]s against each other!
    ///
    /// Note that this is an associated function, so you cannot write `charmander.fight(other)`.
    ///
    /// The [`Charmander`] with the higher level will go first.
    ///
    /// ```
    /// # use pokelab_ref::pokemon::charmander::*;
    /// #
    /// let mut charmander1 = Charmander::new(String::from("David"));
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
        if charmander1.level > charmander2.level {
            charmander1.attack(charmander2);
            charmander2.attack(charmander1);
        } else {
            charmander2.attack(charmander1);
            charmander1.attack(charmander2);
        }
    }
}
