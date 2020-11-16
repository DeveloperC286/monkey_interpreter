use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "rust_monkey_interpreter",
    about = "Monkey interpreter written in Rust."
)]
pub struct Arguments {}
