#[cfg(test)]
mod charmander_tests {
    use crate::pokemon::charmander::*;

    #[test]
    fn test_charmander_new() {
        let charmander = Charmander::new(String::from("Ben"));
        assert_eq!(charmander.get_health(), 100);
        assert_eq!(charmander.get_attack(), 42);
        assert_eq!(charmander.get_defense(), 33);
    }

    #[test]
    fn test_level_up() {
        let mut charmander = Charmander::new(String::from("Steve"));

        charmander.level_up(3);
        assert_eq!(charmander.get_health(), 115); // +15
        assert_eq!(charmander.get_attack(), 51); // +9
        assert_eq!(charmander.get_defense(), 45); // +12

        charmander.level_up(12); // 15 total levels
        assert_eq!(charmander.get_health(), 175); // +75
        assert_eq!(charmander.get_attack(), 87); // +45
        assert_eq!(charmander.get_defense(), 93); // +60
    }

    #[test]
    fn test_take_damage() {
        let mut charmander = Charmander::new(String::from("Ash"));

        charmander.take_damage(10);
        assert_eq!(charmander.get_health(), 100);

        charmander.take_damage(40);
        assert_eq!(charmander.get_health(), 93);

        charmander.level_up(2);
        assert_eq!(charmander.get_health(), 103); // Health should increase

        charmander.take_damage(44);
        assert_eq!(charmander.get_health(), 100);

        charmander.level_up(1000);
        charmander.take_damage(3000);
        assert_eq!(charmander.get_health(), 5100);
    }

    #[test]
    #[should_panic(expected = "C++ fainted!")]
    fn test_should_faint() {
        let mut compiler = Charmander::new(String::from("C++"));
        compiler.take_damage(1000);
    }

    #[test]
    fn test_fight() {
        let mut charmander1 = Charmander::new(String::from("David"));
        let mut charmander2 = Charmander::new(String::from("Connor"));

        Charmander::fight(&mut charmander1, &mut charmander2);
        assert_eq!(charmander1.get_health(), 91);
        assert_eq!(charmander2.get_health(), 91);

        charmander1.level_up(4);
        assert_eq!(charmander1.get_health(), 111); // +20
        assert_eq!(charmander1.get_attack(), 54); // +12
        assert_eq!(charmander1.get_defense(), 49); // +26

        Charmander::fight(&mut charmander1, &mut charmander2);
        assert_eq!(charmander1.get_health(), 111); // no damage taken
        assert_eq!(charmander2.get_health(), 70); // 54 - 33 = 21 damage taken
    }
}

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
    fn test_vaporeon() {
        let eevee = Eevee::new();

        let mut vaporeon = eevee.evolve(ElementalStone::HydroStone);

        assert_eq!(vaporeon, EvolvedEevee::Vaporeon(Eevee::new(), 0));

        vaporeon.set_secondary_attribute(10);
        assert_eq!(vaporeon.get_health(), 110);

        vaporeon.take_damage(50);
        // 50 damage mitigated by 20 defense and 10 overhealth results in 20 actual damage taken
        assert_eq!(vaporeon.get_health(), 80);

        // Even though we devolve, we still need to keep the damage taken
        let new_eevee = vaporeon.devolve();
        assert_eq!(
            new_eevee,
            Eevee {
                health: 80,
                ..Eevee::new()
            }
        )
    }

    #[test]
    fn test_flareon() {
        let eevee = Eevee::new();

        let mut flareon = eevee.evolve(ElementalStone::PyroStone);

        assert_eq!(flareon, EvolvedEevee::Flareon(Eevee::new(), 0));

        flareon.set_secondary_attribute(10);
        assert_eq!(flareon.get_attack(), 65);

        flareon.take_damage(50);
        // 50 damage mitigated by 20 defense results in 30 actual damage taken
        assert_eq!(flareon.get_health(), 70);

        let new_eevee = flareon.devolve();
        assert_eq!(
            new_eevee,
            Eevee {
                health: 70,
                ..Eevee::new()
            }
        )
    }

    #[test]
    fn test_leafeon() {
        let eevee = Eevee::new();

        let mut leafeon = eevee.evolve(ElementalStone::MossyStone);

        assert_eq!(leafeon, EvolvedEevee::Leafeon(Eevee::new(), 0));

        leafeon.set_secondary_attribute(10);
        assert_eq!(leafeon.get_defense(), 30);

        leafeon.take_damage(50);
        // 50 damage mitigated by 30 defense results in 20 actual damage taken
        assert_eq!(leafeon.get_health(), 80);

        let new_eevee = leafeon.devolve();
        assert_eq!(
            new_eevee,
            Eevee {
                health: 80,
                ..Eevee::new()
            }
        )
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
