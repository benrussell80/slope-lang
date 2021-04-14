use std::fmt::{Display, Formatter, self};
use super::operator::Operator;

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Identifier(String),
    IntegerLiteral(i64),
    BooleanLiteral(bool),
    RealLiteral(f64),
    UndefinedLiteral,
    Combination {
        left: Option<Box<Expression>>,
        operator: Operator,
        right: Option<Box<Expression>>
    },
    Call {
        function: Box<Expression>,
        arguments: Vec<Expression>
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use Expression::*;
        match self {
            Identifier(name) => write!(f, "{}", name),
            IntegerLiteral(value) => write!(f, "{}", value),
            BooleanLiteral(value) => write!(f, "{}", value),
            RealLiteral(value) => write!(f, "{}", value),
            UndefinedLiteral => write!(f, "undefined"),
            Combination { left, operator, right } => {
                if let Some(value) = left {
                    write!(f, "{} {} {}", left.as_ref().unwrap(), operator, right.as_ref().unwrap())
                } else {
                    write!(f, "{} {}", operator, right.as_ref().unwrap())
                }
            },
            Call { function, arguments } => {
                write!(f, "{}({})", function, arguments.iter().fold(String::new(), |mut acc, arg| {
                    if !acc.is_empty() {
                        acc.push_str(", ");
                    };
                    acc.push_str(&arg.to_string());
                    acc
                }))
            },
        }
    }
}