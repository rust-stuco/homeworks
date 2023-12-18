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
    fn test_takes_damage() {
        let mut eevee = Eevee::new();
        assert_eq!(eevee.health, 100);

        eevee.take_damage(20);
        assert_eq!(eevee.health, 100);

        eevee.take_damage(30);
        assert_eq!(eevee.health, 90);
    }

    #[test]
    #[should_panic(expected = "Eevee fainted!")]
    fn test_eevee_fainted() {
        let mut eevee = Eevee::new();
        assert_eq!(eevee.health, 100);

        eevee.take_damage(120);
    }

    #[test]
    fn test_eevee_evolves() {
        let alvin = Eevee::new();
        let simon = Eevee::new();
        let theodore = Eevee::new();

        let alvin = alvin.evolve(ElementalStone::PyroStone);
        let simon = simon.evolve(ElementalStone::HydroStone);
         // Supposed to be green but whatever
        let theodore = theodore.evolve(ElementalStone::MossyStone);

        assert!(matches!(alvin, EvolvedEevee::Flareon(_, _)));
        assert!(matches!(simon, EvolvedEevee::Vaporeon(_, _)));
        assert!(matches!(theodore, EvolvedEevee::Leafeon(_, _)));
    }

    #[test]
    #[should_panic(expected = "Encountered a weird rock...")]
    fn test_weird_rock() {
        let eevee = Eevee::new();

        eevee.evolve(ElementalStone::DullRock);
    }

    #[test]
    fn test_vaporean() {
        todo!()
    }

    #[test]
    fn test_flareon() {
        todo!()
    }

    #[test]
    fn test_leafeon() {
        todo!()
    }

    #[test]
    fn test_eevee_evolutions() {
        let alvin = Eevee::new();
        let simon = Eevee::new();
        let theodore = Eevee::new();

        let mut alvin = alvin.evolve(ElementalStone::PyroStone);
        let mut simon = simon.evolve(ElementalStone::HydroStone);
        let mut theodore = theodore.evolve(ElementalStone::MossyStone);

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
