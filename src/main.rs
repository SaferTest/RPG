use entities::fighter::Fighter;
use entities::party::{Party, Alignment};
use turn_order::{TurnOrder, TurnOrderEntity};

fn main() {
    let hero = Fighter {
        name: String::from("Hero"),
        attack: 10,
        hp: 100,
    };
    let support = Fighter {
        name: String::from("Support"),
        attack: 8,
        hp: 120,
    };
    let thug_leader = Fighter {
        name: String::from("Thug Leader"),
        attack: 12,
        hp: 110,
    };
    let thug = Fighter {
        name: String::from("Thug"),
        attack: 8,
        hp: 90,
    };

    let mut hero_party = Party::new(vec![hero, support], Alignment::Ally);
    let mut enemy_party = Party::new(vec![thug_leader, thug], Alignment::Opponent);

    let mut order = TurnOrder::new(&hero_party.get_member_synopsis(), &enemy_party.get_member_synopsis());

    let mut turn: u32 = 1;
    
    while hero_party.is_alive() && enemy_party.is_alive() {
        let attacker_index = order.get_current_fighter();

        match attacker_index {
            TurnOrderEntity::Ally(i) => {
                let attacker = &hero_party.members[i];

                if !attacker.is_alive() {
                    order.remove_fighter(i, Alignment::Ally);
                    order.advance_turn();
                    continue;
                }

                println!("Turn {}: {}'s turn. ", turn, attacker.name);

                let possible_targets: Vec<&mut Fighter> = enemy_party.members.iter_mut().collect();

                let target = if i == 0 {
                    targeting::target_lowest_hp(possible_targets)
                } else {
                    targeting::target_highest_hp(possible_targets)
                };

                if let Some(target) = target {
                    attack(attacker, target);
                }
                else {
                    break;
                }

                println!();

                order.advance_turn();
                turn += 1;
            },
            TurnOrderEntity::Opponent(i) => {
                let attacker = &enemy_party.members[i];

                if !attacker.is_alive() {
                    order.remove_fighter(i, Alignment::Opponent);
                    order.advance_turn();
                    continue;
                }

                println!("Turn {}: {}'s turn. ", turn, attacker.name);

                let possible_targets: Vec<&mut Fighter> = hero_party.members.iter_mut().collect();

                let target = if i == 0 {
                    targeting::target_lowest_hp(possible_targets)
                } else {
                    targeting::target_highest_hp(possible_targets)
                };

                if let Some(target) = target {
                    attack(attacker, target);
                }
                else {
                    break;
                }

                println!();

                order.advance_turn();
                turn += 1;
            },
        }     
    }
}

fn attack(attacker: &Fighter, defender: &mut Fighter) {
    println!("{} attacks {} for {} damage!", attacker.name, defender.name, attacker.attack);

    defender.take_damage(attacker.attack);
    
    println!("{}'s HP is now {}", defender.name, defender.hp);
}