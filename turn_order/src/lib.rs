use entities;

use entities::fighter::Fighter;

pub struct TurnOrder<'a> {
    pub order: Vec<&'a Fighter>,
    current_index: usize,
}

impl<'a> TurnOrder<'a> {
    pub fn get_next_fighter(&mut self) -> &Fighter {
        let index = self.current_index;

        self.current_index =
             if index + 1 >= self.order.len() {
                0
            } else {
                index + 1
            };

        self.order[index]
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