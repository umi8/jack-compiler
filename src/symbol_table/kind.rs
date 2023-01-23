use std::fmt;
use std::fmt::Formatter;

use anyhow::{bail, Error, Result};

#[derive(Debug, PartialEq)]
pub enum Kind {
    Static,
    Field,
    Argument,
    Var,
}

impl Kind {
    pub fn from_str(value: &str) -> Result<Kind> {
        match value {
            "static" => Ok(Kind::Static),
            "filed" => Ok(Kind::Field),
            "argument" => Ok(Kind::Argument),
            "var" => Ok(Kind::Var),
            _ => bail!(Error::msg(format!("Illegal Argument Error: {}", value))),
        }
    }

    pub fn from(kind: &Kind) -> Kind {
        match kind {
            Kind::Static => Kind::Static,
            Kind::Field => Kind::Field,
            Kind::Argument => Kind::Argument,
            Kind::Var => Kind::Var,
        }
    }
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
