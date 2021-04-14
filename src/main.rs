use std::error::Error;
use std::fmt::{Display, Formatter, self};
use argh::FromArgs;
use std::fs;

pub mod interpreter;
pub mod repl;
pub mod ast;

use ast::environment::Environment;


#[derive(FromArgs, Debug)]
/// Configuration for running slope code.
struct Config {
    /// file to run (stdin if None or Some("-"))
    #[argh(positional)]
    file: Option<String>,

    /// run in a repl
    #[argh(switch, short = 'i')]
    repl: bool,
}

#[derive(Debug)]
enum Prompt {
    Start,
    Continue
}

impl Display for Prompt {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Start => write!(f, ">>> "),
            Self::Continue => write!(f, "... "),
        }
    }
}

fn exec(content: String, env: &mut Environment) -> String {
    let lexer = interpreter::lexer::LexerIterator::new(content.chars().peekable());
    let parser = ast::parser::Parser::new(lexer);
    match parser.parse_program() {
        Ok(stmts) => {
            let mut response = String::new();
            for stmt in stmts {
                let obj = env.eval_statement(&stmt);
                response.push_str(&format!("{}", obj));
                response.push_str("\n");
            }
            String::from(response.trim_end())
        },
        Err(error) => format!("Error: {}", error)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // setup
    let mut config: Config = argh::from_env();

    // setup environment (memory, etc.)
    let mut env = Environment::new();

    // read
    if let Some(path) = config.file {
        // execute file using path
        let content = fs::read_to_string(path)?;
        println!("{}", exec(content, &mut env));
    } else {
        config.repl = true;
    }

    if config.repl {
        let mut text = String::new();
        let mut prompt = Prompt::Start;

        loop {
            let input = repl::prompt(&format!("{}", &prompt))?;
            text.push_str(&input);
            
            if !text.trim_end().ends_with(";") {
                prompt = Prompt::Continue;
                continue
            };

            let response = exec(text, &mut env);

            println!("{}", response);
            text = String::new();
            prompt = Prompt::Start;
        }
    }
    Ok(())
}
