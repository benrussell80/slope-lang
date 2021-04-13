use std::io;
use std::io::prelude::*;
use std::error::Error;
use crate::interpreter::base::Lexer;
use crate::ast::base::{Parser};


pub fn prompt(before: &str) -> Result<String, Box<dyn Error>> {
    print!("{}", before);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input)
}

pub struct Context;

pub fn eval(input: &str, _: Context) {
    let lexer = Lexer::new(input);
    let parser = Parser::new(lexer.into_iter());
}
