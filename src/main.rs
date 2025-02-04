use clap::Parser;
use stksplit::{Args, run};

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e}");
        std::process::abort();
    }
}
