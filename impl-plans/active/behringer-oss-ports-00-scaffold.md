# Behringer OSS Ports 00: Workspace Scaffold and Shared Test Contracts

**Status**: Ready
**Plan ID**: bop-00
**Design Reference**: design-docs/specs/design-behringer-oss-ports.md (Global rules; A Parity method/Stimulus; C Parity; Plans table)
**Depends On**: none (wave W1)
**Created**: 2026-10-31
**Last Updated**: 2026-10-31

## Intent and Context

The user wants Rust ports of KICK/IGNITE OSS DSP code (Airwindows, TinySoundFont, pitch tools) in new
crates. This first plan creates the three empty crates, so `Cargo.lock` changes once. It also pins
every contract that more than one later plan relies on:
- the deterministic stimulus generators (C and Rust twins) and their checksums
- FNV-1a-64
- the pitch `ConfigError` type
- the module skeleton of `music-things-soundfont` and `music-things-pitch`

Later plans (wave W2: bop-01, bop-16a, bop-16b, bop-17a, bop-17b, bop-17c) build on this without
touching any of it.

Repository context: virtual workspace `Cargo.toml` (`members = ["crates/*"]`, which must NOT be edited).
The only existing crate is `crates/music-things` (facade). Copy the manifest style of
`crates/music-things/Cargo.toml` (explicit `version`, `edition = "2021"`, `rust-version = "1.83"`,
`license = "MIT"`, `homepage`, `repository`, empty `[dependencies]`, `[lib] path`), and the
crate-doc style of `crates/music-things/src/lib.rs:1` (`//!` module docs).

## Non-goals

- No DSP code, no Airwindows framework (bop-01), no harness except `tools/common`.
- Do not edit the root `Cargo.toml`, `mise.toml`, `crates/music-things/*`, README, notices or AGENTS.md.
- Do not add any dependency or dev-dependency.

## Write Ownership (fanout contract)

Concrete repository-relative paths (no globs). A directory entry owns every file under it.

- writePaths: `crates/airwindows/Cargo.toml`, `crates/soundfont/Cargo.toml`, `crates/pitch/Cargo.toml`,
  `Cargo.lock`, `crates/airwindows/src/lib.rs`, `crates/soundfont/src/lib.rs`,
  `crates/soundfont/src/font/mod.rs`, `crates/soundfont/src/midi/mod.rs`,
  `crates/soundfont/src/synth/mod.rs`, `crates/pitch/src/lib.rs`, `crates/pitch/src/error.rs`,
  `crates/pitch/src/smb.rs`, `crates/pitch/src/dywa.rs`, `crates/pitch/src/stft/mod.rs`,
  `tools/common/fnv.h`, `tools/common/stimulus.h`, `tools/common/stimulus_check.c`,
  `tools/common/README.md`, `crates/airwindows/tests/common/stimulus.rs`,
  `crates/airwindows/tests/common/fnv.rs`, `crates/pitch/tests/common/stimulus.rs`,
  `crates/pitch/tests/common/fnv.rs`, `crates/soundfont/tests/common/fnv.rs`,
  `crates/airwindows/tests/stimulus_checksum.rs`, `crates/pitch/tests/stimulus_checksum.rs`,
  `crates/soundfont/tests/fixture_budget.rs`, `crates/pitch/tests/fixture_budget.rs`,
  `impl-plans/active/behringer-oss-ports-00-scaffold.md`, `target`, `tmp/behringer-oss-ports/bop-00`
- sharedPaths: none
- artifactRoots (digest-only): `target`, `tmp/behringer-oss-ports/bop-00`
- Read-only: `crates/music-things/Cargo.toml` (style reference).
- `Cargo.lock` changes only in bop-00 and bop-18. Never run git checkout/stash/reset or any other git
  operation that rewrites the tree; siblings share the working directory.

## Common Execution Rules (all bop plans)

- The reference checkout is `MUSIC_THINGS_REFS=/Users/taco/gits/tacogips/music-things-refs`. Read only the
  upstream directories named in the design. Never open Music Tribe KICK/IGNITE (`AudIO`, `Kick`, `Source`) code.
- Evidence directory: `tmp/behringer-oss-ports/bop-00/`. First write the immutable intent snapshot
  `tmp/behringer-oss-ports/bop-00/intent.md` (copy of this plan's Deliverables list), then
  `pre-hashes.txt` (`shasum -a 256` of every existing owned file). At the end, write `post-hashes.txt`.
- Re-read every file immediately before editing it. If an owned file differs from your last write
  (drift), stop, record it in the Progress Log and re-plan the edit. Never edit files outside `writePaths`.
- Write each verification log to `tmp/behringer-oss-ports/bop-00/logs/<name>.log`, and record the command,
  exit status, test counts and log path in the Progress Log.
- Format with `rustfmt --edition 2021 <owned .rs files>`. Never run a workspace-wide `cargo fmt`.
- Use `export CARGO_TERM_QUIET=true NEXTEST_STATUS_LEVEL=fail NEXTEST_FAILURE_OUTPUT=immediate-final NEXTEST_HIDE_PROGRESS_BAR=1`.

## Deliverables

### D1. Crate manifests (+ Cargo.lock)
- `crates/airwindows/Cargo.toml`: package `music-things-airwindows`, description "Rust ports of Airwindows effects (MIT)".
- `crates/soundfont/Cargo.toml`: package `music-things-soundfont`.
- `crates/pitch/Cargo.toml`: package `music-things-pitch`.
- Each has an empty `[dependencies]`, no `[dev-dependencies]` and no build script. Then run
  `cargo build --workspace` so `Cargo.lock` gains the three path packages and nothing else.

### D2. Library roots and stubs
- `crates/airwindows/src/lib.rs`: `//!` crate docs only (bop-01 replaces this file).
- `crates/soundfont/src/lib.rs`: crate docs plus `pub mod font; pub mod midi; pub mod synth;`. Each of
  `src/font/mod.rs`, `src/midi/mod.rs` and `src/synth/mod.rs` gets a single `//!` doc line saying which
  plan fills it (bop-16b/16a/16c).
- `crates/pitch/src/lib.rs`: crate docs plus `pub mod dywa; pub mod error; pub mod smb; pub mod stft;`
  and `pub use error::ConfigError;`. `src/smb.rs`, `src/dywa.rs` and `src/stft/mod.rs` get a `//!`
  doc line only.
- `crates/pitch/src/error.rs` (frozen contract, complete in this plan):
  `#[derive(Debug, Clone, Copy, PartialEq, Eq)] pub enum ConfigError { NotPowerOfTwo { what: &'static str, value: usize }, OutOfRange { what: &'static str, value: usize, min: usize, max: usize }, InvalidValue { what: &'static str }, TooManyFactors { max: usize, got: usize } }`,
  plus `impl Display` (lowercase messages, for example "framesize must be a power of two (got 1000)") and `impl std::error::Error`.

### D3. Shared C stimulus/checksum (`tools/common/`)
- `tools/common/fnv.h`: `uint64_t fnv1a64_update(uint64_t h, const unsigned char* p, size_t n)`, with
  offset basis `0xcbf29ce484222325` and prime `0x100000001b3`.
- `tools/common/stimulus.h` (static inline C99, MIT header):
  - `void aw_stimulus(float sample_rate, float* out_l, float* out_r)`: fills 8192 frames.
  - `void pitch_stimulus(float* out)`: fills 11025 mono samples at 44100 Hz.
  - `uint32_t mt_xorshift32(uint32_t* s)`: `x ^= x<<13; x ^= x>>17; x ^= x<<5`, stores the state and returns it.
  - `uint64_t f32_buffer_fnv(const float* a, size_t n)`: hashes the little-endian bytes of each f32.
- `tools/common/stimulus_check.c`: prints `aw44100=<16 hex>`, `aw96000=<16 hex>` and `pitch=<16 hex>`.
  The AW hash covers L then R per frame, interleaved.
- `tools/common/README.md`: the stimulus spec below, how to build (`clang -std=c99 -O1 -ffp-contract=off
  -fno-fast-math -fno-builtin`) and how to run.

**Stimulus spec (exact; all intermediate math in double, final `(float)` / `as f32` rounding)**:
- `parabolic(p)`: with p in [0,1), `x = 2.0*p - 1.0`, return `4.0 * x * (1.0 - fabs(x))`.
- AW stimulus, frames n = 0..8192:
  - `[0,256)`: L = 0.9 at n == 0, otherwise 0. R = -0.5 at n == 0, otherwise 0.
  - `[256,4096)`: phases `phL = phR = 0.0` at n = 256. `incL = 220.0/(double)sr`, `incR = 330.0/(double)sr`.
    `amp = 0.05 + 1.15 * ((double)(n-256) / 3840.0)`. `L = amp*parabolic(phL)`, `R = amp*parabolic(phR)`.
    Then `ph += inc; if (ph >= 1.0) ph -= 1.0;` for each channel.
  - `[4096,6144)`: state `s = 0x12345678`. L = `((double)mt_xorshift32(&s) / 4294967296.0 * 2.0 - 1.0) * 0.5`,
    then R the same with the next draw.
  - `[6144,8192)`: 0.
- Pitch stimulus, n = 0..11025:
  - `f = 220.0 + 110.0 * ((double)n / 11025.0)`, `ph += f/44100.0` (wrap as above, ph starts at 0.0 and
    advances after the sample is computed).
  - `q = 2.0*ph`, and `if (q >= 1.0) q -= 1.0`. Noise state starts at `0x9E3779B9`, and
    `noise = (double)mt_xorshift32(&s) / 4294967296.0 * 2.0 - 1.0`.
  - `y = 0.3*parabolic(ph) + 0.15*parabolic(q) + 0.05*noise`.

### D4. Rust twins and checksum tests
- `crates/airwindows/tests/common/stimulus.rs` and `crates/pitch/tests/common/stimulus.rs` (identical
  content): `pub fn aw_stimulus(sample_rate: f32) -> (Vec<f32>, Vec<f32>)`,
  `pub fn pitch_stimulus() -> Vec<f32>`, `pub fn xorshift32(state: &mut u32) -> u32`,
  `pub const AW_FRAMES: usize = 8192`, `pub const PITCH_SAMPLES: usize = 11025`. The first line is
  `#![allow(dead_code)]`.
- `crates/{airwindows,pitch,soundfont}/tests/common/fnv.rs` (identical): `pub fn fnv1a64(bytes: &[u8]) -> u64`
  and `pub fn f32_fnv(samples: &[f32]) -> u64`.
- `crates/airwindows/tests/stimulus_checksum.rs` and `crates/pitch/tests/stimulus_checksum.rs` include
  the twins with `#[path = "common/stimulus.rs"] mod stimulus;` (and fnv the same way). They assert the
  hex constants printed by `stimulus_check.c`, and the constants are hard-coded in the test.
- `crates/soundfont/tests/fixture_budget.rs` asserts the sum of file sizes under
  `concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures")` is at most 400_000 bytes.
  `crates/pitch/tests/fixture_budget.rs` asserts at most 600_000 bytes. A missing directory counts as 0
  (use `std::fs::read_dir` recursively).

## Key Pitfalls

- C and Rust must evaluate in the same order with no FMA. Write `amp * parabolic(ph)` as two separate
  products, not as a fused expression. Rust never fuses, and C is compiled with `-ffp-contract=off`.
- Do not use `sin`/`cos` anywhere in the stimulus, which would put libm into the inputs.
- `(double)u32 / 4294967296.0` must be done in f64 in Rust (`x as f64 / 4294967296.0`).
- Stub module files must compile with no warnings under `clippy -D warnings`. A doc-only file is fine.
- `tests/common/*.rs` used through `#[path]` gives dead-code warnings for unused items, so keep
  `#![allow(dead_code)]` at the top.

## Tests (situation -> expected)

- `cargo nextest run -p music-things-airwindows --test stimulus_checksum`: Rust checksums equal the C
  `aw44100` and `aw96000` values, and the test passes.
- `cargo nextest run -p music-things-pitch --test stimulus_checksum`: equals the C `pitch` value.
- Fixture budget tests with no fixtures present: they pass (0 bytes).
- Spot asserts in stimulus_checksum: `aw_stimulus(44100.0).0[0] == 0.9f32`, `.1[0] == -0.5f32`, the
  frame at 6144 is 0.0, and `pitch_stimulus().len() == 11025`.

## Verification Commands (evidence)

1. `clang -std=c99 -O1 -ffp-contract=off -fno-fast-math -fno-builtin -o tmp/behringer-oss-ports/bop-00/stimulus_check tools/common/stimulus_check.c && tmp/behringer-oss-ports/bop-00/stimulus_check`. Exit 0; it prints the three hashes, which are copied into the tests.
2. `cargo build --workspace`: exit 0. `git diff --stat Cargo.lock` shows only the three new path packages.
3. `cargo nextest run -p music-things-airwindows -p music-things-soundfont -p music-things-pitch`: exit 0, with 0 failures.
4. `cargo build --workspace --target wasm32-unknown-unknown`: exit 0.
5. `cargo clippy -p music-things-airwindows -p music-things-soundfont -p music-things-pitch --all-targets -- -D warnings`: exit 0.
6. `find crates/airwindows crates/soundfont crates/pitch -name '*.rs' -print0 | xargs -0 rustfmt --edition 2021 --check`: exit 0.

## Completion Criteria

- [ ] D1-D4 files exist with the exact public names above.
- [ ] Checksum constants in both Rust tests equal the C output (verification 1 log cited).
- [ ] Verifications 2-6 exit 0, with logs under `tmp/behringer-oss-ports/bop-00/logs/`.
- [ ] Root `Cargo.toml` is unchanged (`git diff --quiet Cargo.toml`).
- [ ] Progress Log updated with the command, exit status, tests run/passed and log path.

## Progress Log

### Session: 2026-10-31
**Tasks Completed**: Plan created (not started)
**Notes**: -
