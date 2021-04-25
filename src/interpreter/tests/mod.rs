use super::lexer::Lexer;
use super::token::Token::{self, *};

macro_rules! lex {
    ($name:ident, $text:expr, $tokens:expr) => {
        #[test]
        fn $name() {
            let s = $text;
            let lexer = Lexer::new(s);
            assert_eq!(lexer.into_iter().collect::<Vec<Token>>(), $tokens);
        }
    };
}

lex!(
    two_plus_two,
    "four = 2 + 2",
    vec![
        Identifier("four".into()),
        Assign,
        Integer(2),
        Plus,
        Integer(2),
        Eof
    ]
);

lex!(
    multiline_source_code,
    "
        pi = 3.14
        radius = 10
        area = pi * radius ^ 2
        area == 314.0
        ",
    vec![
        Identifier("pi".into()),
        Assign,
        Real(3.14),
        Identifier("radius".into()),
        Assign,
        Integer(10),
        Identifier("area".into()),
        Assign,
        Identifier("pi".into()),
        Multiply,
        Identifier("radius".into()),
        Exponent,
        Integer(2),
        Identifier("area".into()),
        Equals,
        Real(314.0),
        Eof
    ]
);

lex!(
    random,
    "/ *-+=/=^[]{}();:.,3.0pi%2-1+/-/+ -/+",
    vec![
        Division,
        Multiply,
        Minus,
        Plus,
        NotEquals,
        Exponent,
        LeftSquareBracket,
        RightSquareBracket,
        LeftBrace,
        RightBrace,
        LeftParen,
        RightParen,
        Semicolon,
        Colon,
        Illegal(".".into()),
        Comma,
        Real(3.0),
        Identifier("pi".into()),
        Modulo,
        Integer(2),
        Minus,
        Integer(1),
        PlusMinus,
        Division,
        Plus,
        MinusPlus,
        Eof
    ]
);

lex!(
    function,
    "fn circleArea(radius) = radius ^ 2 * pi;",
    vec![
        FuncFn,
        Identifier("circleArea".into()),
        LeftParen,
        Identifier("radius".into()),
        RightParen,
        Assign,
        Identifier("radius".into()),
        Exponent,
        Integer(2),
        Multiply,
        Identifier("pi".into()),
        Semicolon,
        Eof
    ]
);

lex!(
    negative_integer,
    "let negOne = -1;",
    vec![
        Let,
        Identifier("negOne".into()),
        Assign,
        Minus,
        Integer(1),
        Semicolon,
        Eof
    ]
);

lex!(
    lex_fuction_declaration,
    "fn area(radius) = pi * radius ^ 2;",
    vec![
        FuncFn,
        Identifier("area".into()),
        LeftParen,
        Identifier("radius".into()),
        RightParen,
        Assign,
        Identifier("pi".into()),
        Multiply,
        Identifier("radius".into()),
        Exponent,
        Integer(2),
        Semicolon,
        Eof
    ]
);

lex!(
    lex_abs_val,
    "|value|",
    vec![
        Bar,
        Identifier("value".into()),
        Bar,
        Eof
    ]
);

lex!(
    lex_set_union,
    r"A \/ B",
    vec![
        Identifier("A".into()),
        Union,
        Identifier("B".into()),
        Eof
    ]
);

lex!(
    lex_set_intersection,
    r"A /\ B",
    vec![
        Identifier("A".into()),
        Intersection,
        Identifier("B".into()),
        Eof
    ]
);

lex!(
    lex_set_symmetric_difference,
    r"A /_\ B",
    vec![
        Identifier("A".into()),
        SymmetricDifference,
        Identifier("B".into()),
        Eof
    ]
);

lex!(
    lex_set_difference,
    r"A \ B",
    vec![
        Identifier("A".into()),
        SetDifference,
        Identifier("B".into()),
        Eof
    ]
);

lex!(
    lex_less_than,
    r"A < B",
    vec![
        Identifier("A".into()),
        LessThan,
        Identifier("B".into()),
        Eof
    ]
);

lex!(
    lex_less_than_equals,
    r"A <= B",
    vec![
        Identifier("A".into()),
        LessThanEquals,
        Identifier("B".into()),
        Eof
    ]
);

lex!(
    lex_leading_underscore_in_identifier,
    "_foobar",
    vec![
        Illegal("_".into()),
        Identifier("foobar".into()),
        Eof
    ]
);

lex!(
    lex_set_literal,
    "{ 1, 2, 3 };",
    vec![
        LeftBrace,
        Integer(1),
        Comma,
        Integer(2),
        Comma,
        Integer(3),
        RightBrace,
        Semicolon,
        Eof
    ]
);

lex!(
    comments_should_be_ignored,
    r"A \ B  # hello there",
    vec![
        Identifier("A".into()),
        SetDifference,
        Identifier("B".into()),
        Eof
    ]
);

lex!(
    comments_breaking_an_expression_should_be_ignored,
    r"A + 
    # hello comment
    B",
    vec![
        Identifier("A".into()),
        Plus,
        Identifier("B".into()),
        Eof
    ]
);