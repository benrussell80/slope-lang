use std::mem;
use super::errors::RuntimeError;
use super::expression::Expression;
use super::location::Location;
use super::operator::Operator;
use super::statement::Statement;
use crate::interpreter::token::Token;
use std::collections::{HashMap, BTreeSet};
use super::object::Object;
use super::modules::{Module, math_constants_builtins, set_builtins};

#[derive(Debug, Clone)]
pub struct Environment {
    bindings: HashMap<String, Object>,
    parent: Option<Box<Environment>>
}

impl Environment {
    pub fn new() -> Self {
        let mut env = Self {
            bindings: HashMap::new(),
            parent: None
        };
        env.import(set_builtins);
        env.import(math_constants_builtins);
        env
    }

    pub fn import(&mut self, func: Module) {
        func(self)
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
                Token::Plus => self.eval(left)? + self.eval(right)?,
                Token::Multiply => self.eval(left)? * self.eval(right)?,
                Token::Minus => self.eval(left)? - self.eval(right)?,
                Token::Division => self.eval(left)? / self.eval(right)?,
                Token::NotEquals => Ok(Object::Boolean(self.eval(left)? != self.eval(right)?)),
                Token::Equals => Ok(Object::Boolean(self.eval(left)? == self.eval(right)?)),
                Token::GreaterThan => Ok(Object::Boolean(self.eval(left)? > self.eval(right)?)),
                Token::GreaterThanEquals => Ok(Object::Boolean(self.eval(left)? >= self.eval(right)?)),
                Token::LessThan => {
                    let left = self.eval(left)?;
                    let right = self.eval(right)?;
                    match (left, right) {
                        (s1 @ Object::Set { .. }, s2 @ Object::Set { .. }) => {
                            s1.is_proper_subset(&s2)
                        },
                        (obj1, obj2) => Ok(Object::Boolean(obj1 < obj2))
                    }
                },
                Token::LessThanEquals => {
                    let left = self.eval(left)?;
                    let right = self.eval(right)?;
                    match (left, right) {
                        (s1 @ Object::Set { .. }, s2 @ Object::Set { .. }) => {
                            s1.is_subset(&s2)
                        },
                        (obj1, obj2) => Ok(Object::Boolean(obj1 <= obj2))
                    }
                },
                Token::Exponent => self.eval(left)?.pow(&self.eval(right)?),
                Token::Question => Ok(self.eval(left)?.coalesce(&self.eval(right)?)),
                Token::And => self.eval(left)?.and(&self.eval(right)?),
                Token::Or => self.eval(left)?.or(&self.eval(right)?),
                Token::Xor => self.eval(left)?.xor(&self.eval(right)?),
                Token::Modulo => self.eval(left)?.modulo(&self.eval(right)?),
                Token::In => self.eval(left)?.in_(&self.eval(right)?),
                Token::PlusMinus => self.eval(left)?.pm(&self.eval(right)?),
                Token::Union => self.eval(left)?.set_union(&self.eval(right)?),
                Token::SetDifference => self.eval(left)?.set_difference(&self.eval(right)?),
                Token::SymmetricDifference => self.eval(left)?.set_symmetric_difference(&self.eval(right)?),
                Token::Intersection => self.eval(left)?.set_intersection(&self.eval(right)?),
                t => Err(RuntimeError::OperatorError(format!("Cannot use `{}` as an infix operator.", t))),
            },
            Combination {
                operator: Operator(token, Location::Prefix),
                left: None,
                right: Some(right),
            } => match token {
                Token::Minus => match self.eval(right) {
                    Ok(obj) => -obj,
                    Err(e) => Err(e)
                },
                Token::Not => match self.eval(right) {
                    Ok(obj) => !obj,
                    Err(e) => Err(e)
                },
                t => Err(RuntimeError::OperatorError(format!("Cannot use `{}` as a prefix operator.", t))),
            },
            Combination {
                left: Some(left),
                operator: Operator(token, Location::Postfix),
                right: None,
            } => match token {
                Token::Bang => self.eval(left)?.factorial(),
                t => Err(RuntimeError::OperatorError(format!("Cannot use `{}` as a postfix operator.", t))),
            },
            Call {
                function,
                arguments,
            } => {
                // create new environment with current one as its parent
                let mut env = self.new_child();
                
                // get parameters from current definition
                match self.eval(function)? {
                    Object::Function { parameters, expression } => {
                        for (p, v) in parameters.iter().zip(arguments.iter()) {
                            env.set(&p.name, &self.eval(v)?)?;
                        };
                        env.eval(&expression)
                    },
                    Object::BuiltinFunction { body, .. } => {
                        body(arguments.iter().map(|expr| self.eval(expr).unwrap()).collect())
                    },
                    func => Err(RuntimeError::OperatorError(format!("Illegal call expression `{}`.", func)))
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
            AbsoluteValue(expr) => self.eval(expr)?.abs(),
            SetLiteral(expressions) => {
                let mut items = BTreeSet::new();
                let mut kind = None;
                for expr in expressions {
                    let obj = self.eval(expr)?;
                    if obj.is_undefined() {
                        return Err(RuntimeError::TypeError("Cannot put undefined in a set.".into()))
                    };
                    if kind.is_none() {
                        kind = Some(mem::discriminant(&obj));
                    } else if let Some(disc) = kind {
                        if mem::discriminant(&obj) != disc {
                            return Err(RuntimeError::TypeError("Set literal members must all be the same type.".into()))
                        }
                    }
                    items.insert(obj);
                }
                Ok(Object::Set {
                    items,
                    kind
                })
            },
            Combination { .. } => panic!("Illegal expression."),
        }
    }
}
