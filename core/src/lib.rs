use serde::{Deserialize, Serialize};
use std::fmt;

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

impl fmt::Display for Sexpr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Sexpr::Integer(int) => write!(f, "{}", int),
            Sexpr::String(string) => write!(f, "\"{}\"", string),
            Sexpr::Null => write!(f, "()"),
            Sexpr::Cons(hd, tl) => {
                write!(f, "({}", hd)?;
                let mut pair = &**tl;
                while let Sexpr::Cons(hd, tl) = pair {
                    write!(f, " {}", hd)?;
                    pair = &*tl;
                }
                if pair != &Sexpr::Null {
                    write!(f, " . {}", pair)?;
                }
                write!(f, ")")
            },
        }
    }
}
