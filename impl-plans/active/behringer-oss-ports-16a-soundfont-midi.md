# Behringer OSS Ports 16a: SoundFont Crate, MIDI Loader (tml.h port)

**Status**: Ready
**Plan ID**: bop-16a
**Design Reference**: design-docs/specs/design-behringer-oss-ports.md section B (API: midi; Modules: midi/; Parity method; tml tolerance)
**Depends On**: bop-00 (wave W2; parallel with bop-01, bop-16b, bop-17a/b/c)
**Created**: 2026-10-31
**Last Updated**: 2026-10-31

## Intent and Context

Port TinyMidiLoader `tml.h` (schellingb/TinySoundFont @ `853a0a171759f1ddba0de1442133a75912bbeffa`,
MIT) to `music_things_soundfont::midi`, with exact parity against the C library on synthetic MIDI
files built in test code. bop-16c later drives the synth from `midi::parse` output and reuses
`tests/common/midi_builder.rs::scenario_midi()`, so the names below are a contract.

Context: bop-00 created `crates/soundfont/src/lib.rs` (`pub mod font; pub mod midi; pub mod synth;`)
and a doc-only `src/midi/mod.rs`, which you now own and replace. It also created
`crates/soundfont/tests/common/fnv.rs` (`fnv1a64`), which you use without editing.

## Non-goals

- No SF2 or synth code (bop-16b/16c), no file I/O in the library, no `tml_load_filename`/`tml_load` stream variants.
- Do not edit `src/lib.rs`, `src/font/`, `src/synth/`, `tests/common/fnv.rs`, `Cargo.toml`/`Cargo.lock` or other crates.

## Write Ownership (fanout contract)

- writePaths: `crates/soundfont/src/midi/mod.rs`, `crates/soundfont/src/midi/parse.rs`,
  `crates/soundfont/src/midi/info.rs`, `crates/soundfont/tests/common/midi_builder.rs`,
  `crates/soundfont/tests/midi_parity.rs`, `crates/soundfont/tests/fixtures/tml` (directory),
  `tools/soundfont-harness/tml_harness.c`, `tools/soundfont-harness/generate_tml.sh`,
  `impl-plans/active/behringer-oss-ports-16a-soundfont-midi.md`, `target`, `tmp/behringer-oss-ports/bop-16a`
- sharedPaths: none
- artifactRoots (digest-only): `target`, `tmp/behringer-oss-ports/bop-16a`
- Read-only: `crates/soundfont/src/lib.rs`, `crates/soundfont/tests/common/fnv.rs` (bop-00).
- Never run git checkout/stash/reset or any other git operation that rewrites the tree.

## Common Execution Rules

- `MUSIC_THINGS_REFS=/Users/taco/gits/tacogips/music-things-refs`. Read only `TinySoundFont/tml.h`.
  Never open Music Tribe AGPL code.
- Evidence directory: `tmp/behringer-oss-ports/bop-16a/`. It holds `intent.md` (immutable deliverables),
  `pre-hashes.txt` and `post-hashes.txt` (`shasum -a 256` of owned files) and `logs/`. Re-read each
  file before editing; on drift, stop and log it. Only edit `writePaths`.
- Format owned files with `rustfmt --edition 2021`. Cargo env: `CARGO_TERM_QUIET=true NEXTEST_STATUS_LEVEL=fail NEXTEST_FAILURE_OUTPUT=immediate-final NEXTEST_HIDE_PROGRESS_BAR=1`.
- Siblings build other crates in parallel. If an error is in a file you do not own, rerun later and log it.

## Deliverables

### T1. `crates/soundfont/src/midi/{mod.rs,parse.rs,info.rs}`
- Header: the tml.h copyright/license line, the upstream path and the revision.
- `pub struct MidiMessage { pub time_ms: u32, pub kind: u8, pub channel: u8, pub key: u8, pub velocity: u8, pub control: u8, pub control_value: u8, pub program: u8, pub channel_pressure: u8, pub key_pressure: u8, pub pitch_bend: u16, pub tempo: [u8; 3] }`
  (`#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]`). `kind` uses the upstream `TML_*` type
  values exposed as `pub const NOTE_OFF: u8`, `NOTE_ON`, `KEY_PRESSURE`, `CONTROL_CHANGE`,
  `PROGRAM_CHANGE`, `CHANNEL_PRESSURE`, `PITCH_BEND` and `SET_TEMPO`. Copy the other `TML_*` message
  types that tml stores from the header with the same values. Fields that the C union does not use for
  a kind stay 0.
- `pub fn parse(bytes: &[u8]) -> Result<Vec<MidiMessage>, MidiError>` ports `tml_load_memory`: header
  and track parsing, running status, meta and sysex skipping, tempo-aware tick-to-ms conversion, and the
  merge order of messages across tracks. Wherever upstream returns NULL, Rust returns `Err`.
- `pub enum MidiError { InvalidHeader, Truncated, UnsupportedFormat, Empty }` (adjust the variant set
  to the distinct upstream failure points; each upstream NULL return maps to one variant), with
  `Display` and `std::error::Error`.
- `pub struct MidiInfo { pub used_channels: i32, pub used_programs: i32, pub total_notes: i32, pub time_first_note: u32, pub time_length: u32 }`
  and `pub fn info(messages: &[MidiMessage]) -> MidiInfo`, which ports `tml_get_info`.
- `impl MidiMessage { pub fn tempo_value(&self) -> u32 }` ports `tml_get_tempo_value`.

### T2. `crates/soundfont/tests/common/midi_builder.rs` (pinned names)
- `#![allow(dead_code)]`. `pub fn scenario_midi() -> Vec<u8>` builds format 1, 480 PPQ, with a tempo
  track (tempo 500000 us at tick 0, then 400000 at tick 1920) and a music track covering:
  - bank select CC0 and program change on channel 0
  - notes on channels 0, 1 and 9, overlapping notes, a note-on with velocity 0 used as note-off, and
    running status
  - CC7 volume, CC10 pan, CC64 sustain on and off
  - pitch bend up, center and down
  - a key-pressure and a channel-pressure event
  - end of track. The total length is about 2.0 s.
- `pub fn format0_midi() -> Vec<u8>`: format 0, a single track, with a mid-song tempo change.
- `pub fn truncated_midi() -> Vec<u8>`: `scenario_midi()` truncated in the middle of a track.

### T3. Harness `tools/soundfont-harness/tml_harness.c` and `tools/soundfont-harness/generate_tml.sh`
- An MIT header. `#define TML_IMPLEMENTATION` and `#include "tml.h"` from `$MUSIC_THINGS_REFS/TinySoundFont`
  (passed as `-I`). The harness reads a .mid file and writes the dump format below; when
  `tml_load_memory` returns NULL it writes `error`.
- `generate_tml.sh --work-dir DIR`:
  1. Run `SOUNDFONT_FIXTURE_OUT=DIR/inputs cargo test -p music-things-soundfont --test midi_parity -- --ignored export_synthetic_midi`.
  2. Compile with `clang -std=c99 -O1 -ffp-contract=off -fno-fast-math -fno-builtin`.
  3. Write `crates/soundfont/tests/fixtures/tml/{scenario.txt,format0.txt,truncated.txt,inputs.txt}`.
     `inputs.txt` holds `scenario=<fnv hex>`, `format0=<fnv>` and `truncated=<fnv>`.
- Dump format: one line per message, `<time_ms> <kind hex2> <channel> <f1> <f2>`, where f1 and f2 are
  key/velocity (note on/off), key/key_pressure, control/control_value, program/0,
  channel_pressure/0, pitch_bend/0, tempo_value/0, and 0/0 for other kinds. A final line
  `info <used_channels> <used_programs> <total_notes> <time_first_note> <time_length>`.

### T4. `crates/soundfont/tests/midi_parity.rs`
- `#[path = "common/midi_builder.rs"] mod midi_builder; #[path = "common/fnv.rs"] mod fnv;`
- `#[test] #[ignore] fn export_synthetic_midi()` writes the three files to `$SOUNDFONT_FIXTURE_OUT`.
- `scenario`, `format0` and `truncated` tests: the builder FNV equals `inputs.txt`; the Rust dump
  (same format, produced by a test helper) equals the fixture text exactly; truncated input gives Err
  and the fixture says `error`.

## Key Pitfalls

- tml computes ms times through tempo and ticks with specific integer and double arithmetic. Copy the
  exact expressions and types (do not recompute in f64 when upstream uses integers, and keep the
  rounding points).
- The merge order of simultaneous events across tracks must follow upstream's list insertion exactly.
- A note-on with velocity 0: keep whatever tml stores, and do not normalize it.
- `parse` may allocate (it is not realtime). Do not add a streaming parser.

## Tests (situation -> expected)

- `scenario_midi()` -> its dump equals `fixtures/tml/scenario.txt` exactly.
- `format0_midi()` -> its dump equals `format0.txt`.
- `truncated_midi()` -> `Err(_)`, and the fixture is `error`.
- `info()` on the scenario -> matches the `info` line.
- `tempo_value()` on the SET_TEMPO messages -> 500000 and 400000.

## Verification Commands (evidence)

1. `MUSIC_THINGS_REFS=/Users/taco/gits/tacogips/music-things-refs tools/soundfont-harness/generate_tml.sh --work-dir tmp/behringer-oss-ports/bop-16a/harness`: exit 0, 4 fixture files written.
2. `cargo nextest run -p music-things-soundfont --test midi_parity`: exit 0, at least 4 tests, 0 failures.
3. `cargo clippy -p music-things-soundfont --all-targets -- -D warnings`: exit 0 (errors in files outside writePaths are logged, not fixed).
4. `rustfmt --edition 2021 --check crates/soundfont/src/midi/*.rs crates/soundfont/tests/midi_parity.rs crates/soundfont/tests/common/midi_builder.rs`: exit 0.
5. `cargo build -p music-things-soundfont --target wasm32-unknown-unknown`: exit 0.

## Completion Criteria

- [ ] T1-T4 delivered with the pinned public names.
- [ ] Exact parity for all three inputs.
- [ ] Verifications 1-5 exit 0, with logs in `tmp/behringer-oss-ports/bop-16a/logs/`.
- [ ] Only `writePaths` are modified, and every file is under 1000 lines.
- [ ] Progress Log updated with commands, exit status, tests run/passed and log paths.

## Progress Log

### Session: 2026-10-31
**Tasks Completed**: Plan created (not started)
**Notes**: -
