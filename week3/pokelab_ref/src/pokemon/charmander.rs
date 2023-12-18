#![allow(unused_variables)]

//! This module contains the [`Charmander`] type, as well as its method implementations.
//!
//! The [`Charmander`] struct will model the pokemon of the same name.
//! We want to design the struct such that it is easy to create a new fire type pokemon.
//! As well as modify the pokemon's attributes such as its health, level, and name.
//!
//! [`Charmander`] has the following attributes:
//!  - name: String
//!  - level: usize
//!  - health: usize
//!  - attack: usize
//!  - defense: usize
//!
//! Once you've designed the struct, implement the following methods:
//! - [`new`](Charmander::new): This method will take in a name and create a new [`Charmander`]
//! struct with the following default values:
//!     - level: 1
//!     - health: 100
//!     - attack: 42
//!     - defense: 33
//!
//! - [`level_up`](Charmander::level_up): This method will increase the level of the
//! [`Charmander`] struct by argument l.
//!
//! - [`get_health`](Charmander::get_health): This method will return the health value of the
//! [`Charmander`] struct.
//!     - The health value does not change as the [`Charmander`] levels up.
//!
//! - [`get_attack`](Charmander::get_attack): This method will return the attack value of the
//! [`Charmander`] struct.
//!     - The attack value is calculated by the following formula: (attack + (10*level))
//!
//! - [`get_defense`](Charmander::get_defense): This method will return the defense value of the
//! [`Charmander`] struct.
//!     - The defense value is calculated by the following formula: (defense + (15*level))
//!
//! We also want [`Charmander`]s to battle with each other.
//! We'll design the following functions:
//!  - attack: This function will take in two mutable references to [`Charmander`] structs.
//!      - The first argument will be the attacking [`Charmander`].
//!      - The second argument will be the defending [`Charmander`].
//!      - The attacking [`Charmander`] will deal damage to the defending [`Charmander`]
//! has a lower level.
//!      - The damage dealt is based on the attacking [`Charmander`]'s attack value and the
//! defending [`Charmander`]'s defense value.
//!      - The defending [`Charmander`]'s health will be reduced by the damage dealt
//! (or put to 0 if too large)
//!
//! **Note that this pokemon is completely different from the**
//! **[`Eevee`](crate::pokemon::eevee::Eevee)**
//! **pokemon that you will implement in the next part, so make sure not to mix them up!**
pub struct Charmander {}

// Implement the Charmander struct here
impl Charmander {
    // Define the new method here
    pub fn new(name: String) -> Self {
        todo!()
    }

    // Define the level_up method here
    pub fn level_up(&mut self, l: usize) {
        todo!()
    }

    pub fn get_health(&self) -> usize {
        todo!()
    }

    // Define the get_attack method here
    pub fn get_attack(&self) -> usize {
        todo!()
    }

    // Define the get_defense method here
    pub fn get_defense(&self) -> usize {
        todo!()
    }

    // Define the attack function here
    pub fn attack(attacker: &mut Self, defender: &mut Self) {
        todo!()
    }
}
