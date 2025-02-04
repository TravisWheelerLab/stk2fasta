# Split Stockholm format into FASTA

```
$ cargo run -- -h
Split Stockholm format into FASTA

Usage: stk2fasta [OPTIONS] [FILENAME]

Arguments:
  [FILENAME]  Input file [default: -]

Options:
  -o, --outdir <OUTDIR>  Output directory [default: out]
  -n, --no-gap           Strip gap characters (./-)
  -h, --help             Print help
  -V, --version          Print version
```

Cf `make run` for an example.

## Author

Ken Youens-Clark <kyclark@arizona.edu>
