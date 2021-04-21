use std::error::Error;
use std::fmt::{Display, Formatter, self};
use argh::FromArgs;
use std::fs;

pub mod interpreter;
pub mod repl;
pub mod ast;

use ast::environment::Environment;
use repl::exec;


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

fn main() -> Result<(), Box<dyn Error>> {
    // setup
    let mut config: Config = argh::from_env();

    // setup environment (memory, etc.)
    let mut env = Environment::new();

    // read
    if let Some(path) = config.file {
        // execute file using path
        let content = fs::read_to_string(path)?;
        println!(
            "{}",
            match exec(&content, &mut env) {
                Ok(s) => s,
                Err(e) => format!("{}", e)
            }
        );
    } else {
        config.repl = true;
    }

    if config.repl {
        let mut text = String::new();
        let mut prompt = Prompt::Start;

        loop {
            let input = repl::prompt(&format!("{}", &prompt))?;
            text.push_str(&input);

            let open_braces = text.chars().filter(|c| c == &'{' || c == &'}').fold(0, |open_braces, c| {
                if open_braces == 0 && c == '}' {
                    -1
                } else if open_braces < 0 {
                    open_braces
                } else if c == '{' {
                    open_braces + 1
                } else {
                    open_braces - 1
                }
            });
            
            if open_braces < 0 {
                println!("Mismatched braces.");
                text.clear();
                prompt = Prompt::Start;
                continue
            };
            
            if !text.trim_end().ends_with(";") || open_braces != 0 {
                prompt = Prompt::Continue;
                continue
            };

            println!(
                "{}",
                match exec(&text, &mut env) {
                    Ok(s) => s,
                    Err(e) => format!("{}", e)
                }
            );
            text = String::new();
            prompt = Prompt::Start;
        }
    }
    Ok(())
}
