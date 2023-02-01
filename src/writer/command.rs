use std::fmt::{Display, Formatter};

#[allow(dead_code)]
#[derive(Debug)]
pub enum Command {
    Add,
    Sub,
    Neg,
    Eq,
    Gt,
    Lt,
    And,
    Or,
    Not,
}

impl Command {
    pub fn from(value: char) -> Option<Self> {
        match value {
            '+' => Some(Command::Add),
            '-' => Some(Command::Sub),
            '=' => Some(Command::Eq),
            '>' => Some(Command::Gt),
            '<' => Some(Command::Lt),
            '&' => Some(Command::And),
            '|' => Some(Command::Or),
            '~' => Some(Command::Not),
            _ => None,
        }
    }
}

impl Display for Command {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self).to_lowercase())
    }
}
