# Behringer OSS Ports 16c: SoundFont Crate, Synth Runtime (tsf.h voices, channels, render)

**Status**: Completed
**Plan ID**: bop-16c
**Design Reference**: design-docs/specs/design-behringer-oss-ports.md section B (Synth::new, Voice API, Channel API, Realtime behavior, Parity method S1/S2, tsf tolerance), D4
**Depends On**: bop-16a, bop-16b (wave W3; parallel with bop-02..bop-15)
**Created**: 2026-10-31
**Last Updated**: 2026-11-01

## Intent and Context

Port the runtime half of `tsf.h` (@ `853a0a171759f1ddba0de1442133a75912bbeffa`, MIT) to
`music_things_soundfont::synth`:
- output setup, max voices, voices, the volume and mod envelopes, LFOs, the low-pass filter
- the render loop with its effect sample block
- the public note API and every public `tsf_channel_*` function, including MIDI control

Prove render parity against tsf.h on the synthetic SF2 from bop-16b, driven by the synthetic MIDI
from bop-16a.

Contracts you consume (read only):
- `crate::font::{SoundFont, Region, Preset, Envelope}`. The internal structs mirror `tsf_region`,
  `tsf_preset` and `tsf_envelope` field for field (see `crates/soundfont/src/font/mod.rs` docs).
- `crate::midi::{parse, MidiMessage, NOTE_ON, ...}`.
- `tests/common/sf2_builder.rs::synthetic_sf2()`, `tests/common/midi_builder.rs::scenario_midi()`,
  `tests/common/fnv.rs`.

bop-00 created the doc-only `src/synth/mod.rs`, which you now own.

## Non-goals

- No `tsf_render_short`, no file/stream loaders, no changes to `font/` or `midi/` (report a defect
  there in the Progress Log for the owner), and no edits to `src/lib.rs`, `Cargo.toml`/`Cargo.lock`
  or other crates.

## Write Ownership (fanout contract)

- writePaths: `crates/soundfont/src/synth/mod.rs`, `crates/soundfont/src/synth/envelope.rs`,
  `crates/soundfont/src/synth/lfo.rs`, `crates/soundfont/src/synth/filter.rs`,
  `crates/soundfont/src/synth/voice.rs`, `crates/soundfont/src/synth/channel.rs`,
  `crates/soundfont/src/synth/render.rs`, `crates/soundfont/tests/synth_parity.rs`,
  `crates/soundfont/tests/synth_no_alloc.rs`, `crates/soundfont/tests/fixtures/synth` (directory),
  `tools/soundfont-harness/synth_harness.c`, `tools/soundfont-harness/generate_synth.sh`,
  `impl-plans/active/behringer-oss-ports-16c-soundfont-synth.md`, `target`, `tmp/behringer-oss-ports/bop-16c`
- sharedPaths: none
- artifactRoots (digest-only): `target`, `tmp/behringer-oss-ports/bop-16c`
- Read-only: `crates/soundfont/src/lib.rs`, `crates/soundfont/src/font/*`, `crates/soundfont/src/midi/*`,
  `crates/soundfont/tests/common/{fnv,sf2_builder,midi_builder}.rs` (bop-00, bop-16b, bop-16a).
- Never run git checkout/stash/reset or any other git operation that rewrites the tree.

## Common Execution Rules

- `MUSIC_THINGS_REFS=/Users/taco/gits/tacogips/music-things-refs`. Read only `TinySoundFont/tsf.h`
  (and `tml.h` for the harness). Never open Music Tribe AGPL code.
- Evidence directory: `tmp/behringer-oss-ports/bop-16c/`. It holds `intent.md`, `pre-hashes.txt`,
  `post-hashes.txt` and `logs/`. Re-read each file before editing; on drift, stop and log it. Only edit `writePaths`.
- 14 Airwindows group plans build in parallel. If an error is in a file you do not own, rerun later and log it.
- Format owned files with `rustfmt --edition 2021`. Cargo env: `CARGO_TERM_QUIET=true NEXTEST_STATUS_LEVEL=fail NEXTEST_FAILURE_OUTPUT=immediate-final NEXTEST_HIDE_PROGRESS_BAR=1`.

## Deliverables

### T1. `crates/soundfont/src/synth/{mod.rs,envelope.rs,lfo.rs,filter.rs,voice.rs,channel.rs,render.rs}`
- Header: the tsf.h copyright line, the upstream path and the revision.
- `pub enum OutputMode { StereoInterleaved, StereoUnweaved, Mono }`;
  `pub struct SynthConfig { pub output_mode: OutputMode, pub sample_rate: u32, pub global_gain_db: f32, pub max_voices: usize }`
  with a `Default` of StereoInterleaved, 44100, 0.0 and 64.
- `pub struct Synth`:
  - `pub fn new(font: Arc<SoundFont>, config: SynthConfig) -> Synth` preallocates `max_voices` voices
    and 16 channels. It applies `tsf_set_output` semantics, including a sample rate of 0 becoming
    44100 the same way upstream does.
  - `set_volume(f32)`, `reset()`.
  - `note_on(preset_index: usize, key: i32, vel: f32) -> bool` and
    `bank_note_on(bank: i32, preset_number: i32, key: i32, vel: f32) -> bool`, which mirror the
    upstream int returns.
  - `note_off(preset_index: usize, key: i32)`, `bank_note_off(bank, preset_number, key) -> bool`,
    `note_off_all()`, `active_voice_count() -> usize`.
  - `render_float(&mut self, out: &mut [f32], frames: usize, mixing: bool)`. It renders
    `min(frames, out.len() / channels)` frames.
- Channel API: one Rust method per public `tsf_channel_*` function in the pinned header, with the
  `tsf_` prefix dropped (for example `channel_set_presetnumber(channel: usize, preset_number: i32, mididrums: bool) -> bool`,
  `channel_midi_control(channel: usize, controller: i32, value: i32) -> bool`, and the `channel_get_*`
  getters). List the exact mapping in `synth/mod.rs` docs and in the Progress Log. A channel index of
  16 or more returns the upstream failure value and does nothing (D4).
- Lazy channel activation (mirrors tsf; see design B Realtime behavior):
  - `Synth` holds `channel_num: usize`, starting at 0, which mirrors tsf `channelNum`, over
    preallocated storage for 16 channels.
  - Every call that upstream routes through `tsf_channel_init(f, ch)` (setters, `channel_midi_control`,
    and so on) first activates channels `channel_num..=ch` with exactly the `tsf_channel_init`
    defaults, then applies the call.
  - Calls that upstream guards with `channel >= channelNum` (`channel_note_on` and every
    `channel_get_*`) return tsf's fallback value and do nothing on inactive channels.
  - `channel_note_off`, `channel_note_off_all` and `channel_sounds_off_all` are NOT guarded upstream
    (tsf.h:1943-1983): they scan the voice pool for `playing_channel == channel` regardless of
    `channel_num`. Mirror that voice scan; do not add an early return on `channel >= channel_num`
    (only the D4 cap `channel >= 16` returns early).
  - `reset()` ends voices and sets `channel_num = 0`, as `tsf_reset` frees channels. Nothing is
    allocated or freed.
- Voice exhaustion: when every voice is busy, `note_on` behaves as upstream does under
  `tsf_set_max_voices` and never allocates.

### T2. Harness `tools/soundfont-harness/synth_harness.c` and `tools/soundfont-harness/generate_synth.sh`
- The harness includes `tsf.h` and `tml.h` with their implementations. It loads `synthetic.sf2` and
  `scenario.mid`, then renders:
  - **S1**: `tsf_set_output(TSF_STEREO_INTERLEAVED, 44100, 0.0f)`, `tsf_set_max_voices(f, 32)`,
    `tsf_channel_set_bank_preset(f, 9, 128, 0)`. Then 1378 blocks of 64 frames. Before each block,
    `msec += 64.0 * (1000.0 / 44100.0)` (accumulated in double, starting at 0), and every message with
    `time <= msec` is dispatched:
    - PROGRAM_CHANGE -> `tsf_channel_set_presetnumber(f, ch, program, ch == 9)`
    - NOTE_ON -> `tsf_channel_note_on(f, ch, key, velocity / 127.0f)`
    - NOTE_OFF -> `tsf_channel_note_off`
    - PITCH_BEND -> `tsf_channel_set_pitchwheel`
    - CONTROL_CHANGE -> `tsf_channel_midi_control`
    Render with `tsf_render_float(f, buf, 64, 0)`.
  - **S2**: `TSF_MONO`, 44100, gain -6.0, `tsf_set_max_voices(f, 8)`. The output buffer is pre-filled
    with 0.125f and rendered with `mixing = 1`. The scripted events are applied before block k:
    - k=0: `channel_set_presetnumber(0, 0, 0)`; `channel_note_on(0, key, 0.8f)` for keys 48..=60
      (13 notes, more than 8 voices)
    - k=100: `channel_set_pitchrange(0, 12.0f)`; `channel_set_pitchwheel(0, 12000)`
    - k=200: `channel_midi_control(0, 64, 127)`
    - k=220: `channel_note_off(0, key)` for keys 48..=60
    - k=400: `channel_midi_control(0, 64, 0)`
    - k=500: `tsf_note_on(f, 1, 72, 1.0f)`
    - k=600: `channel_set_pan(0, 0.2f)`; `channel_set_volume(0, 0.5f)`
    - k=800: `tsf_note_off_all(f)`
    - k=900: `channel_note_on(0, 60, 0.5f)`
    - k=1000: `channel_sounds_off_all(0)`
    The run lasts 1378 blocks.
  - Stored frames: frames 0..2048 in full, then every 16th frame from 2048 to 88192.
  - Writes `crates/soundfont/tests/fixtures/synth/{render_s1.f32,render_s2.f32,inputs.txt}` (f32 LE).
- `generate_synth.sh --work-dir DIR`:
  1. Run `SOUNDFONT_FIXTURE_OUT=DIR/inputs cargo test -p music-things-soundfont --test synth_parity -- --ignored export_synth_inputs`.
  2. Compile with `clang -std=c99 -O1 -ffp-contract=off -fno-fast-math -fno-builtin -lm` and run.

### T3. Tests
- `crates/soundfont/tests/synth_parity.rs`, which includes the builders and fnv with `#[path]`:
  - `export_synth_inputs` (ignored)
  - an input FNV check against `inputs.txt`
  - S1 and S2 are rendered by the same driver in Rust (S1 uses `midi::parse` output). Max-abs is at
    most 1e-5 at the stored frames. Record the measured max-abs in a test comment and in the Progress Log.
- `crates/soundfont/tests/synth_no_alloc.rs`: a counting `#[global_allocator]`. After `Synth::new`,
  zero allocations across `note_on` beyond `max_voices`, channel calls, `midi_control`, `render_float`
  and `note_off_all`.

## Key Pitfalls

- tsf updates envelopes, LFOs and filter coefficients per `TSF_RENDER_EFFECTSAMPLEBLOCK`. Keep that
  block structure and the same float/double mix (`double` source positions, `float` gains).
- Use `powf`/`expf`/`tan`/`log10` at the same precision as each upstream call site (`TSF_POW` is
  double and `TSF_POWF` is float).
- Voice stealing and `exclusiveClass` handling, and the order of voice iteration, affect output.
  Iterate in the same index order.
- `render_float` with `mixing = true` adds into the buffer; with false it overwrites.
- No `Vec` push, `Box` or `String` on any realtime path. Everything is preallocated in `new`.
- Do not treat all 16 preallocated channels as active. Read `tsf_channel_init` (lazy growth and
  defaults), the `channel >= channelNum` guard in `tsf_channel_note_on` and the getters, the
  unguarded voice scans in `tsf_channel_note_off`/`_note_off_all`/`_sounds_off_all`, and
  `tsf_reset` (frees channels), and mirror the activation state exactly. S1 and S2 do not exercise
  this, so the lazy-channel tests below are the only evidence.
- Every file stays under 1000 lines. Split voice and render if needed (render.rs and voice.rs are separate on purpose).

## Tests (situation -> expected)

- S1 render vs `render_s1.f32` -> max-abs <= 1e-5 (0 expected on the generation host).
- S2 render vs `render_s2.f32` -> max-abs <= 1e-5. Voice exhaustion behavior matches.
- `channel_set_pan(16, 0.5)` -> returns the failure value and nothing changes.
- Fresh synth, `channel_note_on(3, 60, 1.0)` -> `active_voice_count() == 0`.
  `channel_get_pan(3)` and `channel_get_pitchrange(3)` return tsf's fallbacks (0.5 and 2.0; confirm
  against the pinned getters).
- After `channel_set_pan(5, 0.25)` -> channels 0-4 report the `tsf_channel_init` defaults, channel 5
  reports pan 0.25, and `channel_note_on(2, 60, 1.0)` starts a voice.
- After `reset()` -> `channel_note_on(0, 60, 1.0)` starts no voice until a setter runs on channel 0.
- Unguarded voice scan: `channel_set_presetnumber(0, 0, false)`, `channel_note_on(0, 60, 1.0)`,
  render one block, then `reset()` (channel_num becomes 0). If voices remain releasing after reset
  (per the pinned `tsf_reset`), `channel_sounds_off_all(0)` still ends them, so after rendering a
  block `active_voice_count() == 0`; if the pinned `tsf_reset` ends all voices immediately, assert
  instead that `channel_sounds_off_all(0)` on the inactive channel does not panic and changes nothing.
  Record which branch the pinned source takes in the Progress Log.
These four tests go in `crates/soundfont/tests/synth_parity.rs`.
- The no-alloc test -> 0 allocations.

## Verification Commands (evidence)

1. `MUSIC_THINGS_REFS=/Users/taco/gits/tacogips/music-things-refs tools/soundfont-harness/generate_synth.sh --work-dir tmp/behringer-oss-ports/bop-16c/harness`: exit 0.
2. `cargo nextest run -p music-things-soundfont`: exit 0, 0 failures (includes midi_parity, font_parity, synth_parity, synth_no_alloc, fixture_budget).
3. `cargo clippy -p music-things-soundfont --all-targets -- -D warnings`: exit 0.
4. `rustfmt --edition 2021 --check crates/soundfont/src/synth/*.rs crates/soundfont/tests/synth_parity.rs crates/soundfont/tests/synth_no_alloc.rs`: exit 0.
5. `cargo build -p music-things-soundfont --target wasm32-unknown-unknown`: exit 0.
6. `find crates/soundfont/tests/fixtures -type f -exec stat -f %z {} + | awk '{s+=$1} END {print s}'`: prints
   at most 400000 (apparent bytes, the same metric as the `fixture_budget` test, which stays the
   authoritative gate). Do not use `du` (it counts allocated blocks, not bytes).

## Completion Criteria

- [x] All public tsf voice, channel and render functions are mapped (the list is in the docs), with D4 behavior.
- [x] Lazy channel activation (`channel_num`, inactive-channel fallbacks, unguarded note-off/sounds-off voice scans, reset deactivation) is mirrored, and the four lazy-channel tests pass.
- [x] S1 and S2 parity are within 1e-5, and the measured max-abs is recorded.
- [x] Verifications 1-6 exit 0, with logs in `tmp/behringer-oss-ports/bop-16c/logs/`.
- [x] Only `writePaths` are modified, and every file is under 1000 lines.
- [x] Progress Log updated with commands, exit status, tests run/passed and log paths.

## Progress Log

### Session: 2026-10-31
**Tasks Completed**: Plan created (not started)
**Notes**: -

### Session: 2026-11-01
**Tasks Completed**: Synth runtime, reference harness, S1/S2 fixtures, lazy-channel and allocation tests. Formal review and workflow finalization remain downstream.
**API mapping**: `tsf_set_output` and `tsf_set_max_voices` map to `Synth::new`; `tsf_set_volume`, `tsf_reset`, `tsf_note_on`, `tsf_bank_note_on`, `tsf_note_off`, `tsf_bank_note_off`, `tsf_note_off_all`, `tsf_active_voice_count`, and `tsf_render_float` map to the same names without `tsf_`. Channel mappings: `channel_set_presetindex`, `channel_set_presetnumber`, `channel_set_bank`, `channel_set_bank_preset`, `channel_set_pan`, `channel_set_volume`, `channel_set_pitchwheel`, `channel_set_pitchrange`, `channel_set_tuning`, `channel_set_sustain`, `channel_note_on`, `channel_note_off`, `channel_note_off_all`, `channel_sounds_off_all`, `channel_midi_control`, `channel_get_preset_index`, `channel_get_preset_bank`, `channel_get_preset_number`, `channel_get_pan`, `channel_get_volume`, `channel_get_pitchwheel`, `channel_get_pitchrange`, and `channel_get_tuning`.
**Notes**: S1 and S2 measured max-abs 0.000000000 (aarch64-apple-darwin). Upstream reset starts a fast release; the inactive-channel scan test renders 512 frames to cover its 441-frame release. Inactive note-on returns success without starting a voice. Active pan getters mirror the pinned upstream subtraction. All owned Rust files are under 1000 lines.
**Verification**: Commands 1-6 above exited 0; logs `tmp/behringer-oss-ports/bop-16c/logs/current-{1,2,3,4,5,6}.log`. Generator exporter: 1/1 tests; nextest: 17/17, 3 skipped. Clippy, rustfmt, wasm build and budget are non-test checks. Parity measurements: `CARGO_TERM_QUIET=true cargo test -p music-things-soundfont --test synth_parity render_s -- --nocapture`, 2/2 passed, log `tmp/behringer-oss-ports/bop-16c/logs/current-parity.log`. Earlier nextest failed 2/17 before filter precision and fixture-velocity fixes; retained in `logs/verification-2-attempt1.log`.
