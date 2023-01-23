use crate::symbol_table::kind::Kind;

#[derive(Debug, PartialEq)]
pub struct Symbol {
    pub type_name: String,
    pub kind: Kind,
    pub index: usize,
}

impl Symbol {
    #[allow(dead_code)]
    pub fn new(type_name: &str, kind: &Kind, index: usize) -> Self {
        Symbol {
            type_name: String::from(type_name),
            kind: Kind::from(kind),
            index,
        }
    }
}
