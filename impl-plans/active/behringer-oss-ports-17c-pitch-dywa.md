# Behringer OSS Ports 17c: Pitch Crate, dywapitchtrack Port

**Status**: Ready
**Plan ID**: bop-17c
**Design Reference**: design-docs/specs/design-behringer-oss-ports.md section C (dywapitchtrack, Parity table: dywa), D3
**Depends On**: bop-00 (wave W2; parallel with bop-01, bop-16a, bop-16b, bop-17a, bop-17b)
**Created**: 2026-10-31
**Last Updated**: 2026-10-31

## Intent and Context

Port `dywapitchtrack.c` (dywapitchtrack 0.1.0.1, Antoine Schmitt, MIT, `cbits`) to
`music_things_pitch::dywa`: wavelet pitch detection plus the tracker smoothing state. Upstream
`malloc`s a scratch buffer on every call; the port preallocates it at construction. Upstream also
hardcodes 44100 Hz; the port uses the configured sample rate (D3), so it is identical to upstream at 44100.

Context: bop-00 created the doc-only `src/dywa.rs` (now yours), the frozen `ConfigError`, and the
test twins `tests/common/{stimulus,fnv}.rs`.

## Non-goals

- No edits to `src/lib.rs`, `src/error.rs`, `src/smb.rs`, `src/stft/`, `tests/common/*`,
  `Cargo.toml`/`Cargo.lock` or other crates. No new error variants.
- No algorithm changes beyond the sample-rate parameter.

## Write Ownership (fanout contract)

- writePaths: `crates/pitch/src/dywa.rs`, `crates/pitch/tests/dywa_parity.rs`,
  `crates/pitch/tests/dywa_no_alloc.rs`, `crates/pitch/tests/fixtures/dywa` (directory),
  `tools/pitch-harness/dywa` (directory), `impl-plans/active/behringer-oss-ports-17c-pitch-dywa.md`,
  `target`, `tmp/behringer-oss-ports/bop-17c`
- sharedPaths: none
- artifactRoots (digest-only): `target`, `tmp/behringer-oss-ports/bop-17c`
- Read-only: `crates/pitch/src/lib.rs`, `crates/pitch/src/error.rs`, `crates/pitch/tests/common/*`,
  `tools/common/*` (bop-00).
- Never run git checkout/stash/reset or any other git operation that rewrites the tree.

## Common Execution Rules

- `MUSIC_THINGS_REFS=/Users/taco/gits/tacogips/music-things-refs`. Read only
  `dywapitchtrack/dywapitchtrack-0.1.0.1/cbits/*`, `dywapitchtrack/dywapitchtrack-0.1.0.1/include/dywapitchtrack.h`
  (which defines the `dywapitchtracker` struct, including `_prevPitch` and `_pitchConfidence`), and the
  package license file, if present. Never open Music Tribe AGPL code.
- Evidence directory: `tmp/behringer-oss-ports/bop-17c/`. It holds `intent.md`, `pre-hashes.txt`,
  `post-hashes.txt` and `logs/`. Re-read each file before editing; on drift, stop and log it. Only edit `writePaths`.
- Siblings compile this crate at the same time. If an error is in a file you do not own, rerun later and log it.
- Format owned files with `rustfmt --edition 2021`. Cargo env: `CARGO_TERM_QUIET=true NEXTEST_STATUS_LEVEL=fail NEXTEST_FAILURE_OUTPUT=immediate-final NEXTEST_HIDE_PROGRESS_BAR=1`.

## Deliverables

### T1. `crates/pitch/src/dywa.rs`
- Header: the upstream copyright line and MIT license text, the source path and the version.
- `pub struct PitchTracker` with:
  - `pub fn new(sample_rate: f32, window: usize, hop: usize) -> Result<Self, ConfigError>`.
    `window` must be a power of two (`NotPowerOfTwo { what: "window", .. }`) and at least upstream's
    needed sample count (`OutOfRange`). `hop` must satisfy `1 <= hop <= window`
    (`InvalidValue { what: "hop" }`). The scratch buffer and the ring buffer are allocated here.
  - `pub fn compute(&mut self, samples: &[f32]) -> f64` ports `dywapitch_computepitch` on the first
    `window` samples. Each sample is converted with `as f64`, and the result is Hz, where 0.0 means no pitch.
  - `pub fn push(&mut self, input: &[f32]) -> Option<f64>` buffers input and, each time `hop` new
    samples have arrived after the first full window, computes over the latest `window` samples. It
    returns the most recent pitch from this call, or None.
  - `pub fn reset(&mut self)` ports `dywapitch_inittracking`.
- The tracker state fields come from the `dywapitchtracker` struct in `include/dywapitchtrack.h`.
- Port the wavelet loop and `_dywapitch_dynamicprocess` (the tracker smoothing) statement by statement,
  in the upstream precision (double). Upstream's 44100 constant(s) become `self.sample_rate as f64`.

### T2. Harness `tools/pitch-harness/dywa/dywa_harness.c`, `tools/pitch-harness/dywa/generate.sh`, `tools/pitch-harness/dywa/README.md`
- The harness compiles the pinned `dywapitchtrack.c` in place with
  `clang -std=c99 -O1 -ffp-contract=off -fno-fast-math -fno-builtin -I "$MUSIC_THINGS_REFS/dywapitchtrack/dywapitchtrack-0.1.0.1/include" -I tools/common ... -lm`
  (`cbits/dywapitchtrack.c` does `#include "dywapitchtrack.h"`, which lives under `include/`).
  `generate.sh` uses exactly this command.
- It converts `pitch_stimulus()` (from `tools/common/stimulus.h`) to double. For window W in {1024, 2048}
  it initializes the tracker, then for each start in `0, W, 2W, ...` while `start + W <= 11025` it
  calls `dywapitch_computepitch(&t, buf, start, W)` and prints `W start <pitch %.17g>`.
- `generate.sh --work-dir DIR` writes `crates/pitch/tests/fixtures/dywa/{pitches.txt,meta.txt}`.

### T3. Tests
- `crates/pitch/tests/dywa_parity.rs`: for each W, a fresh `PitchTracker::new(44100.0, W, W)`, then
  `compute` on each window slice in the same order. Each result is within relative 1e-9 of the fixture
  and has an identical zero/non-zero decision. The test asserts the stimulus FNV from `meta.txt`, and
  that `push` over the whole stimulus in 300-sample chunks yields the same sequence as `compute`
  (hop = W).
- `crates/pitch/tests/dywa_no_alloc.rs`: a counting `#[global_allocator]`; `compute`, `push` and
  `reset` allocate 0 times after `new`.

## Key Pitfalls

- The tracker state (previous pitch, confidence) carries across calls. Process windows in the same
  order as the harness, starting from a freshly initialized tracker.
- Upstream integer and double math (power-of-two loops, `floor`, `abs` on ints) must use the same
  types. Do not change `int` to `usize` where negative values or wrapping matter.
- Only the f32-to-f64 conversion is new. Do not compute in f32.

## Tests (situation -> expected)

- W=1024 and W=2048 sequences vs `pitches.txt` -> relative 1e-9, with identical zero/non-zero decisions.
- `new(44100.0, 1000, 1000)` -> `Err(NotPowerOfTwo)`. `new(44100.0, 1024, 0)` -> `Err(InvalidValue)`.
- `push` sequence == `compute` sequence.
- No-alloc -> 0.

## Verification Commands (evidence)

1. `MUSIC_THINGS_REFS=/Users/taco/gits/tacogips/music-things-refs tools/pitch-harness/dywa/generate.sh --work-dir tmp/behringer-oss-ports/bop-17c/harness`: exit 0.
2. `cargo nextest run -p music-things-pitch --test dywa_parity --test dywa_no_alloc`: exit 0, 0 failures.
3. `cargo clippy -p music-things-pitch --all-targets -- -D warnings`: exit 0.
4. `rustfmt --edition 2021 --check crates/pitch/src/dywa.rs crates/pitch/tests/dywa_parity.rs crates/pitch/tests/dywa_no_alloc.rs`: exit 0.
5. `cargo build -p music-things-pitch --target wasm32-unknown-unknown`: exit 0.

## Completion Criteria

- [ ] T1-T3 delivered with the pinned API, and D3 is documented in the module docs.
- [ ] Parity within relative 1e-9.
- [ ] Verifications 1-5 exit 0, with logs in `tmp/behringer-oss-ports/bop-17c/logs/`.
- [ ] Only `writePaths` are modified, and every file is under 1000 lines.
- [ ] Progress Log updated with commands, exit status, tests run/passed and log paths.

## Progress Log

### Session: 2026-10-31
**Tasks Completed**: Plan created (not started)
**Notes**: -
