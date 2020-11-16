#[macro_use]
extern crate log;
extern crate pretty_env_logger;

use structopt::StructOpt;

#[cfg(test)]
#[macro_use]
mod tests;

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
