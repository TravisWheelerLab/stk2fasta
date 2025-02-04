use anyhow::Result;
use assert_cmd::Command;
use std::{fs, path::{Path, PathBuf}};
use tempfile::tempdir;
use pretty_assertions::assert_eq;

const PRG: &str = "stksplit";
const STKFILE: &str = "tests/inputs/test.stk";

#[test]
fn run() -> Result<()> {
    let outdir = tempdir()?;
    let output = Command::cargo_bin(PRG)?
        .args([
            "-o".to_string(),
            outdir.path().display().to_string(),
            STKFILE.to_string(),
        ])
        .output()
        .expect("fail");
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

    let expected_dir = Path::new("tests/expected");
    for path in out_paths {
        let actual = fs::read_to_string(&path)?;
        let filename = path.file_name().unwrap();
        let expected = fs::read_to_string(expected_dir.join(filename))?;
        assert_eq!(actual, expected);
    }

    Ok(())
}
