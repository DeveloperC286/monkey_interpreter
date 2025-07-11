use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub(crate) struct Arguments {
    #[arg(
        long,
        help = "Enable verbose output, respects RUST_LOG environment variable if set."
    )]
    pub(crate) verbose: bool,
}
