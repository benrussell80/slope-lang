use std::io;
use std::io::prelude::*;
use std::io::Error as IOError;
use super::ast::environment::Environment;
use super::ast::parser::Parser;
use std::error::Error;
use super::interpreter::lexer::LexerIterator;
use super::ast::statement::Statement;
use super::ast::object::Object;

pub fn prompt(before: &str) -> Result<String, IOError> {
    print!("{}", before);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input)
}


pub fn exec(content: &str, env: &mut Environment) -> Result<String, Box<dyn Error>> {
    let lexer = LexerIterator::new(content.chars().peekable());
    let parser = Parser::new(lexer);
    match parser.parse_program() {
        Ok(stmts) => {
            let mut response = String::new();
            let mut count = 0;
            for stmt in stmts.iter() {
                let obj = env.eval_statement(&stmt)?;
                match (obj, stmt) {
                    (Object::Undefined, Statement::Assignment { .. }) | (Object::Undefined, Statement::FunctionDeclaration { .. }) => {},
                    (o, _) => {
                        if count != 0 {
                            response.push_str("\n");
                        };
                        response.push_str(&format!("{}", o));
                        count += 1;
                    }
                };
            }
            Ok(String::from(response))
        },
        Err(error) => Err(error.into())
    }
}

#[macro_export]
macro_rules! run {
    ($code:expr) => {
        use slope::repl::exec;
        use slope::ast::environment::Environment;
        println!("{}", exec($code, &mut Environment::new()).unwrap());
    };
}