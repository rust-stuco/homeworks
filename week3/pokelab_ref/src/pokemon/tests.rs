#[cfg(test)]
mod eevee_tests {
    use crate::pokemon::eevee::Eevee;

    #[test]
    fn test_eevee_new() {
        let eevee = Eevee::new();
        assert_eq!(eevee.get_level(), 1);
        assert_eq!(eevee.get_health(), 55);
        assert_eq!(eevee.get_attack(), 55);
        assert_eq!(eevee.get_defense(), 50);
        assert_eq!(eevee.get_type(), "Eevee");
    }

    #[test]
    fn test_eevee_evolution_vaporeon() {
        let alvin = Eevee::new();
        let simon = Eevee::new();
        let mut trials = 0;

        while trials < 10 {
            alvin = alvin.evolve();
            simon = simon.evolve();
            assert_eq!(alvin.get_type(), "Vaporeon");
            assert_eq!(simon.get_type(), "Vaporeon");

            if simon.get_attack() != alvin.get_attack() {
                break;
            }

            alvin = alvin.devolve();
            simon = simon.devolve();

            trials += 1;
        }

        assert_ne!(simon.get_attack(), alvin.get_attack());
        assert_ne!(simon.get_defense(), alvin.get_defense());
        assert_ne!(simon.get_health(), alvin.get_health());
    }

    #[test]
    fn test_eevee_evolution_jolteon() {
        let bob = Eevee::new();
        let billy = Eevee::new();

        let bob = bob.level_up(6);
        let billy = billy.level_up(6);

        let mut trials = 0;
        while trials < 10 {
            bob = bob.evolve();
            billy = billy.evolve();
            assert_eq!(bob.get_type(), "Jolteon");
            assert_eq!(billy.get_type(), "Jolteon");

            if billy.get_attack() != bob.get_attack() {
                break;
            }

            bob = bob.devolve();
            billy = billy.devolve();

            trials += 1;
        }

        assert_ne!(billy.get_attack(), bob.get_attack());
        assert_ne!(billy.get_defense(), bob.get_defense());
        assert_ne!(billy.get_health(), bob.get_health());
    }

    #[test]
    fn test_eevee_evolution_flareon() {
        let bob = Eevee::new();
        let billy = Eevee::new();

        let bob = bob.level_up(6).level_up(6);
        let billy = billy.level_up(6).level_up(6);

        let mut trials = 0;
        while trials < 10 {
            bob = bob.evolve();
            billy = billy.evolve();
            assert_eq!(bob.get_type(), "Jolteon");
            assert_eq!(billy.get_type(), "Jolteon");

            if billy.get_attack() != bob.get_attack() {
                break;
            }

            bob = bob.devolve();
            billy = billy.devolve();

            trials += 1;
        }

        assert_ne!(billy.get_attack(), bob.get_attack());
        assert_ne!(billy.get_defense(), bob.get_defense());
        assert_ne!(billy.get_health(), bob.get_health());
    }

    #[test]
    fn test_eevee_level() {
        let mut eevee = Eevee::new();
        assert_eq!(eevee.get_level(), 1);
        eevee = eevee.level_up(5);
        assert_eq!(eevee.get_level(), 6);
        eevee = eevee.level_up(5);
        assert_eq!(eevee.get_level(), 11);
        eevee = eevee.level_up(5);
        assert_eq!(eevee.get_level(), 16);
    }

    #[test]
    fn test_eevee_level_evolve_ranges() {
        let mut eevee = Eevee::new();
        while eevee.get_level() < 5 {
            eevee = eevee.evolve();
            assert_eq!(eevee.get_type(), "Vaporeon");
            eevee.devolve();
            assert_eq!(eevee.get_type(), "Eevee");
            eevee = eevee.level_up(1);
        }

        while eevee.get_level() < 10 {
            eevee = eevee.evolve();
            assert_eq!(eevee.get_type(), "Jolteon");
            eevee.devolve();
            assert_eq!(eevee.get_type(), "Eevee");
            eevee = eevee.level_up(1);
        }

        while eevee.get_level() < 15 {
            eevee = eevee.evolve();
            assert_eq!(eevee.get_type(), "Flareon");
            eevee.devolve();
            assert_eq!(eevee.get_type(), "Eevee");
            eevee = eevee.level_up(1);
        }
    }
}

#[cfg(test)]
mod charmander_tests {
    use crate::pokemon::firetype::Charmander;

    #[test]
    fn test_charmander_new() {
        let charmander = Charmander::new(String::from("Charmander"));
        assert_eq!(charmander.get_level(), 1);
        assert_eq!(charmander.get_health(), 100);
        assert_eq!(charmander.get_attack(), 52);
        assert_eq!(charmander.get_defense(), 43);
    }

    #[test]
    fn test_charmander_level() {
        let mut charmander = Charmander::new(String::from("Charmander"));
        assert_eq!(charmander.get_level(), 1);
        charmander.level_up(5);
        assert_eq!(charmander.get_level(), 6);
        charmander.level_up(5);
        assert_eq!(charmander.get_level(), 11);
        charmander.level_up(5);
        assert_eq!(charmander.get_level(), 16);
    }

    // The attack value is calculated by the following formula: (attack * level) / 2
    #[test]
    fn test_charmander_level_up() {
        let mut charmander = Charmander::new(String::from("Charmander"));
        charmander.level_up(1);
        assert_eq!(charmander.get_attack(), 62);
        assert_eq!(charmander.get_defense(), 53);
        assert_eq!(charmander.get_health(), 100);
    }

    #[test]
    fn test_charmander_attacks() {
        let mut attacker = Charmander::new(String::from("Attacker"));
        let mut defender = Charmander::new(String::from("Defender"));

        attack(&mut attacker, &mut defender);
        assert_eq!(defender.get_health(), 100);

        attacker.level_up(1);
        attack(&mut attacker, &mut defender);
        assert_eq!(defender.get_health(), 23);
    }
}
