mod cli;
mod commands;
mod git;
mod utils;

use std::process;

fn main() {
    if let Err(error) = cli::run(cli::parse()) {
        eprintln!("acta: {error}");
        process::exit(1);
    }
}
