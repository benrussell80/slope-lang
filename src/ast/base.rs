pub use crate::interpreter::base::{Expression, LexerIterator, Token};
use std::error::Error;
use std::fmt::Debug;
use std::iter::Peekable;
use std::fmt::{Display, self, Formatter};

// types of statements are value assignment and function assignment
#[derive(Debug, PartialEq, Clone)]
pub struct Parameter {
    pub name: String,
}

impl Display for Parameter {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    Assignment {
        identifier: String,
        expression: Expression
    },
    FunctionDeclaration {
        identifier: String,
        parameters: Vec<Parameter>,
        expression: Expression
    },
    ExpressionStatement {
        expression: Expression,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub enum Location {
    Prefix,
    Infix,
    Postfix,
}

#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
#[repr(u8)]
pub enum Precedence {
    Lowest = 0,
    AndOrXor,
    LessGreaterEqualCoa,
    PlusMinus,
    MultDivMod,
    As,
    Not,
    Exponent,
    Postfix,
    Call,
    Hightest = u8::MAX,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Operation(pub Token, pub Location);

impl Display for Operation {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Operation {
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

            _ => Err(format!("Illegal combination of token and location: token={:?}, location={:?}.", &self.0, &self.1).into()),
        }
    }
}

#[derive(Debug)]
pub struct Parser<'a> {
    iterator: Peekable<LexerIterator<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(iterator: LexerIterator<'a>) -> Self {
        Self {
            iterator: iterator.peekable(),
        }
    }

    pub fn parse_program(mut self) -> Result<Vec<Statement>, Box<dyn Error>> {
        let mut statements = vec![];
        loop {
            match self.parse_next_statement()? {
                Some(stmt) => statements.push(stmt),
                None => break,
            };
        }
        Ok(statements)
    }

    fn parse_next_statement(&mut self) -> Result<Option<Statement>, Box<dyn Error>> {
        use Token::*;
        match self.iterator.peek() {
            Some(Let) => self.parse_assignment_statement().map(Some),
            Some(FuncFn) => self.parse_function_declaration().map(Some),
            Some(Eof) | None => Ok(None),
            _ => self.parse_expression_statement().map(Some),
        }
    }

    fn parse_prefix_expression(&mut self) -> Result<Expression, Box<dyn Error>> {
        match self.iterator.next().expect("There should be another token here") {
            Token::Undefined => Ok(Expression::UndefinedLiteral),
            Token::Identifier(name) => Ok(Expression::Identifier(name)),
            Token::Integer(value) => Ok(Expression::IntegerLiteral(value)),
            Token::Real(value) => Ok(Expression::RealLiteral(value)),
            Token::True => Ok(Expression::BooleanLiteral(true)),
            Token::False => Ok(Expression::BooleanLiteral(false)),
            t @ Token::Not | t @ Token::Minus => {
                let op = Operation(t, Location::Prefix);
                let expr_result = self.parse_expression(op.precedence()?);
                match expr_result {
                    Ok(expr) => Ok(Expression::Combination {
                        left: None,
                        operator: op,
                        right: Some(Box::new(expr)),
                    }),
                    Err(value) => Err(value),
                }
            },
            Token::LeftParen => {
                let expr = self.parse_expression(Precedence::Lowest)?;
                if let Some(_) = self.iterator.next_if(|token| token == &Token::RightParen) {
                    Ok(expr)
                } else {
                    Err("Missing right parenthesis after grouped expression.".into())
                }
            },
            _ => unreachable!()
        }
    }

    fn parse_infix_expression(&mut self, left: Expression) -> Result<Expression, Box<dyn Error>> {
        let operator = match self.iterator.next() {
            Some(token) => Operation(token, Location::Infix),
            _ => unreachable!()
        };
        let precedence = operator.precedence()?;

        let expression = if operator == Operation(Token::LeftParen, Location::Infix) {
            let mut arguments = vec![];
            match self.iterator.peek() {
                Some(Token::RightParen) => {
                    self.iterator.next().map(drop);
                    return Ok(Expression::Call {
                        function: Box::new(left),
                        arguments
                    })
                },
                _ => arguments.push(self.parse_expression(Precedence::Lowest)?)
            };

            loop {
                if let Some(_) = self.iterator.next_if_eq(&Token::RightParen) {
                    break Expression::Call {
                        function: Box::new(left),
                        arguments
                    }                    
                } else {
                    match self.iterator.next() {
                        Some(Token::Comma) => (),
                        _ => return Err("Missing comma after parameter in call expression.".into())
                    };
                    arguments.push(self.parse_expression(Precedence::Lowest)?);
                }
            }
        } else {
            let left = Some(Box::new(left));
            let right = match self.parse_expression(precedence) {
                Ok(right) => Ok(Some(Box::new(right))),
                Err(value) => Err(value)
            }?;
            
            Expression::Combination {
                left,
                operator,
                right
            }
        };
        Ok(expression)
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Result<Expression, Box<dyn Error>> {
        // println!("{:?}", self.iterator.peek());
        let expression = if let Some(tok) = self.iterator.peek() {
            match tok {
                // literals and prefix expressions
                Token::Identifier(_)
                | Token::Integer(_)
                | Token::Real(_)
                | Token::True
                | Token::False
                | Token::Not
                | Token::Minus
                | Token::Undefined
                | Token::LeftParen => self.parse_prefix_expression().map(Some),

                // other
                _ => Ok(None),
            }
        } else {
            return Err("Unexpected end to expression.".into());
        };

        let mut expression = match expression {
            Ok(Some(expr)) => Ok(expr),
            Ok(None) => Err("Invalid syntax.".into()),
            Err(value) => Err(value)
        }?;

        loop {
            match self.iterator.peek() {
                Some(&Token::Semicolon) | Some(&Token::RightParen) | Some(&Token::Comma) => break Ok(()),
                Some(next_token) => {
                    let peek_precedence = Operation(next_token.clone(), Location::Infix).precedence()?;
                    if precedence < peek_precedence {
                        expression = self.parse_infix_expression(expression)?;
                    } else {
                        break Ok(())
                    }
                }
                None => break Err::<_, Box<dyn Error>>("Unexpected end of token stream.".into())
            }
        }?;

        Ok(expression)
    }

    fn parse_expression_statement(&mut self) -> Result<Statement, Box<dyn Error>> {
        let expression = match self.iterator.peek() {
            Some(_) => self.parse_expression(Precedence::Lowest),
            None => Err("Saw the abyss.".into())
        }?;

        // take the semicolon off
        if self.iterator.next() != Some(Token::Semicolon) {
            // if there is an extra right paren it will also go here. Maybe that's not a bad thing.
            return Err("Missing semicolon after expression statement.".into())
        };
        
        Ok(Statement::ExpressionStatement { expression })
    }

    fn parse_function_declaration(&mut self) -> Result<Statement, Box<dyn Error>> {
        use Statement::*;
        use Token::*;

        // current token is fn
        self.iterator.next();

        // next token should be an identifier
        if let Some(Identifier(identifier)) = self.iterator.next() {
            // next token should be left paren
            if !self.iterator.next_if(|token| token == &LeftParen).is_some() {
                return Err("Expected '(' after function identifier.".into());
            };
            let mut parameters = vec![];
            loop {
                // next should be alternating identifiers and commas
                // after the first parameter there should
                match self.iterator.next() {
                    Some(Identifier(name)) => {
                        parameters.push(Parameter { name });
                        match self.iterator.next() {
                            // or break happily if the next token is a right paren
                            Some(RightParen) => break Ok(()),
                            // eat up to one comma
                            Some(Comma) => continue,
                            _ => break Err("Invalid function declaration syntax."),
                        }

                    }
                    Some(RightParen) => break Ok(()),
                    _ => break Err("Invalid function declaration syntax."),
                };
            }?;

            if !self.iterator.next_if(|token| token == &Assign).is_some() {
                return Err("Expected '=' after function parameters.".into());
            };
            
            let expression = self.parse_expression(Precedence::Lowest)?;

            // eat semicolon
            self.iterator.next().map(drop);

            Ok(FunctionDeclaration {
                identifier,
                parameters,
                expression
            })
        } else {
            Err("Expected identifier after fn.".into())
        }
    }

    fn parse_assignment_statement(&mut self) -> Result<Statement, Box<dyn Error>> {
        use Statement::*;
        use Token::*;

        // current token is Let
        self.iterator.next().map(drop);

        // next token should be an identifier
        if let Some(Identifier(identifier)) = self.iterator.next() {
            // next token should be =
            if self.iterator.next_if(|token| token == &Assign).is_some() {
                // next set of tokens should form an expression
                let expression = self.parse_expression(Precedence::Lowest)?;
                
                // eat semicolon
                self.iterator.next().map(drop);

                // return assignment statement
                Ok(Assignment {
                    identifier,
                    expression
                })
            } else {
                Err("Missing assignment operator after identifier.".into())
            }
        } else {
            Err("Missing identifier after let statement.".into())
        }
    }
}
