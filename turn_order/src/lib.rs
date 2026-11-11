use entities;

use entities::fighter::Fighter;
use entities::party::Party;

// TODO: use vector of indexes instead of references
pub struct TurnOrder<'a> {
    pub order: Vec<&'a Fighter>,
    current_index: usize,
}

impl<'a> TurnOrder<'a> {
    pub fn new(party1 : &'a Party, party2 : &'a Party) -> Self {
        let mut order = Vec::new();
        order.extend(party1.members.iter());
        order.extend(party2.members.iter());

        TurnOrder {
            order,
            current_index: 0,
        }
    }

    pub fn get_current_fighter(&mut self) -> &Fighter {
        self.order[self.current_index]
    }

    pub fn advance_turn(&mut self) {
        self.current_index =
             if self.current_index + 1 >= self.order.len() {
                0
            } else {
                self.current_index + 1
            };
    }

    pub fn remove_fighter(&mut self, fighter: &Fighter) {
        let index = self.order.iter().position(|&f| f == fighter);

        if let Some(i) = index {
            self.order.remove(i);
            
            if self.current_index >= i {
                self.current_index -= 1;
            }
        }
    }
}