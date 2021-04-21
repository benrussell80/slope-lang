use std::fmt::{Display, Formatter, self};
use crate::interpreter::token::Token;
use super::location::Location;
use super::precedence::Precedence;
use super::errors::SyntaxError;


#[derive(Debug, PartialEq, Clone)]
pub struct Operator(pub Token, pub Location);

impl Display for Operator {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Operator {
    fn valid(&self) -> bool {
        match self {
            // prefix operations
            Self(Token::Not, Location::Prefix)
            | Self(Token::Minus, Location::Prefix)

            | Self(Token::And, Location::Infix)
            | Self(Token::Or, Location::Infix)
            | Self(Token::Xor, Location::Infix)
            | Self(Token::LessThan, Location::Infix)
            | Self(Token::LessThanEquals, Location::Infix)
            | Self(Token::GreaterThan, Location::Infix)
            | Self(Token::GreaterThanEquals, Location::Infix)
            | Self(Token::Equals, Location::Infix)
            | Self(Token::NotEquals, Location::Infix)
            | Self(Token::Question, Location::Infix)
            | Self(Token::In, Location::Infix)
            | Self(Token::Plus, Location::Infix)
            | Self(Token::Minus, Location::Infix)
            | Self(Token::PlusMinus, Location::Infix)
            | Self(Token::MinusPlus, Location::Infix)
            | Self(Token::Multiply, Location::Infix)
            | Self(Token::Division, Location::Infix)
            | Self(Token::Modulo, Location::Infix)
            | Self(Token::As, Location::Infix)
            | Self(Token::Exponent, Location::Infix)
            | Self(Token::LeftParen, Location::Infix)
            | Self(Token::Union, Location::Infix)
            | Self(Token::Intersection, Location::Infix)
            | Self(Token::SymmetricDifference, Location::Infix)
            
            | Self(Token::Bang, Location::Postfix) => true,

            _ => false
        }
    }

    fn err_msg(&self) -> String {
        format!(
            "Invalid location `{}` for token `{}`.",
            &self.0, &self.1
        )
    }

    pub fn new<'a>(token: Token, location: Location) -> Result<Self, SyntaxError> {
        let op = Self(token, location);
        if op.valid() {
            Ok(op)
        } else {
            Err(SyntaxError(op.err_msg()))
        }
    }

    pub fn precedence(&self) -> Result<Precedence, SyntaxError> {
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

            // set infix operations
            Self(Token::Union, Location::Infix)
            | Self(Token::Intersection, Location::Infix)
            | Self(Token::SymmetricDifference, Location::Infix) => Ok(Precedence::Exponent),

            // postfix operations
            Self(Token::Bang, Location::Postfix) => Ok(Precedence::Postfix),

            _ => Err(SyntaxError(self.err_msg()))
        }
    }
}
