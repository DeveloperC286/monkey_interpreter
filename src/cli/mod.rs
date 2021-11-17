use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Monkey Interpreter",
    about = "Implementation of an interpreter for the Monkey language written in Rust, currently under active development."
)]
pub(crate) struct Arguments {}
