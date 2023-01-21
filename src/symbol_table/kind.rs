#[derive(Debug, PartialEq)]
pub enum Kind {
    Static,
    Field,
    Argument,
    Var,
}
