use entities;
use std::cmp;

use entities::party::{MemberSynopsis, Alignment};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TurnOrderEntity {
    Ally(usize),
    Opponent(usize),
}

pub struct TurnOrder {
    pub order: Vec<TurnOrderEntity>,
    current_index: usize,
}

impl TurnOrder {
    pub fn new(synopsis_1 : &MemberSynopsis, synopsis_2 : &MemberSynopsis) -> Self {
        let mut order = Vec::new();
        let max_len = cmp::max(synopsis_1.amount(), synopsis_2.amount());

        for i in 0..max_len {
            if i < synopsis_1.amount() {
                if synopsis_1.alignment() == Alignment::Ally {
                    order.push(TurnOrderEntity::Ally(i));
                } else {
                    order.push(TurnOrderEntity::Opponent(i));
                }
            }
            if i < synopsis_2.amount() {
                if synopsis_2.alignment() == Alignment::Ally {
                    order.push(TurnOrderEntity::Ally(i));
                } else {
                    order.push(TurnOrderEntity::Opponent(i));
                }
            }
        }

        TurnOrder {
            order,
            current_index: 0,
        }
    }

    pub fn get_current_fighter(&mut self) -> TurnOrderEntity {
        self.order[self.current_index].clone()
    }

    pub fn advance_turn(&mut self) {
        self.current_index =
             if self.current_index + 1 >= self.order.len() {
                0
            } else {
                self.current_index + 1
            };
    }

    pub fn remove_fighter(&mut self, fighter_index: usize, fighter_alignment: Alignment) {
        let index = self.order.iter()
            .position(|&f| match f {
                TurnOrderEntity::Ally(i) => i == fighter_index && fighter_alignment == Alignment::Ally,
                TurnOrderEntity::Opponent(i) => i == fighter_index && fighter_alignment == Alignment::Opponent,
            });

        if let Some(i) = index {
            self.order.remove(i);
            
            if self.current_index >= i {
                if self.current_index == 0 {
                    self.current_index = self.order.len() - 1;
                } else {
                    self.current_index -= 1;
                }
            }
        }
    }
}