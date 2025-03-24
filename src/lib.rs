use anyhow::{anyhow, bail, Result};
use clap::Parser;
use regex::{Regex, RegexBuilder};
use std::{
    fs::{self, File},
    io::{self, BufRead, BufReader, Write},
    path::Path,
};

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

    /// Strip gap characters (./-)
    #[arg(short, long)]
    no_gap: bool,

    /// Match ID name
    #[arg(short, long)]
    grep: Option<String>,

    /// Verbose
    #[arg(short, long)]
    verbose: bool,
}

// --------------------------------------------------
pub fn run(args: Args) -> Result<()> {
    let outdir = Path::new(&args.outdir);
    fs::create_dir_all(outdir)?;

    let input = open(&args.filename)?;
    let mut outfile: Option<File> = None;
    let comment = Regex::new(r"^#\s").unwrap();
    let meta = Regex::new(r"^#=(\S{2})\s+(\S{2})\s+(.+)").unwrap();
    let sequence = Regex::new(r"^(\S+)\s+(\S+)$").unwrap();
    let delimiter = Regex::new(r"^[/]{2}$").unwrap();
    let grep = args.grep.map(|patt| {
        RegexBuilder::new(&patt)
            .case_insensitive(true)
            .build()
            .unwrap()
    });

    for line in input.lines().map_while(Result::ok) {
        if comment.is_match(&line) {
            continue;
        } else if delimiter.is_match(&line) {
            // Reset output file when we reach the end of a record
            outfile = None;
        } else if let Some(cap) = meta.captures(&line) {
            if &cap[1] == "GF" && &cap[2] == "ID" {
                let id = &cap[3];
                if let Some(re) = &grep {
                    if !re.is_match(id) {
                        continue;
                    }
                }
                if args.verbose {
                    println!("ID '{id}'");
                }
                let filename = outdir.join(format!("{id}.fa"));
                outfile = Some(File::create(&filename)?);
            }
        } else if let Some(cap) = sequence.captures(&line) {
            if let Some(ref mut fh) = outfile {
                let seq = cap[2].replace(".", "-");
                writeln!(
                    fh,
                    ">{}\n{}",
                    &cap[1],
                    if args.no_gap {
                        seq.replace("-", "")
                    } else {
                        seq
                    }
                )?;
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
        _ => Ok(Box::new(BufReader::new(
            File::open(filename).map_err(|e| anyhow!("{filename}: {e}"))?,
        ))),
    }
}
