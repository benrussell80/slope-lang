pub mod test_objects;
use crate::ast::expression::Expression;
use crate::ast::parameter::Parameter;
use crate::ast::parser::Parser;
use crate::ast::statement::Statement::*;
use crate::ast::location::Location::*;
use crate::interpreter::lexer::LexerIterator;
use crate::interpreter::token::Token;
use crate::ast::operator::Operator;

macro_rules! parse {
    ($text:expr, $statements:expr) => {
        assert_eq!(Parser::new(LexerIterator::new($text.chars().peekable())).parse_program().unwrap(), $statements);
    };
}

macro_rules! bad_parsing {
    ($name:ident, $text:expr) => {
        #[test]
        #[should_panic]
        fn $name() {
            let text = $text;
            let parser = Parser::new(LexerIterator::new(text.chars().peekable()));
            parser.parse_program().unwrap();
        }
    };
}

#[test]
fn test_real_assignment() {
    parse!(
        "let pi = 3.14;",
        vec![Assignment {
            identifier: "pi".into(),
            expression: Expression::RealLiteral(3.14)
        }]
    );
}

#[test]
fn test_negative_integer_assignment() {
    parse!(
        "let negOne = -1;",
        vec![Assignment {
            identifier: "negOne".into(),
            expression: Expression::Combination {
                left: None,
                operator: Operator(Token::Minus, Prefix),
                right: Some(Box::new(Expression::IntegerLiteral(1)))
            }
        }]
    );
}

#[test]
fn test_integer_assignment() {
    parse!(
        "let one = 1;",
        vec![Assignment {
            identifier: "one".into(),
            expression: Expression::IntegerLiteral(1)
        }]
    );
}
#[test]
#[should_panic]
fn test_missing_expression() {
    let text = "let value =;";
    let parser = Parser::new(LexerIterator::new(text.chars().peekable()));
    parser.parse_program().unwrap();
}

#[test]
fn test_function_statement() {
    parse!(
        "fn area(radius) = pi * radius ^ 2;",
        vec![FunctionDeclaration {
            identifier: "area".into(),
            parameters: vec![Parameter {
                name: "radius".into()
            }],
            expression: Expression::Combination {
                left: Some(Box::new(Expression::Identifier("pi".into()))),
                operator: Operator(Token::Multiply, Infix),
                right: Some(Box::new(Expression::Combination {
                    left: Some(Box::new(Expression::Identifier("radius".into()))),
                    operator: Operator(Token::Exponent, Infix),
                    right: Some(Box::new(Expression::IntegerLiteral(2)))
                }))
            }
        }]
    );
}

#[test]
fn test_function_statement_with_params() {
    parse!(
        "fn area(width, height) = width * height;",
        vec![FunctionDeclaration {
            identifier: "area".into(),
            parameters: vec![
                Parameter {
                    name: "width".into()
                },
                Parameter {
                    name: "height".into()
                }
            ],
            expression: Expression::Combination {
                left: Some(Box::new(Expression::Identifier("width".into()))),
                operator: Operator(Token::Multiply, Infix),
                right: Some(Box::new(Expression::Identifier("height".into())))
            }
        }]
    );
}

#[test]
fn test_multiple_statements() {
    parse!(
        "\
        let pi = 3.14;
        fn area(radius) = pi * radius ^ 2;
        ",
        vec![
            Assignment {
                identifier: "pi".into(),
                expression: Expression::RealLiteral(3.14)
            },
            FunctionDeclaration {
                identifier: "area".into(),
                parameters: vec![Parameter {
                    name: "radius".into()
                }],
                expression: Expression::Combination {
                    left: Some(Box::new(Expression::Identifier("pi".into()))),
                    operator: Operator(Token::Multiply, Infix),
                    right: Some(Box::new(Expression::Combination {
                        left: Some(Box::new(Expression::Identifier("radius".into()))),
                        operator: Operator(Token::Exponent, Infix),
                        right: Some(Box::new(Expression::IntegerLiteral(2)))
                    }))
                }
            }
        ]
    );
}

#[test]
fn test_expression_statement() {
    parse!(
        "foobar;",
        vec![ExpressionStatement {
            expression: Expression::Identifier("foobar".into())
        }]
    );
}

#[test]
fn test_integer_literal_expression() {
    parse!(
        "5;",
        vec![ExpressionStatement {
            expression: Expression::IntegerLiteral(5)
        }]
    );
}

#[test]
fn test_not_operator() {
    parse!(
        "not true;",
        vec![ExpressionStatement {
            expression: Expression::Combination {
                left: None,
                operator: Operator(Token::Not, Prefix),
                right: Some(Box::new(Expression::BooleanLiteral(true)))
            }
        }]
    );
}

#[test]
fn test_not_not_operator() {
    parse!(
        "not not true;",
        vec![ExpressionStatement {
            expression: Expression::Combination {
                left: None,
                operator: Operator(Token::Not, Prefix),
                right: Some(Box::new(Expression::Combination {
                    left: None,
                    operator: Operator(Token::Not, Prefix),
                    right: Some(Box::new(Expression::BooleanLiteral(true)))
                }))
            }
        }]
    );
}

#[test]
fn test_negative_number() {
    parse!(
        "-5;",
        vec![ExpressionStatement {
            expression: Expression::Combination {
                left: None,
                operator: Operator(Token::Minus, Prefix),
                right: Some(Box::new(Expression::IntegerLiteral(5)))
            }
        }]
    );
}

#[test]
fn test_negative_negative_number() {
    parse!(
        "--5;",
        vec![ExpressionStatement {
            expression: Expression::Combination {
                left: None,
                operator: Operator(Token::Minus, Prefix),
                right: Some(Box::new(Expression::Combination {
                    left: None,
                    operator: Operator(Token::Minus, Prefix),
                    right: Some(Box::new(Expression::IntegerLiteral(5)))
                }))
            }
        }]
    );
}

#[test]
fn composite_expression() {
    parse!(
        "5 + 7 * 2;",
        vec![ExpressionStatement {
            expression: Expression::Combination {
                operator: Operator(Token::Plus, Infix),
                left: Some(Box::new(Expression::IntegerLiteral(5))),
                right: Some(Box::new(Expression::Combination {
                    operator: Operator(Token::Multiply, Infix),
                    left: Some(Box::new(Expression::IntegerLiteral(7))),
                    right: Some(Box::new(Expression::IntegerLiteral(2)))
                }))
            }
        }]
    );
}

#[test]
fn composite_expression_2() {
    parse!(
        "5 * 7 + 2 * 5;",
        vec![ExpressionStatement {
            expression: Expression::Combination {
                operator: Operator(Token::Plus, Infix),
                left: Some(Box::new(Expression::Combination {
                    operator: Operator(Token::Multiply, Infix),
                    left: Some(Box::new(Expression::IntegerLiteral(5))),
                    right: Some(Box::new(Expression::IntegerLiteral(7)))
                })),
                right: Some(Box::new(Expression::Combination {
                    operator: Operator(Token::Multiply, Infix),
                    left: Some(Box::new(Expression::IntegerLiteral(2))),
                    right: Some(Box::new(Expression::IntegerLiteral(5)))
                })),
            }
        }]
    );
}

#[test]
fn test_expression_with_exponent() {
    parse!(
        "- 7 ^ 2;",
        vec![ExpressionStatement {
            expression: Expression::Combination {
                left: None,
                operator: Operator(Token::Minus, Prefix),
                right: Some(Box::new(Expression::Combination {
                    left: Some(Box::new(Expression::IntegerLiteral(7))),
                    operator: Operator(Token::Exponent, Infix),
                    right: Some(Box::new(Expression::IntegerLiteral(2)))
                }))
            }
        }]
    );
}

#[test]
fn test_expression_with_negative_exponent() {
    parse!(
        "-2 ^ -2;",
        vec![ExpressionStatement {
            expression: Expression::Combination {
                left: None,
                operator: Operator(Token::Minus, Prefix),
                right: Some(Box::new(Expression::Combination {
                    left: Some(Box::new(Expression::IntegerLiteral(2))),
                    operator: Operator(Token::Exponent, Infix),
                    right: Some(Box::new(Expression::Combination {
                        left: None,
                        operator: Operator(Token::Minus, Prefix),
                        right: Some(Box::new(Expression::IntegerLiteral(2)))
                    }))
                })),
            }
        }]
    );
}

#[test]
fn test_expression_statement_with_semicolon() {
    parse!(
        "- 7 ^ 2;",
        vec![ExpressionStatement {
            expression: Expression::Combination {
                left: None,
                operator: Operator(Token::Minus, Prefix),
                right: Some(Box::new(Expression::Combination {
                    left: Some(Box::new(Expression::IntegerLiteral(7))),
                    operator: Operator(Token::Exponent, Infix),
                    right: Some(Box::new(Expression::IntegerLiteral(2)))
                }))
            }
        }]
    );
}

#[test]
fn test_not_as_precedence() {
    parse!(
        "not true as N;",
        // (not true) as N
        vec![ExpressionStatement {
            expression: Expression::Combination {
                operator: Operator(Token::As, Infix),
                left: Some(Box::new(Expression::Combination {
                    left: None,
                    operator: Operator(Token::Not, Prefix),
                    right: Some(Box::new(Expression::BooleanLiteral(true)))
                })),
                right: Some(Box::new(Expression::Identifier("N".into())))
            }
        }]
    );
}

#[test]
fn test_negative_subtraction() {
    parse!(
        "- 2 + 2;",
        vec![ExpressionStatement {
            expression: Expression::Combination {
                left: Some(Box::new(Expression::Combination {
                    left: None,
                    operator: Operator(Token::Minus, Prefix),
                    right: Some(Box::new(Expression::IntegerLiteral(2)))
                })),
                operator: Operator(Token::Plus, Infix),
                right: Some(Box::new(Expression::IntegerLiteral(2)))
            }
        }]
    );
}

#[test]
fn test_grouped_expression() {
    parse!(
        "(2 - 2) / (2 + 2);",
        vec![ExpressionStatement {
            expression: Expression::Combination {
                left: Some(Box::new(Expression::Combination {
                    left: Some(Box::new(Expression::IntegerLiteral(2))),
                    operator: Operator(Token::Minus, Infix),
                    right: Some(Box::new(Expression::IntegerLiteral(2)))
                })),
                operator: Operator(Token::Division, Infix),
                right: Some(Box::new(Expression::Combination {
                    left: Some(Box::new(Expression::IntegerLiteral(2))),
                    operator: Operator(Token::Plus, Infix),
                    right: Some(Box::new(Expression::IntegerLiteral(2)))
                }))
            }
        }]
    );
}

#[test]
fn test_nested_grouped_expression() {
    parse!(
        "((2 - 2) / (2 + 2)) ^ 2;",
        vec![ExpressionStatement {
            expression: Expression::Combination {
                left: Some(Box::new(Expression::Combination {
                    left: Some(Box::new(Expression::Combination {
                        left: Some(Box::new(Expression::IntegerLiteral(2))),
                        operator: Operator(Token::Minus, Infix),
                        right: Some(Box::new(Expression::IntegerLiteral(2)))
                    })),
                    operator: Operator(Token::Division, Infix),
                    right: Some(Box::new(Expression::Combination {
                        left: Some(Box::new(Expression::IntegerLiteral(2))),
                        operator: Operator(Token::Plus, Infix),
                        right: Some(Box::new(Expression::IntegerLiteral(2)))
                    }))
                })),
                operator: Operator(Token::Exponent, Infix),
                right: Some(Box::new(Expression::IntegerLiteral(2)))
            }
        }]
    );
}

#[test]
fn test_call_expression() {
    parse!(
        "foo(2 + 2);",
        vec![ExpressionStatement {
            expression: Expression::Call {
                function: Box::new(Expression::Identifier("foo".into())),
                arguments: vec![Expression::Combination {
                    left: Some(Box::new(Expression::IntegerLiteral(2))),
                    operator: Operator(Token::Plus, Infix),
                    right: Some(Box::new(Expression::IntegerLiteral(2)))
                }]
            }
        }]
    );
}

#[test]
fn test_call_expression_with_trailing_comma() {
    parse!(
        "foo(2 + 2, );",
        vec![ExpressionStatement {
            expression: Expression::Call {
                function: Box::new(Expression::Identifier("foo".into())),
                arguments: vec![Expression::Combination {
                    left: Some(Box::new(Expression::IntegerLiteral(2))),
                    operator: Operator(Token::Plus, Infix),
                    right: Some(Box::new(Expression::IntegerLiteral(2)))
                }]
            }
        }]
    );
}

#[test]
fn test_empty_call_expression() {
    parse!(
        "foo();",
        vec![ExpressionStatement {
            expression: Expression::Call {
                function: Box::new(Expression::Identifier("foo".into())),
                arguments: vec![]
            }
        }]
    );
}

#[test]
fn test_call_expression_with_multiple_params() {
    parse!(
        "foo(2, 2);",
        vec![ExpressionStatement {
            expression: Expression::Call {
                function: Box::new(Expression::Identifier("foo".into())),
                arguments: vec![
                    Expression::IntegerLiteral(2),
                    Expression::IntegerLiteral(2)
                ]
            }
        }]
    );
}

#[test]
fn test_call_expression_in_composite_expression() {
    parse!(
        "foo(2, 2) ? 42;",
        vec![ExpressionStatement {
            expression: Expression::Combination {
                left: Some(Box::new(Expression::Call {
                    function: Box::new(Expression::Identifier("foo".into())),
                    arguments: vec![
                        Expression::IntegerLiteral(2),
                        Expression::IntegerLiteral(2)
                    ]
                })),
                operator: Operator(Token::Question, Infix),
                right: Some(Box::new(Expression::IntegerLiteral(42)))
            }
        }]
    );
}

#[test]
fn test_call_expression_in_prefix_expression() {
    parse!(
        "- foo(2, 2);",
        vec![ExpressionStatement {
            expression: Expression::Combination {
                left: None,
                operator: Operator(Token::Minus, Prefix),
                right: Some(Box::new(Expression::Call {
                    function: Box::new(Expression::Identifier("foo".into())),
                    arguments: vec![
                        Expression::IntegerLiteral(2),
                        Expression::IntegerLiteral(2)
                    ]
                })),
            }
        }]
    );
}

#[test]
fn test_piecewise_block_expression() {
    parse!(
        "
        {
            0 if x < 0;
            0.5 if x == 0;
            1 else;
        };
        ",
        vec![ExpressionStatement {
            expression: Expression::PiecewiseBlock(
                vec![
                    (Expression::IntegerLiteral(0), Expression::Combination {
                        left: Some(Box::new(Expression::Identifier("x".into()))),
                        operator: Operator(Token::LessThan, Infix),
                        right: Some(Box::new(Expression::IntegerLiteral(0)))
                    }),
                    (Expression::RealLiteral(0.5), Expression::Combination {
                        left: Some(Box::new(Expression::Identifier("x".into()))),
                        operator: Operator(Token::Equals, Infix),
                        right: Some(Box::new(Expression::IntegerLiteral(0)))
                    }),
                    (Expression::IntegerLiteral(1), Expression::BooleanLiteral(true)),
                ]
            )
        }]
    )
}

#[test]
fn test_piecewise_block_in_assignment_statement() {
    parse!(
        "\
        let x = 2;
        let heavisideX = {
            0 if x < 0;
            0.5 if x == 0;
            1 else;
        };
        ",
        vec![
            Assignment {
                identifier: "x".into(),
                expression: Expression::IntegerLiteral(2)
            },
            Assignment {
                identifier: "heavisideX".into(),
                expression: Expression::PiecewiseBlock(
                    vec![
                        (Expression::IntegerLiteral(0), Expression::Combination {
                            left: Some(Box::new(Expression::Identifier("x".into()))),
                            operator: Operator(Token::LessThan, Infix),
                            right: Some(Box::new(Expression::IntegerLiteral(0)))
                        }),
                        (Expression::RealLiteral(0.5), Expression::Combination {
                            left: Some(Box::new(Expression::Identifier("x".into()))),
                            operator: Operator(Token::Equals, Infix),
                            right: Some(Box::new(Expression::IntegerLiteral(0)))
                        }),
                        (Expression::IntegerLiteral(1), Expression::BooleanLiteral(true)),
                    ]
                )
            }
        ]
    );
}

#[test]
fn test_absolute_value() {
    parse!(
        "|-2 + -4|;",
        vec![
            ExpressionStatement {
                expression: Expression::AbsoluteValue(Box::new(Expression::Combination {
                    left: Some(Box::new(Expression::Combination {
                            left: None,
                            operator: Operator(Token::Minus, Prefix),
                            right: Some(Box::new(Expression::IntegerLiteral(2)))
                    })),
                    operator: Operator(Token::Plus, Infix),
                    right: Some(Box::new(Expression::Combination {
                        left: None,
                        operator: Operator(Token::Minus, Prefix),
                        right: Some(Box::new(Expression::IntegerLiteral(4)))
                    }))
                }))
            }
        ]
    )
}

#[test]
fn test_nested_abs_val() {
    parse!(
        "|-2 + |-4||;",
        vec![
            ExpressionStatement {
                expression: Expression::AbsoluteValue(Box::new(Expression::Combination {
                    left: Some(Box::new(Expression::Combination {
                            left: None,
                            operator: Operator(Token::Minus, Prefix),
                            right: Some(Box::new(Expression::IntegerLiteral(2)))
                    })),
                    operator: Operator(Token::Plus, Infix),
                    right: Some(Box::new(Expression::AbsoluteValue(Box::new(
                        Expression::Combination {
                            left: None,
                            operator: Operator(Token::Minus, Prefix),
                            right: Some(Box::new(Expression::IntegerLiteral(4)))
                        }))
                    ))
                }))
            }
        ]
    )
}

#[test]
fn test_set_literal_expression() {
    parse!(
        "{ 1, 2, 3 };",
        vec![
            ExpressionStatement {
                expression: Expression::SetLiteral(vec![
                    Expression::IntegerLiteral(1),
                    Expression::IntegerLiteral(2),
                    Expression::IntegerLiteral(3),
                ])
            }
        ]
    )
}

#[test]
fn test_set_literal_expression_with_trailing_comma() {
    parse!(
        "{ 1, 2, 3, };",
        vec![
            ExpressionStatement {
                expression: Expression::SetLiteral(vec![
                    Expression::IntegerLiteral(1),
                    Expression::IntegerLiteral(2),
                    Expression::IntegerLiteral(3),
                ])
            }
        ]
    )
}

#[test]
fn test_empty_set_literal() {
    parse!(
        "{ };",
        vec![
            ExpressionStatement {
                expression: Expression::SetLiteral(vec![])
            }
        ]
    )
}

#[test]
fn test_one_element_set_literal() {
    parse!(
        "{ 1 };",
        vec![
            ExpressionStatement {
                expression: Expression::SetLiteral(vec![
                    Expression::IntegerLiteral(1)
                ])
            }
        ]
    )
}

#[test]
fn test_set_literal_with_reals() {
    parse!(
        "{ 1.0, 2.0, 3.0 };",
        vec![
            ExpressionStatement {
                expression: Expression::SetLiteral(vec![
                    Expression::RealLiteral(1.0),
                    Expression::RealLiteral(2.0),
                    Expression::RealLiteral(3.0),
                ])
            }
        ]
    )
}

#[test]
fn test_set_union() {
    parse!(
        r"{ 1.0, 2.0 } \/ { 3.0, 4.0 };",
        vec![
            ExpressionStatement {
                expression: Expression::Combination {
                    left: Some(Box::new(Expression::SetLiteral(vec![
                        Expression::RealLiteral(1.0),
                        Expression::RealLiteral(2.0),
                    ]))),
                    operator: Operator(Token::Union, Infix),
                    right: Some(Box::new(Expression::SetLiteral(vec![
                        Expression::RealLiteral(3.0),
                        Expression::RealLiteral(4.0),
                    ]))),
                }
            }
        ]
    )
}

#[test]
fn test_set_difference() {
    parse!(
        r"{ 1.0, 2.0 } \ { 2.0, 3.0 };",
        vec![
            ExpressionStatement {
                expression: Expression::Combination {
                    left: Some(Box::new(Expression::SetLiteral(vec![
                        Expression::RealLiteral(1.0),
                        Expression::RealLiteral(2.0),
                    ]))),
                    operator: Operator(Token::SetDifference, Infix),
                    right: Some(Box::new(Expression::SetLiteral(vec![
                        Expression::RealLiteral(2.0),
                        Expression::RealLiteral(3.0),
                    ]))),
                }
            }
        ]
    )
}

#[test]
fn test_set_symmetric_difference() {
    parse!(
        r"{ 1.0, 2.0 } /_\ { 2.0, 3.0 };",
        vec![
            ExpressionStatement {
                expression: Expression::Combination {
                    left: Some(Box::new(Expression::SetLiteral(vec![
                        Expression::RealLiteral(1.0),
                        Expression::RealLiteral(2.0),
                    ]))),
                    operator: Operator(Token::SymmetricDifference, Infix),
                    right: Some(Box::new(Expression::SetLiteral(vec![
                        Expression::RealLiteral(2.0),
                        Expression::RealLiteral(3.0),
                    ]))),
                }
            }
        ]
    )
}

#[test]
fn test_set_intersection() {
    parse!(
        r"{ 1.0, 2.0 } /\ { 3.0, 4.0 };",
        vec![
            ExpressionStatement {
                expression: Expression::Combination {
                    left: Some(Box::new(Expression::SetLiteral(vec![
                        Expression::RealLiteral(1.0),
                        Expression::RealLiteral(2.0),
                    ]))),
                    operator: Operator(Token::Intersection, Infix),
                    right: Some(Box::new(Expression::SetLiteral(vec![
                        Expression::RealLiteral(3.0),
                        Expression::RealLiteral(4.0),
                    ]))),
                }
            }
        ]
    )
}

#[test]
fn test_set_subset() {
    parse!(
        r"{ 1.0, 2.0 } < { 3.0, 4.0 };",
        vec![
            ExpressionStatement {
                expression: Expression::Combination {
                    left: Some(Box::new(Expression::SetLiteral(vec![
                        Expression::RealLiteral(1.0),
                        Expression::RealLiteral(2.0),
                    ]))),
                    operator: Operator(Token::LessThan, Infix),
                    right: Some(Box::new(Expression::SetLiteral(vec![
                        Expression::RealLiteral(3.0),
                        Expression::RealLiteral(4.0),
                    ]))),
                }
            }
        ]
    )
}

#[test]
fn test_set_proper_subset() {
    parse!(
        r"{ 1.0, 2.0 } <= { 3.0, 4.0 };",
        vec![
            ExpressionStatement {
                expression: Expression::Combination {
                    left: Some(Box::new(Expression::SetLiteral(vec![
                        Expression::RealLiteral(1.0),
                        Expression::RealLiteral(2.0),
                    ]))),
                    operator: Operator(Token::LessThanEquals, Infix),
                    right: Some(Box::new(Expression::SetLiteral(vec![
                        Expression::RealLiteral(3.0),
                        Expression::RealLiteral(4.0),
                    ]))),
                }
            }
        ]
    )
}

bad_parsing!(test_equality_in_assignment, "let value == 123;");

bad_parsing!(test_missing_semicolon_in_assignment, "let value = 123");

bad_parsing!(test_missing_let_in_assignment, "v2 = 456;");

bad_parsing!(test_missing_semicolon_in_expression_statement, "456");

bad_parsing!(test_bad_function_declaration, "fn foobar(a, b=2) = a * b;");

bad_parsing!(
    test_function_declaration_without_identifier,
    "fn (a, b) = a * b;"
);

bad_parsing!(
    test_function_declaration_without_assisgnment,
    "fn times(a, b);"
);

bad_parsing!(test_function_declaration_without_body, "fn times(a, b) = ;");

bad_parsing!(test_unmatched_left_paren, "(2 + 2;");

bad_parsing!(test_unmatched_right_paren, "2 + 2);");

bad_parsing!(test_bad_infixed_token, "2 not 2;");

bad_parsing!(test_bad_consecutive_tokens, "2 2;");

bad_parsing!(test_bad_consecutive_tokens_in_call_expression, "foo(2 2, 1 1);");

bad_parsing!(test_semicolon_in_grouped_statement, "(2 - 2;);");

bad_parsing!(test_piecewise_block_expression_wo_semicolons, "{ x if x < 0 };");

bad_parsing!(test_piecewise_block_expression_wo_semicolons_2, "{ x else };");

bad_parsing!(test_piecewise_block_expression_with_two_else_arms, "{ x else; y else; };");

bad_parsing!(test_if_outside_piecewise_block, "let x = 2 if true else 3;");

bad_parsing!(test_else_outside_piecewise_block, "undefined else 2;");

bad_parsing!(test_abs_val_never_closed, "|2 - 7;");