use super::errors::RuntimeError;
use super::expression::Expression;
use super::location::Location;
use super::operator::Operator;
use super::statement::Statement;
use crate::interpreter::token::Token;
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
            None => {
                match &self.parent {
                    Some(env) => env.get(key),
                    None => None
                }
            }
        }
    }

    pub fn set(&mut self, key: &String, value: &Object) -> Result<(), RuntimeError> {
        match self.bindings.insert(key.clone(), value.clone()) {
            Some(_) => Err(RuntimeError::NameError(format!("Cannot re-declare value `{}`.", key))),
            None => Ok(())
        }
    }

    pub fn eval_statement(&mut self, stmt: &Statement) -> Result<Object, RuntimeError> {
        match stmt {
            Statement::Assignment {
                expression,
                identifier,
            } => {
                self.set(identifier, &self.eval(expression)?)?;
                Ok(Object::Undefined)
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
                })?;
                Ok(Object::Undefined)
            },
        }
    }
    
    pub fn eval(&self, expr: &Expression) -> Result<Object, RuntimeError> {
        use Expression::*;
        match expr {
            Identifier(key) => {
                if let Some(value) = self.get(key) {
                    Ok(value)
                } else {
                    Err(RuntimeError::NameError(format!("{}", key)))
                }
            },
            IntegerLiteral(value) => Ok(Object::Integer(*value)), // value,
            RealLiteral(value) => Ok(Object::Real(*value)),       // value,
            BooleanLiteral(value) => Ok(Object::Boolean(*value)), // value,
            UndefinedLiteral => Ok(Object::Undefined),
            Combination {
                operator: Operator(token, Location::Infix),
                left: Some(left),
                right: Some(right),
            } => match token {
                Token::Plus => Ok(self.eval(left)? + self.eval(right)?),
                Token::Multiply => Ok(self.eval(left)? * self.eval(right)?),
                Token::Minus => Ok(self.eval(left)? - self.eval(right)?),
                Token::Division => Ok(self.eval(left)? / self.eval(right)?),
                Token::NotEquals => Ok(Object::Boolean(self.eval(left)? != self.eval(right)?)),
                Token::Equals => Ok(Object::Boolean(self.eval(left)? == self.eval(right)?)),
                Token::GreaterThan => Ok(Object::Boolean(self.eval(left)? > self.eval(right)?)),
                Token::GreaterThanEquals => Ok(Object::Boolean(self.eval(left)? >= self.eval(right)?)),
                Token::LessThan => Ok(Object::Boolean(self.eval(left)? < self.eval(right)?)),
                Token::LessThanEquals => Ok(Object::Boolean(self.eval(left)? <= self.eval(right)?)),
                Token::Exponent => Ok(self.eval(left)?.pow(&self.eval(right)?)),
                Token::Question => Ok(self.eval(left)?.coalesce(&self.eval(right)?)),
                Token::And => Ok(self.eval(left)?.and(&self.eval(right)?)),
                Token::Or => Ok(self.eval(left)?.or(&self.eval(right)?)),
                Token::Xor => Ok(self.eval(left)?.xor(&self.eval(right)?)),
                Token::Modulo => Ok(self.eval(left)?.modulo(&self.eval(right)?)),
                t => Err(RuntimeError::OperatorError(format!("Cannot use `{}` as an infix operator.", t))),
            },
            Combination {
                operator: Operator(token, Location::Prefix),
                left: None,
                right: Some(right),
            } => match token {
                Token::Minus => Ok(-self.eval(right)?),
                Token::Not => Ok(!self.eval(right)?),
                t => Err(RuntimeError::OperatorError(format!("Cannot use `{}` as a prefix operator.", t))),
            },
            Combination {
                left: Some(_left),
                operator: Operator(token, Location::Postfix),
                right: None,
            } => match token {
                t => Err(RuntimeError::OperatorError(format!("Cannot use `{}` as a postfix operator.", t))),
            },
            Call {
                function,
                arguments,
            } => {
                // create new environment with current one as its parent
                let mut env = self.new_child();
                
                // get parameters from current definition
                let func = self.eval(function)?;
                if let Object::Function { parameters, expression } = func {
                    for (p, v) in parameters.iter().zip(arguments.iter()) {
                        env.set(&p.name, &self.eval(v)?)?;
                    };
                    env.eval(&expression)
                } else {
                    Err(RuntimeError::OperatorError(format!("Illegal call expression `{}`.", func)))
                }
            },
            PiecewiseBlock(arms) => {
                let mut arm_iter = arms.iter();
                loop {
                    match arm_iter.next() {
                        Some((value_expr, cond_expr)) => {
                            let obj = self.eval(cond_expr)?;
                            if let Some(value) = match obj { Object::Boolean(value) => Some(value), _ => None } {
                                if value {
                                    break self.eval(value_expr)
                                } else {
                                    continue
                                }
                            } else {
                                break Err(RuntimeError::TypeError(format!("Piecewise conditional expression should result in a boolean, got `{}`.", obj)))
                            }
                        },
                        None => {
                            break Ok(Object::Undefined)
                        }
                    }
                }
            },
            AbsoluteValue(expr) => {
                match self.eval(expr)? {
                    Object::Integer(value)  => Ok(Object::Integer(value.abs())),
                    Object::Real(value) => Ok(Object::Real(value.abs())),
                    obj => Err(RuntimeError::OperatorError(format!("Cannot take absolute value of `{}`.", obj)))
                }
            },
            Combination { .. } => panic!("Illegal expression."),
        }
    }
}
