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
            if let Err(error) = run_script(&mut evaluator, &script) {
                error!("{error:?}");
                std::process::exit(1);
            }
        }
        None => loop {
            if let Err(error) = repl(&mut evaluator) {
                error!("{error:?}");
            }
        },
    }
}

fn run_script(evaluator: &mut Evaluator, script: &std::path::Path) -> Result<()> {
    let input = std::fs::read_to_string(script)
        .with_context(|| format!("Unable to read script file {}.", script.display()))?;
    let tokens = LexicalAnalysis::from(&input)?;
    let abstract_syntax_tree = SyntaxAnalysis::from(tokens)?;
    let object = evaluator.evaluate(abstract_syntax_tree)?;
    println!("{object:?}");
    Ok(())
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
