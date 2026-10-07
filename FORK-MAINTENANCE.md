# DataForge fork: ownership, decision and verification

Owner: Platon7788 / DataForge GUI maintainer. Created 2026-10-07 from upstream
`0c7291223fe9e0bc2808f0255cb991066eedd796` (2026-08-06). Upstream was not
unchanged for two years: main is 67 commits ahead of registry tag v0.25.1.
Those commits include graph-work budgets and malformed-font regressions.
The upstream bug-fix-only maintenance policy and registry lifecycle advisory
[RUSTSEC-2026-0192](https://rustsec.org/advisories/RUSTSEC-2026-0192) remain facts.
This fork does not claim a RustSec patched registry version or independent certification.

## Chosen integration and why it fits

Keep compatible package `ttf-parser` 0.25.1 and replace its source with this git
fork using root `[patch.crates-io]` in both DataForge and Dear-imgui-custom-mod.
Their Cargo.lock files record the exact reviewed commit; manifests follow main
and semver dependency ranges. Patches in a dependency's manifest do not propagate
to consumers, so both workspace roots need the override.

This preserves owned_ttf_parser/ab_glyph APIs, feature selection, no_std and the
existing Linux/Wayland decoration path. It directly brings the current upstream
hardening into the consumed parser. A version bump alone would not provide those
source changes or establish ongoing maintenance. Future updates require reviewing
the upstream diff, these tests and the effective downstream graph.

## Modernization and correctness changes

- All four original Cargo packages and downstream smoke: Rust 1.99;
  fuzz package edition 2024/resolver 3.
  rust-toolchain follows stable; CI tests explicit MSRV 1.99.0 and stable.
- Registry API checked 2026-10-07: core_maths 0.1.1, base64 0.23.1,
  pico-args 0.5.0, xmlwriter 0.1.0 and bencher 0.1.5 are current stable.
  Semver ranges retained. Every resolved registry package in all five manifests
  is checked against crates.io non-yanked stable metadata in dependency-audit.json.
  One explicit inactive-target exception remains: getrandom 0.4.3 requires r-efi
  `^6` only on UEFI with the efi_rng backend. Registry r-efi 7.1.0 cannot satisfy
  that major-version contract. No supported Windows/Linux font path compiles it.
  Updating a foreign firmware ABI just to change the version number is outside
  this parser fork; recheck when getrandom supports the new major.
- Removed tiny-skia-path used only for a six-number affine inverse in the SVG
  example. Its current release requires old strict-num. The example now uses a
  checked finite inverse with point-composition tests and actual SVG comparison.
  Singular transforms skip a collapsed paint instead of unwrap panic/invalid matrix.
  This removes four dev packages and an unnecessary dependency constraint.
  demo.ttf SVG is byte-identical. COLR SVG differs only in gradient matrix rounding;
  a point oracle on the 1000-unit square measured maximum error 0.00012208
  font units across 2505 SVG elements (10 changed matrices), below 0.01.
  An omitted gradientTransform is the identity matrix, as required by SVG.
- Modern idioms use is_some_and, div_ceil, is_multiple_of, derived Default and
  let chains. Specification comments and bounded traversal algorithms are retained.
  The large stack enum intentionally stays inline: boxing would violate the
  ordinary-glyph allocation contract. Its narrow Clippy allowance documents this.
- Rect width/height and Face horizontal/vertical height use saturating_sub.
  Before: a valid public Rect spanning i16::MIN..MAX panicked in debug and wrapped
  in release. Now: exact representable differences are unchanged; an unrepresentable
  difference returns the nearest i16 boundary in both profiles. This preserves API
  types, monotonic dimensions and the parser's no-panic contract. Boundary and
  raw-table Face tests prove the specified behavior.
- C API raw-pointer entry points are unsafe for Rust callers with explicit
  lifetime/extent/exclusivity/callback contracts; C signatures remain compatible.
  Internal helpers no longer claim static references. Initialization checks null
  and alignment, uses typed ptr::write, exposes ttfp_face_align_of. The regression
  test uses aligned MaybeUninit storage. Header version now agrees with Cargo.
- NormalizedCoordinate clamps infinities and maps NaN to zero consistently,
  instead of failing a debug-only finite assertion. OS/2 Windows ascent/descent
  are read as unsigned magnitudes, as required by the OpenType specification;
  conversion to the existing signed API saturates. All 65536 stored values are
  checked against a wider-integer oracle. This prevents sign reversal and
  negation overflow without changing the public types.
- Lazy arrays cap representable counts instead of wrapping to zero, and zero-size
  element counts are empty. Byte offsets/products use checked arithmetic; unchecked
  stream advance saturates into an exhausted state. Invalid operations cannot
  wrap back into input. Tests cover usize::MAX and custom element sizes.
- Fuzz harnesses use Result-based Face::parse and stable libfuzzer-sys 0.4.13,
  following the same infrastructure as DataForge without installing cargo-afl.
  AFL's home/windows-sys branch required old windows-link; migrating the unmaintained
  harnesses removes that tooling branch instead of overriding a foreign ABI dependency.
  Outlining
  samples three glyph IDs per input: total input work no longer multiplies the
  parser's per-glyph budget by an attacker-controlled glyph count up to 65535.
- Root disables accidental discovery of the separate benches package. Feature
  tests gate only actual optional APIs; static COLR coverage remains enabled
  without variable-fonts. Examples declare required capabilities.

## Executed evidence and limitations

Native GitHub CI [37574720400](https://github.com/Platon7788/ttf-parser/actions/runs/37574720400)
on `41e66b266b180a728fbb68e872d9074ffc06c671`: all four Windows/Ubuntu ×
MSRV/stable jobs succeeded. Core all-feature tests: 279 tests plus 3 doctests
per profile; SVG matrix tests: 2; C API: 3 in each profile; downstream: 1.
Windows Rust 1.99: upstream full-feature suite plus new metric/mutation tests
passes in debug and release. Strict Clippy passes for root and C API.
C API's three tests pass debug/release and Miri on Linux target interpreted
from Windows. Native Clang C smoke links the MSVC DLL and passes.
Benchmark package builds. Feature permutations include allocator-free no_std,
std-only, alloc-only variable/gvar and all features.
Rustup check confirms installed stable 1.99.0 is current on 2026-10-07.
Google Benchmark's auxiliary wrap moved from 1.4.1 to current stable 1.9.5 using
its native CMake project, and the C++ benchmark uses C++17. Meson direct-rustc
features are explicit and MSRV checked. Meson/Qt/FreeType auxiliary tools were
not installed or executed locally; their full build remains a separate gate.

Hostile smoke covers four existing licensed fonts, sampled truncations, 1024
single-byte corruptions and 512 zero/ff byte buffers. It exercises cmap, outlines,
bounding boxes, metrics, raster/SVG and variation extremes. This is deterministic
finite coverage, not a sustained coverage-guided fuzz campaign or proof of no bugs.
Existing tests exercise COLR/glyf/CFF fanout budgets and malformed tables.

The separate downstream smoke package checks owned_ttf_parser and ab_glyph with
the actual patched parser. CI passed libFuzzer harness compilation and native
C smoke on both platforms, with ASan on the Linux C caller. Native Wayland compositor/window decoration, 32-bit targets, allocator
failure under gvar-alloc, and a full independent line-by-line security review
remain unverified. Source review concentrated on parsing arithmetic, raw-pointer
FFI, recursive work budgets, feature boundaries and stale tooling; it is not a
claim that every possible font format or all source lines have been proven safe.

## Maintenance tasks

Initial instrumented campaign
[37584718516](https://github.com/Platon7788/ttf-parser/actions/runs/37584718516)
on 1a9dc0bb succeeded: all four native Windows/Ubuntu stable/MSRV jobs plus
three Linux ASan/libFuzzer targets, 120 seconds requested (121 observed) each.
Glyph index: 14898804 runs, cov 824; outline: 3120513 runs, cov 1226;
variable outline: 6977206 runs, cov 1345. Total 24996523 executions without
crash, timeout or ASan failure. Raw cov counters are not a source coverage
percentage. This finite campaign supplements deterministic smoke; a longer
campaign with broader font corpora remains a maintenance task. Future manual
CI also preserves evolved corpora/logs/crash artifacts for 14 days.
Windows nightly/cargo-fuzz instrumentation failed to link before any input;
that attempt is not counted as an executed campaign. The Linux script uses
nightly and instrumentation flags directly, without installing cargo-fuzz.

- [x] Preserve upstream fixes and API/feature contracts.
- [x] Modernize MSRV/manifests, parser idioms and unsafe FFI boundary.
- [x] Add executed boundary/malformed-input proofs and downstream oracle.
- [x] Initial bounded coverage-guided ASan campaign for all three font targets.
- [ ] Sustained libFuzzer campaign with corpora and coverage accounting.
- [ ] Native Wayland/CSD end-to-end regression.
- [ ] Independent parser/FFI review, 32-bit and allocation-failure validation.

Evidence that would overturn acceptance: any changed valid-font geometry in
upstream fixture or downstream oracle, panic/UB, unbounded graph amplification,
missing feature permutation, or a source other than this fork in the consumer lock.

Executed logs and SVG oracle: [audit/2026-10-07](audit/2026-10-07).
Current stable graph snapshot: [dependency-audit.json](dependency-audit.json).

Primary references: [upstream](https://github.com/harfbuzz/ttf-parser),
[OpenType OS/2 Windows metrics](https://learn.microsoft.com/en-us/typography/opentype/spec/os2#uswinascent),
[stable dependency metadata](https://crates.io/api/v1/crates/libfuzzer-sys).

Also: registry/main, parser, FFI, fuzz harnesses, wrappers and both consumers:
served; native Wayland and sustained campaign: untested | Frame change:
maintained compatible source with measured invariants: kept.
