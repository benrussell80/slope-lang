use super::environment::Environment;
use super::object::Object;
use super::errors::RuntimeError;
use super::parameter::Parameter;
use std::f64::consts::{E, PI};

// a module is just a rust function that edits the current environment in some way
pub type Module = fn(&mut Environment);

// define some built-in "modules"
pub fn math_constants_builtins(env: &mut Environment) {
    env.set(&"PI".to_string(), &Object::Real(PI)).unwrap();
    env.set(&"E".to_string(), &Object::Real(E)).unwrap();
}

pub fn set_builtins(env: &mut Environment) {
    env.set(&"max".to_string(), &Object::BuiltinFunction {
        parameters: vec![Parameter { name: "s".to_string() }],
        body: |args| {
            match args.len() {
                1 => match args.first().unwrap() {
                    Object::Set { items, .. } => {
                        match items.iter().next_back() {
                            Some(value) => Ok(value.clone()),
                            None => Ok(Object::Undefined)
                        }
                    },
                    obj => Err(RuntimeError::TypeError(format!("Expected a set got {}.", obj)))
                },
                num => Err(RuntimeError::OperatorError(format!("Expected 1 argument to `max` got {}.", num)))
            }
        }
    }).unwrap();

    env.set(&"min".to_string(), &Object::BuiltinFunction {
        parameters: vec![Parameter { name: "s".to_string() }],
        body: |args| {
            match args.len() {
                1 => match args.first().unwrap() {
                    Object::Set { items, .. } => {
                        match items.iter().next() {
                            Some(value) => Ok(value.clone()),
                            None => Ok(Object::Undefined)
                        }
                    },
                    obj => Err(RuntimeError::TypeError(format!("Expected a set got {}.", obj)))
                },
                num => Err(RuntimeError::OperatorError(format!("Expected 1 argument to `min` got {}.", num)))
            }
        }
    }).unwrap();

    env.set(&"sum".to_string(), &Object::BuiltinFunction {
        parameters: vec![Parameter { name: "s".to_string() }],
        body: |args| {
            match args.len() {
                1 => match args.first().unwrap() {
                    Object::Set { items, .. } => {
                        let mut iter = items.iter();
                        if let Some(acc) = iter.next() {
                            let mut acc = acc.clone();
                            loop {
                                match iter.next() {
                                    Some(next) => {
                                        match acc + next.clone() {
                                            Ok(value) => acc = value,
                                            Err(e) => break Err(e)
                                        }
                                    },
                                    None => break Ok(acc.clone())
                                }
                            }
                        } else {
                            Ok(Object::Undefined)
                        }
                    },
                    obj => Err(RuntimeError::TypeError(format!("Expected a set got {}.", obj)))
                },
                num => Err(RuntimeError::OperatorError(format!("Expected 1 argument to `sum` got {}.", num)))
            }
        }
    }).unwrap();

    env.set(&"product".to_string(), &Object::BuiltinFunction {
        parameters: vec![Parameter { name: "s".to_string() }],
        body: |args| {
            match args.len() {
                1 => match args.first().unwrap() {
                    Object::Set { items, .. } => {
                        let mut iter = items.iter();
                        if let Some(acc) = iter.next() {
                            let mut acc = acc.clone();
                            loop {
                                match iter.next() {
                                    Some(next) => {
                                        match acc * next.clone() {
                                            Ok(value) => acc = value,
                                            Err(e) => break Err(e)
                                        }
                                    },
                                    None => break Ok(acc.clone())
                                }
                            }
                        } else {
                            Ok(Object::Undefined)
                        }
                    },
                    obj => Err(RuntimeError::TypeError(format!("Expected a set got {}.", obj)))
                },
                num => Err(RuntimeError::OperatorError(format!("Expected 1 argument to `product` got {}.", num)))
            }
        }
    }).unwrap();

    // env.set(&"power_set".to_string(), &Object::BuiltinFunction {
    //     parameters: vec![Parameter { name: "s".to_string() }],
    //     body: |args| {
    //         match args.len() {
    //             1 => match args.first().unwrap() {
    //                 Object::Set { items, .. } => {
    //                     items.iter().tuple_combinations()
    //                 },
    //                 obj => Err(RuntimeError::TypeError(format!("Expected a set got {}.", obj)))
    //             },
    //             num => Err(RuntimeError::OperatorError(format!("Expected 1 argument to `power_set` got {}.", num)))
    //         }
    //     }
    // })?;
}