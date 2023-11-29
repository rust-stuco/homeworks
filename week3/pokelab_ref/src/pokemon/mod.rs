/// In this module, we'll be implementing a water type pokemon that represents
/// some of the Eevee evolution line:
///  - Eevee
///  - Vaporeon
///  - Jolteon
///  - Flareon
///
/// To model the evolution line, we want to design a struct that acts like a state machine.
/// The struct will have a `EeveeEvolution` enum attribute that will
/// represent the current state of the pokemon.
///
/// In our model we'll allow Eevee to evolve into Vaporeon, Jolteon, or Flareon,
/// as well as "devolve" back into Eevee.
pub mod eevee;
pub mod firetype;
mod tests;
