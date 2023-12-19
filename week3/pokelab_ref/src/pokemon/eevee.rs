#![allow(clippy::new_without_default)]

//! This module contains the [`Eevee`] and [`EvolvedEevee`] type, as well as its
//! method implementations.
//!
//! In this module, we'll be implementing a variable-type pokemon that represents
//! some of the Eevee evolution line:
//!  - Eevee
//!      - Vaporeon
//!      - Flareon
//!      - Leafeon
//!
//! To model the evolution line, we use a type system that represents the possible
//! states that the Eevee can be in.
//!
//! We have an [`Eevee`] type that contains stats about the
//! Eevee, and several methods that can retrieve and modify these stats. We also have an
//! [`EvolvedEevee`] type that can be one of Vaporeon, Flareon, and Leafeon.
//!
//! An [`Eevee`] can [`evolve`](Eevee::evolve) into an [`EvolvedEevee`],
//! and an [`EvolvedEevee`] can [`devolve`](EvolvedEevee::devolve)
//! back into an [`Eevee`].
//!
//! See the documentation below for each of these types to learn more!

/// This type represents a basic [`Eevee`] pokemon. It has a level, as well as health,
/// attack, and defense stats.
///
/// Something to note is that while it is possible to implement getters and setters for struct
/// fields as if they were objects in OOP-paradigm languages, it is generally unnecessary for Rust.
/// It can even get you into trouble with the borrow checker when dealing
/// with fields that are references (we may talk about this when we get to _lifetimes_ in week 7).
///
/// Marking a field as `pub`, coupled with the borrow checker, will give you very similar
/// semantics as to normal getters and setters.
///
/// There are, of course, places where you _do_ want these.
/// And when we talk about traits in week 5,
/// we will _need_ to have them if we want to share behavior among types.
#[derive(Debug, PartialEq, Eq)]
pub struct Eevee {
    /// For this part of the homework,
    /// [`level`](Eevee::level) doesn't actually represent anything important.
    pub level: u8,
    pub health: u16,
    pub attack: u16,
    pub defense: u16,
}

/// These stones are used to evolve an [`Eevee`] into an [`EvolvedEevee`].
///
/// Don't worry too much about the `#[non_exhaustive]` attribute.
/// It's basically a way of saying that
/// there could be more variants of [`ElementalStone`] added in the future.
#[non_exhaustive]
pub enum ElementalStone {
    /// This Water Stone turns an [`Eevee`] into an [`Vaporeon`](EvolvedEevee::Vaporeon)
    HydroStone,
    /// This Fire Stone turns an [`Eevee`] into an [`Flareon`](EvolvedEevee::Flareon)
    PyroStone,
    /// This Electric Stone turns an [`Eevee`] into an [`Leafeon`](EvolvedEevee::Leafeon)
    MossyStone,
    /// Placeholder for future stones we might want to add for new evolutions!
    DullRock,
}

/// This type represent an evolved Eevee in the form of either Vaporeon, Flareon, or Leafeon.
///
/// An [`EvolvedEevee`] contains an inner [`Eevee`] as well as a secondary attribute value.
/// This attribute value changes one of the inner [`Eevee`]'s base stats depending on which
/// of the 3 types the [`EvolvedEevee`] is.
#[derive(Debug, PartialEq, Eq)]
pub enum EvolvedEevee {
    /// The secondary attribute for `Vaporeon` is added to the base health.
    Vaporeon(Eevee, u16),
    /// The secondary attribute for `Flareon` is added to the base attack.
    Flareon(Eevee, u16),
    /// The secondary attribute for `Leafeon` is added to the base defense.
    Leafeon(Eevee, u16),
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
    /// # use pokelab_ref::pokemon::eevee::*;
    /// #
    /// let new_eevee = Eevee::new();
    /// assert_eq!(new_eevee.level, 0);
    /// assert_eq!(new_eevee.health, 100);
    /// assert_eq!(new_eevee.attack, 55);
    /// assert_eq!(new_eevee.defense, 20);
    /// ```
    pub fn new() -> Self {
        Self {
            level: 0,
            health: 100,
            attack: 55,
            defense: 20,
        }
    }

    /// Deals `damage` amount of damage to the Eevee's health.
    ///
    /// ```
    /// # use pokelab_ref::pokemon::eevee::*;
    /// #
    /// let mut new_eevee = Eevee::new();
    /// assert_eq!(new_eevee.health, 100);
    /// assert_eq!(new_eevee.defense, 20);
    ///
    /// new_eevee.take_damage(10);
    /// assert_eq!(new_eevee.health, 100); // Not enough damage to overcome defense
    ///
    /// new_eevee.take_damage(30);
    /// assert_eq!(new_eevee.health, 90); // 30 - 20 = 10 damage taken
    /// ```
    ///
    /// This function should panic with the message `"Eevee fainted!"` if it takes more damage
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
    /// panic with the message `"Encountered a weird rock..."`.
    ///
    /// ```
    /// # use pokelab_ref::pokemon::eevee::*;
    /// #
    /// let mut new_eevee = Eevee::new();
    ///
    /// let vaporeon = new_eevee.evolve(ElementalStone::HydroStone);
    /// assert!(matches!(vaporeon, EvolvedEevee::Vaporeon(_, _)));
    /// ```
    pub fn evolve(self, evolution: ElementalStone) -> EvolvedEevee {
        match evolution {
            ElementalStone::HydroStone => EvolvedEevee::Vaporeon(self, 0),
            ElementalStone::PyroStone => EvolvedEevee::Flareon(self, 0),
            ElementalStone::MossyStone => EvolvedEevee::Leafeon(self, 0),
            _ => panic!("Encountered a weird rock..."),
        }
    }
}

/// There's a lot of boiler plate code here that we'll implement for you.
/// All you need to do is implement
/// [`take_damage`](EvolvedEevee::take_damage) and [`devolve`](EvolvedEevee::devolve).
///
/// In all honesty, if you had to actually implement this logic, you would probably have each
/// [`EvolvedEevee`] variant be its own standalone type,
/// each implementing something called a _trait_.
/// However, we haven't talked about traits yet, so stay tuned till week 5!
impl EvolvedEevee {
    pub fn get_level(&self) -> u8 {
        match self {
            EvolvedEevee::Vaporeon(base, _) => base.level,
            EvolvedEevee::Flareon(base, _) => base.level,
            EvolvedEevee::Leafeon(base, _) => base.level,
        }
    }

    pub fn get_health(&self) -> u16 {
        match self {
            EvolvedEevee::Vaporeon(base, h) => base.health + h,
            EvolvedEevee::Flareon(base, _) => base.health,
            EvolvedEevee::Leafeon(base, _) => base.health,
        }
    }

    pub fn get_attack(&self) -> u16 {
        match self {
            EvolvedEevee::Vaporeon(base, _) => base.attack,
            EvolvedEevee::Flareon(base, a) => base.attack + a,
            EvolvedEevee::Leafeon(base, _) => base.attack,
        }
    }

    pub fn get_defense(&self) -> u16 {
        match self {
            EvolvedEevee::Vaporeon(base, _) => base.defense,
            EvolvedEevee::Flareon(base, _) => base.defense,
            EvolvedEevee::Leafeon(base, d) => base.defense + d,
        }
    }

    pub fn set_secondary_attribute(&mut self, extra: u16) {
        match self {
            EvolvedEevee::Vaporeon(_, attr) => *attr = extra,
            EvolvedEevee::Flareon(_, attr) => *attr = extra,
            EvolvedEevee::Leafeon(_, attr) => *attr = extra,
        }
    }

    /// Deals `damage` amount of damage to the [`EvolvedEevee`]'s health.
    ///
    /// This is similar to [`Eevee::take_damage`],
    /// but the logic is slightly different for [`Vaporeon`](EvolvedEevee::Vaporeon)
    /// and [`Flareon`](EvolvedEevee::Flareon), since they have
    /// extra health and extra defense, respectively.
    ///
    /// For Vaporeon, you will want to apply the damage to the extra health,
    /// until the extra health runs out. For Leafeon, you apply the extra defense
    /// on every [`take_damage`](EvolvedEevee::take_damage) call.
    ///
    /// It's also fine to just panic with the same message, `"Eevee fainted!"`.
    ///
    /// ```
    /// # use pokelab_ref::pokemon::eevee::*;
    /// #
    /// let mut flareon = Eevee::new().evolve(ElementalStone::PyroStone);
    /// flareon.take_damage(40);
    /// assert_eq!(flareon.get_health(), 80);
    ///
    /// let mut vaporeon = Eevee::new().evolve(ElementalStone::HydroStone);
    /// vaporeon.set_secondary_attribute(20); // Adds 20 extra health
    /// vaporeon.take_damage(40);
    /// assert_eq!(vaporeon.get_health(), 100);
    ///
    /// let mut leafeon = Eevee::new().evolve(ElementalStone::MossyStone);
    /// leafeon.set_secondary_attribute(5); // Adds 5 extra defense
    /// for _ in 0..100 {
    ///     leafeon.take_damage(25);
    /// }
    /// assert_eq!(leafeon.get_health(), 100);
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
            EvolvedEevee::Leafeon(base, extra_defense) => {
                // Only need to take inner damage if it exceeds our extra defense
                if damage > *extra_defense {
                    base.take_damage(damage - *extra_defense);
                }
            }
        }
    }

    /// Devolves an [`EvolvedEevee`] into an [`Eevee`].
    ///
    /// Note that base stats like health should not change even after devolving.
    ///
    /// ```
    /// # use pokelab_ref::pokemon::eevee::*;
    /// #
    /// let leafeon = Eevee::new().evolve(ElementalStone::MossyStone);
    /// assert!(matches!(leafeon, EvolvedEevee::Leafeon(_, _)));
    ///
    /// let back_to_eevee: Eevee = leafeon.devolve();
    /// ```
    pub fn devolve(self) -> Eevee {
        match self {
            EvolvedEevee::Vaporeon(base, _) => base,
            EvolvedEevee::Flareon(base, _) => base,
            EvolvedEevee::Leafeon(base, _) => base,
        }
    }
}
