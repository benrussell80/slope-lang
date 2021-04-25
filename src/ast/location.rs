use std::fmt::{Display, Formatter, self};

#[derive(Debug, PartialEq, Clone, Copy, Hash)]
pub enum Location {
    Prefix,
    Infix,
    Postfix,
}

impl Display for Location {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use Location::*;
        match self {
            Prefix => write!(f, "Prefix"),
            Infix => write!(f, "Infix"),
            Postfix => write!(f, "Postfix"),
        }
    }
}