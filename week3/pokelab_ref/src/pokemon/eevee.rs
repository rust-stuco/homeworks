#![allow(unused_variables, dead_code, clippy::new_without_default)]

//! This module contains the [`Eevee`] and [`EvolvedEevee`] type, as well as its
//! method implementations.
//!
//! In this module, we'll be implementing a water type pokemon that represents
//! some of the Eevee evolution line:
//!  - Eevee
//!      - Vaporeon
//!      - Jolteon
//!      - Flareon
//!
//! To model the evolution line, we use a type system that represents the possible
//! states that the Eevee can be in.
//!
//! We have an [`Eevee`] type that contains stats about the
//! Eevee, and several methods that can retrieve and modify these stats. We also have an
//! [`EvolvedEevee`] type that can be one of Vaporeon, Flareon, and Jolteon.
//!
//! An [`Eevee`] can `evolve` into an [`EvolvedEevee`], and an [`EvolvedEevee`] can `devolve`
//! back into an [`Eevee`]. Go to the documentation for each of these types to learn more.

/// This type represents a basic [`Eevee`] pokemon. It has a level, as well as health,
/// attack, and defense stats.
pub struct Eevee {
    /// For this homework, [`Eevee::level`] doesn't actually represent anything important.
    level: u8,
    health: u16,
    attack: u16,
    defense: u16,
}

/// These stones are used to evolve an [`Eevee`] into an [`EvolvedEevee`].
///
/// Don't worry too much about the `#[non_exhaustive]` attribute.
/// It's basically a way of saying that
/// there could be more variants of [`ElementalStone`] added in the future.
#[non_exhaustive]
pub enum ElementalStone {
    /// This Water Stone turns an [`Eevee`] into an [`EvolvedEevee::Vaporeon`]
    WaterStone,
    /// This Fire Stone turns an [`Eevee`] into an [`EvolvedEevee::Flareon`]
    FireStone,
    /// This Electric Stone turns an [`Eevee`] into an [`EvolvedEevee::Jolteon`]
    ElectricStone,
    DullRock,
}

/// This type represent an evolved Eevee in the form of either Vaporeon, Flareon, or Jolteon.
///
/// An [`EvolvedEevee`] contains an inner [`Eevee`] as well as a secondary attribute value.
/// This attribute value changes one of the inner [`Eevee`]'s base stats depending on which
/// of the 3 types the [`EvolvedEevee`] is.
pub enum EvolvedEevee {
    /// The secondary attribute for `Vaporeon` is added to the base health.
    Vaporeon(Eevee, u16),
    /// The secondary attribute for `Flareon` is added to the base attack.
    Flareon(Eevee, u16),
    /// The secondary attribute for `Jolteon` is added to the base defense.
    Jolteon(Eevee, u16),
}

impl Eevee {
    /// Creates an Eevee with the following base stats:
    /// - `level: 0`
    /// - `health: 100`
    /// - `attack: 55`
    /// - `defense: 20`
    ///
    /// _Note: We 0-level because we're programmers_ 😎
    ///
    /// ```
    /// use pokelab_ref::pokemon::eevee::*;
    ///
    /// let new_eevee = Eevee::new();
    /// assert_eq!(new_eevee.get_level(), 0);
    /// assert_eq!(new_eevee.get_health(), 100);
    /// assert_eq!(new_eevee.get_attack(), 55);
    /// assert_eq!(new_eevee.get_defense(), 20);
    /// ```
    pub fn new() -> Self {
        Self {
            level: 0,
            health: 100,
            attack: 55,
            defense: 20,
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

    /// Deals `damage` amount of damage to the Eevee's health.
    ///
    /// ```
    /// use pokelab_ref::pokemon::eevee::*;
    ///
    /// let mut new_eevee = Eevee::new();
    /// assert_eq!(new_eevee.get_health(), 100);
    /// assert_eq!(new_eevee.get_defense(), 20);
    ///
    /// new_eevee.take_damage(10);
    /// assert_eq!(new_eevee.get_health(), 100); // Not enough damage to overcome defense
    ///
    /// new_eevee.take_damage(30);
    /// assert_eq!(new_eevee.get_health(), 90); // 30 - 20 = 10 damage taken
    /// ```
    ///
    /// This function should panic with the message "Eevee fainted!" if it takes more damage
    /// than it has health. In other words, it should faint when it reaches 0 health.
    pub fn take_damage(&mut self, damage: u16) {
        if self.defense >= damage {
            return;
        }
        let damage_taken = damage - self.defense;
        if self.health <= damage_taken {
            panic!("Eevee fainted!");
        }
        self.health -= damage_taken;
    }

    /// Given an Elemental Stone, evolve the Eevee into an [`EvolvedEevee`].
    ///
    /// If given a stone that an Eevee cannot use to evolve, this function should
    /// panic with the message "Encountered a weird rock...".
    ///
    /// ```
    /// use pokelab_ref::pokemon::eevee::*;
    ///
    /// let mut new_eevee = Eevee::new();
    ///
    /// let vaporeon = new_eevee.evolve(ElementalStone::WaterStone);
    /// assert!(matches!(vaporeon, EvolvedEevee::Vaporeon(_, _)));
    /// ```
    pub fn evolve(self, evolution: ElementalStone) -> EvolvedEevee {
        match evolution {
            ElementalStone::WaterStone => EvolvedEevee::Vaporeon(self, 0),
            ElementalStone::FireStone => EvolvedEevee::Flareon(self, 0),
            ElementalStone::ElectricStone => EvolvedEevee::Jolteon(self, 0),
            _ => panic!("Encountered a weird rock..."),
        }
    }
}

/// There's a lot of boiler plate code here that we'll implement for you.
/// All you need to do is implement [`EvolvedEevee::take_damage`] and [`EvolvedEevee::devolve`].
///
/// In all honesty, if you had to actually implement this logic, you would probably have each
/// [`EvolvedEevee`] variant be its own standalone type,
/// each implementing something called a _trait_.
/// However, we haven't talked about traits yet, so stay tuned till week 5!
impl EvolvedEevee {
    pub fn get_level(&self) -> u8 {
        match self {
            EvolvedEevee::Vaporeon(base, _) => base.get_level(),
            EvolvedEevee::Flareon(base, _) => base.get_level(),
            EvolvedEevee::Jolteon(base, _) => base.get_level(),
        }
    }

    pub fn get_health(&self) -> u16 {
        match self {
            EvolvedEevee::Vaporeon(base, _) => base.get_health(),
            EvolvedEevee::Flareon(base, _) => base.get_health(),
            EvolvedEevee::Jolteon(base, _) => base.get_health(),
        }
    }

    pub fn get_attack(&self) -> u16 {
        match self {
            EvolvedEevee::Vaporeon(base, _) => base.get_attack(),
            EvolvedEevee::Flareon(base, _) => base.get_attack(),
            EvolvedEevee::Jolteon(base, _) => base.get_attack(),
        }
    }

    pub fn get_defense(&self) -> u16 {
        match self {
            EvolvedEevee::Vaporeon(base, _) => base.get_defense(),
            EvolvedEevee::Flareon(base, _) => base.get_defense(),
            EvolvedEevee::Jolteon(base, _) => base.get_defense(),
        }
    }

    pub fn set_secondary_attribute(&mut self, extra: u16) {
        match self {
            EvolvedEevee::Vaporeon(_, attr) => *attr = extra,
            EvolvedEevee::Flareon(_, attr) => *attr = extra,
            EvolvedEevee::Jolteon(_, attr) => *attr = extra,
        }
    }

    /// Deals `damage` amount of damage to the [`EvolvedEevee`]'s health.
    ///
    /// This is similar to [`Eevee::take_damage`], but the logic is slightly different for
    /// [`EvolvedEevee::Vaporeon`] and [`EvolvedEevee::Jolteon`], since they have
    /// extra health and extra defense, respectively.
    ///
    /// For Vaporeon, you will want to apply the damage to the extra health,
    /// until the extra health runs out. For Jolteon, you apply the extra defense
    /// on every `take_damage` call.
    ///
    /// It's also fine to just panic with the same message, "Eevee fainted!".
    ///
    /// ```
    /// use pokelab_ref::pokemon::eevee::*;
    ///
    /// let mut flareon = Eevee::new().evolve(ElementalStone::FireStone);
    /// flareon.take_damage(40);
    /// assert_eq!(flareon.get_health(), 80);
    ///
    /// let mut vaporeon = Eevee::new().evolve(ElementalStone::WaterStone);
    /// vaporeon.set_secondary_attribute(20); // Adds 20 extra health
    /// vaporeon.take_damage(40);
    /// assert_eq!(vaporeon.get_health(), 100);
    ///
    /// let mut jolteon = Eevee::new().evolve(ElementalStone::ElectricStone);
    /// jolteon.set_secondary_attribute(5); // Adds 5 extra defense
    /// for _ in 0..100 {
    ///     jolteon.take_damage(25);
    /// }
    /// assert_eq!(jolteon.get_health(), 100);
    /// ```
    pub fn take_damage(&mut self, damage: u16) {
        match self {
            EvolvedEevee::Vaporeon(base, extra_health) => {
                if damage < *extra_health {
                    *extra_health -= damage;
                } else {
                    base.take_damage(damage - *extra_health);
                    *extra_health = 0;
                }
            }
            EvolvedEevee::Flareon(base, _) => base.take_damage(damage),
            EvolvedEevee::Jolteon(base, extra_defense) => {
                // Only need to take inner damage if it exceeds our extra defense
                if damage > *extra_defense {
                    base.take_damage(damage - *extra_defense);
                }
            }
        }
    }

    /// Devolves an [`EvolvedEevee`] into an [`Eevee`].
    ///
    /// ```
    /// use pokelab_ref::pokemon::eevee::*;
    ///
    /// let jolteon = Eevee::new().evolve(ElementalStone::ElectricStone);
    /// assert!(matches!(jolteon, EvolvedEevee::Jolteon(_, _)));
    ///
    /// let back_to_eevee: Eevee = jolteon.devolve();
    /// ```
    pub fn devolve(self) -> Eevee {
        match self {
            EvolvedEevee::Vaporeon(base, _) => base,
            EvolvedEevee::Flareon(base, _) => base,
            EvolvedEevee::Jolteon(base, _) => base,
        }
    }
}
