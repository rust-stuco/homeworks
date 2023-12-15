#[cfg(test)]
mod eevee_tests {
    use crate::pokemon::eevee::*;

    #[test]
    fn test_eevee_new() {
        let eevee = Eevee::new();
        assert_eq!(eevee.level, 0);
        assert_eq!(eevee.health, 100);
        assert_eq!(eevee.attack, 55);
        assert_eq!(eevee.defense, 20);
    }

    #[test]
    fn test_eevee_evolutions() {
        let alvin = Eevee::new();
        let simon = Eevee::new();
        let theodore = Eevee::new();

        let mut alvin = alvin.evolve(ElementalStone::FireStone);
        let mut simon = simon.evolve(ElementalStone::WaterStone);
         // Supposed to be green but whatever
        let mut theodore = theodore.evolve(ElementalStone::ElectricStone);

        assert!(matches!(alvin, EvolvedEevee::Flareon(_, _)));
        assert!(matches!(simon, EvolvedEevee::Vaporeon(_, _)));
        assert!(matches!(theodore, EvolvedEevee::Jolteon(_, _)));

        alvin.set_secondary_attribute(10);
        simon.set_secondary_attribute(10);
        theodore.set_secondary_attribute(10);

        assert!(alvin.get_health() < simon.get_health());
        assert!(alvin.get_attack() > simon.get_attack());
        assert!(theodore.get_defense() > alvin.get_defense());

        let alvin = alvin.devolve();
        let simon = simon.devolve();

        assert!(alvin.attack == simon.attack);
    }

    // TODO more tests
}

// #[cfg(test)]
// mod charmander_tests {
//     use crate::pokemon::firetype::Charmander;

//     #[test]
//     fn test_charmander_new() {
//         let charmander = Charmander::new(String::from("Charmander"));
//         assert_eq!(charmander.get_level(), 1);
//         assert_eq!(charmander.get_health(), 100);
//         assert_eq!(charmander.get_attack(), 52);
//         assert_eq!(charmander.get_defense(), 43);
//     }

//     #[test]
//     fn test_charmander_level() {
//         let mut charmander = Charmander::new(String::from("Charmander"));
//         assert_eq!(charmander.get_level(), 1);
//         charmander.level_up(5);
//         assert_eq!(charmander.get_level(), 6);
//         charmander.level_up(5);
//         assert_eq!(charmander.get_level(), 11);
//         charmander.level_up(5);
//         assert_eq!(charmander.get_level(), 16);
//     }

//     // The attack value is calculated by the following formula: (attack * level) / 2
//     #[test]
//     fn test_charmander_level_up() {
//         let mut charmander = Charmander::new(String::from("Charmander"));
//         charmander.level_up(1);
//         assert_eq!(charmander.get_attack(), 62);
//         assert_eq!(charmander.get_defense(), 53);
//         assert_eq!(charmander.get_health(), 100);
//     }

//     #[test]
//     fn test_charmander_attacks() {
//         let mut attacker = Charmander::new(String::from("Attacker"));
//         let mut defender = Charmander::new(String::from("Defender"));

//         attack(&mut attacker, &mut defender);
//         assert_eq!(defender.get_health(), 100);

//         attacker.level_up(1);
//         attack(&mut attacker, &mut defender);
//         assert_eq!(defender.get_health(), 23);
//     }
// }
