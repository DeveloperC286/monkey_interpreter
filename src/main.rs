extern crate pretty_env_logger;

#[macro_use]
extern crate log;

#[macro_use]
extern crate lazy_static;

mod interpreter;
mod lexical_analysis;
mod syntax_analysis;
mod utilities;

use structopt::StructOpt;

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
