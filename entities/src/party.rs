mod fighter;

pub struct Party {
    pub members: Vec<fighter::Fighter>,
}

impl Party {
    pub fn is_alive(&self) -> bool {
        self.members.iter().any(|member| member.is_alive())
    }
}