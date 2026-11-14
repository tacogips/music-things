# Behringer KICK/IGNITE OSS Ports (Batch 1)

Design for porting the permissively licensed DSP components bundled with the Behringer
KICK and IGNITE open firmware to dependency-free Rust crates. Batch 1 covers the shared
Airwindows framework plus the first 110 Airwindows plugins, TinySoundFont (tsf.h and tml.h),
and the pitch tools (smbPitchShift, stftPitchShift, dywapitchtrack). It also adds optional
re-export features to the facade crate.

## Overview

| Crate (dir) | Package | Ports | Upstream (pinned) | License |
|-------------|---------|-------|-------------------|---------|
| `crates/airwindows` | `music-things-airwindows` | Airwindows plugin framework + batch-1 plugins | airwindows/airwindows @ `b110fac3991fff7c4655cc2d25d6a3e221f9799c` (`plugins/LinuxVST/src/<Name>/`) | MIT |
| `crates/soundfont` | `music-things-soundfont` | `tsf.h`, `tml.h` | schellingb/TinySoundFont @ `853a0a171759f1ddba0de1442133a75912bbeffa` | MIT |
| `crates/pitch` | `music-things-pitch` | smbPitchShift 1.2 (ESP32 source), stftPitchShift `cpp/` core, dywapitchtrack 0.1.0.1 `cbits` | jurihock/stftPitchShift @ `70c96db087458f8631edbc1bb4068d42ea5b52cc`; others as listed in `THIRD_PARTY_NOTICES.md` | WOL, MIT, MIT |
| `crates/music-things` | `music-things` | optional features `airwindows`, `soundfont`, `pitch` (re-exports only) | - | MIT |

Global rules (apply to every crate and plan):

- **Source boundary**: port only from the pinned upstream checkouts. The operator's checkout lives at
  `$MUSIC_THINGS_REFS` (see its `REFERENCES.md`). Never read Music Tribe AGPL code (KICK/IGNITE
  `AudIO`, `Kick`, `Source`). Upstream C/C++ is never copied into this repository. The harnesses
  under `tools/` compile it in place from `$MUSIC_THINGS_REFS`.
- **Notices**: every ported Rust file starts with the upstream copyright/license header, the upstream
  file path and the pinned revision. `THIRD_PARTY_NOTICES.md` records the URL, revision and full license
  text per component.
- **Dependencies**: none. `[dev-dependencies]` stay empty, and test helpers live in `tests/common/`.
- **Targets**: no host or engine types. Everything builds for `wasm32-unknown-unknown` under the
  toolchain pinned in `mise.toml` (Rust 1.83). Tests may use `std::fs`. Library code does no I/O.
- **Realtime**: allocation happens only in constructors and loaders. `process`/`render`/`note_on`/
  channel and controller calls/`push` never allocate, and a counting-global-allocator test per crate
  enforces this.
- **File size**: every Rust file stays under 1000 lines. Large ports are split by concern (module lists below).
- **Workspace**: the root `Cargo.toml` is untouched (`members = ["crates/*"]`). `Cargo.lock` gains
  path-package entries only, written by the scaffold plan and again by the facade plan.

---

## A. Airwindows (`crates/airwindows`)

### Public API

- `trait AirwindowsEffect: Send` (object safe):
  - `info(&self) -> &'static PluginInfo`
  - `param_count(&self) -> usize`, `param_info(&self, index) -> Option<&'static ParamInfo>`
  - `get_param(&self, index) -> f32`. `set_param(&mut self, index, value: f32)` clamps to `[0, 1]`,
    ignores NaN and ignores an out-of-range index.
  - `param_display(&self, index) -> String` and `param_label(&self, index) -> &'static str`. These
    reproduce upstream `getParameterDisplay` / `getParameterLabel` for the current value.
  - `reset(&mut self)` restores the post-construction DSP state in place (buffers zeroed, fpd re-derived
    from the construction seed, rand stream replayed). Parameters and sample rate are kept. It does not allocate.
  - `process(&mut self, in_l: &[f32], in_r: &[f32], out_l: &mut [f32], out_r: &mut [f32])` processes
    `min(len)` frames and mirrors upstream `processReplacing` (the float path, including the fpd dither).
- `PluginInfo { name, upstream_name, description, params: &'static [ParamInfo] }` and
  `ParamInfo { name, label, default: f32 }`. `name` is the KICK list name. `upstream_name` is the
  upstream directory/class name (they differ only for `Console8BusOut` -> `Console8BussOut`).
- Factory and catalog in `registry.rs`:
  - `create(name, sample_rate: f32) -> Option<Box<dyn AirwindowsEffect>>`
  - `create_with_seed(name, sample_rate: f32, seed: u32) -> Option<Box<dyn AirwindowsEffect>>`
  - `catalog() -> impl Iterator<Item = &'static PluginInfo>`, in kick-list order
  - Lookup is case-sensitive and matches `name` or `upstream_name`. `create` uses `DEFAULT_SEED`.
- Concrete plugin types are `pub(crate)`. The stable public surface is the trait plus the factory and
  catalog, so the internal batch/group layout never leaks into the public API.

`description` is the plugin's entry text from the pinned upstream `what.txt`, verbatim. P1 records
the entry-extraction rule in `tools/airwindows-harness/README.md`. The harness writes the
extracted text into each fixture's metadata, and the parity test asserts equality with it.

### Framework internals (frozen after P1)

- `dither.rs`: `fpd_from_seed(seed, lane) -> u32`. It mixes `seed + lane * 0x9E3779B9` with the
  lowbias32 hash (`x ^= x>>16; x *= 0x7feb352d; x ^= x>>15; x *= 0x846ca68b; x ^= x>>16`) and adds
  16386 if the result is below 16386, which keeps upstream's `fpd >= 16386` invariant. Lane 0 seeds fpdL, lane 1 fpdR,
  lane 2 seeds the rand stream. The module also holds the xorshift step (`<<13, >>17, <<5` on `u32`),
  `frexpf_exponent(f32) -> i32` (bit-level, matching C `frexpf` for zero, subnormals and normals)
  and the common float-path dither helper `fpd * 5.5e-36 * 2^(expon+62)`, where the power of two is
  built exactly.
- `rand.rs`: `AwRand` replaces libc `rand()` with a deterministic stream (xorshift32 seeded from lane 2,
  returning `state >> 1`, `RAND_MAX = 0x7fffffff`). Upstream constructors seed fpd with
  `while (fpdL < 16386) fpdL = rand()*UINT32_MAX;`. A Rust constructor replays the upstream
  constructor's `rand()` call sequence in order: it discards the fpd-loop draws and overrides fpd
  with `fpd_from_seed`. Any later `rand()`-dependent state therefore matches the harness, which
  seeds the same stream before construction and overrides fpd after it.
- `display.rs`: clean-room, VST2-style `float2string`, `dB2string` (`v <= 0` -> `-oo`, else
  `20*log10(v)` through `float2string`) and `int2string`. They follow one written behavioral spec,
  which is implemented identically in the MIT shim (`tools/vst-shim`) and in Rust. The shim dumps a
  vector of formatting results to `crates/airwindows/tests/fixtures/shim_format_vectors.txt`, and a
  test locks the two implementations together. See Divergences D1.
- `registry.rs`, `plugins/mod.rs`: aggregation (below).

### Additive registration (no hand-edited global registry)

```
src/plugins/mod.rs                 framework: mod batch1; mod batch2; mod batch3;
                                   BATCHES = [batch1::GROUPS, batch2::GROUPS, batch3::GROUPS]
src/plugins/batch1/mod.rs          batch-1 registration: mod g01..g14; GROUPS = [g01::ENTRIES, ..]
src/plugins/batch1/gNN/mod.rs      group registration: mod <plugin>; ENTRIES = [<plugin>::ENTRY, ..]
src/plugins/batch1/gNN/<plugin>.rs one plugin (struct + PluginInfo + ENTRY + constructor)
src/plugins/batch2/mod.rs          GROUPS = [] (owned and filled by the batch-2 run)
src/plugins/batch3/mod.rs          GROUPS = [] (owned and filled by the batch-3 run)
```

- `PluginEntry { info: &'static PluginInfo, construct: fn(f32, u32) -> Box<dyn AirwindowsEffect> }`.
- P1 creates `batch1/mod.rs` declaring all 14 groups, and every `gNN/mod.rs` with `ENTRIES = &[]`.
  Each group plan then edits only its own `gNN/mod.rs`, its plugin files, its test file and its fixtures.
- The empty `batch2`/`batch3` registration files exist only so later batches never edit `plugins/mod.rs`.
  No batch-2/3 plugins are stubbed.
- Plugin module name = KICK name lowercased (`adclip7`, `console8busout`). Struct name = upstream
  class name. Rare large plugins may become `<plugin>/mod.rs` plus submodules, kept under 1000 lines.
- Arrays above 4096 elements are `Box<[T]>`, allocated in the constructor, which avoids stack overflow
  when the plugin is boxed.

### Batch-1 grouping (fixed, by line of `kick-airwindows-set.txt`)

| Group | Lines | Group | Lines |
|-------|-------|-------|-------|
| g01 | 1-8 | g08 | 57-64 |
| g02 | 9-16 | g09 | 65-72 |
| g03 | 17-24 | g10 | 73-80 |
| g04 | 25-32 | g11 | 81-88 |
| g05 | 33-40 | g12 | 89-96 |
| g06 | 41-48 | g13 | 97-104 |
| g07 | 49-56 | g14 | 105-110 |

Line 1 is `Acceleration` and line 110 is `DitherMeDiskers`. The implementation plans enumerate the
names per group from the pinned list. Each group plan holds at most 8 plugin modules, which fits
the plan-size limit in `impl-plans/README.md`.

### Porting rules (determine parity)

1. Port `<Name>.h` (state, constructor defaults), `<Name>.cpp` (parameter metadata/display) and
   `<Name>Proc.cpp` `processReplacing` only. `processDoubleReplacing`, chunks and programs are out of scope.
2. Precision follows C++ exactly. `float` members become `f32` and `double`/`long double` become `f64`.
   `long double` equals `double` on the generation host, and the harness asserts this. Mixed float/double
   expressions are evaluated in `f64` and rounded where C++ assigns to `float`. Libm calls use the
   precision chosen by C++ overload resolution (a `float` argument resolves to `sinf` and the like).
3. Operation order is preserved, and no algebraic simplification is allowed (for example, `pow(x,2)` is
   not rewritten as `x*x`). Integer arithmetic uses wrapping ops wherever C++ unsigned wraps. Float-to-int
   casts must only see values that upstream keeps in range; a cast whose input could leave that range
   is a documented per-plugin deviation.
4. `getSampleRate()` is the construction `sample_rate: f32`, and `overallscale` is computed as upstream does.
5. fpd seeding and `rand()` follow `dither.rs`/`rand.rs` above. No other source of nondeterminism is permitted.

### Parity method (Airwindows)

- **Harness** (`tools/`, MIT, written from scratch):
  - `tools/vst-shim/audioeffectx.{h,cpp}`: the minimal `AudioEffect`/`AudioEffectX` surface that
    the 110 batch-1 plugins use (types, `getSampleRate`, `vst_strncpy` with a 64-byte limit so names
    are not truncated, the display helpers per D1, and no-op host calls).
  - `tools/airwindows-harness/prelude.h`: force-included into every plugin translation unit. It
    includes the standard headers, `static_assert(sizeof(long double) == sizeof(double))`, maps
    `rand()` to the `AwRand` C twin, and then defines `private`/`protected` as `public` so the driver
    can set `fpdL`/`fpdR`.
  - `tools/airwindows-harness/driver.cpp`: renders the stimulus, dumps metadata, and writes fixtures. It detects `fpdL`/`fpdR`
    at compile time, so older plugins without fpd still work.
  - `tools/airwindows-harness/generate.sh [--work-dir DIR] [NAME...]`: builds one executable per plugin
    from `$MUSIC_THINGS_REFS/airwindows/plugins/LinuxVST/src/<Upstream>/` with
    `clang++ -std=c++17 -O1 -ffp-contract=off -fno-fast-math -fno-builtin`, runs it, and writes fixtures.
    Build products go to `tmp/behringer-oss-ports/<plan>/`.
  - `tools/common/stimulus.h` and `tools/common/fnv.h` (owned by P0): the shared stimulus and checksum.
    Each crate keeps its Rust twin in `tests/common/`. Every fixture records the stimulus checksum,
    and every parity test asserts it, which catches drift between the C and Rust generators.
- **Stimulus** (`tools/common/stimulus.h` mirrors `crates/airwindows/tests/common/stimulus.rs`). It is
  8192 stereo frames generated from basic IEEE ops only:
  - `[0,256)`: an impulse at frame 0 (L 0.9, R -0.5), then silence. This exercises fpd on near-silent input.
  - `[256,4096)`: a phase-accumulator parabolic-sine tone (L 220 Hz, R 330 Hz at the render rate), with
    amplitude ramping linearly from 0.05 to 1.2 so clipping thresholds are crossed.
  - `[4096,6144)`: xorshift32 noise at 0.5 amplitude.
  - `[6144,8192)`: silence, which exposes tails.
  An FNV-1a-64 checksum of the stimulus bits goes into each metadata file, and the Rust test asserts it.
- **Renders per plugin**, each in 128-frame blocks with parameters set before the first block and seed `0x00C0FFEE`:
  - R0: defaults at 44100 Hz.
  - R1: the alternate set at 44100 Hz. Parameter `i` takes `ALT[i % 9]` with
    `ALT = [0.30, 0.85, 0.15, 0.65, 0.95, 0.45, 0.05, 0.70, 0.25]` (f32 literals).
  - R2: the alternate set at 96000 Hz, which exercises `overallscale`.
- **Stored frames**: frames 0-255 in full, plus every 32nd frame from 256 to 8191 (248 frames), for
  504 stereo frames per render. Fixtures live in `tests/fixtures/batch1/gNN/` (one directory per
  group, so group plans own disjoint directories). `<Name>.f32` holds R0, R1, R2 interleaved L/R as
  f32 LE (12,096 bytes).
  `<Name>.txt` holds the metadata (`key=value` lines):
  - upstream name, harness clang version and host triple, stimulus checksum, `what.txt` description
  - parameter count, then per parameter the name, label, default, and display/label at R0 and R1 values
- **Rust test** (`tests/parity_batch1_gNN.rs`, one per group): each plugin is constructed through the
  factory by name, which proves registration. The test checks the metadata and description, renders
  R0-R2 with the same block size, and compares the stored frames against the plugin's `ParityRecord`.
  A failure reports the plugin, render, frame, channel, expected and actual values.
- **Fixture budget**: about 13 KB per plugin, so roughly 1.45 MB for 110 plugins. A test asserts that
  `crates/airwindows/tests/fixtures/` is at most 1.6 MB.

### Tolerances (Airwindows)

Each plugin declares a `ParityRecord { name, class, measured_max_abs, note }` in its group test file.
It repeats the record in its module header doc comment.

| Class | Assertion | When |
|-------|-----------|------|
| `Exact` | bit-identical f32 output and identical metadata | data path uses only IEEE basic ops, sqrt/fabs/floor/frexp and exact powers of two |
| `Libm` | max-abs <= 1e-6 (about -120 dBFS) and identical metadata | data path calls libm transcendentals (sin/cos/tan/atan/exp/log/pow) |
| `Custom(x)` | max-abs <= x, where x > 1e-6 needs a written reason in `note` and in the module header | measured deviation that is understood and accepted |

On the generation host (aarch64-apple-darwin), Rust and the harness share the system libm, so every
plugin is expected to measure 0. A plugin is `Custom` only after investigation. `measured_max_abs` is
the value seen on the generation host. The `Libm` bound covers ulp differences across hosts (glibc,
wasm libm) and is not calibrated against a second host (see Risks).

---

## B. SoundFont (`crates/soundfont`)

### API (1:1 with the tsf/tml public functions, renamed to Rust style)

- `SoundFont::load(bytes: &[u8]) -> Result<SoundFont, LoadError>`. This is `tsf_load_memory`: RIFF/sfbk
  parsing, preset/instrument/region generator resolution, and 16-bit plus `sm24` sample decoding to f32.
  `SoundFont` is immutable and `Send + Sync`. Preset queries are `preset_count`, `preset_index(bank, number)`,
  `preset_name(index)` and `bank_preset_name(bank, number)`.
- `Synth::new(font: Arc<SoundFont>, SynthConfig { output_mode, sample_rate: u32, global_gain_db: f32, max_voices: usize })`.
  It covers `tsf_set_output`, `tsf_set_max_voices` and `tsf_copy` (shared font). `OutputMode` is one of
  `StereoInterleaved`, `StereoUnweaved` or `Mono`. The voice pool (`max_voices`) and 16 channels are
  preallocated here.
- Voice API: `note_on`, `bank_note_on`, `note_off`, `bank_note_off`, `note_off_all`,
  `active_voice_count`, `set_volume`, `reset`. Rendering is `render_float(&mut self, out: &mut [f32], frames, mixing: bool)`.
- Channel API: every public `tsf_channel_*` function in the pinned `tsf.h` (setters, getters,
  note on/off, all notes/sounds off, and `channel_midi_control`), with the `tsf_` prefix dropped and
  integer channel indices kept. P16c lists the exact set from the pinned header. Omitting any public
  `tsf_*` function other than those excluded below is a defect.
- `midi::parse(bytes: &[u8]) -> Result<Vec<MidiMessage>, MidiError>` ports `tml_load_memory`.
  `MidiMessage` has `time_ms: u32`, `kind`, `channel` and the per-kind data fields of `tml_message`.
  `midi::info(&[MidiMessage])` ports `tml_get_info`, and `MidiMessage::tempo_value` ports `tml_get_tempo_value`.
- Out of scope: file and stream loaders (hosts pass bytes) and `tsf_render_short`, because the API is f32 only.

Modules (three disjoint public modules, so the work can be split across plans):

- `midi/` (tml): `mod.rs`, `parse.rs`, `info.rs`.
- `font/` (SF2 loading, which produces `font::SoundFont` and `font::LoadError`): `mod.rs`, `riff.rs`,
  `hydra.rs`, `region.rs`, `preset.rs`. The internal `Region`/`Preset`/`Envelope` structs mirror tsf's
  `tsf_region`/`tsf_preset`/`tsf_envelope` field for field (snake_case names) and are `pub(crate)`
  for `synth/`.
- `synth/` (runtime, which produces `synth::{Synth, SynthConfig, OutputMode}`): `mod.rs`,
  `envelope.rs`, `lfo.rs`, `filter.rs`, `voice.rs`, `channel.rs`, `render.rs`.

Each file stays under 1000 lines.

### Realtime behavior

- `note_on` never grows the pool. Under exhaustion it mirrors tsf's behavior with
  `tsf_set_max_voices` set; the harness sets the same limit, so parity covers this.
- Channel numbers at 16 or above are ignored, whereas tsf grows its channel array (Divergence D4).
- Below 16, channels activate lazily exactly as in tsf, which is not a divergence. A `channel_num`
  count mirrors tsf `channelNum`:
  - Setters, and anything upstream routes through `tsf_channel_init`, activate channels `0..=ch` with
    the `tsf_channel_init` defaults, using preallocated storage.
  - `channel_note_on` and the getters are guarded by `channel >= channelNum` and return tsf's
    fallbacks on inactive channels. `channel_note_off`, `channel_note_off_all` and
    `channel_sounds_off_all` are not guarded upstream: they scan voices by `playingChannel`
    regardless of `channelNum`, and the port mirrors that.
  - `reset()` ends voices and sets `channel_num` back to 0, as `tsf_reset` does.

### Parity method (SoundFont)

- Test inputs are synthetic, built in `tests/common/sf2_builder.rs` and `tests/common/midi_builder.rs`.
  No copyrighted SoundFont is used.
  - The SF2 has two melodic presets and a bank-128 drum preset. It covers key and velocity ranges,
    loop modes, root key and tuning, pan, the volume and modulation envelopes, the mod and vib LFOs
    routed to pitch, filter and volume, and the initial filter Fc/Q, using 16-bit sine and saw samples.
  - The MIDI file is format 1 with a tempo change and note, CC (volume, pan, sustain, bank select),
    program-change and pitch-bend events.
- Ignored `export_*` tests write the synthetic bytes to `$SOUNDFONT_FIXTURE_OUT`. Three scripts under
  `tools/soundfont-harness/` (`generate_tml.sh`, `generate_font.sh`, `generate_synth.sh`) run the
  export test and compile their C harness with `TML_IMPLEMENTATION`/`TSF_IMPLEMENTATION` from
  `$MUSIC_THINGS_REFS/TinySoundFont`, using the same clang flags. They write:
  - `tests/fixtures/tml/`: one line per parsed message plus `tml_get_info`, per synthetic MIDI file.
  - `tests/fixtures/font/`: a dump of the loaded presets and regions (tsf's post-load region fields),
    which proves load parity separately from rendering.
  - `tests/fixtures/synth/render_s1.f32`: stereo interleaved, 44100 Hz, about 2 s. It is driven by the
    parsed MIDI applied through the channel API at 64-frame block starts, with `max_voices` 32.
  - `tests/fixtures/synth/render_s2.f32`: mono mode with mixing, 8 voices so that exhaustion is
    reached, pitch wheel and range changes, sustain, and `note_off_all`/`sounds_off_all`.
  - `inputs.txt` in each directory: the FNV-1a-64 checksum of the SF2/MIDI bytes, so builder drift is
    detected.
- Stored frames are frames 0-2047 in full, then every 16th frame. That is about 59 KB per render.
  Budget assertion: `crates/soundfont/tests/fixtures/` is at most 0.4 MB.
- Tolerances:
  - tml: exact equality of every field.
  - font load dump: integer fields exact. Float fields within relative 1e-6, because load-time
    timecent and centibel conversions call powf.
  - tsf render: max-abs <= 1e-5. The justification is that powf, expf, tan and log10 feed per-voice
    coefficients that differ by ulps across libms. Exact equality is expected on the generation host,
    and the measured value is recorded in the test.

---

## C. Pitch (`crates/pitch`)

Modules: `lib.rs`, `smb.rs`, `stft/{mod.rs, rfft.rs, core.rs, stft.rs, vocoder.rs, pitcher.rs, resampler.rs, cepster.rs, stream.rs}`, `dywa.rs`.

- **smbPitchShift** (`smb.rs`): a port of `smbPitchShift.esp32.cpp`.
  - The pinned upstream interface is
    `smbPitchShiftInit(float pitchShift, long fftFrameSize, long osamp, float sampleRate)`, which fixes
    the shift and `calloc`s buffers sized by `fftFrameSize`, followed by
    `smbPitchShift(size_t n, int16_t *indata, int16_t *outdata)`.
  - Upstream converts input with `(float)(indata[i]) / INT_FLT` and output with
    `(int16_t)(gOutFIFO[...] * INT_FLT)`, a truncating cast, where `INT_FLT` is 32768.0.
  - Upstream keeps its state in file statics (including `gRover` and `gInit`).
  - Rust API (Divergence D7):
    - `SmbPitchShifter::new(pitch_shift: f32, fft_frame_size: usize, osamp: usize, sample_rate: f32) -> Result<_, ConfigError>`
      ports `smbPitchShiftInit`. The shift is fixed at construction, as upstream fixes it, and all
      buffers are sized by `fft_frame_size`.
    - `process(&mut self, input: &[f32], output: &mut [f32])` streams like upstream and keeps upstream's
      latency (`fft_frame_size - step_size`).
    - The f32 samples are exactly the values upstream holds after its input conversion and before its
      output conversion. Both int16 conversions are omitted.
    - The source's own `smbFft` is ported as-is so precision matches.
- **stftPitchShift** (`stft/`): ports the float instantiation of the `cpp/StftPitchShift` core (STFT,
  Vocoder, Pitcher, Resampler, Cepster formant lifter, normalization, distortion and multi-factor
  shifting).
  - `shift_pitch(&mut self, input: &[f32], output: &mut [f32])` mirrors the upstream offline
    `StftPitchShift::shiftpitch`.
  - `StftPitchShiftStream` builds on the same core. Its `process(chunk_in, chunk_out)` has a fixed,
    documented latency and does not allocate.
  - Configuration is `StftConfig { analysis_framesize, synthesis_framesize, hopsize, sample_rate: f32,
    quefrency, distortion, normalization, max_factors }`. This mirrors upstream's
    `std::tuple<size_t,size_t>` framesize, and the asymmetric analysis/synthesis window path in `STFT.h`
    is ported; the symmetric case sets both sizes equal.
  - Validation equals upstream's checks (for example analysis >= synthesis, and the RFFT size rule
    below). Hop size has no "divides framesize" rule. The only check added beyond upstream is
    `hopsize > 0` (Divergence D8).
  - `set_factors(&[f32])` accepts between 1 and `max_factors` entries.
  - `stft/rfft.rs` ports the pinned header-only real FFT `cpp/StftPitchShift/RFFT.h`, together with
    the `FFT.h` interface it implements. These are what upstream `StftPitchShift.cpp` instantiates via
    `std::make_shared<RFFT>()`. The port keeps the upstream operation order and the
    `std::complex`/`std::polar` arithmetic semantics. Frame sizes follow upstream's own check (a power
    of two, as `std::has_single_bit` tests it, for both the full size and the half size, so size 1 is
    rejected), and other sizes are rejected with `ConfigError`. Upstream throws in the same cases, so
    this is not a divergence. No other FFT is written.
- **dywapitchtrack** (`dywa.rs`): a port of `dywapitchtrack.c`.
  - `PitchTracker::new(sample_rate: f32, window: usize, hop: usize)` preallocates the scratch buffer that upstream
    `malloc`s on every call. `compute(&mut self, samples: &[f32]) -> f64` converts samples to f64 the
    same way the harness does, and returns Hz, where 0.0 means no pitch. The upstream tracking smoothing
    is ported.
  - `push(&mut self, input: &[f32]) -> Option<f64>` is the streaming form, with the hop set at construction.
  - Any 44100 Hz constant hardcoded upstream becomes the configured sample rate, and fixtures run at
    44100 Hz (Divergence D3).
- **Parity** (`tools/pitch-harness/{smb,stft,dywa}/`, each with its harness source and `generate.sh`).
  All three use the parity flags `-O1 -ffp-contract=off -fno-fast-math -fno-builtin`.
  `smb_harness.cpp` builds with `clang++ -std=c++17`. `stft_harness.cpp` builds with
  `clang++ -std=c++20`, because the pinned core includes `<bit>` and `<span>` and uses
  `std::has_single_bit`. `dywa_harness.c` builds with `clang` as C, with
  `-I $MUSIC_THINGS_REFS/dywapitchtrack/dywapitchtrack-0.1.0.1/include`. The smb harness may supply
  minimal from-scratch stub headers (for example `appGlobals.h`) for ESP32-only includes. The stimulus comes from `tools/common/stimulus.h` (a basic-op parabolic-sine
  harmonic tone with a pitch glide, plus noise), and its checksum is asserted.

| Component | Fixture | Assertion |
|-----------|---------|-----------|
| smb | 2 configs (shift 1.5 at frame 1024 / osamp 4, and shift 0.75 at frame 2048 / osamp 8), 0.25 s mono, full, stored as upstream's int16 output (`c1.i16`/`c2.i16`, i16 LE). See the int16 bridge below | compared in the int16 domain: max-abs <= 3 LSB (3/32768, about 9.2e-5). Upstream's truncating int16 cast turns a 1-ulp float difference at a boundary into 1 LSB, and phase accumulation spreads libm ulp differences across hosts. 0 LSB is expected on the generation host, and the measured value is recorded |
| stft offline | 3 configs (C1 symmetric, single factor with quefrency; C2 symmetric, 2 factors with normalization; C3 asymmetric analysis/synthesis frame sizes), 0.25 s, full | max-abs <= 1e-5 at stimulus peak 0.5. The FFT and core are direct ports evaluated in f32 with the same operation order, so only the libm ulp differences of sin/cos/atan2/hypot/exp/log across hosts remain, spread through phase accumulation and the cepstral lifter. 0 is expected on the generation host, and the measured value is recorded |
| stft stream | none (internal) | the stream output equals the offline output delayed by the stated latency, over the region past the first frame |
| dywa | pitch per window as text, 2 window sizes | relative error <= 1e-9 and identical pitch/no-pitch decisions |

**smb int16 bridge** (the parity rule for D7):
- The harness quantizes the stimulus with `q[i] = (int16_t)(x[i] * 32768.0)`, a truncating cast. It
  is in range because the stimulus peak is at most 0.5.
- It runs upstream `smbPitchShiftInit` and then `smbPitchShift` in 256-sample chunks, one process per
  config because upstream state is static. It stores the int16 output.
- The harness asserts `|out| < 32767` for every produced sample, without editing upstream.
- The Rust test feeds `q[i] as f32 / 32768.0`, which is bit-identical to upstream's input conversion.
  It asserts `|y * 32768| < 32767` for every f32 output `y`, which (given parity) rules out undefined
  casts upstream. It maps each output through `(y as f64 * 32768.0).trunc()` to i16 and compares in i16.

Budget assertion: `crates/pitch/tests/fixtures/` is at most 0.6 MB. All three crate budgets add up to
2.6 MB, which is within the 3 MB cap for the batch.

**Out-of-budget rule**: a measured error above a stated bound is a defect to investigate. The bound
is raised only by updating this document with the justification.

---

## D. Facade (`crates/music-things`)

`[features]` `airwindows`, `soundfont` and `pitch` each enable an optional path dependency. No feature
is on by default. `lib.rs` adds `#[cfg(feature = "x")] pub use music_things_x as x;` and nothing else.

---

## Plans, write ownership and order

Every plan declares the artifact roots `target/` and `tmp/behringer-oss-ports/<plan-id>/`, never a bare `tmp/`.

| Plan | Depends on | Owns (writePaths) |
|------|------------|-------------------|
| P0 scaffold (`bop-00`) | - | The three crate manifests and `Cargo.lock`. Minimal `lib.rs` files: airwindows has a doc comment only; soundfont declares `midi`, `font`, `synth`; pitch declares `error`, `smb`, `stft`, `dywa`. Stub module roots, plus the pinned `crates/pitch/src/error.rs` (`ConfigError`). `tools/common/` (C stimulus, FNV, checksum printer) and its Rust twins `tests/common/{stimulus,fnv}.rs` with `tests/stimulus_checksum.rs` (airwindows, pitch). `tests/common/fnv.rs` (soundfont). `tests/fixture_budget.rs` (soundfont, pitch) |
| P1 airwindows-framework (`bop-01`) | P0 | `crates/airwindows/src/` framework files, `src/plugins/mod.rs`, `src/plugins/batch{1,2,3}/mod.rs`, the 14 empty `src/plugins/batch1/gNN/mod.rs`, `tests/common/{mod,parity,fixture}.rs`, `tests/{catalog,no_alloc,fixture_budget}.rs`, `tests/fixtures/shim_format_vectors.txt` (checked by a unit test in `display.rs`), the `tests/fixtures/batch1/` tree for all 110 plugins, `tools/vst-shim/`, `tools/airwindows-harness/`, `tools/README.md` |
| P2-P15 group g01-g14 (`bop-02`..`bop-15`) | P1 | The directory `src/plugins/batch1/gNN/`, `tests/parity_batch1_gNN.rs`, and the directory `tests/fixtures/batch1/gNN/` (regenerated only after a P1 harness fix) |
| P16a soundfont-midi (`bop-16a`) | P0 | `crates/soundfont/src/midi/`, `tests/common/midi_builder.rs`, `tests/midi_parity.rs`, `tests/fixtures/tml/`, `tools/soundfont-harness/{tml_harness.c,generate_tml.sh}` |
| P16b soundfont-font (`bop-16b`) | P0 | `crates/soundfont/src/font/`, `tests/common/sf2_builder.rs`, `tests/font_parity.rs`, `tests/fixtures/font/`, `tools/soundfont-harness/{font_harness.c,generate_font.sh}` |
| P16c soundfont-synth (`bop-16c`) | P16a, P16b | `crates/soundfont/src/synth/`, `tests/{synth_parity,synth_no_alloc}.rs`, `tests/fixtures/synth/`, `tools/soundfont-harness/{synth_harness.c,generate_synth.sh}` |
| P17a pitch-smb (`bop-17a`) | P0 | `crates/pitch/src/smb.rs`, `tests/{smb_parity,smb_no_alloc}.rs`, `tests/fixtures/smb/`, `tools/pitch-harness/smb/` |
| P17b pitch-stft (`bop-17b`) | P0 | `crates/pitch/src/stft/`, `tests/{stft_parity,stft_stream,stft_no_alloc}.rs`, `tests/fixtures/stft/`, `tools/pitch-harness/stft/` |
| P17c pitch-dywa (`bop-17c`) | P0 | `crates/pitch/src/dywa.rs`, `tests/{dywa_parity,dywa_no_alloc}.rs`, `tests/fixtures/dywa/`, `tools/pitch-harness/dywa/` |
| P18 integration (`bop-18`) | all | `crates/music-things/{Cargo.toml,src/lib.rs}`, `Cargo.lock`, `crates/airwindows/tests/batch1_complete.rs` (the exact 110 names in order), `THIRD_PARTY_NOTICES.md`, `README.md`, `AGENTS.md` project-structure section |

Waves: W1 = P0. W2 = P1, P16a, P16b, P17a, P17b, P17c. W3 = P2-P15, P16c. W4 = P18.
Test helpers that more than one plan needs are owned by P0, or by P1 for the airwindows parity
helpers. Pitch and soundfont tests pull helpers in with `#[path = "common/<file>.rs"] mod <name>;`
instead of a shared `tests/common/mod.rs`.

P1 completion includes building and running the harness for all 110 batch-1 plugins. The shim is
therefore complete before any group plan starts, and group plans never edit `tools/`. Every plan
leaves the workspace compiling, with empty groups registering nothing.

- **Harness-defect escalation**: when a group plan finds a defect on the `tools/` side, it records it
  in its progress log and marks the affected plugins blocked. The fix is a follow-up task on the P1 plan
  (the owner of `tools/`). Once that fix lands, the group plan regenerates only its own fixtures.
- **Concrete paths**: globs and `<Name>` patterns in this table are expanded into concrete file lists
  by the implementation plans. A plan whose expanded list would exceed the 512-entry fanout cap is
  split, for example by moving P1's initial fixture generation into a separate plan per half of the
  group range.

### Additive procedure for batches 2 and 3

1. Partition the batch's kick-list lines into groups of at most 8, named `batchN/gNN`.
2. The batch's first plan fills `src/plugins/batchN/mod.rs` (group declarations and `GROUPS`), creates
   empty `gNN/mod.rs` files, and generates fixtures with `tools/airwindows-harness/generate.sh <names>`.
   It extends `tools/vst-shim` only additively, if a new plugin needs a missing symbol.
3. Group plans add `<plugin>.rs` files, their `ENTRIES`, `tests/parity_batchN_gNN.rs` and their own fixtures.
4. Framework files (`lib.rs`, `effect.rs`, `params.rs`, `dither.rs`, `rand.rs`, `display.rs`,
   `registry.rs`, `plugins/mod.rs`) stay untouched. Each batch has its own fixture budget of at most 3 MB.

## Verification

```sh
export CARGO_TERM_QUIET=true NEXTEST_STATUS_LEVEL=fail NEXTEST_FAILURE_OUTPUT=immediate-final NEXTEST_HIDE_PROGRESS_BAR=1
cargo nextest run --workspace
cargo build --workspace --target wasm32-unknown-unknown
cargo build -p music-things --all-features --target wasm32-unknown-unknown
cargo clippy --workspace --all-targets -- -D warnings
cargo clippy -p music-things --all-features -- -D warnings
cargo fmt --check
tools/airwindows-harness/generate.sh --work-dir tmp/behringer-oss-ports/<plan>/harness   # P1 only; requires $MUSIC_THINGS_REFS
```

## Intentional divergences

- **D1**: the VST2 SDK is unavailable, so `float2string`/`dB2string`/`int2string` follow the clean-room
  spec in the MIT shim. Display parity means Rust matches the upstream per-plugin display logic driven
  through that shim. Character-exact fidelity to Steinberg's SDK formatting is not claimed.
- **D2**: `StftPitchShiftStream` (the realtime streaming wrapper) has no upstream equivalent. It reuses
  the ported core and is verified only against the ported offline path, not against C++. The offline
  `shift_pitch` and the RFFT port follow upstream exactly, including its power-of-two frame-size rule.
- **D3**: dywapitchtrack takes an explicit sample rate. It is identical to upstream at 44100 Hz. Its
  tracker struct comes from `dywapitchtrack-0.1.0.1/include/dywapitchtrack.h`; `cbits/dywapitchtrack.c`
  includes it.
- **D4**: tsf channels are capped at 16 (storage preallocated; below the cap, channels still activate
  lazily as in tsf), and the voice pool is fixed at construction, so no realtime allocation happens.
  File and stream loaders and `render_short` are omitted.
- **D5**: fpd seeding is deterministic from a `u32` seed, and `rand()` is a deterministic stream, in
  place of libc `rand()`.
- **D6**: there is no `processDoubleReplacing`, chunk or program support. `reset()` is added (it has
  no upstream equivalent).
- **D7**: smbPitchShift uses f32 I/O instead of upstream's int16 PCM. Rust omits both int16
  conversions (`/ INT_FLT` on input, the truncating `* INT_FLT` cast on output), and the f32 values
  are upstream's internal float samples. The pitch shift stays fixed at construction, as upstream
  fixes it in `smbPitchShiftInit`. Parity is proven through the int16 bridge in section C.
- **D8**: constructors return `ConfigError` for configurations that upstream would accept with
  undefined behavior:
  - smb: a frame size that is not a power of two or exceeds `MAX_FRAME_LENGTH` (8192); `osamp == 0`;
    a frame size not divisible by `osamp`
  - stft: `hopsize == 0`
  - dywa: a window that is not a power of two, or is below upstream's needed sample count; `hop == 0`
  All configurations upstream handles are accepted unchanged.

## Risks

- Cross-host libm differences may exceed the `Libm` or tsf/smb bounds on non-macOS hosts, which have
  not been measured. The acceptance host is aarch64-apple-darwin.
- Fixtures are reproducible only where `long double == double` and no FMA contraction occurs. The
  harness enforces both.
- Scale: the 110 plugin ports are spread over 14 parallel group plans.

## References

See `design-docs/references/README.md` (Behringer OSS ports section).
