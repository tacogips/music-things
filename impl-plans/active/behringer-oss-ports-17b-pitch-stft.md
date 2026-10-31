# Behringer OSS Ports 17b: Pitch Crate, stftPitchShift Core Port

**Status**: Ready
**Plan ID**: bop-17b
**Design Reference**: design-docs/specs/design-behringer-oss-ports.md section C (stftPitchShift, rfft rule, asymmetric framesize, Parity table: stft offline/stream), D2, D8
**Depends On**: bop-00 (wave W2; parallel with bop-01, bop-16a, bop-16b, bop-17a, bop-17c)
**Created**: 2026-10-31
**Last Updated**: 2026-10-31

## Intent and Context

Port the float instantiation of the jurihock/stftPitchShift C++ library core
(@ `70c96db087458f8631edbc1bb4068d42ea5b52cc`, MIT, `cpp/StftPitchShift/`) to
`music_things_pitch::stft`:
- `RFFT.h` and the `FFT.h` interface: the power-of-two real FFT that upstream instantiates with
  `std::make_shared<RFFT>()`
- the STFT framing, Vocoder, Pitcher, Resampler, Cepster (formant lifter), normalization, distortion,
  and multi-factor shifting
- the offline `StftPitchShift::shiftpitch` entry point

Also add the realtime streaming wrapper `StftPitchShiftStream` (D2: an addition with no upstream
equivalent). Prove offline parity with the upstream C++.

Context: bop-00 created `crates/pitch/src/stft/mod.rs` as a doc-only file (now yours), plus the frozen
`ConfigError` and the test twins `tests/common/{stimulus,fnv}.rs`.

## Non-goals

- No other FFT, and no dependency (no pocketfft). No `chronometry`, no file I/O, no CLI.
- No validation rules beyond upstream's, except `hopsize > 0` (D8). In particular, there is no
  "hop divides framesize" rule.
- No edits to `src/lib.rs`, `src/error.rs`, `src/smb.rs`, `src/dywa.rs`, `tests/common/*`,
  `Cargo.toml`/`Cargo.lock` or other crates.

## Write Ownership (fanout contract)

- writePaths: `crates/pitch/src/stft/mod.rs`, `crates/pitch/src/stft/rfft.rs`, `crates/pitch/src/stft/core.rs`,
  `crates/pitch/src/stft/stft.rs`, `crates/pitch/src/stft/vocoder.rs`, `crates/pitch/src/stft/pitcher.rs`,
  `crates/pitch/src/stft/resampler.rs`, `crates/pitch/src/stft/cepster.rs`, `crates/pitch/src/stft/stream.rs`,
  `crates/pitch/tests/stft_parity.rs`, `crates/pitch/tests/stft_stream.rs`, `crates/pitch/tests/stft_no_alloc.rs`,
  `crates/pitch/tests/fixtures/stft` (directory), `tools/pitch-harness/stft` (directory),
  `impl-plans/active/behringer-oss-ports-17b-pitch-stft.md`, `target`, `tmp/behringer-oss-ports/bop-17b`
- sharedPaths: none
- artifactRoots (digest-only): `target`, `tmp/behringer-oss-ports/bop-17b`
- Read-only: `crates/pitch/src/lib.rs`, `crates/pitch/src/error.rs`, `crates/pitch/tests/common/*`,
  `tools/common/*` (bop-00).
- Never run git checkout/stash/reset or any other git operation that rewrites the tree.

## Common Execution Rules

- `MUSIC_THINGS_REFS=/Users/taco/gits/tacogips/music-things-refs`. Read only `stftPitchShift/cpp/StftPitchShift/*`.
  Never open Music Tribe AGPL code.
- Evidence directory: `tmp/behringer-oss-ports/bop-17b/`. It holds `intent.md`, `pre-hashes.txt`,
  `post-hashes.txt` and `logs/`. Re-read each file before editing; on drift, stop and log it. Only edit `writePaths`.
- Siblings compile this crate at the same time. If an error is in a file you do not own, rerun later and log it.
- Format owned files with `rustfmt --edition 2021`. Cargo env: `CARGO_TERM_QUIET=true NEXTEST_STATUS_LEVEL=fail NEXTEST_FAILURE_OUTPUT=immediate-final NEXTEST_HIDE_PROGRESS_BAR=1`.

## Deliverables

### T1. `crates/pitch/src/stft/{mod.rs,rfft.rs,core.rs,stft.rs,vocoder.rs,pitcher.rs,resampler.rs,cepster.rs,stream.rs}`
- Each file mirrors one upstream header or source and carries its MIT copyright line, the upstream
  path and the revision.
- `rfft.rs` ports `RFFT.h` with the same operation order. `std::complex<float>` arithmetic and
  `std::polar` are reproduced with explicit (re, im) f32 math that follows the libc++ formulas. Size
  validation follows upstream: both the full size and the half size must be powers of two
  (`std::has_single_bit`), so size 1 is rejected. Failure is
  `ConfigError::NotPowerOfTwo { what: "framesize", value }`.
- `mod.rs` public API:
  - `pub struct StftConfig { pub analysis_framesize: usize, pub synthesis_framesize: usize, pub hopsize: usize, pub sample_rate: f32, pub quefrency: f32, pub distortion: f32, pub normalization: bool, pub max_factors: usize }`.
    This mirrors upstream's `std::tuple<size_t,size_t>` framesize, and `StftConfig::symmetric(framesize, hopsize, sample_rate)`
    sets both sizes equal (the other fields default to quefrency 0, distortion 1, normalization off and max_factors 1).
    Scalars are converted to the precision upstream uses (for example `f64`) exactly once, at configuration time.
  - Port the asymmetric analysis/synthesis window path of `STFT.h` exactly (separate analysis and
    synthesis windows; upstream's check that analysis >= synthesis).
  - `pub struct StftPitchShift` with:
    - `new(config: StftConfig) -> Result<Self, ConfigError>`. Validation is exactly upstream's: the
      RFFT size rule, analysis >= synthesis (`InvalidValue { what: "framesize" }`), and any other check
      that upstream throws on, plus `hopsize > 0` (`InvalidValue { what: "hopsize" }`, D8).
      `max_factors` must be at least 1. All buffers are allocated here, sized by `max_factors`.
    - `set_factors(&mut self, factors: &[f32]) -> Result<(), ConfigError>` returns
      `TooManyFactors` when `len > max_factors`, and `InvalidValue` when the slice is empty.
    - `shift_pitch(&mut self, input: &[f32], output: &mut [f32])`, which mirrors upstream
      `shiftpitch`, including its framing, padding and normalization behavior.
- `stream.rs`: `pub struct StftPitchShiftStream` with `new(StftConfig)`, `set_factors`,
  `process(&mut self, input: &[f32], output: &mut [f32])` and `latency(&self) -> usize`. It reuses the
  same core and holds fixed ring buffers allocated in `new`. Its docs state the latency.

### T2. Harness `tools/pitch-harness/stft/stft_harness.cpp`, `tools/pitch-harness/stft/generate.sh`, `tools/pitch-harness/stft/README.md`
- The harness compiles the pinned `cpp/StftPitchShift/*.cpp` in place with
  `clang++ -std=c++20 -O1 -ffp-contract=off -fno-fast-math -fno-builtin -I $MUSIC_THINGS_REFS/stftPitchShift/cpp`
  (C++20 because the core includes `<bit>` and `<span>` and uses `std::has_single_bit`).
- It reads `pitch_stimulus()` from `tools/common/stimulus.h` and calls the upstream float `shiftpitch`:
  - **C1**: framesize 1024, hop 256, sr 44100, factors {1.5}, quefrency 0.001, distortion 1, normalization off.
  - **C2**: framesize 1024, hop 256, factors {0.75, 1.25}, quefrency 0, normalization on.
  - **C3** (asymmetric): the tuple constructor with analysis 1024 and synthesis 512, hop 128, factors
    {1.25}, quefrency 0, normalization off. If upstream rejects this combination, choose the nearest
    accepted asymmetric one, and record it in `meta.txt` and the Progress Log.
- `generate.sh --work-dir DIR` writes `crates/pitch/tests/fixtures/stft/{c1.f32,c2.f32,c3.f32,meta.txt}`, all
  11025 samples per config, f32 LE. `meta.txt` holds the stimulus FNV, the clang version and the host.

### T3. Tests
- `crates/pitch/tests/stft_parity.rs`: C1, C2 and C3 offline against the fixtures, with max-abs at
  most 1e-5. Record the measured value (0 is expected on the generation host). The test also asserts
  the stimulus FNV, that `new` with framesize 1000 or 1 returns `NotPowerOfTwo`, that analysis below
  synthesis returns `InvalidValue`, that hopsize 0 returns `InvalidValue`, and that hop 384 with
  framesize 1024 (not a divisor) is accepted when upstream accepts it.
- `crates/pitch/tests/stft_stream.rs`: for C1, C2 and C3 (framesize meaning the analysis size), `StftPitchShiftStream` fed in 128-sample chunks
  must give `stream[n + latency] == offline[n]` within max-abs 1e-6, for n in
  `[framesize, len - 2*framesize - latency)`.
- `crates/pitch/tests/stft_no_alloc.rs`: a counting `#[global_allocator]`. After `new`, `set_factors`
  (within `max_factors`), `shift_pitch` (on a preallocated output) and stream `process` all allocate 0 times.

## Key Pitfalls

- libc++ `std::complex` multiplication and division have specific formulas (and NaN recovery paths).
  Reproduce the finite path exactly, and do not use a "simplified" formula that changes rounding.
- `std::arg`/`std::abs` on complex values map to `atan2` and `hypot`. Use `f32::atan2` and `f32::hypot`
  for float, matching upstream precision.
- Cepster/quefrency: convert quefrency in seconds to samples exactly as upstream does (same rounding function).
- Multi-factor pitching sums shifted spectra in the order of the factor list. Keep the order.
- `shift_pitch` must not allocate on calls after construction (buffers sized in `new` from framesize,
  hop and `max_factors`). Inputs longer than the internal frame buffers are processed frame by frame,
  as upstream does.

## Tests (situation -> expected)

- C1/C2 offline vs the fixtures -> max-abs <= 1e-5.
- Stream vs offline (delayed by latency) -> within 1e-6 on the interior region.
- `new(framesize 1)` -> `Err(NotPowerOfTwo)`. `set_factors(&[])` -> `Err(InvalidValue)`.
  `set_factors` with 3 factors when `max_factors` is 2 -> `Err(TooManyFactors)`.
- No-alloc -> 0.

## Verification Commands (evidence)

1. `MUSIC_THINGS_REFS=/Users/taco/gits/tacogips/music-things-refs tools/pitch-harness/stft/generate.sh --work-dir tmp/behringer-oss-ports/bop-17b/harness`: exit 0.
2. `cargo nextest run -p music-things-pitch --test stft_parity --test stft_stream --test stft_no_alloc`: exit 0, 0 failures.
3. `cargo clippy -p music-things-pitch --all-targets -- -D warnings`: exit 0.
4. `rustfmt --edition 2021 --check crates/pitch/src/stft/*.rs crates/pitch/tests/stft_parity.rs crates/pitch/tests/stft_stream.rs crates/pitch/tests/stft_no_alloc.rs`: exit 0.
5. `cargo build -p music-things-pitch --target wasm32-unknown-unknown`: exit 0.

## Completion Criteria

- [ ] T1-T3 delivered with the pinned API, an RFFT port (no other FFT), and C++20 harness flags.
- [ ] Offline parity within 1e-5 and stream consistency within 1e-6, with measured values recorded.
- [ ] Verifications 1-5 exit 0, with logs in `tmp/behringer-oss-ports/bop-17b/logs/`.
- [ ] Only `writePaths` are modified, and every file is under 1000 lines.
- [ ] Asymmetric frame sizes ported and covered by C3; validation equals upstream plus D8 only.
- [ ] Progress Log updated with commands, exit status, tests run/passed and log paths.

## Progress Log

### Session: 2026-10-31
**Tasks Completed**: Plan created (not started)
**Notes**: -
