#[macro_use]
extern crate log;
extern crate pretty_env_logger;

mod evaluator;
mod interpreter;
mod lexical_analysis;
mod syntax_analysis;

fn main() {
    pretty_env_logger::init();
    interpreter::repl();
}

#[cfg(test)]
mod tests;
