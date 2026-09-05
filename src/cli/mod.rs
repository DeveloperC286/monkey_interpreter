use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub(crate) struct Arguments {
    #[arg(
        long,
        help = "Enable verbose output, respects RUST_LOG environment variable if set."
    )]
    pub(crate) verbose: bool,

    #[arg(help = "Path to a Monkey script (.mk) to execute. If omitted, starts a REPL.")]
    pub(crate) script: Option<PathBuf>,
}
