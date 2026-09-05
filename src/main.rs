use std::io::{Write, stdin, stdout};

use anyhow::{Context, Result};
use clap::Parser;
use log::{debug, error, info};

use crate::cli::Arguments;
use crate::evaluator::Evaluator;
use crate::lexical_analysis::LexicalAnalysis;
use crate::syntax_analysis::SyntaxAnalysis;

mod cli;
mod evaluator;
mod lexical_analysis;
mod syntax_analysis;

fn main() {
    let arguments = Arguments::parse();

    // Set up logging. Log level precedence:
    // - RUST_LOG, if set.
    // - info, if --verbose is passed.
    let mut logger = pretty_env_logger::formatted_builder();
    match std::env::var("RUST_LOG") {
        Ok(rust_log) => {
            logger.parse_filters(&rust_log);
        }
        Err(_) if arguments.verbose => {
            logger.filter_level(log::LevelFilter::Info);
        }
        Err(_) => {}
    }
    logger.init();

    info!("Version {}.", env!("CARGO_PKG_VERSION"));
    debug!("The command line arguments provided are {arguments:?}.");

    let mut evaluator = Evaluator::new();

    match arguments.script {
        Some(script) => {
            if let Err(error) = evaluate(&mut evaluator, || read_script(&script)) {
                error!("{error:?}");
                std::process::exit(1);
            }
        }
        None => loop {
            match evaluate(&mut evaluator, read_repl_line) {
                Ok(true) => {}
                Ok(false) => break,
                Err(error) => error!("{error:?}"),
            }
        },
    }
}

fn evaluate(
    evaluator: &mut Evaluator,
    read_input: impl FnOnce() -> Result<Option<String>>,
) -> Result<bool> {
    let input = match read_input()? {
        Some(input) => input,
        None => return Ok(false),
    };
    let tokens = LexicalAnalysis::from(&input)?;
    let abstract_syntax_tree = SyntaxAnalysis::from(tokens)?;
    let object = evaluator.evaluate(abstract_syntax_tree)?;
    println!("{object}");
    Ok(true)
}

fn read_script(script: &std::path::Path) -> Result<Option<String>> {
    std::fs::read_to_string(script)
        .with_context(|| format!("Unable to read script file {}.", script.display()))
        .map(Some)
}

fn read_repl_line() -> Result<Option<String>> {
    print!(" >>> ");
    let mut buffer = String::new();

    let _ = stdout().flush();
    let bytes_read = stdin()
        .read_line(&mut buffer)
        .context("Unable to read user input from standard input.")?;

    if bytes_read == 0 {
        println!();
        return Ok(None);
    }

    Ok(Some(buffer))
}

#[cfg(test)]
mod tests;
