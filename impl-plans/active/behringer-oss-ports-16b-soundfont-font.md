# Behringer OSS Ports 16b: SoundFont Crate, SF2 Loading (tsf.h load path)

**Status**: In Progress
**Plan ID**: bop-16b
**Design Reference**: design-docs/specs/design-behringer-oss-ports.md section B (API: SoundFont::load and preset queries; Modules: font/; Parity method: font load dump; tolerances)
**Depends On**: bop-00 (wave W2; parallel with bop-01, bop-16a, bop-17a/b/c)
**Created**: 2026-10-31
**Last Updated**: 2026-10-31

## Intent and Context

Port the load half of TinySoundFont `tsf.h` (@ `853a0a171759f1ddba0de1442133a75912bbeffa`, MIT):
- RIFF/sfbk parsing and the hydra chunks (phdr, pbag, pmod, pgen, inst, ibag, imod, igen, shdr)
- region generator resolution (global zones, preset and instrument generator merging, operator
  application, timecent/centibel conversions)
- 16-bit sample decoding into the shared f32 sample buffer; `sm24` is ignored
- the preset query functions

The result is `music_things_soundfont::font`. bop-16c builds the synth on the internal structs, so
the internal data shapes are a contract: `Region`, `Preset` and `Envelope` mirror `struct tsf_region`,
`struct tsf_preset` and `struct tsf_envelope` field for field, with snake_case names and the same
scalar types (float -> f32, int -> i32, unsigned -> u32, short -> i16). They are `pub(crate)` with
`pub(crate)` fields. `SoundFont` holds `pub(crate) presets: Vec<Preset>`,
`pub(crate) samples: Vec<f32>` (tsf `fontSamples`) and `pub(crate) sample_count: u32`.

Context: bop-00 created `crates/soundfont/src/lib.rs` (declaring `pub mod font;`) and a doc-only
`src/font/mod.rs`, which you now own. Use `crates/soundfont/tests/common/fnv.rs` from bop-00 without
editing it.

## Non-goals

- No voice, render or channel code (bop-16c) and no MIDI (bop-16a).
- No `tsf_load_filename`/`tsf_load` stream API; bytes in only.
- Do not edit `src/lib.rs`, `src/midi/`, `src/synth/`, `tests/common/fnv.rs`, `Cargo.toml`/`Cargo.lock` or other crates.

## Write Ownership (fanout contract)

- writePaths: `crates/soundfont/src/font/mod.rs`, `crates/soundfont/src/font/riff.rs`,
  `crates/soundfont/src/font/hydra.rs`, `crates/soundfont/src/font/region.rs`,
  `crates/soundfont/src/font/preset.rs`, `crates/soundfont/tests/common/sf2_builder.rs`,
  `crates/soundfont/tests/font_parity.rs`, `crates/soundfont/tests/fixtures/font` (directory),
  `tools/soundfont-harness/font_harness.c`, `tools/soundfont-harness/generate_font.sh`,
  `impl-plans/active/behringer-oss-ports-16b-soundfont-font.md`, `target`, `tmp/behringer-oss-ports/bop-16b`
- sharedPaths: none
- artifactRoots (digest-only): `target`, `tmp/behringer-oss-ports/bop-16b`
- Read-only: `crates/soundfont/src/lib.rs`, `crates/soundfont/tests/common/fnv.rs` (bop-00).
- Never run git checkout/stash/reset or any other git operation that rewrites the tree.

## Common Execution Rules

- `MUSIC_THINGS_REFS=/Users/taco/gits/tacogips/music-things-refs`. Read only `TinySoundFont/tsf.h`.
  Never open Music Tribe AGPL code.
- Evidence directory: `tmp/behringer-oss-ports/bop-16b/`. It holds `intent.md`, `pre-hashes.txt`,
  `post-hashes.txt` and `logs/`. Re-read each file before editing; on drift, stop and log it. Only edit `writePaths`.
- Format owned files with `rustfmt --edition 2021`. Cargo env: `CARGO_TERM_QUIET=true NEXTEST_STATUS_LEVEL=fail NEXTEST_FAILURE_OUTPUT=immediate-final NEXTEST_HIDE_PROGRESS_BAR=1`.

## Deliverables

### T1. `crates/soundfont/src/font/{mod.rs,riff.rs,hydra.rs,region.rs,preset.rs}`
- Header: the tsf.h copyright/license line, the upstream path and the revision.
- `pub struct SoundFont` (`Send + Sync`, immutable after load):
  - `pub fn load(bytes: &[u8]) -> Result<SoundFont, LoadError>`
  - `pub fn preset_count(&self) -> usize`
  - `pub fn preset_index(&self, bank: i32, number: i32) -> Option<usize>`
  - `pub fn preset_name(&self, index: usize) -> Option<&str>`
  - `pub fn bank_preset_name(&self, bank: i32, number: i32) -> Option<&str>`
- `pub enum LoadError { NotRiff, NotSfbk, MissingChunk(&'static str), Truncated, InvalidHydra }`, with
  `Display` and `std::error::Error`. Each upstream failure return maps to one variant.
- Internal `Region`, `Preset` and `Envelope` as described in Intent. Generator handling and its order
  follow `tsf_region_operator`, `tsf_region_envtosecs` and `tsf_load_presets` exactly.

### T2. `crates/soundfont/tests/common/sf2_builder.rs` (pinned names)
- `#![allow(dead_code)]`. `pub fn synthetic_sf2() -> Vec<u8>` is a valid SF2.01 file with terminal
  records (EOP/EOI/EOS). It contains:
  - samples: a 16-bit sine loop (256 frames, root key 69) and a saw loop (128 frames, root 60), plus
    one sample with an `sm24` chunk present
  - preset 0 bank 0 "SynthSine": an instrument with a global zone. Its regions cover key ranges 0-63
    and 64-127, a velocity range, loop modes 1 and 3, coarse/fine tune, pan, a volume envelope
    (delay/attack/hold/decay/sustain/release), a mod envelope to pitch and filter, a mod LFO to
    pitch/filter/volume, a vib LFO, `initialFilterFc`/`initialFilterQ`, and `keynumToVolEnvHold`/`Decay`
  - preset 1 bank 0 "SynthSaw": a simpler instrument with no loop (mode 0) and an attenuation
  - preset 0 bank 128 "Drums": key-ranged one-shot regions with an `exclusiveClass`
- `pub fn broken_sf2_truncated() -> Vec<u8>`: the result of `synthetic_sf2()` cut inside pdta.

### T3. Harness `tools/soundfont-harness/font_harness.c` and `tools/soundfont-harness/generate_font.sh`
- `#define TSF_IMPLEMENTATION`, `#include "tsf.h"` from `$MUSIC_THINGS_REFS/TinySoundFont`. It loads
  the file with `tsf_load_memory` and dumps:
  - `presets <n>`
  - per preset: `preset <i> <bank> <number> <name> <region_count>`
  - per region, one line with every `tsf_region` field in declaration order. Integers print in decimal;
    floats print as `%08x` f32 bits.
  - `samples <count> <fnv1a64 of fontSamples bytes>`
  - For a NULL load result it writes `error`.
- `generate_font.sh --work-dir DIR`:
  1. Run `SOUNDFONT_FIXTURE_OUT=DIR/inputs cargo test -p music-things-soundfont --test font_parity -- --ignored export_synthetic_sf2`.
  2. Compile with `clang -std=c99 -O1 -ffp-contract=off -fno-fast-math -fno-builtin -lm`.
  3. Write `crates/soundfont/tests/fixtures/font/{synthetic.txt,truncated.txt,inputs.txt}`.

### T4. `crates/soundfont/tests/font_parity.rs`
- `#[test] #[ignore] fn export_synthetic_sf2()` writes `synthetic.sf2` and `truncated.sf2`.
- Tests:
  - the builder FNV matches `inputs.txt`
  - Rust produces the same dump through a `#[doc(hidden)] pub fn debug_dump(&self) -> String` on
    `SoundFont`, in exactly the harness format. That function is the only addition beyond the design
    API, and it exists for testing.
  - integer fields and the sample FNV are compared exactly; float fields are compared by value with
    relative 1e-6
  - truncated input -> `Err`
  - preset queries: `preset_index(128, 0)` is Some, `preset_name` gives "Drums", and an unknown preset gives None.

## Key Pitfalls

- Decode each i16 as `((s as f64) / 32767.0) as f32`, matching pinned `tsf.h:964,1000`; upstream does not read `sm24`.
- Generator amount types differ (signed/unsigned, and ranges as lo/hi bytes). Reproduce tsf's union
  reads exactly.
- Global-zone inheritance and the instrument/preset additive merge order must match upstream
  (preset generators add to instrument values).
- Sample offsets and loop points are post-processed (`end` clamping, `offset` adjustments). Copy them exactly.
- Do not make `Region` or `Preset` public API, so that the public surface stays as the design states.

## Tests (situation -> expected)

- `synthetic_sf2()` -> the Rust dump equals `font/synthetic.txt` (exact ints and sample FNV, floats within relative 1e-6).
- `broken_sf2_truncated()` -> `Err(LoadError::...)`, and the fixture is `error`.
- `preset_count()` == 3. `bank_preset_name(0, 1) == Some("SynthSaw")`.

## Verification Commands (evidence)

1. `MUSIC_THINGS_REFS=/Users/taco/gits/tacogips/music-things-refs tools/soundfont-harness/generate_font.sh --work-dir tmp/behringer-oss-ports/bop-16b/harness`: exit 0.
2. `cargo nextest run -p music-things-soundfont --test font_parity`: exit 0, 0 failures.
3. `cargo clippy -p music-things-soundfont --all-targets -- -D warnings`: exit 0 (errors in files you do not own are logged, not fixed).
4. `rustfmt --edition 2021 --check crates/soundfont/src/font/*.rs crates/soundfont/tests/font_parity.rs crates/soundfont/tests/common/sf2_builder.rs`: exit 0.
5. `cargo build -p music-things-soundfont --target wasm32-unknown-unknown`: exit 0.

## Completion Criteria

- [x] T1-T4 delivered, with the internal struct mirroring documented in `font/mod.rs` docs for bop-16c.
- [x] Load parity holds for the synthetic and truncated inputs.
- [x] Verifications 1-5 exit 0, with logs in `tmp/behringer-oss-ports/bop-16b/logs/`.
- [x] Only `writePaths` are modified, and every file is under 1000 lines.
- [x] Progress Log updated with commands, exit status, tests run/passed and log paths.

## Progress Log

### Session: 2026-10-31
**Tasks Completed**: Plan created (not started)
**Notes**: -

### Session: 2026-10-31 — implementation blocked
**Tasks Completed**: Dependency admission and design alignment checked; no loader implementation completed.
**Blocker**: Pinned `tsf.h` divides 16-bit samples by `32767.0` and skips `sm24`, while this plan requires division by `32768.0f`, `sm24` decoding, and exact upstream sample FNV parity. An accepted design/plan amendment must resolve these incompatible requirements.
**Evidence**: `tmp/behringer-oss-ports/bop-16b/logs/upstream-readiness.log` records the source inspection and revision check (both exit 0; no tests). Verification commands 1–5 were not run because T1–T4 are incomplete.

### Session: 2026-11-01 — IR-W2-16B-DECODE repair
**Decision**: Follow pinned `tsf.h:964,1000`: divide in f64 by 32767.0 then cast to f32; ignore `sm24`. The design author must amend `design-docs/specs/design-behringer-oss-ports.md:213` to match. Implementation and verification are in progress.

**Implementation**: T1–T4 delivered. All owned Rust files are below 1000 lines; maximum 227. Sample FNV is `a3a7a827a2d9075f`. Formal review and integration remain downstream.

| Verification command | Exit | Tests passed/run | Complete log |
| --- | --- | --- | --- |
| `MUSIC_THINGS_REFS=/Users/taco/gits/tacogips/music-things-refs tools/soundfont-harness/generate_font.sh --work-dir tmp/behringer-oss-ports/bop-16b/harness` | 0 | 1/1 | `tmp/behringer-oss-ports/bop-16b/logs/check-agent-final-generate.log` |
| `cargo nextest run -p music-things-soundfont --test font_parity` | 0 | 2/2 | `tmp/behringer-oss-ports/bop-16b/logs/check-agent-final-font.log` |
| `cargo clippy -p music-things-soundfont --all-targets -- -D warnings` | 0 | 0/0 | `tmp/behringer-oss-ports/bop-16b/logs/check-agent-final-clippy.log` |
| `find crates/soundfont/src/font crates/soundfont/tests/font_parity.rs crates/soundfont/tests/common/sf2_builder.rs -name '*.rs' -print0 | xargs -0 rustfmt --edition 2021 --check` | 0 | 0/0 | `tmp/behringer-oss-ports/bop-16b/logs/check-agent-final-rustfmt.log` |
| `cargo build -p music-things-soundfont --target wasm32-unknown-unknown` | 0 | 0/0 | `tmp/behringer-oss-ports/bop-16b/logs/check-agent-final-wasm.log` |
| `cargo nextest run -p music-things-soundfont --test midi_parity` | 0 | 6/6 | `tmp/behringer-oss-ports/bop-16b/logs/check-agent-final-midi.log` |

Cargo commands used `CARGO_TERM_QUIET=true`, `NEXTEST_STATUS_LEVEL=fail`, `NEXTEST_FAILURE_OUTPUT=immediate-final`, `NEXTEST_HIDE_PROGRESS_BAR=1` and plan-local `CARGO_TARGET_DIR`.
