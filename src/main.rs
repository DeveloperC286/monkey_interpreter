extern crate pretty_env_logger;

#[macro_use]
extern crate log;

#[macro_use]
extern crate lazy_static;

mod lexer;
mod repl;
mod token;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "rust-monkey-interpreter",
    about = "Monkey interpreter written in Rust."
)]
struct Args {}

fn main() {
    pretty_env_logger::init();
    let args = Args::from_args();
    repl::repl();
}
