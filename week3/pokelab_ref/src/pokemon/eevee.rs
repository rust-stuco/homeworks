#![allow(unused_variables, dead_code, clippy::new_without_default)]

//! In this module, we'll be implementing a water type pokemon that represents
//! some of the Eevee evolution line:
//!  - Eevee
//!    - Vaporeon
//!    - Jolteon
//!    - Flareon
///
//! To model the evolution line, we want to design a struct that acts like a state machine.
//! The struct will have a `EeveeEvolution` enum attribute that will
//! represent the current state of the pokemon.
///
//! In our model we'll allow Eevee to evolve into Vaporeon, Jolteon, or Flareon,
//! as well as "devolve" back into Eevee.

pub struct Eevee {
    level: u8,
    health: usize,
    attack: usize,
    defense: usize,
}

struct EeveeEvolutionStats {
    inner: Eevee,
    multiplier: f32,
}


pub enum EeveeEvolution {
    Vaporeon(EeveeEvolution, f32),
    Flareon(EeveeEvoltuion, usize),
    Jolteon(EeveeEvolution, u8),
}

impl Eevee {
    /// Creates an Eevee with the following base stats:
    /// - level: 1
    /// - health: 55
    /// - attack: 55
    /// - defense: 50
    /// - evolution: EeveeEvolution::Eevee
    /// TODO code example
    pub fn new() -> Eevee {
        Self {
            level: 1,
            health: 55,
            attack: 55,
            defense: 50,
            evolution: EeveeEvolution::Eevee,
        }
    }

    /// Get the attack value of the Eevee.
    /// The attack value is calculated by multiplying the base attack
    /// by the Eevee state multiplier, and rounding
    pub fn get_attack(&self) -> f32 {
    }

    /// Get the defense value of the Eevee.
    /// The defense value is calculated by multiplying the base defense by the Eevee state multiplier.
    pub fn get_defense(&self) -> f32 {
        match self.evolution {
            EeveeEvolution::Eevee => self.defense as f32,
            EeveeEvolution::Vaporeon(m) => self.defense as f32 * m,
            EeveeEvolution::Jolteon(m) => self.defense as f32 * m,
            EeveeEvolution::Flareon(m) => self.defense as f32 * m,
        }
    }

    /// Get the health value of the Eevee.
    /// The health value is calculated by multiplying the base health by the Eevee state multiplier.
    pub fn get_health(&self) -> f32 {
        match self.evolution {
            EeveeEvolution::Eevee => self.health as f32,
            EeveeEvolution::Vaporeon(m) => self.health as f32 * m,
            EeveeEvolution::Jolteon(m) => self.health as f32 * m,
            EeveeEvolution::Flareon(m) => self.health as f32 * m,
        }
    }

    /// Get the level of the Eevee.
    pub fn get_level(&self) -> u16 {
        self.level
    }

    /// Level up the Eevee by l. Preserves the rest of the state.
    pub fn level_up(&mut self, l: u16) {
        self.level += l;
    }

    /// Get the type of the Eevee as a String.
    /// ex: "Eevee", "Vaporeon", "Jolteon", "Flareon"
    /// TODO code example
    pub fn get_type(&self) -> String {
        match self.evolution {
            EeveeEvolution::Eevee => "Eevee".to_string(),
            EeveeEvolution::Vaporeon(_) => "Vaporeon".to_string(),
            EeveeEvolution::Jolteon(_) => "Jolteon".to_string(),
            EeveeEvolution::Flareon(_) => "Flareon".to_string(),
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
    pub fn evolve(&mut self, evolution: EeveeEvolution) {
        if self.evolution != EeveeEvolution::Eevee {
            panic!("Tried to evolve an already evolved Eevee");
        }
        match evolution {
            EeveeEvolution::Eevee => panic!("Tried to evolve an Eevee into another Eevee"),
            EeveeEvolution::Vaporeon(_) => todo!(),
            EeveeEvolution::Jolteon(_) => todo!(),
            EeveeEvolution::Flareon(_) => todo!(),
        }
    }

    // Here we want to implement the devolve method for Eevee.
    // This method must accomplish the following:
    // - If the current state is Eevee, do nothing.
    // - If the current state is an evolution (not Eevee), devolve into Eevee.
    pub fn devolve(&mut self) {
        if self.evolution == EeveeEvolution::Eevee {
            panic!("Tried to devolve an Eevee");
        }
        self.evolution = EeveeEvolution::Eevee;
    }
}
