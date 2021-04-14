use std::error::Error;
use super::expression::Expression;
use super::location::Location;
use super::operator::Operator;
use super::statement::Statement;
use crate::interpreter::token::Token;
use std::fmt::{Display, Formatter, self};
use std::collections::HashMap;
use super::object::Object;

#[derive(Debug, Clone)]
pub struct Environment {
    bindings: HashMap<String, Object>,
    parent: Option<Box<Environment>>
}

impl Environment {
    pub fn new() -> Self {
        Self {
            bindings: HashMap::new(),
            parent: None
        }
    }

    pub fn new_child(&self) -> Self {
        let mut child = Self::new();
        child.parent = Some(Box::new(self.clone()));
        child
    }

    pub fn get(&self, key: &str) -> Option<Object> {
        match self.bindings.get(key) {
            Some(value) => Some(value.clone()),
            None => None
        }
    }

    pub fn set(&mut self, key: &String, value: &Object) {
        match self.bindings.insert(key.clone(), value.clone()) {
            Some(_) => panic!("Cannot re-bind value."),
            None => {}
        }
    }

    pub fn eval_statement(&mut self, stmt: &Statement) -> Object {
        match stmt {
            Statement::Assignment {
                expression,
                identifier,
            } => {
                self.set(identifier, &self.eval(expression));
                Object::Undefined
            },
            Statement::ExpressionStatement { expression } => self.eval(expression),
            Statement::FunctionDeclaration {
                identifier,
                parameters,
                expression,
            } => {
                self.set(identifier, &Object::Function {
                    parameters: parameters.clone().to_vec(),
                    expression: expression.clone()
                });
                Object::Undefined
            },
        }
    }
    
    pub fn eval(&self, expr: &Expression) -> Object {
        use Expression::*;
        match expr {
            Identifier(key) => {
                if let Some(value) = self.get(key) {
                    value
                } else {
                   match &self.parent {
                       Some(env) => env.get(key).expect("NameError"),
                       None => panic!("NameError")
                   } 
                }
            },
            IntegerLiteral(value) => Object::Integer(*value), // value,
            RealLiteral(value) => Object::Real(*value),       // value,
            BooleanLiteral(value) => Object::Boolean(*value), // value,
            UndefinedLiteral => Object::Undefined,
            Combination {
                operator: Operator(token, Location::Infix),
                left: Some(left),
                right: Some(right),
            } => match token {
                Token::Plus => self.eval(left) + self.eval(right),
                Token::Multiply => self.eval(left) * self.eval(right),
                Token::Minus => self.eval(left) - self.eval(right),
                Token::Division => self.eval(left) / self.eval(right),
                Token::NotEquals => Object::Boolean(self.eval(left) != self.eval(right)),
                Token::Equals => Object::Boolean(self.eval(left) == self.eval(right)),
                Token::GreaterThan => Object::Boolean(self.eval(left) > self.eval(right)),
                Token::GreaterThanEquals => Object::Boolean(self.eval(left) >= self.eval(right)),
                Token::LessThan => Object::Boolean(self.eval(left) < self.eval(right)),
                Token::LessThanEquals => Object::Boolean(self.eval(left) <= self.eval(right)),
                Token::Exponent => self.eval(left).pow(&self.eval(right)),
                Token::Question => self.eval(left).coalesce(&self.eval(right)),
                Token::And => self.eval(left).and(&self.eval(right)),
                Token::Or => self.eval(left).or(&self.eval(right)),
                Token::Xor => self.eval(left).xor(&self.eval(right)),
                Token::Modulo => self.eval(left).modulo(&self.eval(right)),
                _ => panic!("Operator not implemented."),
            },
            Combination {
                operator: Operator(token, Location::Prefix),
                left: None,
                right: Some(right),
            } => match token {
                Token::Minus => -self.eval(right),
                Token::Not => !self.eval(right),
                _ => panic!("Operator not implemented."),
            },
            Combination {
                left: Some(left),
                operator: Operator(token, Location::Postfix),
                right: None,
            } => match token {
                _ => panic!("Operator not implemented."),
            },
            Call {
                function,
                arguments,
            } => {
                // create new environment with current one as its parent
                let mut env = self.new_child();
                
                // get parameters from current definition
                let func = self.eval(function);
                if let Object::Function { parameters, expression } = func {
                    for (p, v) in parameters.iter().zip(arguments.iter()) {
                        env.set(&p.name, &self.eval(v));
                    };
                    env.eval(&expression)
                } else {
                    panic!("Illegal call expression")
                }
            },
            Combination { .. } => panic!("Illegal expression."),
        }
    }
}

#[derive(Debug)]
pub enum RuntimeError {
    NameError,
    TypeError,
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use RuntimeError::*;
        match self {
            NameError => write!(f, "NameError"),
            TypeError => write!(f, "TypeError"),
        }
    }
}

impl Error for RuntimeError {}
