#![allow(unused_variables, dead_code, clippy::new_without_default)]

//! In this module, we'll be implementing a water type pokemon that represents
//! some of the Eevee evolution line:
//!  - Eevee
//!      - Vaporeon
//!      - Jolteon
//!      - Flareon
//!
//! To model the evolution line, we will want to use a type system that represents the possible
//! states that the Eevee can be in. We have an [`Eevee`] type that contains stats about the
//! Eevee, and several methods that can retrieve and modify these stats. We also have an
//! [`EvolvedEevee`] type that can be one of Vaporeon, Flareon, and Jolteon.
//!
//! An [`Eevee`] can `evolve` into an [`EvolvedEevee`], and an [`EvolvedEevee`] can `devolve`
//! back into an [`Eevee`]. Go to the documentation for each of these types to learn more.

/// This type represents a basic `Eevee` pokemon. It has a level, as well as health,
/// attack, and defense stats.
pub struct Eevee {
    level: u8,
    health: u16,
    attack: u16,
    defense: u16,
}

/// These stones are used to evolve an [`Eevee`] into an [`EvolvedEevee`].
pub enum ElementalStone {
    /// This Water Stone turns an [`Eevee`] into an [`EvolvedEevee::Vaporeon`]
    WaterStone,
    /// This Fire Stone turns an [`Eevee`] into an [`EvolvedEevee::Flareon`]
    FireStone,
    /// This Electric Stone turns an [`Eevee`] into an [`EvolvedEevee::Jolteon`]
    ElectricStone,
}

/// This type represent an evolved Eevee in the form of either Vaporeon, Flareon, or Jolteon.
///
/// An [`EvolvedEevee`] contains an inner [`Eevee`] as well as a secondary attribute value.
/// This attribute value changes one of the inner [`Eevee`]'s base stats depending on which
/// of the 3 types the [`EvolvedEevee`] is.
///
/// - If it is a Vaporeon, then the secondary attribute is added to the base health
/// - If it is a Flareon, then the secondary attribute is added to the base attack
/// - If it is a Jolteon, then the secondary attribute is added to the base defense
pub enum EvolvedEevee {
    Vaporeon(Eevee, u16),
    Flareon(Eevee, u16),
    Jolteon(Eevee, u16),
}

impl Eevee {
    /// Creates an Eevee with the following base stats:
    /// - level: 1
    /// - health: 55
    /// - attack: 55
    /// - defense: 50
    ///
    /// Example:
    /// ```
    /// let new_eevee = Eevee::new();
    /// assert_eq!(new_eevee.level, 1);
    /// assert_eq!(new_eevee.health, 55)
    /// assert_eq!(new_eevee.attack, 55);
    /// assert_eq!(new_eevee.defense, 50);
    /// ```
    pub fn new() -> Self {
        Self {
            level: 1,
            health: 55,
            attack: 55,
            defense: 50,
        }
    }

    /// Retrieves the level of the Eevee
    pub fn get_level(&self) -> u8 {
        self.level
    }

    /// Retrieves the health of the Eevee
    pub fn get_health(&self) -> u16 {
        self.health
    }

    /// Retrieves the attack of the Eevee
    pub fn get_attack(&self) -> u16 {
        self.attack
    }

    /// Retrieves the defense of the Eevee
    pub fn get_defense(&self) -> u16 {
        self.defense
    }

    /// Level up the Eevee by `levels`.
    ///
    /// Example:
    /// ```
    /// let new_eevee = Eevee::new();
    /// assert_eq!(new_eevee.level, 1);
    /// ```
    pub fn level_up(&mut self, levels: u8) {
        self.level += levels;
    }

    pub fn take_damage(&mut self, damage: u16) {
        // TODO what happens when no more health left
        self.health -= damage;
    }

    /// This method must accomplish the following:
    /// - If the current state is Eevee, evolve into a random evolution.
    /// - If the current state is an evolution (not Eevee), do nothing.
    /// Evolution will be based on the level of the pokemon.
    /// - If the level is less than 5, evolve into Vaporeon.
    /// - If the level is less than 10 but greater than 5, evolve into Jolteon.
    /// - If the level is greater 10, evolve into Flareon.
    /// The change in state will also change the pokemon's base stats by
    /// a randomly generated multiplier between 0.5 and 1.75
    /// TODO docs and code example
    pub fn evolve(self, evolution: ElementalStone) -> EvolvedEevee {
        match evolution {
            ElementalStone::WaterStone => EvolvedEevee::Vaporeon(self, 20),
            ElementalStone::FireStone => EvolvedEevee::Flareon(self, 1.5),
            ElementalStone::ElectricStone => EvolvedEevee::Jolteon(self, 1),
        }
    }
}
