#[derive(Debug)]
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