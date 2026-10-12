#[derive(Debug)]
struct Fighter {
    name: String,
    attack: u32,
    hp: u32,
}

impl Fighter {
    fn is_alive(&self) -> bool {
        self.hp > 0
    }

    fn take_damage(&mut self, damage: u32) {
        self.hp = self.hp.saturating_sub(damage);
    }
}

fn main() {
    let mut hero = Fighter {
        name: String::from("Hero"),
        attack: 10,
        hp: 100,
    };
    let mut support = Fighter {
        name: String::from("Support"),
        attack: 8,
        hp: 120,
    };
    let mut thug_leader = Fighter {
        name: String::from("Thug Leader"),
        attack: 12,
        hp: 110,
    };
    let mut thug = Fighter {
        name: String::from("Thug"),
        attack: 8,
        hp: 90,
    };

    let mut turn: u32 = 1;

    while (hero.is_alive() || support.is_alive()) && (thug_leader.is_alive() || thug.is_alive()) {
        println!("Turn {}", turn);

        if turn % 4 == 1 && hero.is_alive() {
            if !thug_leader.is_alive() {
                attack(&hero, &mut thug);
            }
            else if !thug.is_alive() {
                attack(&hero, &mut thug_leader);
            }
            else if thug_leader.hp < thug.hp {
                attack(&hero, &mut thug_leader);
            }
            else {
                attack(&hero, &mut thug);
            }
        }
        else if turn % 4 == 2 && thug_leader.is_alive() {
            if !hero.is_alive() {
                attack(&thug_leader, &mut support);
            }
            else if !support.is_alive() {
                attack(&thug_leader, &mut hero);
            }
            else if hero.hp < support.hp {
                attack(&thug_leader, &mut hero);
            }
            else {
                attack(&thug_leader, &mut support);
            }
        }
        else if turn % 4 == 3 && support.is_alive() {
            if !thug_leader.is_alive() {
                attack(&support, &mut thug);
            }
            else if !thug.is_alive() {
                attack(&support, &mut thug_leader);
            }
            else if thug_leader.hp > thug.hp {
                attack(&support, &mut thug_leader);
            }
            else {
                attack(&support, &mut thug);
            }
        }
        else if turn % 4 == 0 && thug.is_alive() {
            if !hero.is_alive() {
                attack(&thug, &mut support);
            }
            else if !support.is_alive() {
                attack(&thug, &mut hero);
            }
            else if hero.hp > support.hp {
                attack(&thug, &mut hero);
            }
            else {
                attack(&thug, &mut support);
            }
        }

        println!();

        turn += 1;        
    }
}

fn attack(attacker: &Fighter, defender: &mut Fighter) {
    println!("{} attacks {} for {} damage!", attacker.name, defender.name, attacker.attack);

    defender.take_damage(attacker.attack);
    
    println!("{}'s HP is now {}", defender.name, defender.hp);
}