#[macro_use]
extern crate log;
extern crate pretty_env_logger;

use std::io::{stdin, stdout, Write};

use anyhow::{Context, Result};
use clap::Parser;

use crate::cli::Arguments;
use crate::evaluator::Evaluator;
use crate::lexical_analysis::LexicalAnalysis;
use crate::syntax_analysis::SyntaxAnalysis;

mod cli;
mod evaluator;
mod lexical_analysis;
mod syntax_analysis;

fn main() {
    info!("Version {}.", env!("CARGO_PKG_VERSION"));
    let arguments = Arguments::parse();
    debug!("The command line arguments provided are {arguments:?}.");

    // Set up logging: if verbose is true and RUST_LOG is not set, default to info level
    if arguments.verbose && std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }

    pretty_env_logger::init();
    let mut evaluator = crate::evaluator::Evaluator::new();

    loop {
        if let Err(error) = repl(&mut evaluator) {
            error!("{error}");
        }
    }
}
fn repl(evaluator: &mut Evaluator) -> Result<()> {
    print!(" >>> ");
    let input = read()?;
    let tokens = LexicalAnalysis::from(&input)?;
    let abstract_syntax_tree = SyntaxAnalysis::from(tokens)?;
    let object = evaluator.evaluate(abstract_syntax_tree)?;
    println!("{object:?}");
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
