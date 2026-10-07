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
