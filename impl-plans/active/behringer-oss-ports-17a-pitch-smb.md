# Behringer OSS Ports 17a: Pitch Crate, smbPitchShift Port

**Status**: Ready
**Plan ID**: bop-17a
**Design Reference**: design-docs/specs/design-behringer-oss-ports.md section C (smbPitchShift, smb int16 bridge, Parity table: smb), D7, D8
**Depends On**: bop-00 (wave W2; parallel with bop-01, bop-16a, bop-16b, bop-17b, bop-17c)
**Created**: 2026-10-31
**Last Updated**: 2026-10-31

## Intent and Context

Port `smbPitchShift.esp32.cpp` (smbPitchShift 1.2, Stephan M. Bernsee, Wide Open License) to
`music_things_pitch::smb` as a realtime-safe streaming shifter with an f32 API, and prove parity with
the original C++ through the int16 bridge pinned in the design.

Facts about the pinned upstream (verified by review):
- `#define M_PI`, `MAX_FRAME_LENGTH` (8192) and `INT_FLT 32768.0`.
- `smbPitchShiftInit(float pitchShift, long fftFrameSize, long osamp, float sampleRate)` fixes the
  shift and `calloc`s buffers sized by `fftFrameSize`.
- `smbPitchShift(size_t n, int16_t *indata, int16_t *outdata)` converts input with
  `(float)(indata[i]) / INT_FLT` and output with `(int16_t)(gOutFIFO[...] * INT_FLT)`.
- The state lives in file statics (`gRover`, `gInit`, and so on).

Context: bop-00 created `crates/pitch/src/lib.rs` (`pub mod smb;` ...), a doc-only `src/smb.rs`
(now yours), the frozen `src/error.rs` `ConfigError`, and the test twins
`tests/common/{stimulus,fnv}.rs` (`pitch_stimulus()`).

## Non-goals

- No i16 public API (D7: the API is f32), no per-call pitch shift, no extra FFT, and no "fixes" to upstream numerics.
- No edits to `src/lib.rs`, `src/error.rs`, `src/stft/`, `src/dywa.rs`, `tests/common/*`,
  `Cargo.toml`/`Cargo.lock` or other crates. No new `ConfigError` variants.

## Write Ownership (fanout contract)

- writePaths: `crates/pitch/src/smb.rs`, `crates/pitch/tests/smb_parity.rs`,
  `crates/pitch/tests/smb_no_alloc.rs`, `crates/pitch/tests/fixtures/smb` (directory),
  `tools/pitch-harness/smb` (directory: harness, generate.sh, README.md, stubs/),
  `impl-plans/active/behringer-oss-ports-17a-pitch-smb.md`, `target`, `tmp/behringer-oss-ports/bop-17a`
- sharedPaths: none
- artifactRoots (digest-only): `target`, `tmp/behringer-oss-ports/bop-17a`
- Read-only: `crates/pitch/src/lib.rs`, `crates/pitch/src/error.rs`, `crates/pitch/tests/common/*`,
  `tools/common/*` (bop-00).
- Never run git checkout/stash/reset or any other git operation that rewrites the tree.

## Common Execution Rules

- `MUSIC_THINGS_REFS=/Users/taco/gits/tacogips/music-things-refs`. Read only `smbPitchShift/` (the esp32
  source, plus any header it includes from that directory). Never open Music Tribe AGPL code.
- Evidence directory: `tmp/behringer-oss-ports/bop-17a/`. It holds `intent.md`, `pre-hashes.txt`,
  `post-hashes.txt` and `logs/`. Re-read each file before editing; on drift, stop and log it. Only edit `writePaths`.
- Siblings bop-17b and bop-17c compile this crate at the same time. If an error is in a file you do
  not own, rerun later and log it.
- Format owned files with `rustfmt --edition 2021`. Cargo env: `CARGO_TERM_QUIET=true NEXTEST_STATUS_LEVEL=fail NEXTEST_FAILURE_OUTPUT=immediate-final NEXTEST_HIDE_PROGRESS_BAR=1`.

## Deliverables

### T1. `crates/pitch/src/smb.rs`
- Header: the full upstream copyright and WOL license text, and the source path. The module docs
  state D7 and D8.
- `pub const MAX_FRAME_LENGTH: usize = 8192`.
- `pub struct SmbPitchShifter` with:
  - `pub fn new(pitch_shift: f32, fft_frame_size: usize, osamp: usize, sample_rate: f32) -> Result<Self, ConfigError>`
    ports `smbPitchShiftInit`, with buffers sized by `fft_frame_size` as upstream sizes them. Errors (D8):
    - `NotPowerOfTwo { what: "fft_frame_size", .. }`
    - `OutOfRange { what: "fft_frame_size", min: 2, max: MAX_FRAME_LENGTH, .. }`
    - `InvalidValue { what: "osamp" }` when `osamp == 0` or `fft_frame_size % osamp != 0`
  - `pub fn process(&mut self, input: &[f32], output: &mut [f32])` processes `min(len)` samples. It
    runs the upstream per-sample body without the int16 conversions: the input sample goes straight
    into the in-FIFO, and the out-FIFO value is written directly as f32.
  - `pub fn latency(&self) -> usize` (`fft_frame_size - step_size`).
  - `pub fn reset(&mut self)` zeroes the state in place, as a fresh `smbPitchShiftInit` would leave it.
- Port `smbFft` and the analysis/synthesis loops statement by statement, keeping each variable's
  float/double/long precision as declared in the esp32 source.

### T2. Harness `tools/pitch-harness/smb/smb_harness.cpp`, `tools/pitch-harness/smb/generate.sh`, `tools/pitch-harness/smb/README.md`, `tools/pitch-harness/smb/stubs/`
- The harness compiles the pinned source in place. For ESP32-only includes (for example
  `appGlobals.h`), it adds minimal stub headers written from scratch under
  `tools/pitch-harness/smb/stubs/`, which provide only the symbols the source needs.
- Per config, one process (upstream state is static):
  1. Quantize `pitch_stimulus()` with `q[i] = (int16_t)(x[i] * 32768.0)` (truncating; `|x| <= 0.5`).
  2. Call `smbPitchShiftInit(shift, frame, osamp, 44100.0f)`.
  3. Call `smbPitchShift` on 256-sample chunks.
  4. Assert `|out[i]| < 32767` for every produced int16 sample, and fail generation otherwise. Upstream
     is never edited.
  5. Write the 11025 int16 outputs as i16 LE.
- Configs: **C1** shift 1.5, frame 1024, osamp 4. **C2** shift 0.75, frame 2048, osamp 8.
- `generate.sh --work-dir DIR` compiles with `clang++ -std=c++17 -O1 -ffp-contract=off -fno-fast-math -fno-builtin -I tools/pitch-harness/smb/stubs -I tools/common`
  and writes `crates/pitch/tests/fixtures/smb/{c1.i16,c2.i16,meta.txt}`. `meta.txt` holds the
  stimulus FNV, the clang version, the host and the quantization rule.

### T3. Tests
- `crates/pitch/tests/smb_parity.rs`: `#[path = "common/stimulus.rs"] mod stimulus; #[path = "common/fnv.rs"] mod fnv;`
  For C1 and C2:
  - quantize the stimulus exactly as the harness does (`(x as f64 * 32768.0) as i16`, which truncates;
    assert it is in range)
  - feed `q as f32 / 32768.0` in 256-sample chunks
  - assert `|y as f64 * 32768.0| < 32767.0` for every Rust output `y` (with parity, this rules out
    undefined casts on the upstream side), then map each output through `(y as f64 * 32768.0).trunc()`
    to i16
  - compare with the fixture: max-abs at most 3 LSB. Record the measured LSB value (0 is expected on
    the generation host). The test also asserts the stimulus FNV from `meta.txt`.
- Also in `smb_parity.rs`:
  - `new(1.5, 1000, 4, 44100.0)` -> `NotPowerOfTwo`
  - `new(1.5, 16384, 4, 44100.0)` -> `OutOfRange`
  - `new(1.5, 1024, 0, 44100.0)` -> `InvalidValue`
  - chunk-size invariance: 64-sample chunks give f32 output bit-identical to 256-sample chunks
- `crates/pitch/tests/smb_no_alloc.rs`: a counting `#[global_allocator]`; `process` and `reset`
  allocate 0 times after `new`.

## Key Pitfalls

- Input equivalence: upstream computes `(float)q / 32768.0` in double, then stores to float. Because
  the divisor is a power of two, `q as f32 / 32768.0f32` is bit-identical. Do not scale by any other constant.
- The comparison must truncate, as the C cast does (not round), when mapping Rust output to i16.
- Mirror each libm call at its precision (`atan2`/`sin`/`cos`/`sqrt` on float vs double, as written in
  the esp32 source). `M_PI` must have the source's precision.
- The streaming FIFO indexing (`gRover`) must persist across `process` calls.
- No per-call allocation.

## Tests (situation -> expected)

- C1/C2 in the int16 domain vs the fixtures -> max-abs <= 3 LSB.
- The invalid configs above -> the stated `ConfigError` variants.
- Chunk 64 vs chunk 256 -> bit-identical f32.
- No-alloc -> 0.

## Verification Commands (evidence)

1. `MUSIC_THINGS_REFS=/Users/taco/gits/tacogips/music-things-refs tools/pitch-harness/smb/generate.sh --work-dir tmp/behringer-oss-ports/bop-17a/harness`: exit 0, writing c1.i16 and c2.i16 (22050 bytes each).
2. `cargo nextest run -p music-things-pitch --test smb_parity --test smb_no_alloc`: exit 0, 0 failures.
3. `cargo clippy -p music-things-pitch --all-targets -- -D warnings`: exit 0 (errors in files you do not own are logged).
4. `rustfmt --edition 2021 --check crates/pitch/src/smb.rs crates/pitch/tests/smb_parity.rs crates/pitch/tests/smb_no_alloc.rs`: exit 0.
5. `cargo build -p music-things-pitch --target wasm32-unknown-unknown`: exit 0.

## Completion Criteria

- [ ] T1-T3 delivered with the pinned API (shift at construction, f32 I/O), with D7/D8 and the WOL notice in the header.
- [ ] Int16-domain parity within 3 LSB, with the measured value recorded.
- [ ] Verifications 1-5 exit 0, with logs in `tmp/behringer-oss-ports/bop-17a/logs/`.
- [ ] Only `writePaths` are modified, and every file is under 1000 lines.
- [ ] Progress Log updated with commands, exit status, tests run/passed and log paths.

## Progress Log

### Session: 2026-10-31
**Tasks Completed**: Plan created; revised for IPR-1 (int16 upstream interface, D7/D8)
**Notes**: -
