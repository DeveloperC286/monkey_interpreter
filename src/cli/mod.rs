use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "monkey_interpreter",
    about = "Implementation of an interpreter for the Monkey language written in Rust, currently under active development."
)]
pub struct Arguments {}
