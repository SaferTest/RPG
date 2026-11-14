use crate::fighter;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Alignment {
    Ally,
    Opponent,
}

pub struct MemberSynopsis {
    amount: usize,
    alignment: Alignment,
}

impl MemberSynopsis {
    pub fn amount(&self) -> usize {
        self.amount
    }

    pub fn alignment(&self) -> Alignment {
        self.alignment.clone()
    }
}

pub struct Party {
    pub members: Vec<fighter::Fighter>,
    pub alignment: Alignment,
}

// TODO: enforce at least 1 member at all times
impl Party {
    pub fn new(members: Vec<fighter::Fighter>, alignment: Alignment) -> Self {
        Party {
            members,
            alignment,
        }
    }

    pub fn is_alive(&self) -> bool {
        self.members.iter().any(|member| member.is_alive())
    }

    pub fn get_member_synopsis(&self) -> MemberSynopsis {
        MemberSynopsis {
            amount: self.members.len(),
            alignment: self.alignment.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn generate_test_party() -> Party {
        Party::new(
            vec![
                fighter::Fighter {
                    name: String::from("Member 1"),
                    attack: 10,
                    hp: 100,
                },
                fighter::Fighter {
                    name: String::from("Member 2"),
                    attack: 10,
                    hp: 100,
                },
                fighter::Fighter {
                    name: String::from("Member 3"),
                    attack: 10,
                    hp: 100,
                }
            ],
            Alignment::Ally,
        )
    }

    #[test]
    fn test_is_alive_everyone_alive_returns_true() {
        let party = generate_test_party();

        assert!(party.is_alive());
    }

    #[test]
    fn test_is_alive_some_dead_returns_true() {
        let mut party = generate_test_party();

        let to_defeat = &mut party.members[0];

        to_defeat.take_damage(to_defeat.hp);

        assert!(party.is_alive());
        
        let to_defeat = &mut party.members[1];

        to_defeat.take_damage(to_defeat.hp);

        assert!(party.is_alive());
    }

    #[test]
    fn test_is_alive_everyone_dead_returns_false() {
        let mut party = generate_test_party();

        for to_defeat in party.members.iter_mut() {
            to_defeat.take_damage(to_defeat.hp);
        }

        assert!(!party.is_alive());
    }
}