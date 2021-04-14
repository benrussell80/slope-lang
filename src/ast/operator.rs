use std::fmt::{Display, Formatter, self};
use crate::interpreter::token::Token;
use super::location::Location;
use super::precedence::Precedence;
use std::error::Error;


#[derive(Debug, PartialEq, Clone)]
pub struct Operator(pub Token, pub Location);

impl Display for Operator {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Operator {
    pub fn precedence(&self) -> Result<Precedence, Box<dyn Error>> {
        match self {
            // prefix operations
            Self(Token::Not, Location::Prefix) => Ok(Precedence::Not),
            Self(Token::Minus, Location::Prefix) => Ok(Precedence::Not),

            // infix operations
            Self(Token::And, Location::Infix)
            | Self(Token::Or, Location::Infix)
            | Self(Token::Xor, Location::Infix) => Ok(Precedence::AndOrXor),

            Self(Token::LessThan, Location::Infix)
            | Self(Token::LessThanEquals, Location::Infix)
            | Self(Token::GreaterThan, Location::Infix)
            | Self(Token::GreaterThanEquals, Location::Infix)
            | Self(Token::Equals, Location::Infix)
            | Self(Token::NotEquals, Location::Infix)
            | Self(Token::Question, Location::Infix)
            | Self(Token::In, Location::Infix) => Ok(Precedence::LessGreaterEqualCoa),

            Self(Token::Plus, Location::Infix)
            | Self(Token::Minus, Location::Infix)
            | Self(Token::PlusMinus, Location::Infix)
            | Self(Token::MinusPlus, Location::Infix) => Ok(Precedence::PlusMinus),

            Self(Token::Multiply, Location::Infix)
            | Self(Token::Division, Location::Infix)
            | Self(Token::Modulo, Location::Infix) => Ok(Precedence::MultDivMod),

            Self(Token::As, Location::Infix) => Ok(Precedence::As),

            Self(Token::Exponent, Location::Infix) => Ok(Precedence::Exponent),

            Self(Token::LeftParen, Location::Infix) => Ok(Precedence::Call),

            // postfix operations
            Self(Token::Bang, Location::Postfix) => Ok(Precedence::Postfix),

            _ => Err(format!(
                "Illegal combination of token and location: token={:?}, location={:?}.",
                &self.0, &self.1
            )
            .into()),
        }
    }
}
