use anyhow::Result;
use assert_cmd::Command;
use pretty_assertions::assert_eq;
use rand::{distr::Alphanumeric, Rng};
use std::{
    fs,
    path::{Path, PathBuf},
};
use tempfile::tempdir;

const PRG: &str = "stk2fasta";
const VALID_STK: &str = "tests/inputs/test.stk";
const INVALID_STK: &str = "tests/inputs/bad.stk";

// --------------------------------------------------
#[test]
fn dies_invalid_file() -> Result<()> {
    let bad = gen_bad_file();
    let output = Command::cargo_bin(PRG)?.arg(&bad).output().expect("fail");
    assert!(!output.status.success());

    let err = String::from_utf8(output.stderr)?;
    assert_eq!(err, format!("{bad}: No such file or directory (os error 2)\n"));
    Ok(())
}

// --------------------------------------------------
#[test]
fn dies_missing_id() -> Result<()> {
    let output = Command::cargo_bin(PRG)?.arg(INVALID_STK).output().expect("fail");
    assert!(!output.status.success());
    Ok(())
}

// --------------------------------------------------
#[test]
fn run_no_gap() -> Result<()> {
    run(false)
}

// --------------------------------------------------
#[test]
fn run_gapped() -> Result<()> {
    run(true)
}

// --------------------------------------------------
fn run(gapped: bool) -> Result<()> {
    let outdir = tempdir()?;
    let mut args = vec![
        "-o".to_string(),
        outdir.path().display().to_string(),
        VALID_STK.to_string(),
    ];

    if !gapped {
        args.push("-n".to_string());
    }

    let output = Command::cargo_bin(PRG)?.args(&args).output().expect("fail");
    assert!(output.status.success());

    let out_paths: Vec<PathBuf> = fs::read_dir(&outdir)?
        .map_while(Result::ok)
        .map(|entry| entry.path())
        .collect();

    let mut out_files: Vec<String> = out_paths
        .iter()
        .map(|path| path.file_name().unwrap().to_string_lossy().to_string())
        .collect();
    out_files.sort();

    assert_eq!(
        out_files,
        &["Charlie1-L_tua.fa", "Chompy-2a_tua.fa", "Chompy-2b-tua.fa"]
    );

    let expected_dir = if gapped {
        "tests/expected/gapped"
    } else {
        "tests/expected/no_gap"
    };
    for path in out_paths {
        let actual = fs::read_to_string(&path)?;
        let filename = path.file_name().unwrap();
        let expected = fs::read_to_string(Path::new(expected_dir).join(filename))?;
        assert_eq!(actual, expected);
    }

    Ok(())
}

// --------------------------------------------------
fn random_string() -> String {
    rand::rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect()
}

// --------------------------------------------------
fn gen_bad_file() -> String {
    loop {
        let filename = random_string();
        if fs::metadata(&filename).is_err() {
            return filename;
        }
    }
}
