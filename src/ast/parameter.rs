use std::fmt::{Display, Formatter, self};

#[derive(Debug, PartialEq, Clone)]
pub struct Parameter {
    pub name: String,
}

impl Display for Parameter {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}