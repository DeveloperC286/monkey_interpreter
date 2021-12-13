// Disabling because of rstest_reuse.
#![allow(clippy::single_component_path_imports)]

#[macro_use]
extern crate log;
extern crate pretty_env_logger;

use structopt::StructOpt;

mod cli;
mod evaluator;
mod interpreter;
mod lexical_analysis;
mod syntax_analysis;

fn main() {
    pretty_env_logger::init();
    let _arguments = cli::Arguments::from_args();
    interpreter::repl();
}

#[cfg(test)]
use rstest_reuse;

#[cfg(test)]
mod tests;
