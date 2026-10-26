use entities;

use entities::fighter::Fighter;
use entities::party::Party;

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

    let mut hero_party = Party { members: vec![hero, support] };
    let mut enemy_party = Party { members: vec![thug_leader, thug] };

    let mut turn: u32 = 1;

    while hero_party.is_alive() && enemy_party.is_alive() {
        println!("Turn {}", turn);

        if turn % 4 == 1 {
            let attacker: &Fighter = &hero_party.members[0];

            if attacker.is_alive() {
                let possible_targets: Vec<&mut Fighter> = enemy_party.members.iter_mut().collect();

                if let Some(target) = target_lowest_hp(possible_targets) {
                    attack(attacker, target);
                }
                else {
                    break;
                }
            }
        }
        else if turn % 4 == 2 {
            let attacker: &Fighter = &enemy_party.members[0];

            if attacker.is_alive() {
                let possible_targets: Vec<&mut Fighter> = hero_party.members.iter_mut().collect();

                if let Some(target) = target_lowest_hp(possible_targets) {
                    attack(attacker, target);
                }
                else {
                    break;
                }
            }
        }
        else if turn % 4 == 3 {
            let attacker: &Fighter = &hero_party.members[1];

            if attacker.is_alive() {
                let possible_targets: Vec<&mut Fighter> = enemy_party.members.iter_mut().collect();

                if let Some(target) = target_highest_hp(possible_targets) {
                    attack(attacker, target);
                }
                else {
                    break;
                }
            }
        }
        else if turn % 4 == 0 {
            let attacker: &Fighter = &enemy_party.members[1];

            if attacker.is_alive() {
                let possible_targets: Vec<&mut Fighter> = hero_party.members.iter_mut().collect();

                if let Some(target) = target_highest_hp(possible_targets) {
                    attack(attacker, target);
                }
                else {
                    break;
                }
            }
        }

        println!();

        turn += 1;        
    }
}

/*
    TODO: refactor targetting functions to:
        - return a usize instead of a fighter
        - use closures to determine target
        - rethink borrowing/ownership
 */
fn target_lowest_hp<'a>(fighters: Vec<&'a mut Fighter>) -> Option<&'a mut Fighter>
{
    fighters.into_iter()
        .filter(|f| f.is_alive())
        .min_by_key(|f| f.hp)
}

fn target_highest_hp<'a>(fighters: Vec<&'a mut Fighter>) -> Option<&'a mut Fighter>
{
    fighters.into_iter()
        .filter(|f| f.is_alive())
        .max_by_key(|f| f.hp)
}

fn attack(attacker: &Fighter, defender: &mut Fighter) {
    println!("{} attacks {} for {} damage!", attacker.name, defender.name, attacker.attack);

    defender.take_damage(attacker.attack);
    
    println!("{}'s HP is now {}", defender.name, defender.hp);
}