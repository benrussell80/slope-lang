use crate::ast::base::{Expression, Parameter, Parser, Statement::*, Operation, Location::*};
use crate::interpreter::base::{LexerIterator, Token};

macro_rules! parse {
    ($text:expr, $statements:expr) => {
        let text = $text;
        let parser = Parser::new(LexerIterator::new(text.chars().peekable()));
        assert_eq!(parser.parse_program().unwrap(), $statements);
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
                operator: Operation(Token::Minus, Prefix),
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
                operator: Operation(Token::Multiply, Infix),
                right: Some(Box::new(Expression::Combination {
                    left: Some(Box::new(Expression::Identifier("radius".into()))),
                    operator: Operation(Token::Exponent, Infix),
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
                operator: Operation(Token::Multiply, Infix),
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
                    operator: Operation(Token::Multiply, Infix),
                    right: Some(Box::new(Expression::Combination {
                        left: Some(Box::new(Expression::Identifier("radius".into()))),
                        operator: Operation(Token::Exponent, Infix),
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
                operator: Operation(Token::Not, Prefix),
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
                operator: Operation(Token::Not, Prefix),
                right: Some(Box::new(Expression::Combination {
                    left: None,
                    operator: Operation(Token::Not, Prefix),
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
                operator: Operation(Token::Minus, Prefix),
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
                operator: Operation(Token::Minus, Prefix),
                right: Some(Box::new(Expression::Combination {
                    left: None,
                    operator: Operation(Token::Minus, Prefix),
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
                operator: Operation(Token::Plus, Infix),
                left: Some(Box::new(Expression::IntegerLiteral(5))),
                right: Some(Box::new(Expression::Combination {
                    operator: Operation(Token::Multiply, Infix),
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
                operator: Operation(Token::Plus, Infix),
                left: Some(Box::new(Expression::Combination {
                    operator: Operation(Token::Multiply, Infix),
                    left: Some(Box::new(Expression::IntegerLiteral(5))),
                    right: Some(Box::new(Expression::IntegerLiteral(7)))
                })),
                right: Some(Box::new(Expression::Combination {
                    operator: Operation(Token::Multiply, Infix),
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
                operator: Operation(Token::Minus, Prefix),
                right: Some(Box::new(Expression::Combination {
                    left: Some(Box::new(Expression::IntegerLiteral(7))),
                    operator: Operation(Token::Exponent, Infix),
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
                operator: Operation(Token::Minus, Prefix),
                right: Some(Box::new(Expression::Combination {
                    left: Some(Box::new(Expression::IntegerLiteral(2))),
                    operator: Operation(Token::Exponent, Infix),
                    right: Some(Box::new(Expression::Combination {
                        left: None,
                        operator: Operation(Token::Minus, Prefix),
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
                operator: Operation(Token::Minus, Prefix),
                right: Some(Box::new(Expression::Combination {
                    left: Some(Box::new(Expression::IntegerLiteral(7))),
                    operator: Operation(Token::Exponent, Infix),
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
                operator: Operation(Token::As, Infix),
                left: Some(Box::new(Expression::Combination {
                    left: None,
                    operator: Operation(Token::Not, Prefix),
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
                    operator: Operation(Token::Minus, Prefix),
                    right: Some(Box::new(Expression::IntegerLiteral(2)))
                })),
                operator: Operation(Token::Plus, Infix),
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
                    operator: Operation(Token::Minus, Infix),
                    right: Some(Box::new(Expression::IntegerLiteral(2)))
                })),
                operator: Operation(Token::Division, Infix),
                right: Some(Box::new(Expression::Combination {
                    left: Some(Box::new(Expression::IntegerLiteral(2))),
                    operator: Operation(Token::Plus, Infix),
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
                        operator: Operation(Token::Minus, Infix),
                        right: Some(Box::new(Expression::IntegerLiteral(2)))
                    })),
                    operator: Operation(Token::Division, Infix),
                    right: Some(Box::new(Expression::Combination {
                        left: Some(Box::new(Expression::IntegerLiteral(2))),
                        operator: Operation(Token::Plus, Infix),
                        right: Some(Box::new(Expression::IntegerLiteral(2)))
                    }))
                })),
                operator: Operation(Token::Exponent, Infix),
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
                    operator: Operation(Token::Plus, Infix),
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
                operator: Operation(Token::Question, Infix),
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
                operator: Operation(Token::Minus, Prefix),
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

// write tests for statements that should fail
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
