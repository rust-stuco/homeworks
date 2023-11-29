#![allow(unused_variables, dead_code, clippy::new_without_default)]

//! In this module, we'll be implementing a water type pokemon that represents
//! some of the Eevee evolution line:
//!  - Eevee
//!    - Vaporeon
//!    - Jolteon
//!    - Flareon
//!
//! To model the evolution line, we will want to use a type system that represents the possible
//! states that the Eevee can be in.
//!
//! In our model we'll allow Eevee to evolve into Vaporeon, Jolteon, or Flareon,
//! as well as "devolve" back into Eevee.

pub struct Eevee {
    pub level: u8,
    pub health: u16,
    pub attack: u16,
    pub defense: u16,
}

pub enum ElementalStone {
    WaterStone,
    FireStone,
    ElectricStone,
}

pub enum EvolvedEevee {
    Vaporeon(Eevee, u16),
    Flareon(Eevee, f32),
    Jolteon(Eevee, u8),
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

    /// Level up the Eevee by `l`.
    ///
    /// Example:
    /// ```
    /// let new_eevee = Eevee::new();
    /// assert_eq!(new_eevee.level, 1);
    /// ```
    pub fn level_up(&mut self, l: u8) {
        self.level += l;
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
