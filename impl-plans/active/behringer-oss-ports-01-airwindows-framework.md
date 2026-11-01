# Behringer OSS Ports 01: Airwindows Framework, Harness and Batch-1 Fixtures

**Status**: In Progress
**Plan ID**: bop-01
**Design Reference**: design-docs/specs/design-behringer-oss-ports.md section A (Public API, Framework internals, Additive registration, Batch-1 grouping, Porting rules, Parity method, Tolerances), D1/D5/D6
**Depends On**: bop-00 (wave W2; runs in parallel with bop-16a, bop-16b, bop-17a/b/c)
**Created**: 2026-10-31
**Last Updated**: 2026-10-31

## Intent and Context

This plan builds the frozen Airwindows framework that all 110 batch-1 plugins (and later batches 2/3)
plug into: trait, metadata, seeded fpd dither, AwRand, display helpers, and the factory/catalog over
additive per-batch/per-group registration files. It also builds the MIT reference harness (VST shim,
prelude, driver, generator) and generates the committed parity fixtures for all 110 batch-1 plugins.
After this plan, the 14 group plans (bop-02..bop-15) only add plugin files, group registrations and
group parity tests.

Existing patterns to imitate:
- `crates/music-things/src/lib.rs:1` for crate docs.
- bop-00's `crates/airwindows/tests/common/stimulus.rs` and `fnv.rs`, which must be used, not copied.
- `tools/common/stimulus.h` and `tools/common/fnv.h` from bop-00, which the driver `#include`s.

## Non-goals

- No plugin ports. Every `gNN/mod.rs` has `ENTRIES = &[]`.
- No `processDoubleReplacing`, chunks or programs, and no public concrete plugin types.
- No edits to `tools/common/*`, the stimulus twins, `Cargo.lock`, the root `Cargo.toml` or other crates.
- Do not copy upstream C++ into the repo. The harness compiles it in place from `$MUSIC_THINGS_REFS`.

## Write Ownership (fanout contract)

Concrete repository-relative paths (no globs). A directory entry owns every file under it.

- writePaths:
  - `crates/airwindows/src/lib.rs` (replaces the bop-00 doc-only file), `crates/airwindows/src/effect.rs`,
    `crates/airwindows/src/params.rs`, `crates/airwindows/src/dither.rs`, `crates/airwindows/src/rand.rs`,
    `crates/airwindows/src/display.rs`, `crates/airwindows/src/registry.rs`
  - `crates/airwindows/src/plugins/mod.rs`, `crates/airwindows/src/plugins/batch1/mod.rs`,
    `crates/airwindows/src/plugins/batch2/mod.rs`, `crates/airwindows/src/plugins/batch3/mod.rs`
  - `crates/airwindows/src/plugins/batch1/g01/mod.rs` ... `crates/airwindows/src/plugins/batch1/g14/mod.rs`
    (14 concrete files, g01 through g14; after this plan each belongs to its group plan)
  - `crates/airwindows/tests/common/mod.rs`, `crates/airwindows/tests/common/parity.rs`,
    `crates/airwindows/tests/common/fixture.rs`, `crates/airwindows/tests/catalog.rs`,
    `crates/airwindows/tests/no_alloc.rs`, `crates/airwindows/tests/fixture_budget.rs`
  - `crates/airwindows/tests/fixtures/shim_format_vectors.txt`, `crates/airwindows/tests/fixtures/batch1`
    (directory; 14 group dirs, 220 files)
  - `tools/vst-shim/audioeffectx.h`, `tools/vst-shim/audioeffectx.cpp`, `tools/vst-shim/LICENSE-NOTE.md`
  - `tools/airwindows-harness/prelude.h`, `tools/airwindows-harness/driver.cpp`,
    `tools/airwindows-harness/generate.sh`, `tools/airwindows-harness/format_vectors.cpp`,
    `tools/airwindows-harness/README.md`, `tools/README.md`
  - `impl-plans/active/behringer-oss-ports-01-airwindows-framework.md`, `target`, `tmp/behringer-oss-ports/bop-01`
- sharedPaths: none
- artifactRoots (digest-only): `target`, `tmp/behringer-oss-ports/bop-01`
- Read-only: `tools/common/*`, `crates/airwindows/tests/common/stimulus.rs`, `crates/airwindows/tests/common/fnv.rs` (bop-00).
- Never run git checkout/stash/reset or any other git operation that rewrites the tree.

## Common Execution Rules

- `MUSIC_THINGS_REFS=/Users/taco/gits/tacogips/music-things-refs`. Read only
  `airwindows/plugins/LinuxVST/src/<Upstream>/`, the airwindows `what.txt` and `kick-airwindows-set.txt`.
  Never open Music Tribe AGPL code.
- Evidence directory: `tmp/behringer-oss-ports/bop-01/`. It holds `intent.md` (an immutable copy of the
  Deliverables), `pre-hashes.txt` and `post-hashes.txt` (`shasum -a 256` of owned files) and `logs/`.
  Re-read each file before editing it, and stop and log any drift. Only edit `writePaths`.
- Format only owned files with `rustfmt --edition 2021`. Never run a workspace-wide `cargo fmt`.
- Cargo env: `CARGO_TERM_QUIET=true NEXTEST_STATUS_LEVEL=fail NEXTEST_FAILURE_OUTPUT=immediate-final NEXTEST_HIDE_PROGRESS_BAR=1`.
- Sibling W2 plans build other crates in the same tree. If a build error is in a file you do not own,
  rerun later and log it; do not edit that file.

## Deliverables

### T1. Public API (`src/lib.rs`, `src/effect.rs`, `src/params.rs`)
- `lib.rs`: crate docs (port provenance, the D1/D5/D6 summary), `mod dither; mod display; mod effect;
  mod params; mod plugins; mod rand; mod registry;`, plus
  `pub use effect::AirwindowsEffect; pub use params::{ParamInfo, PluginInfo}; pub use registry::{catalog, create, create_with_seed, DEFAULT_SEED};`.
- `effect.rs`, which pins the trait signature:
  `pub trait AirwindowsEffect: Send { fn info(&self) -> &'static PluginInfo; fn param_count(&self) -> usize /*default: info().params.len()*/; fn param_info(&self, index: usize) -> Option<&'static ParamInfo> /*default*/; fn get_param(&self, index: usize) -> f32; fn set_param(&mut self, index: usize, value: f32); fn param_display(&self, index: usize) -> String; fn param_label(&self, index: usize) -> &'static str /*default: ParamInfo.label or ""*/; fn reset(&mut self); fn process(&mut self, in_l: &[f32], in_r: &[f32], out_l: &mut [f32], out_r: &mut [f32]); }`.
  The docs state that `process` handles `min` of the four lengths, never allocates, and mirrors
  upstream `processReplacing`.
- `params.rs`: `#[derive(Debug, Clone, Copy, PartialEq)] pub struct ParamInfo { pub name: &'static str, pub label: &'static str, pub default: f32 }`;
  `#[derive(Debug, Clone, Copy, PartialEq)] pub struct PluginInfo { pub name: &'static str, pub upstream_name: &'static str, pub description: &'static str, pub params: &'static [ParamInfo] }`;
  `pub(crate) fn sanitize_param(value: f32) -> Option<f32>` returns None for NaN and otherwise clamps
  to [0,1]. Plugins call it in `set_param`.

### T2. Determinism helpers (`src/dither.rs`, `src/rand.rs`)
- `pub(crate) fn fpd_from_seed(seed: u32, lane: u32) -> u32`:
  `x = seed.wrapping_add(lane.wrapping_mul(0x9E37_79B9))`, then lowbias32
  (`x ^= x >> 16; x = x.wrapping_mul(0x7feb_352d); x ^= x >> 15; x = x.wrapping_mul(0x846c_a68b); x ^= x >> 16`),
  then `if x < 16386 { x += 16386 }`.
- `pub(crate) fn xorshift(fpd: &mut u32)`: `^= <<13; ^= >>17; ^= <<5`.
- `pub(crate) fn frexpf_exponent(x: f32) -> i32`: C `frexpf` exponent from the bits. Zero gives 0;
  subnormals are normalized; normal numbers give `biased - 126`. Inf and NaN give 0.
- `pub(crate) fn pow2(e: i32) -> f64`: exact (`f64::from_bits(((e + 1023) as u64) << 52)`), with a
  debug_assert that the result is in the normal range.
- `pub(crate) fn fpd_dither(sample: f64, fpd: &mut u32) -> f64` is the canonical upstream float-path
  block, in this order: `expon = frexpf_exponent(sample as f32)`, then `xorshift(fpd)`, then
  `sample + ((*fpd as f64 - 2147483647.0) * 5.5e-36) * pow2(expon + 62)`. Multiplication goes left to right.
- `rand.rs`: `pub(crate) const RAND_MAX: u32 = 0x7fff_ffff`;
  `pub(crate) struct AwRand { state: u32 }` with `new(seed: u32)` (state = `fpd_from_seed(seed, 2)`)
  and `next(&mut self) -> u32` (`xorshift(&mut state)`, then return `state >> 1`).
  Also `pub(crate) fn replay_fpd_seed_loop(rng: &mut AwRand) -> u32`, which replays upstream's
  `fpd = 1; while (fpd < 16386) fpd = rand()*UINT32_MAX;` (u32 `wrapping_mul(u32::MAX)`) and returns
  the value, which the caller discards.

### T3. Display helpers (`src/display.rs`, clean-room spec D1, shared byte for byte with the shim)
- `pub(crate) fn float2string(v: f32) -> String`. The algorithm runs in f64 with `x = v as f64`:
  - NaN gives "nan". If `|x| >= 1e8`, return "Huge!".
  - Sign "-" when `x < 0`. Let `a = |x|`, integer part `ip = floor(a)`, and `id` = its decimal digit
    count (1 when ip == 0). Emit the digits of ip.
  - If `id < 8`, emit "." and then `8 - id` fraction digits by repeating `f = a - floor(a)`,
    `f *= 10.0; d = floor(f); f -= d`. This truncates and never rounds.
  - Examples: 0.5 -> "0.5000000", 12.25 -> "12.250000", -3.0 -> "-3.0000000".
- `pub(crate) fn db2string(v: f32) -> String`: if `v <= 0.0`, return "-oo"; otherwise
  `float2string((20.0 * (v as f64).log10()) as f32)`.
- `pub(crate) fn int2string(v: i32) -> String`: "Huge!" if `v >= 100_000_000` or `v <= -10_000_000`,
  otherwise the decimal string.
- `#[cfg(test)] mod tests` reads `include_str!("../tests/fixtures/shim_format_vectors.txt")`. The
  lines are `f2s <f32 bits hex> <string>`, `db <f32 bits hex> <string>` and `i2s <i32> <string>`
  (string to end of line). Every Rust result must equal the shim's.

### T4. Registration (`src/registry.rs`, `src/plugins/**/mod.rs`)
- `registry.rs`: `pub(crate) struct PluginEntry { pub(crate) info: &'static PluginInfo, pub(crate) construct: fn(f32, u32) -> Box<dyn AirwindowsEffect> }`;
  `pub const DEFAULT_SEED: u32 = 0x00C0_FFEE`;
  `pub fn create(name: &str, sample_rate: f32) -> Option<Box<dyn AirwindowsEffect>>`;
  `pub fn create_with_seed(name: &str, sample_rate: f32, seed: u32) -> Option<Box<dyn AirwindowsEffect>>`;
  `pub fn catalog() -> impl Iterator<Item = &'static PluginInfo>`. These iterate
  `plugins::BATCHES -> groups -> entries` in order, and match `name` or `upstream_name` exactly.
- `plugins/mod.rs`: `mod batch1; mod batch2; mod batch3; pub(crate) const BATCHES: &[&[&[PluginEntry]]] = &[batch1::GROUPS, batch2::GROUPS, batch3::GROUPS];`
- `plugins/batch1/mod.rs`: `mod g01; ... mod g14;` and `pub(crate) const GROUPS: &[&[PluginEntry]] = &[g01::ENTRIES, ..., g14::ENTRIES];`.
  `batch2/mod.rs` and `batch3/mod.rs` hold `pub(crate) const GROUPS: &[&[PluginEntry]] = &[];` with
  a doc line saying they are owned by the batch-2 and batch-3 runs.
- `plugins/batch1/gNN/mod.rs` (14 files): `//!` doc stating the lines (g01 = 1-8, ..., g13 = 97-104,
  g14 = 105-110), `use crate::registry::PluginEntry;`, `pub(crate) const ENTRIES: &[PluginEntry] = &[];`.
- Pinned per-plugin module contract (documented in `plugins/mod.rs` docs for the group plans): each
  plugin file defines a `pub(crate) const INFO: PluginInfo`, a `pub(crate) const ENTRY: PluginEntry`
  (built as `PluginEntry { info: &INFO, construct }`) and `fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect>`.
  Use `const`, not `static`, and arrays longer than 4096 become `Box<[T]>`.

### T5. Test infrastructure (`tests/common/{mod,fixture,parity}.rs`, `tests/{catalog,no_alloc,fixture_budget}.rs`)
- `tests/common/mod.rs`: `#![allow(dead_code)] pub mod fixture; pub mod fnv; pub mod parity; pub mod stimulus;`
  (fnv and stimulus come from bop-00).
- `fixture.rs` parses the metadata format below into `Fixture { name, upstream, description, stimulus_fnv_44100: u64, stimulus_fnv_96000: u64, params: Vec<FixtureParam>, renders: [Vec<f32>; 3] }`.
  `FixtureParam` has name, default bits, label_r0, display_r0, label_r1 and display_r1. The binary
  splits into 3 x 504 x 2 f32 LE values.
- `parity.rs` pins the API the group tests call:
  `pub enum ParityClass { Exact, Libm, Custom(f32) }`;
  `pub struct ParityRecord { pub name: &'static str, pub class: ParityClass, pub measured_max_abs: f32, pub note: &'static str }`;
  `pub fn check_plugin(record: &ParityRecord, meta: &str, data: &[u8])`. It must:
  - assert the stimulus checksums against `stimulus::aw_stimulus`
  - `create_with_seed(name, sr, 0x00C0FFEE)` for each render
  - assert the info name and upstream name, the description (exact), the parameter count, names and
    default bits, plus labels and displays at R0 (defaults) and after setting `ALT[i % 9]` (R1)
  - render 8192 frames in 128-frame blocks
  - compare the stored frames: indices 0..256, then `256 + 32*k` for k in 0..248
  - for Exact, compare bits; for Libm, require max-abs <= 1e-6; for Custom(x), require max-abs <= x
  - call `reset()`, re-render R0, and require bit-identity with the first R0 render
  - name the render, frame, channel, expected and actual values in every panic message
  - `ALT = [0.30, 0.85, 0.15, 0.65, 0.95, 0.45, 0.05, 0.70, 0.25]`.
  `Custom(x)` with `x <= 1e-6` or an empty `note` must panic, which enforces the design.
- `catalog.rs`: names are unique across `name` and `upstream_name`; defaults are in [0,1];
  `param_count == params.len()`; descriptions are non-empty; `create` and `create_with_seed` succeed for
  every entry; an unknown name gives None; an `upstream_name` lookup works. It passes with 0 entries.
- `no_alloc.rs`: a `#[global_allocator]` that counts calls. For every catalog entry it constructs the
  effect, processes one 128-frame warm-up block and then a measured block, and asserts the measured
  allocation count is 0. It also checks `set_param`, `get_param` and `reset`. It passes with 0 entries.
- `fixture_budget.rs`: the recursive size of `tests/fixtures` is at most 1_600_000 bytes.

### T6. VST shim (`tools/vst-shim/audioeffectx.h`, `tools/vst-shim/audioeffectx.cpp`, `tools/vst-shim/LICENSE-NOTE.md`)
- An MIT header saying it was written from scratch with no Steinberg code. It provides only what the
  110 batch-1 plugins use: `VstInt32` and friends, `audioMasterCallback`, the `AudioEffect`/`AudioEffectX`
  constructor `(audioMaster, numPrograms, numParams)`, setter no-ops, `getSampleRate()` (a float
  member), `setSampleRate`, `vst_strncpy`/`vst_strncat` with `kVstMaxParamStrLen = 64`, the category
  enums, and `float2string`/`dB2string`/`int2string` implementing the T3 spec exactly.
- `AudioEffect`'s constructor initializes `sampleRate` from the global `aw_shim_initial_sample_rate`,
  which the driver sets before construction. That way a constructor that reads `getSampleRate()`
  sees the render rate.
- Extend it only when the T8 smoke build shows a missing symbol.

### T7. Harness (`tools/airwindows-harness/{prelude.h,driver.cpp,generate.sh,format_vectors.cpp,README.md}`, `tools/README.md`)
- `prelude.h` (passed with `-include`): include `<cstdlib> <cmath> <cstring> <cstdio> <cstdint>
  <string> <type_traits>` first, then `static_assert(sizeof(long double) == sizeof(double))`, then
  `aw_harness_srand(uint32_t)`/`aw_harness_rand()` (the C twin of AwRand), then `#define rand() aw_harness_rand()`,
  `#undef RAND_MAX`, `#define RAND_MAX 0x7fffffff`, and finally `#define private public` and
  `#define protected public`. The defines go after the std headers so libc++ is unaffected.
- `driver.cpp`, compiled with `-DAW_CLASS=<Upstream>` and `-DAW_HEADER="<Upstream>.h"`. For each render
  (R0: 44100/defaults, R1: 44100/ALT, R2: 96000/ALT):
  1. Set `aw_shim_initial_sample_rate`, then `aw_harness_srand(0x00C0FFEE)`.
  2. Construct `new AW_CLASS(nullptr)` and `setSampleRate(sr)`.
  3. Override fpd after construction. If members `fpdL`/`fpdR` exist (detected with a SFINAE trait),
     set them to `fpd_from_seed(seed, 0/1)`. If only a single `fpd` exists, use lane 0.
  4. Set parameters and render the `aw_stimulus` (from `tools/common/stimulus.h`) in 128-frame
     `processReplacing` blocks. Keep the stored frames.
  Then write `<Name>.f32` and `<Name>.txt`.
  Metadata format `aw-fixture-v1` uses `key=value` lines:
  - header keys: `format`, `name`, `upstream`, `host`, `clang`, `stimulus_fnv_44100`,
    `stimulus_fnv_96000`, `seed`, `block=128`, `frames=8192`, `description`
  - the description escapes `\\` and `\n`
  - `params=<N>`, then per parameter `param.<i>.name`, `param.<i>.default` (f32 bits hex),
    `param.<i>.label_r0`, `param.<i>.display_r0`, `param.<i>.label_r1`, `param.<i>.display_r1`
- `generate.sh --work-dir DIR --out-root DIR [--group gNN] [NAME...]`:
  - requires `MUSIC_THINGS_REFS` and fails clearly when it is missing
  - maps kick-list line L to group `g$(( (L-1)/8 + 1 ))` zero-padded, for lines 1-110 only
  - maps KICK `Console8BusOut` to upstream `Console8BussOut`
  - compiles `<Upstream>.cpp`, `<Upstream>Proc.cpp`, `driver.cpp` and `audioeffectx.cpp` with
    `clang++ -std=c++17 -O1 -ffp-contract=off -fno-fast-math -fno-builtin -include prelude.h -I tools/vst-shim -I tools/common`
  - writes `<out-root>/batch1/gNN/<Name>.{txt,f32}`, and exits non-zero on the first failure
  - extracts the `what.txt` entry with a rule you determine from the pinned file's real format and
    document in the README. The description is written verbatim (whitespace trimmed at both ends).
- `format_vectors.cpp` writes `crates/airwindows/tests/fixtures/shim_format_vectors.txt`. It covers
  about 40 values: 0, 0.5, 1, -1, 1e-7, 0.123456789, 12.25, 99999999, 1e8, -1e7, NaN, and dB values
  for 0, 1e-6, 0.5, 1 and 2, plus int edge cases.
- `README.md` covers prerequisites, commands, the flags with their rationale (no FMA, no builtin
  folding, long double == double), the what.txt rule, and the escalation rule (group plans never edit
  tools/; they report to bop-01). `tools/README.md` indexes `common/`, `vst-shim/`, `airwindows-harness/`,
  `soundfont-harness/` (`generate_tml.sh`, `generate_font.sh`, `generate_synth.sh`) and
  `pitch-harness/{smb,stft,dywa}/generate.sh`.

### T8. Generate all batch-1 fixtures
- Run `tools/airwindows-harness/generate.sh --work-dir tmp/behringer-oss-ports/bop-01/harness --out-root crates/airwindows/tests/fixtures`
  with no names, which means all of lines 1-110. All 110 must build and run. Fix shim gaps in T6.
- Expected output: 220 files in `crates/airwindows/tests/fixtures/batch1/g01..g14/`, each `.f32`
  exactly 12096 bytes, about 1.45 MB in total.

## Key Pitfalls

- `#define private public` before the std headers breaks libc++. The ordering in the prelude is mandatory.
- Upstream `rand()` returns `int`. `rand()*UINT32_MAX` is evaluated as unsigned 32-bit wrapping
  multiplication, and the loop's iteration count depends on the values drawn. Replay it exactly.
- The fpd override must happen after construction and must not consume `aw_harness_rand()`.
- `float2string` must truncate and must not use printf rounding. Use the same digit loop in C and Rust.
- A `Box<dyn ...>` with huge inline arrays overflows the stack in debug builds, hence the `Box<[T]>` rule.
- Do not `#include` the plugin `.cpp` into the driver. Compile it as its own translation unit (each
  plugin defines `createEffectInstance`), and build one executable per plugin.
- Keep every Rust file under 1000 lines, and clippy-clean with `-D warnings`.

## Tests (situation -> expected)

- `fpd_from_seed(0, 0)` returns a value of at least 16386; distinct lanes give distinct values; it is deterministic.
- `frexpf_exponent`: 1.0 -> 1, 0.5 -> 0, 0.0 -> 0, `f32::MIN_POSITIVE` -> -125, the smallest subnormal -> -148.
- `fpd_dither(0.25, &mut fpd)` changes the value by less than 1e-15 relative and advances fpd.
- `float2string(0.5) == "0.5000000"`; `db2string(0.0) == "-oo"`; `int2string(100_000_000) == "Huge!"`.
- The display unit test matches every line of `shim_format_vectors.txt`.
- `create("NoSuchPlugin", 44100.0)` is None; `catalog().count() == 0` at the end of this plan.
- `parity::check_plugin` with `Custom(1e-7)` panics (a unit test in `tests/catalog.rs` using a fake record is enough).

## Verification Commands (evidence)

1. `MUSIC_THINGS_REFS=/Users/taco/gits/tacogips/music-things-refs tools/airwindows-harness/generate.sh --work-dir tmp/behringer-oss-ports/bop-01/harness --out-root crates/airwindows/tests/fixtures`: exit 0, with 110 plugins built and run.
2. `find crates/airwindows/tests/fixtures/batch1 -name '*.f32' | wc -l` prints 110, and every `.f32` is 12096 bytes (`find ... -size -12096c` is empty).
3. `cargo nextest run -p music-things-airwindows`: exit 0, 0 failures (unit tests, catalog, no_alloc, fixture_budget, stimulus_checksum).
4. `cargo build -p music-things-airwindows --target wasm32-unknown-unknown`: exit 0.
5. `cargo clippy -p music-things-airwindows --all-targets -- -D warnings`: exit 0.
6. `find crates/airwindows -name '*.rs' -print0 | xargs -0 rustfmt --edition 2021 --check`: exit 0.
7. `find crates/airwindows/tests/fixtures -type f -exec stat -f %z {} + | awk '{s+=$1} END {print s}'`: prints
   at most 1600000 (apparent bytes). This is the same metric `tests/fixture_budget.rs` uses (the sum of
   file lengths), and that test remains the authoritative gate. Do not use `du`: it counts allocated
   4 KB APFS blocks, so the 221 correct fixture files would report about 1760 KB and fail falsely. Do not
   shrink fixtures or change the R0-R2 renders or the 504-frame layout to fit a block count. Record the
   measured byte total (and the .f32/.txt split) in the Progress Log; the .f32 files alone are 1,330,560
   bytes, so the .txt files have about 2.4 KB each on average.

## Completion Criteria

- [x] T1-T7 symbols exist exactly as pinned, and `cargo doc -p music-things-airwindows --no-deps` builds.
- [x] All 110 batch-1 plugins compile and run in the harness, and 220 fixture files exist under `batch1/gNN/`.
- [x] Verifications 1-7 pass with logs in `tmp/behringer-oss-ports/bop-01/logs/`.
- [x] No file outside this plan's `writePaths` is modified (`git status` checked).
- [x] Progress Log updated with commands, exit status, tests run/passed and log paths.

## Progress Log

### Session: 2026-10-31
**Tasks Completed**: Plan created (not started)
**Notes**: -

### Session: 2026-10-31 — implementation
**Tasks Completed**: T1–T8; formal reviews and integration are downstream. Only bop-01 owned files were edited; `git status --short` also shows concurrent sibling-plan changes.

| Command | Exit | Tests run / passed | Log |
|---|---:|---:|---|
| `TMPDIR=$PWD/tmp/behringer-oss-ports/bop-01/harness/tmp MUSIC_THINGS_REFS=/Users/taco/gits/tacogips/music-things-refs tools/airwindows-harness/generate.sh --work-dir tmp/behringer-oss-ports/bop-01/harness --out-root crates/airwindows/tests/fixtures` | 0 | 0 / 0 | `tmp/behringer-oss-ports/bop-01/logs/generate.log` |
| `test "$(find crates/airwindows/tests/fixtures/batch1 -name '*.f32' \| wc -l \| tr -d ' ')" = 110` | 0 | 0 / 0 | `tmp/behringer-oss-ports/bop-01/logs/count.log` |
| `test -z "$(find crates/airwindows/tests/fixtures/batch1 -name '*.f32' ! -size 12096c)"` | 0 | 0 / 0 | `tmp/behringer-oss-ports/bop-01/logs/sizes.log` |
| `CARGO_TERM_QUIET=true NEXTEST_STATUS_LEVEL=fail NEXTEST_FAILURE_OUTPUT=immediate-final NEXTEST_HIDE_PROGRESS_BAR=1 CARGO_TARGET_DIR=tmp/behringer-oss-ports/bop-01/check-target cargo nextest run -p music-things-airwindows` | 0 | 8 / 8 | `tmp/behringer-oss-ports/bop-01/logs/check-agent/nextest-after-generation.log` |
| `CARGO_TERM_QUIET=true CARGO_TARGET_DIR=tmp/behringer-oss-ports/bop-01/check-target cargo build -p music-things-airwindows --target wasm32-unknown-unknown` | 0 | 0 / 0 | `tmp/behringer-oss-ports/bop-01/logs/check-agent/wasm-after-generation.log` |
| `CARGO_TERM_QUIET=true CARGO_TARGET_DIR=tmp/behringer-oss-ports/bop-01/check-target cargo clippy -p music-things-airwindows --all-targets -- -D warnings` | 0 | 0 / 0 | `tmp/behringer-oss-ports/bop-01/logs/check-agent/clippy-after-generation.log` |
| `find crates/airwindows -name '*.rs' -print0 \| xargs -0 rustfmt --edition 2021 --check` | 0 | 0 / 0 | `tmp/behringer-oss-ports/bop-01/logs/check-agent/rustfmt-after-generation.log` |
| `test "$(find crates/airwindows/tests/fixtures -type f -exec stat -f %z {} + \| awk '{s+=$1} END {print s+0}')" -le 1600000` | 0 | 0 / 0 | `tmp/behringer-oss-ports/bop-01/logs/budget.log` |
| `CARGO_TERM_QUIET=true CARGO_TARGET_DIR=tmp/behringer-oss-ports/bop-01/check-target cargo doc -p music-things-airwindows --no-deps` | 0 | 0 / 0 | `tmp/behringer-oss-ports/bop-01/logs/check-agent/doc-after-generation.log` |

Fixture apparent bytes: 1,406,647 total; 1,330,560 `.f32`, 76,087 `.txt` (including format vectors). Earlier full generation exposed missing inherited parameter methods for ClipOnly; the shim was repaired and the full run succeeded. Earlier failure: `tmp/behringer-oss-ports/bop-01/logs/generate-attempt-3.log`.
