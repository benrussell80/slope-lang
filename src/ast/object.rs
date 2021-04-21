use std::fmt::{Display, Formatter, self};
use super::parameter::Parameter;
use super::expression::Expression;
use std::ops::{Add, Sub, Mul, Div, Neg, Not};
use std::cmp::Ordering;

#[derive(Debug, Clone)]
pub enum Object {
    Integer(i64),
    Real(f64),
    Boolean(bool),
    Undefined,
    Function {
        parameters: Vec<Parameter>,
        expression: Expression,
    },
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
            _ => self.clone(),
        }
    }

    pub fn abs(&self) -> Self {
        use Object::*;
        match self {
            Integer(value) => Integer(value.abs()),
            Real(value) => Real(value.abs()),
            Undefined => Undefined,
            _ => Undefined,
        }
    }

    pub fn and(&self, rhs: &Self) -> Self {
        use Object::*;
        match (self, rhs) {
            (Boolean(left), Boolean(right)) => Boolean(*left && *right),
            _ => Undefined,
        }
    }

    pub fn or(&self, rhs: &Self) -> Self {
        use Object::*;
        // maybe use trait Into<bool>
        match (self, rhs) {
            (Boolean(left), Boolean(right)) => Boolean(*left || *right),
            _ => Undefined,
        }
    }

    pub fn xor(&self, rhs: &Self) -> Self {
        use Object::*;
        match (self, rhs) {
            (Boolean(left), Boolean(right)) => Boolean((*left || *right) && !(*left && *right)),
            _ => Undefined,
        }
    }

    pub fn modulo(&self, rhs: &Self) -> Self {
        use Object::*;
        match (self, rhs) {
            // use Rem trait instead
            (Integer(left), Integer(right)) => Integer(left % right),
            _ => Undefined,
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
            Function {
                parameters,
                expression,
            } => write!(
                f,
                "fn({}) = {};",
                parameters.iter().fold(String::new(), |mut acc, param| {
                    if !acc.is_empty() {
                        acc.push_str(", ");
                    };
                    acc.push_str(&param.to_string());
                    acc
                }),
                expression
            )
        }
    }
}

impl Eq for Object {}

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

impl Ord for Object {
    fn cmp(&self, other: &Self) -> Ordering {
        use Object::*;
        match (self, other) {
            (Integer(left), Integer(right)) => left.cmp(&right),

            _ => panic!("Cannot compare these two types."),
        }
    }
}

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
