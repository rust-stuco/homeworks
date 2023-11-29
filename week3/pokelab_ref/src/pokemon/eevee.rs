/*
 * In this module we'll be implementing a water type pokemon that represents some of the Eevee evolution line:
 *  - Eevee
 *  - Vaporeon
 *  - Jolteon
 *  - Flareon
 *
 * To model the evolution line, we wan to design a struct such that it acts like a finite state machine.
 * The struct will have a state attribute that will represent the current state of the pokemon.
 *
 * In our model we'll allow Eevee to evolve into Vaporeon, Jolteon, or Flareon as well as "devolve" back into Eevee.
 *
 * The Eevee struct will have the following attributes:
 * - level: usize
 * - health: usize
 * - attack: usize
 * - defense: usize
 * - state: EeveeState (an enum that represents the current state of the pokemon)
 *
 * The EeveeState enum will have the following variants:
 * - Eevee
 * - Vaporeon(f32)
 * - Jolteon(f32)
 * - Flareon(f32)
 *
 * The usize argument represents a multiplier applied to the pokemon's base stats (health, attack, defense).
 */

enum EeveeState {
    Eevee,
}

// Define the Evee struct here
pub struct Eevee {}

impl Eevee {
    /// Define the new method here
    /// Creates an Eevee with the following base stats:
    /// - health: 55
    /// - attack: 55
    /// - defense: 50
    /// - level: 1
    /// - state: Eevee
    pub fn new() -> Eevee {}

    /// Here we want to implement the evolve method for Eevee.
    /// This method must accomplish the following:
    /// - If the current state is Eevee, evolve into a random evolution.
    /// - If the current state is an evolution (not Eevee), do nothing.
    /// Evolution will be based on the level of the pokemon.
    /// - If the level is less than 5, evolve into Vaporeon.
    /// - If the level is less than 10 but greater than 5, evolve into Jolteon.
    /// - If the level is greater 10, evolve into Flareon.
    /// The change in state will also change the pokemon's base stats by
    /// a randomly generated multiplier between 0.5 and 1.75
    /// We'll use the rand crate to generate a random number to determine which evolution to evolve into.
    /// To add the rand crate to your program run `cargo add rand` in the root directory of your project.
    /// Consult https://rust-lang-nursery.github.io/rust-cookbook/algorithms/randomness.html for more information.
    pub fn evolve(self) -> Eevee {}

    // Here we want to implement the devolve method for Eevee.
    // This method must accomplish the following:
    // - If the current state is Eevee, do nothing.
    // - If the current state is an evolution (not Eevee), devolve into Eevee.
    pub fn devolve(self) -> Eevee {}

    /// Get the type of the Eevee as a String.
    /// ex: "Eevee", "Vaporeon", "Jolteon", "Flareon"
    pub fn get_type(&self) -> String {}

    /// Get the attack value of the Eevee.
    /// The attack value is calculated by multiplying the base attack by the Eevee state multiplier.
    pub fn get_attack(&self) -> f32 {}

    /// Get the defense value of the Eevee.
    /// The defense value is calculated by multiplying the base defense by the Eevee state multiplier.
    pub fn get_defense(&self) -> f32 {}

    /// Get the health value of the Eevee.
    /// The health value is calculated by multiplying the base health by the Eevee state multiplier.
    pub fn get_health(&self) -> f32 {}

    /// Get the level of the Eevee.
    pub fn get_level(&self) -> f32 {}

    /// Level up the Eevee by l. Preserves the rest of the state.
    pub fn level_up(self, l: usize) -> Eevee {}
}
