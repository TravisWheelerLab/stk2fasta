use clap::Parser;
use stk2fasta::{Args, run};

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e}");
        std::process::abort();
    }
}
