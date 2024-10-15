#[macro_use]
extern crate log;
extern crate pretty_env_logger;

use std::io::{stdin, stdout, Write};

use anyhow::{Context, Result};

use crate::evaluator::Evaluator;
use crate::lexical_analysis::LexicalAnalysis;
use crate::syntax_analysis::SyntaxAnalysis;

mod evaluator;
mod lexical_analysis;
mod syntax_analysis;

fn main() {
    pretty_env_logger::init();
    trace!("Version {}.", env!("CARGO_PKG_VERSION"));
    let mut evaluator = crate::evaluator::Evaluator::new();

    loop {
        if let Err(error) = repl(&mut evaluator) {
            error!("{}", error);
        }
    }
}
fn repl(evaluator: &mut Evaluator) -> Result<()> {
    print!(" >>> ");
    let input = read()?;
    let tokens = LexicalAnalysis::from(&input)?;
    let abstract_syntax_tree = SyntaxAnalysis::from(tokens)?;
    let object = evaluator.evaluate(abstract_syntax_tree)?;
    println!("{:?}", object);
    Ok(())
}

fn read() -> Result<String> {
    let mut buffer = String::new();

    let _ = stdout().flush();
    stdin()
        .read_line(&mut buffer)
        .context("Unable to read user input from standard input.")?;

    Ok(buffer)
}

#[cfg(test)]
mod tests;
