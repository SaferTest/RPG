#[derive(Debug, PartialEq, Eq)]
pub struct Fighter {
    pub name: String,
    pub attack: u32,
    pub hp: u32,
}

impl Fighter {
    pub fn is_alive(&self) -> bool {
        self.hp > 0
    }

    pub fn take_damage(&mut self, damage: u32) {
        self.hp = self.hp.saturating_sub(damage);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn generate_test_fighter() -> Fighter {
        Fighter {
            name: String::from("Test Dummy"),
            attack: 10,
            hp: 100,
        }
    }

    #[test]
    fn test_take_damage_zero_damage_results_in_same_hp() {
        let mut fighter = generate_test_fighter();
        let starting_hp = fighter.hp;

        fighter.take_damage(0);

        assert_eq!(fighter.hp, starting_hp);
    }

    #[test]
    fn test_take_damage_less_than_hp_results_in_reduced_hp() {
        let mut fighter = generate_test_fighter();
        let starting_hp = fighter.hp;

        fighter.take_damage(starting_hp - 20);

        assert!(fighter.hp < starting_hp);
        assert!(fighter.hp > 0);
    }

    #[test]
    fn test_take_damage_equal_to_hp_results_in_zero_hp() {
        let mut fighter = generate_test_fighter();
        let starting_hp = fighter.hp;

        fighter.take_damage(starting_hp);

        assert_eq!(fighter.hp, 0);
    }

    #[test]
    fn test_take_damage_larger_than_hp_results_in_zero_hp() {
        let mut fighter = generate_test_fighter();
        let starting_hp = fighter.hp;

        fighter.take_damage(starting_hp + 20);

        assert_eq!(fighter.hp, 0);
    }

    #[test]
    fn test_is_alive_hp_not_alteres_returns_true() {
        let fighter = generate_test_fighter();

        assert!(fighter.is_alive());
    }

    #[test]
    fn test_is_alive_reduced_hp_returns_true() {
        let mut fighter = generate_test_fighter();
        let starting_hp = fighter.hp;

        fighter.take_damage(starting_hp - 20);

        assert!(fighter.is_alive());
    }

    #[test]
    fn test_is_alive_zero_or_negative_hp_returns_false() {
        let mut fighter = generate_test_fighter();
        let starting_hp = fighter.hp;

        fighter.take_damage(starting_hp + 20);

        assert!(!fighter.is_alive());
    }
}