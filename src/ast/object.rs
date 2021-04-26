use super::errors::RuntimeError;
use std::fmt::{Display, Formatter, self};
use super::parameter::Parameter;
use super::expression::Expression;
use std::ops::{Add, Sub, Mul, Div, Neg, Not};
use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::mem::{Discriminant, self};
use std::hash::{Hash, Hasher};
use rust_decimal::prelude::*;
use std::convert::From;

#[derive(Debug, Clone)]
struct Condition {
    expression: Expression,
}

#[derive(Debug, Clone)]
pub enum Object {
    Integer(i64),
    // Natural(u64),
    // Rational {
    //     numerator: i64,
    //     denominator: i64
    // },
    // Complex {
    //     real: f64,
    //     imag: f64,
    // },
    Real(f64),
    Boolean(bool),
    Undefined,
    Function {
        parameters: Vec<Parameter>,
        expression: Expression,
    },
    Set {
        items: BTreeSet<Object>,
        kind: Option<Discriminant<Object>>,  // sets should contain elements of the same "type" (e.g. integers only)
    },
    // SetBuilder {
    //     expression: Expression,

    //     parent_set: Box<Object>,
    //     conditions: Vec<Condition>
    // },
    // Tuple {
    //     items: Vec<Object>
    // },
    BuiltinFunction {
        parameters: Vec<Parameter>,
        body: fn(Vec<Object>) -> Result<Object, RuntimeError>
    }
}

impl From<bool> for Object {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}

impl From<i64> for Object {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}

impl From<f64> for Object {
    fn from(value: f64) -> Self {
        Self::Real(value)
    }
}

impl Hash for Object {
    fn hash<H: Hasher>(&self, state: &mut H) {
        use Object::*;
        match self {
            Integer(value) => value.hash(state),
            Real(value) => Decimal::from_f64(*value).unwrap().hash(state),
            Boolean(value) => value.hash(state),
            Undefined => Undefined.hash(state),
            Function { parameters, expression } => {
                parameters.hash(state);
                expression.hash(state);
            },
            Set { items, kind } => {
                items.hash(state);
                kind.hash(state);
            }
            BuiltinFunction { parameters, body } => {
                parameters.hash(state);
                body.hash(state);
            }
        }
    }
}

impl Object {
    pub fn is_undefined(&self) -> bool {
        mem::discriminant(self) == mem::discriminant(&Object::Undefined)
    }

    pub fn factorial(&self) -> Result<Self, RuntimeError> {
        fn fact(num: &i64) -> i64 {
            if num < &0 {
                panic!("Expected positive integer.")
            } else if num == &0 {
                1
            } else {
                num * fact(&(num - 1))
            }
        }
        use Object::*;
        match self {
            Integer(value) => {
                if value < &0 {
                    Err(RuntimeError::OperatorError(format!("Cannot use factorial on a negative integer {}.", value)))
                } else {
                    Ok(Integer(fact(value)))
                }
            },
            obj => Err(RuntimeError::TypeError(format!("Cannot use factorial on {} (expected a positive integer or zero).", obj)))
        }
    }

    pub fn pow(&self, rhs: &Self) -> Result<Self, RuntimeError> {
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
                Ok(Undefined)
            } else {
                Ok(Real(num))
            }
        } else {
            Err(RuntimeError::OperatorError(format!("Cannot exponentiate {} and {}.", self, rhs)))
        }
    }

    pub fn coalesce(&self, rhs: &Self) -> Self {
        use Object::*;
        match self {
            Undefined => rhs.clone(),
            _ => self.clone(),
        }
    }

    pub fn abs(&self) -> Result<Self, RuntimeError> {
        use Object::*;
        match self {
            Integer(value) => Ok(Integer(value.abs())),
            Real(value) => Ok(Real(value.abs())),
            Set { items, .. } => Ok(Integer(items.len() as i64)),
            obj => Err(RuntimeError::OperatorError(format!("Cannot take absolute value of {}.", obj)))
        }
    }

    pub fn and(&self, rhs: &Self) -> Result<Self, RuntimeError> {
        use Object::*;
        match (self, rhs) {
            (Boolean(left), Boolean(right)) => Ok(Boolean(*left && *right)),
            (left, right) => Err(RuntimeError::OperatorError(format!("Cannot take boolean-and of {} and {}.", left, right)))
        }
    }

    pub fn or(&self, rhs: &Self) -> Result<Self, RuntimeError> {
        use Object::*;
        // maybe use trait Into<bool>
        match (self, rhs) {
            (Boolean(left), Boolean(right)) => Ok(Boolean(*left || *right)),
            (left, right) => Err(RuntimeError::OperatorError(format!("Cannot take boolean-or of {} and {}.", left, right)))
        }
    }

    pub fn xor(&self, rhs: &Self) -> Result<Self, RuntimeError> {
        use Object::*;
        match (self, rhs) {
            (Boolean(left), Boolean(right)) => Ok(Boolean((*left || *right) && !(*left && *right))),
            (left, right) => Err(RuntimeError::OperatorError(format!("Cannot take boolean-xor of {} and {}.", left, right))),
        }
    }

    pub fn modulo(&self, rhs: &Self) -> Result<Self, RuntimeError> {
        use Object::*;
        match (self, rhs) {
            // use Rem trait instead
            (Integer(left), Integer(right)) => Ok(Integer(left % right)),
            (left, right) => Err(RuntimeError::OperatorError(format!("Cannot take modulo of {} and {}.", left, right)))
        }
    }

    pub fn in_(&self, rhs: &Self) -> Result<Self, RuntimeError> {
        use Object::*;
        match (self, rhs) {
            (any, Set { items, kind }) => {
                match kind {
                    None => Ok(Boolean(false)),
                    Some(disc) => {
                        if disc == &mem::discriminant(any) {
                            Ok(Boolean(items.contains(any)))
                        } else {
                            Err(RuntimeError::TypeError("Cannot check for containment with differing types.".into()))
                        }
                    }
                }
            },
            (left, right) => Err(RuntimeError::OperatorError(format!("Cannot use containment operator for {} and {}.", left, right)))
        }
    }

    pub fn pm(&self, rhs: &Self) -> Result<Self, RuntimeError> {
        let one = self.clone().add(rhs.clone())?;
        let two = self.clone().sub(rhs.clone())?;
        let mut items = BTreeSet::new();
        let kind = Some(mem::discriminant(&one));
        items.insert(one);
        items.insert(two);
        Ok(Object::Set {
            items,
            kind
        })
    }

    pub fn set_difference(&self, rhs: &Self) -> Result<Self, RuntimeError> {
        if let (Object::Set { items: items1, kind: kind1 }, Object::Set { items: items2, kind: kind2 }) = (self, rhs) {
            if kind1 == kind2 {
                let items = items1.difference(&items2).cloned().collect::<BTreeSet<Object>>();
                Ok(Object::Set {
                    items: items,
                    kind: *kind1
                })
            } else {
                Err(RuntimeError::OperatorError(format!("Cannot use set-difference for {} and {}.", self, rhs)))
            }
        } else {
            Err(RuntimeError::OperatorError(format!("Cannot use set-difference for {} and {}.", self, rhs)))
        }
    }

    pub fn set_intersection(&self, rhs: &Self) -> Result<Self, RuntimeError> {
        if let (Object::Set { items: items1, kind: kind1 }, Object::Set { items: items2, kind: kind2 }) = (self, rhs) {
            if kind1 == kind2 {
                let items = items1.intersection(&items2).cloned().collect::<BTreeSet<Object>>();
                Ok(Object::Set {
                    items: items,
                    kind: *kind1
                })
            } else {
                Err(RuntimeError::OperatorError(format!("Cannot use set-intersection for {} and {}.", self, rhs)))
            }
        } else {
            Err(RuntimeError::OperatorError(format!("Cannot use set-intersection for {} and {}.", self, rhs)))
        }
    }

    pub fn set_symmetric_difference(&self, rhs: &Self) -> Result<Self, RuntimeError> {
        if let (Object::Set { items: items1, kind: kind1 }, Object::Set { items: items2, kind: kind2 }) = (self, rhs) {
            if kind1 == kind2 {
                let items = items1.symmetric_difference(&items2).cloned().collect::<BTreeSet<Object>>();
                Ok(Object::Set {
                    items: items,
                    kind: *kind1
                })
            } else {
                Err(RuntimeError::OperatorError(format!("Cannot use set-symmetric difference for {} and {}.", self, rhs)))
            }
        } else {
            Err(RuntimeError::OperatorError(format!("Cannot use set-symmetric difference for {} and {}.", self, rhs)))
        }
    }

    pub fn set_union(&self, rhs: &Self) -> Result<Self, RuntimeError> {
        if let (Object::Set { items: items1, kind: kind1 }, Object::Set { items: items2, kind: kind2 }) = (self, rhs) {
            if kind1 == kind2 {
                let items = items1.union(&items2).cloned().collect::<BTreeSet<Object>>();
                Ok(Object::Set {
                    items: items,
                    kind: *kind1
                })
            } else {
                Err(RuntimeError::OperatorError(format!("Cannot use set-union for {} and {}.", self, rhs)))
            }
        } else {
            Err(RuntimeError::OperatorError(format!("Cannot use set-union for {} and {}.", self, rhs)))
        }
    }

    pub fn is_subset(&self, rhs: &Self) -> Result<Self, RuntimeError> {
        if let (Object::Set { items: items1, kind: kind1 }, Object::Set { items: items2, kind: kind2 }) = (self, rhs) {
            if kind1 == kind2 {
                Ok(Object::Boolean(items1.is_subset(&items2)))
            } else {
                Err(RuntimeError::OperatorError(format!("Cannot use set-subset for {} and {}.", self, rhs)))
            }
        } else {
            Err(RuntimeError::OperatorError(format!("Cannot use set-subset for {} and {}.", self, rhs)))
        }
    }

    pub fn is_proper_subset(&self, rhs: &Self) -> Result<Self, RuntimeError> {
        if let (Object::Set { items: items1, kind: kind1 }, Object::Set { items: items2, kind: kind2 }) = (self, rhs) {
            if kind1 == kind2 {
                Ok(Object::Boolean(items1.is_subset(&items2) && items1.len() != items2.len()))
            } else {
                Err(RuntimeError::OperatorError(format!("Cannot use set-propert subset for {} and {}.", self, rhs)))
            }
        } else {
            Err(RuntimeError::OperatorError(format!("Cannot use set-propert subset for {} and {}.", self, rhs)))
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
            ),
            Set { items, .. } => write!(f, "{{ {} }}", items.iter().fold(String::new(), |mut acc, member| {
                if !acc.is_empty() {
                    acc.push_str(", ");
                };
                acc.push_str(&member.to_string());
                acc
            })),
            BuiltinFunction { parameters, .. } => write!(
                f,
                "fn({}) = <builtin-function>;",
                parameters.iter().fold(String::new(), |mut acc, param| {
                    if !acc.is_empty() {
                        acc.push_str(", ");
                    };
                    acc.push_str(&param.to_string());
                    acc
                })
            )
        }
    }
}

impl Eq for Object {}

// changes these operations to use just functions that return a result
// that way the error can be handled

impl PartialOrd for Object {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        use Object::*;
        match (self, other) {
            (Integer(left), Integer(right)) => left.partial_cmp(&right),
            (Real(left), Integer(right)) => left.partial_cmp(&(*right as f64)),
            (Integer(left), Real(right)) => left.partial_cmp(&(*right as i64)),
            (Real(left), Real(right)) => left.partial_cmp(right),
            (left, right) => panic!("Cannot compare {} and {}.", left, right),
        }
    }
}

impl Ord for Object {
    fn cmp(&self, other: &Self) -> Ordering {
        use Object::*;
        match (self, other) {
            (Integer(left), Integer(right)) => left.cmp(&right),
            (Real(left), Real(right)) => Decimal::from_f64(*left).cmp(&Decimal::from_f64(*right)),
            (Set { items, kind }, Set { items: items2, kind: kind2 }) if kind == kind2 => {
                items.cmp(&items2)
            },
            (Boolean(left), Boolean(right)) => left.cmp(right),
            (left, right) => panic!("Cannot compare {} and {}.", left, right),
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
            (left, right) => panic!("Cannot equate {} and {}.", left, right),
        }
    }
}

impl Add for Object {
    type Output = Result<Object, RuntimeError>;
    fn add(self, rhs: Self) -> Self::Output {
        use Object::*;
        match (self, rhs) {
            (Integer(left), Integer(right)) => Ok(Integer(left + right)),
            (Integer(left), Real(right)) => Ok(Real(left as f64 + right)),
            (Real(left), Integer(right)) => Ok(Real(left + right as f64)),
            (Real(left), Real(right)) => Ok(Real(left + right)),
            (left, right) => Err(RuntimeError::OperatorError(format!("Cannot add {} and {}.", left, right))),
        }
    }
}

impl Mul for Object {
    type Output = Result<Object, RuntimeError>;
    fn mul(self, rhs: Self) -> Self::Output {
        use Object::*;
        match (self, rhs) {
            (Integer(left), Integer(right)) => Ok(Integer(left * right)),
            (Integer(left), Real(right)) => Ok(Real(left as f64 * right)),
            (Real(left), Integer(right)) => Ok(Real(left * right as f64)),
            (Real(left), Real(right)) => Ok(Real(left * right)),
            (left, right) => Err(RuntimeError::OperatorError(format!("Cannot multiply {} and {}.", left, right))),
        }
    }
}

impl Sub for Object {
    type Output = Result<Object, RuntimeError>;
    fn sub(self, rhs: Self) -> Self::Output {
        use Object::*;
        match (self, rhs) {
            (Integer(left), Integer(right)) => Ok(Integer(left - right)),
            (Integer(left), Real(right)) => Ok(Real(left as f64 - right)),
            (Real(left), Integer(right)) => Ok(Real(left - right as f64)),
            (Real(left), Real(right)) => Ok(Real(left - right)),
            (left, right) => Err(RuntimeError::OperatorError(format!("Cannot subtract {} and {}.", left, right))),
        }
    }
}

impl Div for Object {
    type Output = Result<Object, RuntimeError>;
    fn div(self, rhs: Self) -> Self::Output {
        use Object::*;
        match (self, rhs) {
            (Integer(left), Integer(right)) => {
                if right == 0 {
                    Ok(Undefined)
                } else {
                    Ok(Real(left as f64 / right as f64))
                }
            }
            (Integer(left), Real(right)) => {
                let value = left as f64 / right;
                if value.is_nan() || value.is_infinite() {
                    Ok(Undefined)
                } else {
                    Ok(Real(value))
                }
            }
            (Real(left), Integer(right)) => {
                let value = left / right as f64;
                if value.is_nan() || value.is_infinite() {
                    Ok(Undefined)
                } else {
                    Ok(Real(value))
                }
            }
            (Real(left), Real(right)) => {
                let value = left / right;
                if value.is_nan() || value.is_infinite() {
                    Ok(Undefined)
                } else {
                    Ok(Real(value))
                }
            }
            (left, right) => Err(RuntimeError::OperatorError(format!("Cannot divide {} and {}.", left, right))),
        }
    }
}

impl Neg for Object {
    type Output = Result<Object, RuntimeError>;
    fn neg(self) -> Self::Output {
        use Object::*;
        match self {
            Integer(value) => Ok(Integer(-value)),
            Real(value) => Ok(Real(-value)),
            obj => Err(RuntimeError::OperatorError(format!("Cannot negate {}.", obj))),
        }
    }
}

impl Not for Object {
    type Output = Result<Object, RuntimeError>;
    fn not(self) -> Self::Output {
        use Object::*;
        match self {
            Boolean(value) => Ok(Boolean(!value)),
            obj => Err(RuntimeError::OperatorError(format!("Cannot boolean-negate {}.", obj))),
        }
    }
}
