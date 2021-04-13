use std::error::Error;
use crate::ast::base::{Expression, Location, Operation, Statement, Parameter};
use crate::interpreter::base::Token;
use std::cmp::Ordering;
use std::ops::{Add, Div, Mul, Neg, Not, Sub};
use std::fmt::{Display, Formatter, self, Debug};
use std::collections::HashMap;

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
                operator: Operation(token, Location::Infix),
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
                _ => panic!("Operation not implemented."),
            },
            Combination {
                operator: Operation(token, Location::Prefix),
                left: None,
                right: Some(right),
            } => match token {
                Token::Minus => -self.eval(right),
                Token::Not => !self.eval(right),
                _ => panic!("Operation not implemented."),
            },
            Combination {
                left: Some(left),
                operator: Operation(token, Location::Postfix),
                right: None,
            } => match token {
                _ => panic!("Operation not implemented."),
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

#[derive(Debug, Clone)]
pub enum Object {
    Integer(i64),
    Real(f64),
    Boolean(bool),
    Undefined,
    Function {
        parameters: Vec<Parameter>,
        expression: Expression
    }
}

impl Object {
    pub fn pow(&self, rhs: &Self) -> Self {
        use Object::*;
        if let Real(num) = match (self, rhs) {
            // All the conversion here is pretty dodgy
            (Integer(left), Integer(right)) => Real((*left as f64).powi(*right as i32)),
            (Integer(left), Real(right)) => Real((*left as f64).powf(*right)),
            (Real(left), Integer(right)) => Real(left.powi(*right as i32)),
            (Real(left), Real(right)) => Real(left.powf(*right)),
            _ => Undefined,
        } {
            if num.is_nan() || num.is_infinite() {
                Undefined
            } else {
                Real(num)
            }
        } else {
            Undefined
        }
    }

    pub fn coalesce(&self, rhs: &Self) -> Self {
        use Object::*;
        match self {
            Undefined => rhs.clone(),
            _ => self.clone()
        }
    }

    pub fn abs(&self) -> Self {
        use Object::*;
        match self {
            Integer(value) => Integer(value.abs()),
            Real(value) => Real(value.abs()),
            Undefined => Undefined,
            _ => Undefined
        }
    }

    pub fn and(&self, rhs: &Self) -> Self {
        use Object::*;
        match (self, rhs) {
            (Boolean(left), Boolean(right)) => Boolean(*left && *right),
            _ => Undefined
        }
    }

    pub fn or(&self, rhs: &Self) -> Self {
        use Object::*;
        // maybe use trait Into<bool>
        match (self, rhs) {
            (Boolean(left), Boolean(right)) => Boolean(*left || *right),
            _ => Undefined
        }
    }

    pub fn xor(&self, rhs: &Self) -> Self {
        use Object::*;
        match (self, rhs) {
            (Boolean(left), Boolean(right)) => {
                Boolean((
                    *left || *right
                ) && !(
                    *left && *right
                ))
            },
            _ => Undefined
        }
    }

    pub fn modulo(&self, rhs: &Self) -> Self {
        use Object::*;
        match (self, rhs) {
            // use Rem trait instead
            (Integer(left), Integer(right)) => Integer(left % right),
            _ => Undefined
        }
    }
}

impl Display for Object {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use Object::*;
        match self {
            Integer(value) => write!(f, "{}", value),
            Real(value) => write!(f, "{}", value),
            Boolean(value) => write!(f, "{}", value),
            Undefined => write!(f, "undefined"),
            Function { parameters, expression } => write!(f, "fn({}) = {};", parameters.iter().fold(String::new(), |mut acc, param| {
                if !acc.is_empty() {
                    acc.push_str(", ");
                };
                acc.push_str(&param.to_string());
                acc
            }), expression),
            _ => write!(f, "<other>")
        }
    }
}

// impl Eq for &Object {}
impl Eq for Object {}

// impl PartialOrd for &Object {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         use Object::*;
//         match (self, other) {
//             (Integer(left), Integer(right)) => left.partial_cmp(&right),
//             (Real(left), Integer(right)) => left.partial_cmp(&(*right as f64)),
//             (Integer(left), Real(right)) => left.partial_cmp(&(*right as i64)),
//             (Real(left), Real(right)) => left.partial_cmp(right),
//             _ => panic!("Cannot compare those types."),
//         }
//     }
// }

impl PartialOrd for Object {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        use Object::*;
        match (self, other) {
            (Integer(left), Integer(right)) => left.partial_cmp(&right),
            (Real(left), Integer(right)) => left.partial_cmp(&(*right as f64)),
            (Integer(left), Real(right)) => left.partial_cmp(&(*right as i64)),
            (Real(left), Real(right)) => left.partial_cmp(right),
            _ => panic!("Cannot compare those types."),
        }
    }
}

// impl Ord for &Object {
//     fn cmp(&self, other: &Self) -> Ordering {
//         use Object::*;
//         match (self, other) {
//             (Integer(left), Integer(right)) => left.cmp(&right),

//             _ => panic!("Cannot compare these two types."),
//         }
//     }
// }

impl Ord for Object {
    fn cmp(&self, other: &Self) -> Ordering {
        use Object::*;
        match (self, other) {
            (Integer(left), Integer(right)) => left.cmp(&right),

            _ => panic!("Cannot compare these two types."),
        }
    }
}

// impl PartialEq for &Object {
//     fn eq(&self, rhs: &Self) -> bool {
//         use Object::*;
//         match (self, rhs) {
//             (Integer(left), Integer(right)) => left == right,
//             (Real(left), Real(right)) => left == right,
//             (Integer(left), Real(right)) => *left as f64 == *right,
//             (Real(left), Integer(right)) => *left == *right as f64,
//             (Boolean(left), Boolean(right)) => left == right,
//             (Undefined, _) => false,
//             _ => panic!("Cannot equate these two types."),
//         }
//     }
// }

impl PartialEq for Object {
    fn eq(&self, rhs: &Self) -> bool {
        use Object::*;
        match (self, rhs) {
            (Integer(left), Integer(right)) => left == right,
            (Real(left), Real(right)) => left == right,
            (Integer(left), Real(right)) => *left as f64 == *right,
            (Real(left), Integer(right)) => *left == *right as f64,
            (Boolean(left), Boolean(right)) => left == right,
            (Undefined, _) => false,
            _ => panic!("Cannot equate these two types."),
        }
    }
}

impl Add for &Object {
    type Output = Object;
    fn add(self, rhs: Self) -> Self::Output {
        use Object::*;
        match (self, rhs) {
            (Integer(left), Integer(right)) => Integer(left + right),
            (Integer(left), Real(right)) => Real(*left as f64 + right),
            (Real(left), Integer(right)) => Real(left + *right as f64),
            (Real(left), Real(right)) => Real(left + right),
            (Undefined, _) => Undefined,
            _ => Undefined,
        }
    }
}

impl Add for Object {
    type Output = Object;
    fn add(self, rhs: Self) -> Self::Output {
        use Object::*;
        match (self, rhs) {
            (Integer(left), Integer(right)) => Integer(left + right),
            (Integer(left), Real(right)) => Real(left as f64 + right),
            (Real(left), Integer(right)) => Real(left + right as f64),
            (Real(left), Real(right)) => Real(left + right),
            (Undefined, _) => Undefined,
            _ => Undefined,
        }
    }
}

impl Mul for &Object {
    type Output = Object;
    fn mul(self, rhs: Self) -> Self::Output {
        use Object::*;
        match (self, rhs) {
            (Integer(left), Integer(right)) => Integer(left * right),
            (Integer(left), Real(right)) => Real(*left as f64 * right),
            (Real(left), Integer(right)) => Real(left * *right as f64),
            (Real(left), Real(right)) => Real(left * right),
            (Undefined, _) => Undefined,
            _ => Undefined,
        }
    }
}

impl Mul for Object {
    type Output = Object;
    fn mul(self, rhs: Self) -> Self::Output {
        use Object::*;
        match (self, rhs) {
            (Integer(left), Integer(right)) => Integer(left * right),
            (Integer(left), Real(right)) => Real(left as f64 * right),
            (Real(left), Integer(right)) => Real(left * right as f64),
            (Real(left), Real(right)) => Real(left * right),
            (Undefined, _) => Undefined,
            _ => Undefined,
        }
    }
}

impl Sub for &Object {
    type Output = Object;
    fn sub(self, rhs: Self) -> Self::Output {
        use Object::*;
        match (self, rhs) {
            (Integer(left), Integer(right)) => Integer(left - right),
            (Integer(left), Real(right)) => Real(*left as f64 - right),
            (Real(left), Integer(right)) => Real(left - *right as f64),
            (Real(left), Real(right)) => Real(left - right),
            (Undefined, _) => Undefined,
            _ => Undefined,
        }
    }
}

impl Sub for Object {
    type Output = Object;
    fn sub(self, rhs: Self) -> Self::Output {
        use Object::*;
        match (self, rhs) {
            (Integer(left), Integer(right)) => Integer(left - right),
            (Integer(left), Real(right)) => Real(left as f64 - right),
            (Real(left), Integer(right)) => Real(left - right as f64),
            (Real(left), Real(right)) => Real(left - right),
            (Undefined, _) => Undefined,
            _ => Undefined,
        }
    }
}

impl Div for &Object {
    type Output = Object;
    fn div(self, rhs: Self) -> Self::Output {
        use Object::*;
        match (self, rhs) {
            (Integer(left), Integer(right)) => {
                if *right == 0 {
                    Undefined
                } else {
                    Real(*left as f64 / *right as f64)
                }
            }
            (Integer(left), Real(right)) => {
                let value = *left as f64 / right;
                if value.is_nan() || value.is_infinite() {
                    Undefined
                } else {
                    Real(value)
                }
            }
            (Real(left), Integer(right)) => {
                let value = left / *right as f64;
                if value.is_nan() || value.is_infinite() {
                    Undefined
                } else {
                    Real(value)
                }
            }
            (Real(left), Real(right)) => {
                let value = left / right;
                if value.is_nan() || value.is_infinite() {
                    Undefined
                } else {
                    Real(value)
                }
            }
            (Undefined, _) => Undefined,
            _ => Undefined,
        }
    }
}

impl Div for Object {
    type Output = Object;
    fn div(self, rhs: Self) -> Self::Output {
        use Object::*;
        match (self, rhs) {
            (Integer(left), Integer(right)) => {
                if right == 0 {
                    Undefined
                } else {
                    Real(left as f64 / right as f64)
                }
            }
            (Integer(left), Real(right)) => {
                let value = left as f64 / right;
                if value.is_nan() || value.is_infinite() {
                    Undefined
                } else {
                    Real(value)
                }
            }
            (Real(left), Integer(right)) => {
                let value = left / right as f64;
                if value.is_nan() || value.is_infinite() {
                    Undefined
                } else {
                    Real(value)
                }
            }
            (Real(left), Real(right)) => {
                let value = left / right;
                if value.is_nan() || value.is_infinite() {
                    Undefined
                } else {
                    Real(value)
                }
            }
            (Undefined, _) => Undefined,
            _ => Undefined,
        }
    }
}

impl Neg for &Object {
    type Output = Object;
    fn neg(self) -> Self::Output {
        use Object::*;
        match self {
            Integer(value) => Integer(-value),
            Real(value) => Real(-value),
            Undefined => Undefined,
            _ => Undefined,
        }
    }
}

impl Neg for Object {
    type Output = Object;
    fn neg(self) -> Self::Output {
        use Object::*;
        match self {
            Integer(value) => Integer(-value),
            Real(value) => Real(-value),
            Undefined => Undefined,
            _ => Undefined,
        }
    }
}

impl Not for &Object {
    type Output = Object;
    fn not(self) -> Self::Output {
        use Object::*;
        match self {
            Boolean(value) => Boolean(!value),
            Undefined => Undefined,
            _ => Undefined,
        }
    }
}

impl Not for Object {
    type Output = Object;
    fn not(self) -> Self::Output {
        use Object::*;
        match self {
            Boolean(value) => Boolean(!value),
            Undefined => Undefined,
            _ => Undefined,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Environment, Expression::*, Object::*, Token, Operation, Location, Statement};
    use std::mem;
    // PlusMinus, MinusPlus (once sets are implemented)

    macro_rules! assert_evals {
        ($token:expr, $right:expr, $obj:expr) => {
            // prefix expression
            assert_eq!(
                Environment::new().eval(
                    &Combination {
                        left: None,
                        operator: Operation($token, Location::Prefix),
                        right: Some(Box::new($right))
                    }
                ),
                $obj
            );
        };
        ($left:expr, $token:expr, $right:expr, $obj:expr) => {
            // infix expression
            assert_eq!(
                Environment::new().eval(
                    &Combination {
                        left: Some(Box::new($left)),
                        operator: Operation($token, Location::Infix),
                        right: Some(Box::new($right))
                    }
                ),
                $obj
            );
        };
        ($left:expr, $token:expr, $right:expr, $obj:expr, tol=$tol:expr) => {
            // equality with tolerance
            let val = Environment::new().eval(
                &Combination {
                    left: Some(Box::new($left)),
                    operator: Operation($token, Location::Infix),
                    right: Some(Box::new($right))
                },
            );
            assert!(Boolean(-$tol <= val.clone() - $obj).and(&Boolean(val - $obj <= $tol)) == Boolean(true));
        };
    }

    #[test]
    fn test_add_int_real() {
        assert_evals!(
            IntegerLiteral(2), Token::Plus, RealLiteral(40.0),
            Real(42.0)
        );
    }

    #[test]
    fn test_add_int_int() {
        assert_evals!(
            IntegerLiteral(2), Token::Plus, IntegerLiteral(40),
            Integer(42)
        );
    }

    #[test]
    fn test_add_real_real() {
        assert_evals!(
            RealLiteral(2.9), Token::Plus, RealLiteral(39.1),
            Real(42.0)
        );
    }

    #[test]
    fn test_add_real_int() {
        assert_evals!(
            RealLiteral(2.0), Token::Plus, IntegerLiteral(40),
            Real(42.0)
        );
    }

    #[test]
    fn test_sub_int_int() {
        assert_evals!(
            IntegerLiteral(40), Token::Minus, IntegerLiteral(2),
            Integer(38)
        );
    }

    #[test]
    fn test_sub_real_int() {
        assert_evals!(
            RealLiteral(40.0), Token::Minus, IntegerLiteral(2),
            Real(38.0)
        );
    }

    #[test]
    fn test_sub_int_real() {
        assert_evals!(
            IntegerLiteral(2), Token::Minus, RealLiteral(40.0),
            Real(-38.0)
        );
    }

    #[test]
    fn test_sub_real_real() {
        assert_evals!(
            RealLiteral(40.0), Token::Minus, RealLiteral(2.0),
            Real(38.0)
        );
    }

    #[test]
    fn test_mult_int_int() {
        assert_evals!(
            IntegerLiteral(3), Token::Multiply, IntegerLiteral(3),
            Integer(9)
        );
    }

    #[test]
    fn test_mult_int_real() {
        assert_evals!(
            IntegerLiteral(3), Token::Multiply, RealLiteral(3.2),
            Real(9.6), tol=Real(0.0001)
        );
    }

    #[test]
    fn test_mult_real_int() {
        assert_evals!(
            RealLiteral(3.2), Token::Multiply, IntegerLiteral(3),
            Real(9.6), tol=Real(0.0001)
        );
    }

    #[test]
    fn test_mult_real_real() {
        assert_evals!(
            RealLiteral(3.0), Token::Multiply, RealLiteral(3.2),
            Real(9.6), tol=Real(0.0001)
        );
    }

    #[test]
    fn test_div_int_int() {
        assert_evals!(
            IntegerLiteral(3), Token::Multiply, IntegerLiteral(3),
            Integer(9)
        );
    }

    #[test]
    fn test_div_int_real() {
        assert_evals!(
            IntegerLiteral(3), Token::Multiply, IntegerLiteral(3),
            Integer(9)
        );
    }

    #[test]
    fn test_div_real_int() {
        assert_evals!(
            IntegerLiteral(3), Token::Multiply, IntegerLiteral(3),
            Integer(9)
        );
    }

    #[test]
    fn test_div_real_real() {
        assert_evals!(
            IntegerLiteral(3), Token::Multiply, IntegerLiteral(3),
            Integer(9)
        );
    }

    #[test]
    fn test_eq_int_int() {
        assert_evals!(
            IntegerLiteral(2), Token::Equals, IntegerLiteral(2),
            Boolean(true)
        )
    }

    #[test]
    fn test_eq_int_real() {
        assert_evals!(
            IntegerLiteral(2), Token::Equals, RealLiteral(2.0),
            Boolean(true)
        )
    }

    #[test]
    fn test_eq_real_int() {
        assert_evals!(
            RealLiteral(2.0), Token::Equals, IntegerLiteral(2),
            Boolean(true)
        )
    }

    #[test]
    fn test_eq_real_real() {
        assert_evals!(
            RealLiteral(2.0), Token::Equals, RealLiteral(2.0),
            Boolean(true)
        )
    }

    #[test]
    fn test_eq_undefined() {
        assert_evals!(
            UndefinedLiteral, Token::Equals, IntegerLiteral(2),
            Boolean(false)
        )
    }

    #[test]
    fn test_eq_undefined_2() {
        assert_evals!(
            UndefinedLiteral, Token::Equals, UndefinedLiteral,
            Boolean(false)
        )
    }

    #[test]
    fn test_eq_boolean() {
        assert_evals!(
            BooleanLiteral(true), Token::Equals, BooleanLiteral(false),
            Boolean(false)
        )
    }

    #[test]
    fn test_ne_int_int() {
        assert_evals!(
            IntegerLiteral(2), Token::NotEquals, IntegerLiteral(1),
            Boolean(true)
        )
    }

    #[test]
    fn test_ne_int_real() {
        assert_evals!(
            IntegerLiteral(2), Token::NotEquals, RealLiteral(2.0),
            Boolean(false)
        )
    }

    #[test]
    fn test_ne_real_int() {
        assert_evals!(
            RealLiteral(2.0), Token::NotEquals, IntegerLiteral(2),
            Boolean(false)
        )
    }

    #[test]
    fn test_ne_real_real() {
        assert_evals!(
            RealLiteral(2.0), Token::NotEquals, RealLiteral(2.0),
            Boolean(false)
        )
    }

    #[test]
    fn test_ne_undefined() {
        assert_evals!(
            UndefinedLiteral, Token::NotEquals, UndefinedLiteral,
            Boolean(true)
        )
    }

    #[test]
    fn test_ne_boolean() {
        assert_evals!(
            BooleanLiteral(true), Token::NotEquals, BooleanLiteral(false),
            Boolean(true)
        )
    }

    #[test]
    fn test_gt_int_int() {
        assert_evals!(
            IntegerLiteral(1), Token::GreaterThan, IntegerLiteral(1),
            Boolean(false)
        )
    }

    #[test]
    fn test_gt_int_real() {
        assert_evals!(
            IntegerLiteral(1), Token::GreaterThan, RealLiteral(1.0),
            Boolean(false)
        )
    }

    #[test]
    fn test_gt_real_int() {
        assert_evals!(
            RealLiteral(1.0), Token::GreaterThan, IntegerLiteral(1),
            Boolean(false)
        )
    }

    #[test]
    fn test_gt_real_real() {
        assert_evals!(
            RealLiteral(1.0), Token::GreaterThan, RealLiteral(1.0),
            Boolean(false)
        )
    }

    #[test]
    fn test_gte_int_int() {
        assert_evals!(
            IntegerLiteral(1), Token::GreaterThanEquals, IntegerLiteral(1),
            Boolean(true)
        )
    }

    #[test]
    fn test_gte_int_real() {
        assert_evals!(
            IntegerLiteral(1), Token::GreaterThanEquals, RealLiteral(1.0),
            Boolean(true)
        )
    }

    #[test]
    fn test_gte_real_int() {
        assert_evals!(
            RealLiteral(1.0), Token::GreaterThanEquals, IntegerLiteral(1),
            Boolean(true)
        )
    }

    #[test]
    fn test_gte_real_real() {
        assert_evals!(
            RealLiteral(1.0), Token::GreaterThanEquals, RealLiteral(1.0),
            Boolean(true)
        )
    }

    #[test]
    fn test_lt_int_int() {
        assert_evals!(
            IntegerLiteral(1), Token::LessThan, IntegerLiteral(1),
            Boolean(false)
        )
    }

    #[test]
    fn test_lt_int_real() {
        assert_evals!(
            IntegerLiteral(1), Token::LessThan, RealLiteral(1.0),
            Boolean(false)
        )
    }

    #[test]
    fn test_lt_real_int() {
        assert_evals!(
            RealLiteral(1.0), Token::LessThan, IntegerLiteral(1),
            Boolean(false)
        )
    }

    #[test]
    fn test_lt_real_real() {
        assert_evals!(
            RealLiteral(1.0), Token::LessThan, RealLiteral(1.0),
            Boolean(false)
        )
    }

    #[test]
    fn test_lte_int_int() {
        assert_evals!(
            IntegerLiteral(1), Token::LessThanEquals, IntegerLiteral(1),
            Boolean(true)
        )
    }

    #[test]
    fn test_lte_int_real() {
        assert_evals!(
            IntegerLiteral(1), Token::LessThanEquals, RealLiteral(1.0),
            Boolean(true)
        )
    }

    #[test]
    fn test_lte_real_int() {
        assert_evals!(
            RealLiteral(1.0), Token::LessThanEquals, IntegerLiteral(1),
            Boolean(true)
        )
    }

    #[test]
    fn test_lte_real_real() {
        assert_evals!(
            RealLiteral(1.0), Token::LessThanEquals, RealLiteral(1.0),
            Boolean(true)
        )
    }

    #[test]
    fn test_exp_int_int() {
        assert_evals!(
            IntegerLiteral(1), Token::Exponent, IntegerLiteral(1),
            Integer(1)
        )
    }

    #[test]
    fn test_exp_int_real() {
        assert_evals!(
            IntegerLiteral(1), Token::Exponent, RealLiteral(1.0),
            Real(1.0)
        )
    }

    #[test]
    fn test_exp_real_int() {
        assert_evals!(
            RealLiteral(1.0), Token::Exponent, IntegerLiteral(1),
            Real(1.0)
        )
    }

    #[test]
    fn test_exp_real_real() {
        assert_evals!(
            RealLiteral(1.0), Token::Exponent, RealLiteral(1.0),
            Real(1.0)
        )
    }

    #[test]
    fn test_neg_int() {
        assert_evals!(
            Token::Minus, IntegerLiteral(1),
            Integer(-1)
        )
    }

    #[test]
    fn test_neg_real() {
        assert_evals!(
            Token::Minus, RealLiteral(1.0),
            Real(-1.0)
        )
    }

    #[test]
    fn test_not_bool_1() {
        assert_evals!(
            Token::Not, BooleanLiteral(true),
            Boolean(false)
        )
    }

    #[test]
    fn test_not_bool_2() {
        assert_evals!(
            Token::Not, BooleanLiteral(false),
            Boolean(true)
        )
    }

    #[test]
    fn test_or() {
        assert_evals!(
            BooleanLiteral(true), Token::Or, BooleanLiteral(false),
            Boolean(true)
        )
    }

    #[test]
    fn test_and() {
        assert_evals!(
            BooleanLiteral(true), Token::And, BooleanLiteral(false),
            Boolean(false)
        )
    }

    #[test]
    fn test_xor() {
        assert_evals!(
            BooleanLiteral(true), Token::Xor, BooleanLiteral(true),
            Boolean(false)
        )
    }

    #[test]
    fn test_coalesce_1() {
        assert_evals!(
            UndefinedLiteral, Token::Question, BooleanLiteral(true),
            Boolean(true)
        )
    }

    #[test]
    fn test_coalesce_2() {
        assert_evals!(
            BooleanLiteral(false), Token::Question, BooleanLiteral(true),
            Boolean(false)
        )
    }

    #[test]
    fn test_mod_int_int() {
        assert_evals!(
            IntegerLiteral(15), Token::Modulo, IntegerLiteral(4),
            Integer(3)
        )
    }

    #[test]
    fn test_assignment_set() {
        let mut env = Environment::new();
        let stmt = Statement::Assignment {
            identifier: String::from("foobar"),
            expression: IntegerLiteral(123)
        };
        let obj = env.eval_statement(&stmt);
        assert_eq!(
            mem::discriminant(&obj),
            mem::discriminant(&Undefined)
        );
    }

    #[test]
    fn test_assignment_get() {
        let mut env = Environment::new();
        let stmt = Statement::Assignment {
            identifier: String::from("foobar"),
            expression: IntegerLiteral(123)
        };
        env.eval_statement(&stmt);

        let stmt = Statement::ExpressionStatement {
            expression: Combination {
                left: Some(Box::new(Identifier(String::from("foobar")))),
                operator: Operation(Token::Plus, Location::Infix),
                right: Some(Box::new(IntegerLiteral(321)))
            }
        };
        let obj = env.eval_statement(&stmt);
        assert_eq!(obj, Integer(444));
    }
}
