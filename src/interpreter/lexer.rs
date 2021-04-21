use std::iter::Peekable;
use std::str::Chars;
use super::token::Token;
use itertools::Itertools;

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
                    match self.iterator.next() {
                        Some('=') => NotEquals,
                        Some(ch) => Illegal(format!("=/{}", ch)),
                        None => Illegal("=/".into())
                    }
                },
                ('=', Some('>')) => {
                    self.iterator.next();
                    FatArrow
                },
                ('-', Some('>')) => {
                    self.iterator.next();
                    SkinnyArrow
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
                    match self.iterator.next() {
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
                ('\\', Some('/')) => {
                    self.iterator.next();
                    Union
                },
                ('/', Some('\\')) => {
                    self.iterator.next();
                    Intersection
                },
                ('/', Some('_')) => {
                    // this may clash with something like a/_b
                    self.iterator.next();
                    match self.iterator.next() {
                        Some('\\') => SymmetricDifference,
                        Some(ch) => Illegal(format!("/_{}", ch)),
                        None => Illegal("/_".into())
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
                ('|', _) => Bar,
                // ('i', _) => Imaginary,  // check for Identifier(...) | Real(...) | Integer(...) then Identifier("i")
                (_, _) => {
                    if is_leading_identifier_char(&ch) {
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
                            "for" => For,
                            "where" => Where,
                            "import" => Import,
                            "use" => Use,
                            "export" => Export,
                            "pub" => Pub,
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

// beef up the identifier tokens to include numbers in reasonable places
fn is_identifier(ch: &char) -> bool {
    'a' <= *ch && *ch <= 'z' || 'A' <= *ch && *ch <= 'Z' || *ch == '_' || '0' <= *ch && *ch <= '9'
}

fn is_leading_identifier_char(ch: &char) -> bool {
    // identifiers cannot have leading numbers or underscores
    'a' <= *ch && *ch <= 'z' || 'A' <= *ch && *ch <= 'Z'
}
