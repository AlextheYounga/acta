mod cli;
mod commands;
mod git;
mod utils;

fn main() {
    if let Err(error) = cli::run(cli::parse()) {
        eprintln!("acta: {error}");
        std::process::exit(1);
    }
}
