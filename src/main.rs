#[macro_use]
extern crate log;
extern crate pretty_env_logger;

use structopt::StructOpt;

#[cfg(test)]
#[macro_use]
mod tests;

mod evaluator;
mod interpreter;
mod lexical_analysis;
mod syntax_analysis;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "rust_monkey_interpreter",
    about = "Monkey interpreter written in Rust."
)]
struct Args {}

fn main() {
    pretty_env_logger::init();
    let _args = Args::from_args();
    interpreter::repl();
}
