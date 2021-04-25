use std::fmt::{Display, Formatter, self};
use std::hash::Hash;

#[derive(Debug, PartialEq, Clone, Hash)]
pub struct Parameter {
    pub name: String,
}

impl Display for Parameter {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}