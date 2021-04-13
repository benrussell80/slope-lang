use std::fmt::Debug;
use std::iter::{Peekable};
use std::string::ToString;
use std::str::Chars;
use itertools::Itertools;
use crate::ast::base::Operation;
use std::fmt::{Display, Formatter, self};


#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Illegal(String),
    Eof,
    Identifier(String),
    Real(f64),
    Integer(i64),
    Plus,
    Minus,
    Exponent,
    Division,
    Multiply,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftSquareBracket,
    RightSquareBracket,
    Comma,
    Semicolon,
    And,
    Or,
    Xor,
    True,
    False,
    Colon,
    If,
    Else,
    Modulo,
    Undefined,
    Assign,
    Equals,
    NotEquals,
    LessThan,
    LessThanEquals,
    GreaterThan,
    GreaterThanEquals,
    Not,
    Let,
    FuncFn,
    PlusMinus,
    MinusPlus,
    As,
    Question,
    In,
    Bang,
    Imaginary,
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use Token::*;
        match self {
            Illegal(value) => write!(f, "{}", value),
            Eof => write!(f, "EOF"),
            Identifier(value) => write!(f, "{}", value),
            Real(value) => write!(f, "{}", value),
            Integer(value) => write!(f, "{}", value),
            Plus => write!(f, "+"),
            Minus => write!(f, "-"),
            Exponent => write!(f, "^"),
            Division => write!(f, "/"),
            Multiply => write!(f, "*"),
            LeftParen => write!(f, "("),
            RightParen => write!(f, ")"),
            LeftBrace => write!(f, "{{"),
            RightBrace => write!(f, "}}"),
            LeftSquareBracket => write!(f, "["),
            RightSquareBracket => write!(f, "]"),
            Comma => write!(f, ","),
            Semicolon => write!(f, ";"),
            And => write!(f, "and"),
            Or => write!(f, "or"),
            Xor => write!(f, "xor"),
            True => write!(f, "true"),
            False => write!(f, "false"),
            Colon => write!(f, ":"),
            If => write!(f, "if"),
            Else => write!(f, "else"),
            Modulo => write!(f, "%"),
            Undefined => write!(f, "undefined"),
            Assign => write!(f, "="),
            Equals => write!(f, "=="),
            NotEquals => write!(f, "=/="),
            LessThan => write!(f, "<"),
            LessThanEquals => write!(f, "<="),
            GreaterThan => write!(f, ">"),
            GreaterThanEquals => write!(f, ">="),
            Not => write!(f, "not"),
            Let => write!(f, "let"),
            FuncFn => write!(f, "fn"),
            PlusMinus => write!(f, "+/-"),
            MinusPlus => write!(f, "-/+"),
            As => write!(f, "as"),
            Question => write!(f, "?"),
            In => write!(f, "in"),
            Bang => write!(f, "!"),
            Imaginary => write!(f, "i"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Identifier(String),
    IntegerLiteral(i64),
    BooleanLiteral(bool),
    RealLiteral(f64),
    UndefinedLiteral,
    Combination {
        left: Option<Box<Expression>>,
        operator: Operation,
        right: Option<Box<Expression>>
    },
    Call {
        function: Box<Expression>,
        arguments: Vec<Expression>
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use Expression::*;
        match self {
            Identifier(name) => write!(f, "{}", name),
            IntegerLiteral(value) => write!(f, "{}", value),
            BooleanLiteral(value) => write!(f, "{}", value),
            RealLiteral(value) => write!(f, "{}", value),
            UndefinedLiteral => write!(f, "undefined"),
            Combination { left, operator, right } => {
                if let Some(value) = left {
                    write!(f, "{} {} {}", left.as_ref().unwrap(), operator, right.as_ref().unwrap())
                } else {
                    write!(f, "{} {}", operator, right.as_ref().unwrap())
                }
            },
            Call { function, arguments } => {
                write!(f, "{}({})", function, arguments.iter().fold(String::new(), |mut acc, arg| {
                    if !acc.is_empty() {
                        acc.push_str(", ");
                    };
                    acc.push_str(&arg.to_string());
                    acc
                }))
            },
        }
    }
}

// impl From<Token> for f64 {
//     fn from(token: Token) -> Self {
//         if let Token::Real(inner) = token {
//             inner
//         } else {
//             panic!("Cannot convert {:?} to f64.", token);
//         }
//     }
// }

// impl From<Token> for i64 {
//     fn from(token: Token) -> Self {
//         if let Token::Integer(inner) = token {
//             inner
//         } else {
//             panic!("Cannot convert {:?} to i64.", token);
//         }
//     }
// }

// impl From<Token> for u64 {
//     fn from(token: Token) -> Self {
//         if let Token::Natural(inner) = token {
//             inner
//         } else {
//             panic!("Cannot convert {:?} to u64.", token);
//         }
//     }
// }

#[derive(Debug)]
pub struct Lexer<'a> {
    input: &'a str
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input
        }
    }
}

// fix this trait implementation
impl<'a> IntoIterator for Lexer<'a> {
    type Item = Token;
    type IntoIter = LexerIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        LexerIterator {
            iterator: self.input.chars().peekable(),
            done: false
        }
    }
}

#[derive(Debug)]
pub struct LexerIterator<'b> {
    iterator: Peekable<Chars<'b>>,
    done: bool
}

impl<'b> LexerIterator<'b> {
    pub fn new(iterator: Peekable<Chars<'b>>) -> Self {
        Self {
            iterator,
            done: false
        }
    }
}

impl Iterator for LexerIterator<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        use Token::*;

        // eat white space
        self.iterator.take_while_ref(|ch| ch.is_whitespace()).for_each(drop);

        // take next one
        if let Some(ch) = self.iterator.next() {
            let next_ch = self.iterator.peek();
            let next_token = match (ch, next_ch) {
                ('=', Some('=')) => {
                    self.iterator.next();
                    Equals
                },
                ('=', Some('/')) => {
                    self.iterator.next();
                    let ch = self.iterator.next();
                    match ch {
                        Some('=') => NotEquals,
                        Some(ch) => Illegal(format!("=/{}", ch)),
                        None => Illegal("=/".into())
                    }
                },
                ('<', Some('=')) => {
                    self.iterator.next();
                    LessThanEquals
                },
                ('>', Some('=')) => {
                    self.iterator.next();
                    GreaterThanEquals
                },
                ('+', Some('/')) => {
                    self.iterator.next();
                    let ch = self.iterator.next();
                    match ch {
                        Some('-') => PlusMinus,
                        Some(ch) => Illegal(format!("+/{}", ch)),
                        None => Illegal("+/".into())
                    }
                },
                ('-', Some('/')) => {
                    self.iterator.next();
                    let ch = self.iterator.next();
                    match ch {
                        Some('+') => MinusPlus,
                        Some(ch) => Illegal(format!("-/{}", ch)),
                        None => Illegal("-/".into())
                    }
                },
                ('?', _) => Question,
                ('<', _) => LessThan,
                ('>', _) => GreaterThan,
                ('=', _) => Assign,
                ('+', _) => Plus,
                ('-', _) => Minus,
                ('*', _) => Multiply,
                ('/', _) => Division,
                ('^', _) => Exponent,
                ('{', _) => LeftBrace,
                ('}', _) => RightBrace,
                ('(', _) => LeftParen,
                (')', _) => RightParen,
                ('[', _) => LeftSquareBracket,
                (']', _) => RightSquareBracket,
                (',', _) => Comma,
                (':', _) => Colon,
                (';', _) => Semicolon,
                ('%', _) => Modulo,
                ('!', _) => Bang,
                // ('i', _) => Imaginary,  // check for Identifier(...) | Real(...) | Integer(...) then Identifier("i")
                (_, _) => {
                    if is_identifier(&ch) {
                        let mut identifier = self.iterator.take_while_ref(|ch| !ch.is_whitespace() && is_identifier(ch))
                            .collect::<String>();

                        identifier.insert(0, ch);
                        
                        match &*identifier {
                            "and" => And,
                            "or" => Or,
                            "xor" => Xor,
                            "true" => True,
                            "false" => False,
                            "undefined" => Undefined,
                            "if" => If,
                            "else" => Else,
                            "let" => Let,
                            "fn" => FuncFn,
                            "not" => Not,
                            "as" => As,
                            "in" => In,
                            _ => Identifier(identifier)
                        }
                    } else if ch.is_numeric() || ch == '.' {
                        let mut number_string = self.iterator.take_while_ref(|ch| ch.is_numeric() || *ch == '.')
                            .collect::<String>();

                        number_string.insert(0, ch);
                        
                        if let Ok(number) = number_string.parse::<i64>() {
                            Integer(number)
                        } else if let Ok(number) = number_string.parse::<f64>() {
                            Real(number)
                        } else {
                            Illegal(ch.to_string())
                        }
                    } else {
                        Illegal(ch.to_string())
                    }
                }
            };
            Some(next_token)
        } else {
            if !self.done {
                self.done = !self.done;
                Some(Eof)
            } else {
                None
            }
        }
    }
}

fn is_identifier(ch: &char) -> bool {
    'a' <= *ch && *ch <= 'z' || 'A' <= *ch && *ch <= 'Z' || *ch == '_'
}

#[cfg(test)]
mod tests {
    use super::{Lexer, Token::*, Token};

    macro_rules! lex {
        ($name:ident, $text:expr, $tokens:expr) => {
            #[test]
            fn $name() {
                let s = $text;
                let lexer = Lexer::new(s);
                assert_eq!(
                    lexer.into_iter().collect::<Vec<Token>>(),
                    $tokens
                );
            }
        };
    }

    lex!(
        two_plus_two,
        "four = 2 + 2",
        vec![Identifier("four".into()), Assign, Integer(2), Plus, Integer(2), Eof]
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
            Identifier("pi".into()), Assign, Real(3.14),
            Identifier("radius".into()), Assign, Integer(10),
            Identifier("area".into()), Assign, Identifier("pi".into()), Multiply, Identifier("radius".into()), Exponent, Integer(2),
            Identifier("area".into()), Equals, Real(314.0),
            Eof
        ]
    );

    lex!(
        random,
        "/*-+=/=^[]{}();:.,3.0pi%2-1+/-/+ -/+",
        vec![
            Division, Multiply, Minus, Plus, NotEquals, Exponent,
            LeftSquareBracket, RightSquareBracket, LeftBrace, RightBrace, LeftParen, RightParen, Semicolon, Colon,
            Illegal(".".into()), Comma, Real(3.0), Identifier("pi".into()), Modulo, Integer(2), Minus, Integer(1),
            PlusMinus, Division, Plus, MinusPlus, Eof
        ]
    );

    lex!(
        function,
        "fn circleArea(radius) = radius ^ 2 * pi;",
        vec![
            FuncFn, Identifier("circleArea".into()), LeftParen, Identifier("radius".into()), RightParen,
            Assign, Identifier("radius".into()), Exponent, Integer(2), Multiply, Identifier("pi".into()), Semicolon, Eof
        ]
    );

    lex!(
        negative_integer,
        "let negOne = -1;",
        vec![
            Let, Identifier("negOne".into()), Assign, Minus, Integer(1), Semicolon, Eof
        ]
    );

    lex!(
        lex_fuction_declaration,
        "fn area(radius) = pi * radius ^ 2;",
        vec![
            FuncFn, Identifier("area".into()), LeftParen, Identifier("radius".into()), RightParen,
            Assign, Identifier("pi".into()), Multiply, Identifier("radius".into()), Exponent, Integer(2),
            Semicolon, Eof
        ]
    );
}
