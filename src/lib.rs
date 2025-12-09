use anyhow::{anyhow, Result};
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

    /// Match ID names
    #[arg(short, long, num_args = 0..)]
    grep: Vec<String>,
}

// --------------------------------------------------
pub fn run(args: Args) -> Result<()> {
    let outdir = Path::new(&args.outdir);
    fs::create_dir_all(outdir)?;

    let mut outfile: Option<File> = None;
    let comment = Regex::new(r"^#\s").unwrap();
    let meta = Regex::new(r"^#=(\S{2})\s+(\S{2})\s+(.+)").unwrap();
    let sequence = Regex::new(r"^(\S+)\s+(\S+)$").unwrap();
    let delimiter = Regex::new(r"^[/]{2}$").unwrap();
    let mut grep = args.grep.into_iter().map(|patt| {
        RegexBuilder::new(&patt.trim().to_lowercase())
            .case_insensitive(true)
            .build()
            .unwrap()
    });

    let mut file = open(&args.filename)?;
    let mut num_inspected = 0;
    let mut num_taken = 0;
    loop {
        let mut buf = vec![];
        let bytes = file.read_until(b'\n', &mut buf)?;
        if bytes == 0 {
            break;
        }

        // Converts ISO-8859 to UTF-8
        let line: String = buf.iter().map(|&c| c as char).collect();
        let line = line.trim();

        if comment.is_match(&line) {
            continue;
        } else if delimiter.is_match(&line) {
            // Reset output file when we reach the end of a record
            outfile = None;
            num_inspected += 1;
        } else if let Some(cap) = meta.captures(&line) {
            if &cap[1] == "GF" && &cap[2] == "ID" {
                let id = &cap[3].trim();
                if grep.len() > 0 && !grep.any(|re| re.is_match(id)) {
                    continue;
                }
                num_taken += 1;
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

    println!(
        "Done, inspected {num_inspected}, took {num_taken}. See output in '{}'",
        outdir.display()
    );

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
