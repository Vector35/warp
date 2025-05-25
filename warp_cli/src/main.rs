pub mod operation;
mod utility;

use crate::operation::Operation;
use clap::Parser;
use std::path::PathBuf;

/// Command line tools for reading and writing WARP files.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path of the file to perform the operation on.
    #[arg(short, long)]
    path: PathBuf,

    /// Operation to perform on the file.
    #[command(subcommand)]
    operation: Operation,
}

fn main() {
    let args = Args::parse();
    args.operation.run(&args.path);
}
