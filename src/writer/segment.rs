use std::fmt::{Display, Formatter};

use crate::symbol_table::kind::Kind;

#[allow(dead_code)]
#[derive(Debug)]
pub enum Segment {
    Constant,
    Argument,
    Local,
    Static,
    This,
    That,
    Pointer,
    Temp,
}

impl Display for Segment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self).to_lowercase())
    }
}

impl From<&Kind> for Segment {
    fn from(value: &Kind) -> Self {
        match value {
            Kind::Static => Segment::Static,
            Kind::Field => Segment::Static,
            Kind::Argument => Segment::Argument,
            Kind::Var => Segment::Local,
        }
    }
}
