use anyhow::{bail, Result};
use clap::Parser;
use std::{
    fs::{self, File},
    io::{self, BufRead, BufReader, Write},
    path::Path,
};
use regex::Regex;

/// Split Stockholm format into FASTA
#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Args {
    /// Input file
    #[arg(default_value = "-")]
    filename: String,

    /// Output directory
    #[arg(short, long, default_value = "out")]
    outdir: String,
}

// --------------------------------------------------
pub fn run(args: Args) -> Result<()> {
    let outdir = Path::new(&args.outdir);
    fs::create_dir_all(outdir)?;

    let input = open(&args.filename)?;
    let mut outfile: Option<File> = None;
    let comment = Regex::new(r"^#\s").unwrap();
    let meta = Regex::new(r"^#=(\S{2})\s+(\S{2})\s+(.+)").unwrap();
    let sequence = Regex::new(r"(\S+)\s+(\S+)").unwrap();

    for line in input.lines().map_while(Result::ok) {
        if comment.is_match(&line) {
            continue;
        }
        else if let Some(cap) = meta.captures(&line) {
            if &cap[1] == "GF" && &cap[2] == "ID" {
                let filename = outdir.join(format!("{}.fa", &cap[3]));
                outfile = Some(File::create(&filename)?);
            }
        }
        else if let Some(cap) = sequence.captures(&line) {
            match outfile {
                Some(ref mut fh) => 
                    writeln!(fh, ">{}\n{}", &cap[1], &cap[2].replace(".", ""))?,
                _ => bail!("Found sequence before GF ID")
            }
        }
    }

    println!("Done, see output in '{}'", outdir.display());

    Ok(())
}

// --------------------------------------------------
fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
