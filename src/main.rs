mod fighter;

fn main() {
    let mut hero = fighter::Fighter {
        name: String::from("Hero"),
        attack: 10,
        hp: 100,
    };
    let mut support = fighter::Fighter {
        name: String::from("Support"),
        attack: 8,
        hp: 120,
    };
    let mut thug_leader = fighter::Fighter {
        name: String::from("Thug Leader"),
        attack: 12,
        hp: 110,
    };
    let mut thug = fighter::Fighter {
        name: String::from("Thug"),
        attack: 8,
        hp: 90,
    };

    let mut turn: u32 = 1;

    while (hero.is_alive() || support.is_alive()) && (thug_leader.is_alive() || thug.is_alive()) {
        println!("Turn {}", turn);

        if turn % 4 == 1 && hero.is_alive() {
            let attacker: &fighter::Fighter = &hero;
            let possible_targets: Vec<&mut fighter::Fighter> = vec![&mut thug_leader, &mut thug];

            if let Some(target) = target_lowest_hp(possible_targets) {
                attack(attacker, target);
            }
            else {
                break;
            }
        }
        else if turn % 4 == 2 && thug_leader.is_alive() {
            let attacker: &fighter::Fighter = &thug_leader;
            let possible_targets: Vec<&mut fighter::Fighter> = vec![&mut hero, &mut support];

            if let Some(target) = target_lowest_hp(possible_targets) {
                attack(attacker, target);
            }
            else {
                break;
            }
        }
        else if turn % 4 == 3 && support.is_alive() {
            let attacker: &fighter::Fighter = &support;
            let possible_targets: Vec<&mut fighter::Fighter> = vec![&mut thug_leader, &mut thug];

            if let Some(target) = target_highest_hp(possible_targets) {
                attack(attacker, target);
            }
            else {
                break;
            }
        }
        else if turn % 4 == 0 && thug.is_alive() {
            let attacker: &fighter::Fighter = &thug;
            let possible_targets: Vec<&mut fighter::Fighter> = vec![&mut hero, &mut support];

            if let Some(target) = target_highest_hp(possible_targets) {
                attack(attacker, target);
            }
            else {
                break;
            }
        }

        println!();

        turn += 1;        
    }
}

fn target_lowest_hp(fighters: Vec<&mut fighter::Fighter>) -> Option<&mut fighter::Fighter>
{
    let mut lowest_hp = u32::MAX;
    let mut lowest_hp_fighter: Option<&mut fighter::Fighter> = None;

    for fighter in fighters {
        if !fighter.is_alive()
        {
            continue;
        }

        if fighter.hp < lowest_hp {
            lowest_hp = fighter.hp;
            lowest_hp_fighter = Some(fighter);
        }
    }

    lowest_hp_fighter
}

fn target_highest_hp(fighters: Vec<&mut fighter::Fighter>) -> Option<&mut fighter::Fighter>
{
    let mut highest_hp = 0;
    let mut highest_hp_fighter: Option<&mut fighter::Fighter> = None;

    for fighter in fighters {
        if !fighter.is_alive()
        {
            continue;
        }

        if fighter.hp > highest_hp {
            highest_hp = fighter.hp;
            highest_hp_fighter = Some(fighter);
        }
    }

    highest_hp_fighter
}

fn attack(attacker: &fighter::Fighter, defender: &mut fighter::Fighter) {
    println!("{} attacks {} for {} damage!", attacker.name, defender.name, attacker.attack);

    defender.take_damage(attacker.attack);
    
    println!("{}'s HP is now {}", defender.name, defender.hp);
}