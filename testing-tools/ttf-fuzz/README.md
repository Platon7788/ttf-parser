# Font fuzz targets

The DataForge fork uses current libfuzzer-sys and Result-based Face::parse.
No cargo-afl installation is required. Compilation with existing tools:

```sh
cargo check --release --all-targets
```

With an already available cargo-fuzz, run a coverage-guided campaign:

```sh
cargo +nightly fuzz run fuzz-glyph-index corpus
cargo +nightly fuzz run fuzz-outline corpus
cargo +nightly fuzz run fuzz-variable-outline corpus
```

Raw licensed fonts provide seeds. The existing strip-tables.py helper can reduce
font data when fontTools is already available. Glyph-outline targets sample three
IDs per input, bounding harness work independently of font glyph count.
Compilation and deterministic regression smoke are not a sustained campaign.

On Linux, an existing nightly toolchain can run all three actual instrumented
targets without installing cargo-fuzz:

```sh
bash testing-tools/ttf-fuzz/run-bounded.sh 120
```

The optional seconds argument bounds each target to 1..=3600 seconds.
The script uses the instrumentation flags from
[cargo-fuzz 0.13.2](https://github.com/rust-fuzz/cargo-fuzz/blob/0.13.2/src/project.rs),
ASan, debug assertions, three licensed font seeds, 10-second per-input timeout
and a 2 GiB RSS limit. Logs, evolved corpora and crash artifacts remain under
target/bounded-fuzz. Exit failure is preserved through log capture.
Manual CI executes this gate. A short successful run is finite evidence,
not exhaustive coverage. Windows nightly 1.100 plus cargo-fuzz 0.13.2 failed
before execution with unresolved __start/__stop___sancov coverage symbols;
that attempt must not be reported as parser acceptance or an executed campaign.
