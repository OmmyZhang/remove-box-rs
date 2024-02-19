#[derive(Debug, PartialEq)]
pub enum Role {
    Local,
    // Remote,
    AI,
}

#[derive(Debug, PartialEq)]
pub enum Stage {
    Waiting,
    Press1,
    Press2,
}
