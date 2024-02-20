#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Role {
    Local,
    Remote,
    AI,
}

impl Role {
    pub fn intro(&self) -> &'static str {
        match self {
            Role::Local => "You",
            Role::Remote => "Remote",
            Role::AI => "AI",
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Stage {
    Waiting,
    Press1,
    Press2,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Mode {
    Home,
    Pve,
    Pvp,
    Online,
}
