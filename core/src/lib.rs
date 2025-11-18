use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub enum Sexpr {
    Integer(i32),
    String(String),
    Null,
    Cons(Box<Sexpr>, Box<Sexpr>),
}

impl From<Vec<Sexpr>> for Sexpr {
    fn from(vec: Vec<Sexpr>) -> Self {
        let mut sexpr = Self::Null;
        for elt in vec.into_iter().rev() {
            sexpr = Self::Cons(Box::new(elt), Box::new(sexpr));
        }
        sexpr
    }
}

impl From<i32> for Sexpr {
    fn from(value: i32) -> Self {
        Sexpr::Integer(value)
    }
}

impl From<String> for Sexpr {
    fn from(value: String) -> Self {
        Sexpr::String(value)
    }
}
