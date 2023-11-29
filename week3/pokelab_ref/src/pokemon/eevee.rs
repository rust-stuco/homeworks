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

#[derive(Clone, Copy)]
pub enum Eevee {
    Basic(BasicEeveeStats),
    Evolved(BasicEeveeStats, EvolvedEeveeStats),
}

#[derive(Clone, Copy)]
pub struct BasicEeveeStats {
    level: u8,
    health: u16,
    attack: u16,
    defense: u16,
}

#[derive(Clone, Copy)]
pub enum EvolvedEeveeStats {
    Vaporeon(u16),
    Flareon(f32),
    Jolteon(u8),
}

impl Eevee {
    /// Creates a basic Eevee with the following base stats:
    /// - level: 1
    /// - health: 55
    /// - attack: 55
    /// - defense: 50
    /// TODO code example
    pub fn new() -> Self {
        Self::Basic(BasicEeveeStats {
            level: 1,
            health: 55,
            attack: 55,
            defense: 50,
        })
    }

    /// TODO docs and code example
    pub fn is_evolved(&self) -> bool {
        match self {
            Eevee::Basic(_) => false,
            Eevee::Evolved(_, _) => true,
        }
    }

    /// Get the attack value of the Eevee.
    /// The attack value is calculated by multiplying the base attack
    /// by the Eevee state multiplier, and rounding
    /// TODO docs and code example
    pub fn get_attack(&self) -> u16 {
        match self {
            Eevee::Basic(base) => base.attack + base.level as u16,
            Eevee::Evolved(base, evolved) => match evolved {
                EvolvedEeveeStats::Flareon(multiplier) => {
                    (base.attack as f32 * multiplier) as u16 + base.level as u16
                }
                _ => base.attack + base.level as u16,
            },
        }
    }

    /// Get the defense value of the Eevee.
    /// The defense value is calculated by multiplying the base defense by the Eevee state multiplier.
    /// TODO docs and code example
    pub fn get_defense(&self) -> u16 {
        match self {
            Eevee::Basic(base) => base.defense + base.level as u16,
            Eevee::Evolved(base, _) => base.attack + base.level as u16,
        }
    }

    /// Get the health value of the Eevee.
    /// The health value is calculated by multiplying the base health by the Eevee state multiplier.
    /// TODO docs and code example
    pub fn get_health(&self) -> u16 {
        match self {
            Eevee::Basic(base) => base.health + base.level as u16,
            Eevee::Evolved(base, evolved) => match evolved {
                EvolvedEeveeStats::Vaporeon(extra_health) => {
                    base.health + extra_health + base.level as u16
                }
                _ => base.health + base.level as u16,
            },
        }
    }

    /// Get the level of the Eevee.
    /// TODO docs and code example
    pub fn get_level(&self) -> u8 {
        match self {
            Eevee::Basic(base) => base.level,
            Eevee::Evolved(base, _) => base.level,
        }
    }

    /// Level up the Eevee by l. Preserves the rest of the state.
    /// TODO docs and code example
    pub fn level_up(&mut self, l: u16) {
        match self {
            Eevee::Basic(base) => base.level += 1,
            Eevee::Evolved(base, _) => base.level += 1,
        }
    }

    /// Get the type of the Eevee as a String.
    /// ex: "Eevee", "Vaporeon", "Jolteon", "Flareon"
    /// TODO docs and code example
    pub fn get_type(&self) -> String {
        match self {
            Eevee::Basic(_) => "Eevee".to_string(),
            Eevee::Evolved(_, evolved) => match evolved {
                EvolvedEeveeStats::Vaporeon(_) => "Vaporeon".to_string(),
                EvolvedEeveeStats::Jolteon(_) => "Jolteon".to_string(),
                EvolvedEeveeStats::Flareon(_) => "Flareon".to_string(),
            },
        }
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
    pub fn evolve(&mut self, evolution: EvolvedEeveeStats) {
        match self {
            Eevee::Basic(base) => {
                *self = Self::Evolved(*base, evolution);
            }
            Eevee::Evolved(_, _) => panic!("Tried to evolve an already evolved Eevee"),
        }
    }

    // Here we want to implement the devolve method for Eevee.
    // This method must accomplish the following:
    // - If the current state is Eevee, do nothing.
    // - If the current state is an evolution (not Eevee), devolve into Eevee.
    /// TODO docs and code example
    pub fn devolve(&mut self) {
        match self {
            Eevee::Basic(base) => panic!("Tried to devolve a basic Eevee"),
            Eevee::Evolved(base, _) => *self = Self::Basic(*base),
        }
    }
}
