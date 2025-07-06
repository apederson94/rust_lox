#[derive(Debug, Clone, PartialEq)]
pub enum LoxValue {
    Number(f64),
    Str(String),
    Bool(bool),
    Nil,
}
