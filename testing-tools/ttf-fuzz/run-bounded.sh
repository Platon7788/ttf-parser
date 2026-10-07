#!/usr/bin/env bash
# Run actual coverage-guided targets with ASan; no cargo-fuzz installation.
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$root"
seconds="${1:-120}"
if [[ ! "$seconds" =~ ^[1-9][0-9]*$ ]] || (( seconds > 3600 )); then
    echo 'Duration must be an integer in 1..=3600 seconds per target' >&2
    exit 2
fi
target_dir="$root/target/bounded-fuzz"
export RUSTFLAGS='-Cpasses=sancov-module -Cllvm-args=-sanitizer-coverage-level=4 -Cllvm-args=-sanitizer-coverage-inline-8bit-counters -Cllvm-args=-sanitizer-coverage-pc-table -Cllvm-args=-sanitizer-coverage-trace-compares -Cllvm-args=-simplifycfg-branch-fold-threshold=0 -Cdebug-assertions -Ccodegen-units=1 -Zsanitizer=address --cfg fuzzing'
export ASAN_OPTIONS='detect_odr_violation=0'
cargo +nightly build --release --target x86_64-unknown-linux-gnu \
    --manifest-path testing-tools/ttf-fuzz/Cargo.toml --bins --target-dir "$target_dir"
for name in fuzz-glyph-index fuzz-outline fuzz-variable-outline; do
    corpus="$target_dir/corpus/$name"
    artifacts="$target_dir/artifacts/$name"
    mkdir -p "$corpus" "$artifacts"
    cp tests/fonts/*.ttf "$corpus/"
    "$target_dir/x86_64-unknown-linux-gnu/release/$name" "$corpus" \
        -max_total_time="$seconds" -timeout=10 -rss_limit_mb=2048 \
        -seed=20261007 -artifact_prefix="$artifacts/" 2>&1 | tee "$target_dir/$name.log"
done
