// use super::{Environment, Expression::*, Object::*, Token, Operator, Location, Statement};
use crate::ast::environment::Environment;
use crate::ast::expression::Expression::*;
use crate::ast::operator::Operator;
use crate::ast::location::Location;
use crate::ast::object::Object::*;
use crate::ast::statement::Statement;
use crate::interpreter::token::Token;
use std::mem;
// PlusMinus, MinusPlus (once sets are implemented)

macro_rules! assert_evals {
    ($token:expr, $right:expr, $obj:expr) => {
        // prefix expression
        assert_eq!(
            Environment::new().eval(&Combination {
                left: None,
                operator: Operator($token, Location::Prefix),
                right: Some(Box::new($right))
            }).unwrap(),
            $obj
        );
    };
    (postfix $left:expr, $token:expr, $obj:expr) => {
        // prefix expression
        assert_eq!(
            Environment::new().eval(&Combination {
                left: Some(Box::new($left)),
                operator: Operator($token, Location::Postfix),
                right: None,
            }).unwrap(),
            $obj
        );
    };
    ($left:expr, $token:expr, $right:expr, $obj:expr) => {
        // infix expression
        assert_eq!(
            Environment::new().eval(&Combination {
                left: Some(Box::new($left)),
                operator: Operator($token, Location::Infix),
                right: Some(Box::new($right))
            }).unwrap(),
            $obj
        );
    };
    ($left:expr, $token:expr, $right:expr, $obj:expr, tol=$tol:expr) => {
        // equality with tolerance
        let val = Environment::new().eval(&Combination {
            left: Some(Box::new($left)),
            operator: Operator($token, Location::Infix),
            right: Some(Box::new($right)),
        }).unwrap();
        assert!(
            Boolean((-$tol).unwrap() <= (val.clone() - $obj).unwrap()).and(&Boolean((val - $obj).unwrap() <= $tol)).unwrap() == Boolean(true)
        );
    };
}

#[test]
fn test_add_int_real() {
    assert_evals!(
        IntegerLiteral(2),
        Token::Plus,
        RealLiteral(40.0),
        Real(42.0)
    );
}

#[test]
fn test_add_int_int() {
    assert_evals!(
        IntegerLiteral(2),
        Token::Plus,
        IntegerLiteral(40),
        Integer(42)
    );
}

#[test]
fn test_add_real_real() {
    assert_evals!(RealLiteral(2.9), Token::Plus, RealLiteral(39.1), Real(42.0));
}

#[test]
fn test_add_real_int() {
    assert_evals!(
        RealLiteral(2.0),
        Token::Plus,
        IntegerLiteral(40),
        Real(42.0)
    );
}

#[test]
fn test_sub_int_int() {
    assert_evals!(
        IntegerLiteral(40),
        Token::Minus,
        IntegerLiteral(2),
        Integer(38)
    );
}

#[test]
fn test_sub_real_int() {
    assert_evals!(
        RealLiteral(40.0),
        Token::Minus,
        IntegerLiteral(2),
        Real(38.0)
    );
}

#[test]
fn test_sub_int_real() {
    assert_evals!(
        IntegerLiteral(2),
        Token::Minus,
        RealLiteral(40.0),
        Real(-38.0)
    );
}

#[test]
fn test_sub_real_real() {
    assert_evals!(
        RealLiteral(40.0),
        Token::Minus,
        RealLiteral(2.0),
        Real(38.0)
    );
}

#[test]
fn test_mult_int_int() {
    assert_evals!(
        IntegerLiteral(3),
        Token::Multiply,
        IntegerLiteral(3),
        Integer(9)
    );
}

#[test]
fn test_mult_int_real() {
    assert_evals!(
        IntegerLiteral(3),
        Token::Multiply,
        RealLiteral(3.2),
        Real(9.6),
        tol = Real(0.0001)
    );
}

#[test]
fn test_mult_real_int() {
    assert_evals!(
        RealLiteral(3.2),
        Token::Multiply,
        IntegerLiteral(3),
        Real(9.6),
        tol = Real(0.0001)
    );
}

#[test]
fn test_mult_real_real() {
    assert_evals!(
        RealLiteral(3.0),
        Token::Multiply,
        RealLiteral(3.2),
        Real(9.6),
        tol = Real(0.0001)
    );
}

#[test]
fn test_div_int_int() {
    assert_evals!(
        IntegerLiteral(3),
        Token::Multiply,
        IntegerLiteral(3),
        Integer(9)
    );
}

#[test]
fn test_div_int_real() {
    assert_evals!(
        IntegerLiteral(3),
        Token::Multiply,
        IntegerLiteral(3),
        Integer(9)
    );
}

#[test]
fn test_div_real_int() {
    assert_evals!(
        IntegerLiteral(3),
        Token::Multiply,
        IntegerLiteral(3),
        Integer(9)
    );
}

#[test]
fn test_div_real_real() {
    assert_evals!(
        IntegerLiteral(3),
        Token::Multiply,
        IntegerLiteral(3),
        Integer(9)
    );
}

#[test]
fn test_eq_int_int() {
    assert_evals!(
        IntegerLiteral(2),
        Token::Equals,
        IntegerLiteral(2),
        Boolean(true)
    )
}

#[test]
fn test_eq_int_real() {
    assert_evals!(
        IntegerLiteral(2),
        Token::Equals,
        RealLiteral(2.0),
        Boolean(true)
    )
}

#[test]
fn test_eq_real_int() {
    assert_evals!(
        RealLiteral(2.0),
        Token::Equals,
        IntegerLiteral(2),
        Boolean(true)
    )
}

#[test]
fn test_eq_real_real() {
    assert_evals!(
        RealLiteral(2.0),
        Token::Equals,
        RealLiteral(2.0),
        Boolean(true)
    )
}

#[test]
fn test_eq_undefined() {
    assert_evals!(
        UndefinedLiteral,
        Token::Equals,
        IntegerLiteral(2),
        Boolean(false)
    )
}

#[test]
fn test_eq_undefined_2() {
    assert_evals!(
        UndefinedLiteral,
        Token::Equals,
        UndefinedLiteral,
        Boolean(false)
    )
}

#[test]
fn test_eq_boolean() {
    assert_evals!(
        BooleanLiteral(true),
        Token::Equals,
        BooleanLiteral(false),
        Boolean(false)
    )
}

#[test]
fn test_ne_int_int() {
    assert_evals!(
        IntegerLiteral(2),
        Token::NotEquals,
        IntegerLiteral(1),
        Boolean(true)
    )
}

#[test]
fn test_ne_int_real() {
    assert_evals!(
        IntegerLiteral(2),
        Token::NotEquals,
        RealLiteral(2.0),
        Boolean(false)
    )
}

#[test]
fn test_ne_real_int() {
    assert_evals!(
        RealLiteral(2.0),
        Token::NotEquals,
        IntegerLiteral(2),
        Boolean(false)
    )
}

#[test]
fn test_ne_real_real() {
    assert_evals!(
        RealLiteral(2.0),
        Token::NotEquals,
        RealLiteral(2.0),
        Boolean(false)
    )
}

#[test]
fn test_ne_undefined() {
    assert_evals!(
        UndefinedLiteral,
        Token::NotEquals,
        UndefinedLiteral,
        Boolean(true)
    )
}

#[test]
fn test_ne_boolean() {
    assert_evals!(
        BooleanLiteral(true),
        Token::NotEquals,
        BooleanLiteral(false),
        Boolean(true)
    )
}

#[test]
fn test_gt_int_int() {
    assert_evals!(
        IntegerLiteral(1),
        Token::GreaterThan,
        IntegerLiteral(1),
        Boolean(false)
    )
}

#[test]
fn test_gt_int_real() {
    assert_evals!(
        IntegerLiteral(1),
        Token::GreaterThan,
        RealLiteral(1.0),
        Boolean(false)
    )
}

#[test]
fn test_gt_real_int() {
    assert_evals!(
        RealLiteral(1.0),
        Token::GreaterThan,
        IntegerLiteral(1),
        Boolean(false)
    )
}

#[test]
fn test_gt_real_real() {
    assert_evals!(
        RealLiteral(1.0),
        Token::GreaterThan,
        RealLiteral(1.0),
        Boolean(false)
    )
}

#[test]
fn test_gte_int_int() {
    assert_evals!(
        IntegerLiteral(1),
        Token::GreaterThanEquals,
        IntegerLiteral(1),
        Boolean(true)
    )
}

#[test]
fn test_gte_int_real() {
    assert_evals!(
        IntegerLiteral(1),
        Token::GreaterThanEquals,
        RealLiteral(1.0),
        Boolean(true)
    )
}

#[test]
fn test_gte_real_int() {
    assert_evals!(
        RealLiteral(1.0),
        Token::GreaterThanEquals,
        IntegerLiteral(1),
        Boolean(true)
    )
}

#[test]
fn test_gte_real_real() {
    assert_evals!(
        RealLiteral(1.0),
        Token::GreaterThanEquals,
        RealLiteral(1.0),
        Boolean(true)
    )
}

#[test]
fn test_lt_int_int() {
    assert_evals!(
        IntegerLiteral(1),
        Token::LessThan,
        IntegerLiteral(1),
        Boolean(false)
    )
}

#[test]
fn test_lt_int_real() {
    assert_evals!(
        IntegerLiteral(1),
        Token::LessThan,
        RealLiteral(1.0),
        Boolean(false)
    )
}

#[test]
fn test_lt_real_int() {
    assert_evals!(
        RealLiteral(1.0),
        Token::LessThan,
        IntegerLiteral(1),
        Boolean(false)
    )
}

#[test]
fn test_lt_real_real() {
    assert_evals!(
        RealLiteral(1.0),
        Token::LessThan,
        RealLiteral(1.0),
        Boolean(false)
    )
}

#[test]
fn test_lte_int_int() {
    assert_evals!(
        IntegerLiteral(1),
        Token::LessThanEquals,
        IntegerLiteral(1),
        Boolean(true)
    )
}

#[test]
fn test_lte_int_real() {
    assert_evals!(
        IntegerLiteral(1),
        Token::LessThanEquals,
        RealLiteral(1.0),
        Boolean(true)
    )
}

#[test]
fn test_lte_real_int() {
    assert_evals!(
        RealLiteral(1.0),
        Token::LessThanEquals,
        IntegerLiteral(1),
        Boolean(true)
    )
}

#[test]
fn test_lte_real_real() {
    assert_evals!(
        RealLiteral(1.0),
        Token::LessThanEquals,
        RealLiteral(1.0),
        Boolean(true)
    )
}

#[test]
fn test_exp_int_int() {
    assert_evals!(
        IntegerLiteral(1),
        Token::Exponent,
        IntegerLiteral(1),
        Integer(1)
    )
}

#[test]
fn test_exp_int_real() {
    assert_evals!(
        IntegerLiteral(1),
        Token::Exponent,
        RealLiteral(1.0),
        Real(1.0)
    )
}

#[test]
fn test_exp_real_int() {
    assert_evals!(
        RealLiteral(1.0),
        Token::Exponent,
        IntegerLiteral(1),
        Real(1.0)
    )
}

#[test]
fn test_exp_real_real() {
    assert_evals!(
        RealLiteral(1.0),
        Token::Exponent,
        RealLiteral(1.0),
        Real(1.0)
    )
}

#[test]
fn test_neg_int() {
    assert_evals!(Token::Minus, IntegerLiteral(1), Integer(-1))
}

#[test]
fn test_neg_real() {
    assert_evals!(Token::Minus, RealLiteral(1.0), Real(-1.0))
}

#[test]
fn test_not_bool_1() {
    assert_evals!(Token::Not, BooleanLiteral(true), Boolean(false))
}

#[test]
fn test_not_bool_2() {
    assert_evals!(Token::Not, BooleanLiteral(false), Boolean(true))
}

#[test]
fn test_or() {
    assert_evals!(
        BooleanLiteral(true),
        Token::Or,
        BooleanLiteral(false),
        Boolean(true)
    )
}

#[test]
fn test_and() {
    assert_evals!(
        BooleanLiteral(true),
        Token::And,
        BooleanLiteral(false),
        Boolean(false)
    )
}

#[test]
fn test_xor() {
    assert_evals!(
        BooleanLiteral(true),
        Token::Xor,
        BooleanLiteral(true),
        Boolean(false)
    )
}

#[test]
fn test_coalesce_1() {
    assert_evals!(
        UndefinedLiteral,
        Token::Question,
        BooleanLiteral(true),
        Boolean(true)
    )
}

#[test]
fn test_coalesce_2() {
    assert_evals!(
        BooleanLiteral(false),
        Token::Question,
        BooleanLiteral(true),
        Boolean(false)
    )
}

#[test]
fn test_mod_int_int() {
    assert_evals!(
        IntegerLiteral(15),
        Token::Modulo,
        IntegerLiteral(4),
        Integer(3)
    )
}

#[test]
fn test_assignment_set() {
    let mut env = Environment::new();
    let stmt = Statement::Assignment {
        identifier: String::from("foobar"),
        expression: IntegerLiteral(123),
    };
    let obj = env.eval_statement(&stmt).unwrap();
    assert_eq!(mem::discriminant(&obj), mem::discriminant(&Undefined));
}

#[test]
fn test_assignment_get() {
    let mut env = Environment::new();
    let stmt = Statement::Assignment {
        identifier: String::from("foobar"),
        expression: IntegerLiteral(123),
    };
    env.eval_statement(&stmt).unwrap();

    let stmt = Statement::ExpressionStatement {
        expression: Combination {
            left: Some(Box::new(Identifier(String::from("foobar")))),
            operator: Operator(Token::Plus, Location::Infix),
            right: Some(Box::new(IntegerLiteral(321))),
        },
    };
    let obj = env.eval_statement(&stmt).unwrap();
    assert_eq!(obj, Integer(444));
}

#[test]
fn test_abs_val() {
    let mut env = Environment::new();
    // |-2|;
    let stmt = Statement::ExpressionStatement {
        expression: AbsoluteValue(Box::new(Combination {
            left: None,
            operator: Operator(Token::Minus, Location::Prefix),
            right: Some(Box::new(IntegerLiteral(2)))
        })),
    };
    let obj = env.eval_statement(&stmt).unwrap();
    assert_eq!(obj, Integer(2))
}

#[test]
fn test_nested_abs_val() {
    let mut env = Environment::new();
    // |-2 + |-4||; 
    let stmt = Statement::ExpressionStatement {
        expression: AbsoluteValue(Box::new(Combination {
            left: Some(Box::new(Combination {
                    left: None,
                    operator: Operator(Token::Minus, Location::Prefix),
                    right: Some(Box::new(IntegerLiteral(2)))
            })),
            operator: Operator(Token::Plus, Location::Infix),
            right: Some(Box::new(AbsoluteValue(Box::new(
                Combination {
                    left: None,
                    operator: Operator(Token::Minus, Location::Prefix),
                    right: Some(Box::new(IntegerLiteral(4)))
                }))
            ))
        }))
    };
    let obj = env.eval_statement(&stmt).unwrap();
    assert_eq!(obj, Integer(2))
}

#[test]
fn test_set_literal_assignment() {
    let mut env = Environment::new();
    let stmt = Statement::Assignment {
        identifier: String::from("foobar"),
        expression: SetLiteral(vec![
            IntegerLiteral(1),
            IntegerLiteral(2),
            IntegerLiteral(3),
        ]),
    };
    let obj = env.eval_statement(&stmt).unwrap();
    assert_eq!(mem::discriminant(&obj), mem::discriminant(&Undefined));
}

#[test]
fn test_set_literal_assignment_with_reals() {
    let mut env = Environment::new();
    let stmt = Statement::Assignment {
        identifier: String::from("foobar"),
        expression: SetLiteral(vec![
            RealLiteral(1.0),
            RealLiteral(2.0),
            RealLiteral(3.0),
        ]),
    };
    let obj = env.eval_statement(&stmt).unwrap();
    assert_eq!(mem::discriminant(&obj), mem::discriminant(&Undefined));
}

#[test]
#[should_panic]
fn test_set_literal_with_different_types() {
    let mut env = Environment::new();
    let stmt = Statement::Assignment {
        identifier: String::from("foobar"),
        expression: SetLiteral(vec![
            RealLiteral(1.0),
            BooleanLiteral(true),
        ]),
    };
    env.eval_statement(&stmt).unwrap();
}

#[test]
#[should_panic]
fn test_set_literal_with_undefined() {
    let mut env = Environment::new();
    let stmt = Statement::Assignment {
        identifier: String::from("foobar"),
        expression: SetLiteral(vec![
            UndefinedLiteral
        ]),
    };
    env.eval_statement(&stmt).unwrap();
}

#[test]
fn test_set_of_sets() {
    let mut env = Environment::new();
    let stmt = Statement::Assignment {
        identifier: String::from("foobar"),
        expression: SetLiteral(vec![
            SetLiteral(vec![
                IntegerLiteral(1)
            ]),
            SetLiteral(vec![
                IntegerLiteral(2)
            ])
        ]),
    };
    env.eval_statement(&stmt).unwrap();
    assert_eq!("{ { 1 }, { 2 } }", env.get("foobar").unwrap().to_string());
}

#[test]
fn test_set_of_duplicate_integers() {
    let mut env = Environment::new();
    let stmt = Statement::Assignment {
        identifier: String::from("foobar"),
        expression: SetLiteral(vec![
            SetLiteral(vec![
                IntegerLiteral(1)
            ]),
            SetLiteral(vec![
                IntegerLiteral(1)
            ])
        ]),
    };
    env.eval_statement(&stmt).unwrap();
    assert_eq!("{ { 1 } }", env.get("foobar").unwrap().to_string());
}

#[test]
fn test_integer_in_set() {
    let mut env = Environment::new();
    let stmt = Statement::ExpressionStatement {
        expression: Combination {
            left: Some(Box::new(IntegerLiteral(1))),
            operator: Operator(Token::In, Location::Infix),
            right: Some(Box::new(SetLiteral(vec![
                IntegerLiteral(1),
                IntegerLiteral(2),
                IntegerLiteral(3),
            ])))
        }
    };
    let obj = env.eval_statement(&stmt).unwrap();
    assert_eq!(obj, Boolean(true));
}

#[test]
fn test_set_union() {
    let mut env = Environment::new();
    let stmt = Statement::ExpressionStatement {
        expression: Combination {
            left: Some(Box::new(SetLiteral(vec![
                RealLiteral(1.1),
                RealLiteral(2.1),
            ]))),
            operator: Operator(Token::Union, Location::Infix),
            right: Some(Box::new(SetLiteral(vec![
                RealLiteral(3.1),
                RealLiteral(4.1),
            ]))),
        }
    };
    let obj = env.eval_statement(&stmt).unwrap();
    assert_eq!(obj.to_string(), "{ 1.1, 2.1, 3.1, 4.1 }");
}

#[test]
fn test_set_difference() {
    let mut env = Environment::new();
    let stmt = Statement::ExpressionStatement {
        expression: Combination {
            left: Some(Box::new(SetLiteral(vec![
                RealLiteral(1.1),
                RealLiteral(2.1),
            ]))),
            operator: Operator(Token::SetDifference, Location::Infix),
            right: Some(Box::new(SetLiteral(vec![
                RealLiteral(2.1),
                RealLiteral(3.1),
            ]))),
        }
    };
    let obj = env.eval_statement(&stmt).unwrap();
    assert_eq!(obj.to_string(), "{ 1.1 }");
}

#[test]
fn test_set_intersection() {
    let mut env = Environment::new();
    let stmt = Statement::ExpressionStatement {
        expression: Combination {
            left: Some(Box::new(SetLiteral(vec![
                RealLiteral(1.1),
                RealLiteral(2.1),
            ]))),
            operator: Operator(Token::Intersection, Location::Infix),
            right: Some(Box::new(SetLiteral(vec![
                RealLiteral(2.1),
                RealLiteral(3.1),
            ]))),
        }
    };
    let obj = env.eval_statement(&stmt).unwrap();
    assert_eq!(obj.to_string(), "{ 2.1 }");
}

#[test]
fn test_set_symmetric_difference() {
    let mut env = Environment::new();
    let stmt = Statement::ExpressionStatement {
        expression: Combination {
            left: Some(Box::new(SetLiteral(vec![
                RealLiteral(1.1),
                RealLiteral(2.1),
                RealLiteral(3.1),
            ]))),
            operator: Operator(Token::SymmetricDifference, Location::Infix),
            right: Some(Box::new(SetLiteral(vec![
                RealLiteral(2.1),
                RealLiteral(3.1),
                RealLiteral(4.1),
            ]))),
        }
    };
    let obj = env.eval_statement(&stmt).unwrap();
    assert_eq!(obj.to_string(), "{ 1.1, 4.1 }");
}

#[test]
fn test_set_subset() {
    let mut env = Environment::new();
    let stmt = Statement::ExpressionStatement {
        expression: Combination {
            left: Some(Box::new(SetLiteral(vec![
                RealLiteral(1.1),
                RealLiteral(2.1),
                RealLiteral(3.1),
            ]))),
            operator: Operator(Token::LessThanEquals, Location::Infix),
            right: Some(Box::new(SetLiteral(vec![
                RealLiteral(1.1),
                RealLiteral(2.1),
                RealLiteral(3.1),
                RealLiteral(4.1),
                RealLiteral(5.1),
            ]))),
        }
    };
    let obj = env.eval_statement(&stmt).unwrap();
    assert_eq!(obj, Boolean(true));
}

#[test]
fn test_set_proper_subset() {
    let mut env = Environment::new();
    let stmt = Statement::ExpressionStatement {
        expression: Combination {
            left: Some(Box::new(SetLiteral(vec![
                RealLiteral(1.1),
                RealLiteral(2.1),
                RealLiteral(3.1),
            ]))),
            operator: Operator(Token::LessThan, Location::Infix),
            right: Some(Box::new(SetLiteral(vec![
                RealLiteral(1.1),
                RealLiteral(2.1),
                RealLiteral(3.1),
                RealLiteral(4.1),
                RealLiteral(5.1),
            ]))),
        }
    };
    let obj = env.eval_statement(&stmt).unwrap();
    assert_eq!(obj, Boolean(true));
}

#[test]
fn test_factorial() {
    assert_evals!(
        postfix IntegerLiteral(4),
        Token::Bang,
        Integer(24)
    )
}