use std::ops::Neg;

pub enum LoxValue {
    Number(f64),
    Str(String),
    Bool(bool),
    Nil,
}
