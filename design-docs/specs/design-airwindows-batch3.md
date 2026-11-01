# Airwindows Batch 3 (PowerSag .. ZRegion2)

Batch 3 adds Rust ports of the 109 Airwindows plugins on lines 221-329 of
`$MUSIC_THINGS_REFS/kick-airwindows-set.txt` to `crates/airwindows`. This document is a delta on
`design-docs/specs/design-behringer-oss-ports.md` (the baseline). The baseline's section A (public API,
framework internals, additive registration, porting rules, parity method, tolerances), its
"Additive procedure for batches 2 and 3" and divergences D1, D5 and D6 apply unchanged. This file
records only what batch 3 adds or has to resolve. It is a separate file so that the concurrent batch-2
run, which may edit the baseline, never conflicts with it.

## Scope

- Upstream: airwindows @ `b110fac3991fff7c4655cc2d25d6a3e221f9799c` (MIT),
  `$MUSIC_THINGS_REFS/airwindows/plugins/LinuxVST/src/<Name>/`. Never read Music Tribe AGPL code.
- Each plugin ports `<Name>.h`, `<Name>.cpp` and the `processReplacing` float path of
  `<Name>Proc.cpp`, including its fpd dither (baseline porting rules 1-5).
- No batch-3 plugin is name-mapped: `name == upstream_name` for all 109 (the `Console8BusOut`
  mapping belongs to batch 1).
- No upstream `<Name>Proc.cpp` exceeds 1000 lines. The largest are Righteous4 (949), PurestSquish (948),
  ToVinyl4 (880) and Reverb (764). No plugin needs its own plan.
- **Completeness rule**: every one of the 109 plugins is fully ported, registered and parity-tested.
  A stub, an unregistered module, a skipped or `#[ignore]`d test, or a plugin marked "blocked" is a
  failure of its plan, not a deferral. Batch 1's "mark BLOCKED with evidence" exit does not exist in
  batch 3. A harness defect blocks the plan until the defect is fixed (see Harness).

## Grouping (by kick-list line)

Group `gNN` holds lines `221 + 8*(NN-1)` .. `min(228 + 8*(NN-1), 329)`. Module order in `gNN/mod.rs`
and `ENTRIES` follows line order. `batch3/mod.rs` declares `g01`..`g14` and
`GROUPS = [g01::ENTRIES, .., g14::ENTRIES]`.

| Group | Lines | Plugins |
|-------|-------|---------|
| g01 | 221-228 | PowerSag, PowerSag2, Precious, Preponderant, Pressure4, Pressure5, PurestAir, PurestConsole2Buss |
| g02 | 229-236 | PurestConsole2Channel, PurestConsoleBuss, PurestConsoleChannel, PurestDrive, PurestEcho, PurestFade, PurestGain, PurestSquish |
| g03 | 237-244 | PurestWarm, PurestWarm2, Pyewacket, RawGlitters, RawTimbers, Recurve, Remap, ResEQ |
| g04 | 245-252 | Reverb, Righteous4, RightoMono, Shape, SideDull, Sidepass, Silhouette, SingleEndedTriode |
| g05 | 253-260 | Slew, Slew2, Slew3, SlewOnly, Smooth, SoftGate, SpatializeDither, Spiral |
| g06 | 261-268 | Spiral2, Srsly, Srsly2, StarChild, StereoChorus, StereoDoubler, StereoEnsemble, StereoFX |
| g07 | 269-276 | StudioTan, SubsOnly, Surge, SurgeTide, Swell, Tape, TapeDelay, TapeDelay2 |
| g08 | 277-284 | TapeDither, TapeDust, TapeFat, Texturize, TexturizeMS, Thunder, ToneSlant, ToTape5 |
| g09 | 285-292 | ToTape6, ToVinyl4, TPDFDither, TPDFWide, TransDesk, Tremolo, TremoSquare, TripleSpread |
| g10 | 293-300 | Tube, Tube2, TubeDesk, uLawDecode, uLawEncode, Ultrasonic, UltrasonicLite, UltrasonicMed |
| g11 | 301-308 | UltrasonX, UnBox, VariMu, Verbity, Verbity2, Vibrato, VinylDither, VoiceOfTheStarship |
| g12 | 309-316 | VoiceTrick, Wider, XBandpass, XHighpass, XLowpass, XNotch, XRegion, YBandpass |
| g13 | 317-324 | YHighpass, YLowpass, YNotch, ZBandpass, ZBandpass2, ZHighpass, ZHighpass2, ZLowpass |
| g14 | 325-329 | ZLowpass2, ZNotch, ZNotch2, ZRegion, ZRegion2 |

The table comes from the workflow issue list. The bootstrap plan checks it against the pinned
list file, line by line. On any mismatch the file wins, and the bootstrap plan corrects this table
before any group plan starts.

## Module layout and naming

- `src/plugins/batch3/mod.rs`: `mod g01..g14` and `GROUPS` (the only batch-3 registration point).
- `src/plugins/batch3/gNN/mod.rs`: `//! Batch 3 list lines A-B.`, `mod <plugin>;` lines and
  `ENTRIES`, following `batch1/g03/mod.rs`.
- `src/plugins/batch3/gNN/<plugin>.rs`: one plugin, following `batch1/g03/beam.rs`. The module name
  is the KICK name lowercased (`ulawdecode`, `voiceofthestarship`). The struct name is the upstream
  class name in UpperCamelCase (`uLawDecode` becomes `ULawDecode`, as batch 1 made `curve` into
  `Curve`), so no `non_camel_case_types` allow is needed.
- A port that would reach 1000 lines becomes `gNN/<plugin>/mod.rs` plus submodules split by concern,
  following `batch1/g03/bigamp/`. This is expected for Righteous4, PurestSquish, ToVinyl4 and possibly
  Reverb. The split is decided by the port's line count, not the upstream's.
- Each file starts with the upstream copyright header, the upstream path and the pinned revision, and
  its module doc repeats the plugin's `ParityRecord` (class and measured max-abs).
- Arrays above 4096 elements are `Box<[T]>`, allocated in the constructor. `process`, `set_param` and
  `reset` never allocate. The existing `tests/no_alloc.rs` covers every catalog entry, so it covers
  batch 3 automatically.
- Every upstream `rand()` call, in the constructor and in `processReplacing` (the dither plugins
  SpatializeDither, TapeDither, TPDFDither, TPDFWide, VinylDither and others may draw per sample), goes
  through `AwRand` in upstream call order. `reset()` replays the stream from the construction seed.
  No other randomness source is allowed.

## Harness (additive only)

- `tools/airwindows-harness/generate.sh` is hardcoded to lines 1-110 and `batch1/`, and batch 2 may be
  editing it. Batch 3 does not edit it. It adds a new script, `tools/airwindows-harness/generate_batch3.sh`,
  with the same options (`--work-dir`, `--out-root`, `--group gNN`, `NAME...`). Its differences:
  - It reads lines 221-329 of the list file.
  - It derives the group as `g%02d` of `(line-221)/8+1`.
  - It writes `$out/batch3/gNN/<Name>.{txt,f32}`.
  - It uses `upstream = name`.
  It reuses `driver.cpp`, `prelude.h`, the `what.txt` description rule, and the exact clang command
  line and flags of `generate.sh`. Usage goes in the script's header comment, and
  `tools/airwindows-harness/README.md` is not edited.
- `tools/vst-shim/audioeffectx.{h,cpp}` are extended only if a batch-3 plugin fails to compile against
  the shim. An extension adds declarations and definitions only and never changes an existing
  signature or behavior. A changed shim must still reproduce the batch-1 fixtures byte for byte (proven
  by regenerating batch 1 into a scratch dir under the plan's evidence directory and running `cmp`
  against the committed files). It never rewrites `tests/fixtures/batch1/`.
- Fixture format, stimulus, renders R0-R2, ALT parameters, seed `0x00C0FFEE`, 128-frame blocks and the
  504 stored frames per render are unchanged (baseline "Parity method").
- **Harness-defect escalation**: a group plan that finds a `tools/`-side defect records it, and the
  bootstrap plan fixes it as a follow-up task. The group plan then regenerates only its own
  `tests/fixtures/batch3/gNN/` with `generate_batch3.sh --group gNN`. The group plan stays open
  until its plugins pass.

## Tests

- `tests/parity_batch3_gNN.rs` (one per group): one `#[test]` per plugin, following
  `tests/parity_batch1_g03.rs`. Each test runs `check_reset_preserves_params(name)` and then
  `check_plugin(&RECORD_<NAME>, include_str!("fixtures/batch3/gNN/<Name>.txt"), include_bytes!(..f32))`.
  It reuses `tests/common/` unchanged. Construction is by name through the factory, which proves
  registration.
- Tolerance classes follow the baseline: `Exact`, `Libm` (max-abs <= 1e-6) and `Custom(x)`.
  `measured_max_abs` is the value measured on aarch64-apple-darwin, recorded in the record and the
  module header. 0 is expected. A plugin is `Custom` only after investigation, with a written `note`.
  Any `Custom(x)` with x > 1e-6 is also added to Divergences below. Loosening silently is a defect.
- `tests/catalog_batch3.rs` (new):
  - `BATCH3: [&str; 109]` lists the names in line order.
  - It asserts that the last 109 entries of `catalog()` equal `BATCH3`, in order.
    `plugins/mod.rs` orders `BATCHES` as batch1, batch2, batch3, so this holds both before and after
    batch 2 merges.
  - It asserts that `create(name, 48_000.0)` and `create_with_seed(name, 44_100.0, 1)` are `Some`
    for each name, and that `upstream_name == name` for each name.
- `tests/fixture_budget_batch3.rs` (new): asserts that `tests/fixtures/batch3/` is at most
  3,000,000 bytes. The expected size is about 1.4 MB, because each plugin's `.f32` is 12,096 bytes
  plus a `.txt`, matching batch 1's measured average of about 12.8 KB.

### Required edits to two batch-1-era test files (decision R1)

Two existing tests contradict the accepted acceptance criteria once batch 3 lands, because both
were written when only batch 1 existed:

1. `tests/fixture_budget.rs` asserts that the whole `tests/fixtures/` tree is at most 1,600,000 bytes.
   Batch 1 and the shim vectors already use 1,406,647 bytes, which leaves about 193 KB of headroom
   for all later batches. The baseline already states "each batch has its own fixture budget of at
   most 3 MB", so the test lags behind the design. Shrinking batch 3 into the headroom would need under
   1.8 KB per plugin, and the headroom is also shared with batch 2. That would gut parity coverage,
   so it is rejected.
2. `tests/batch1_complete.rs` asserts that `catalog()` equals exactly the 110 batch-1 names. Any
   registered batch-3 plugin makes it fail.

Resolution, owned by the bootstrap plan (`aw-b3-00`) and made before any fixture or registration
lands, so that the tree stays green:

- `fixture_budget.rs`: the 1,600,000-byte assertion measures `tests/fixtures` minus
  `tests/fixtures/batch3`. Nothing else changes, and batch 2's accounting is left to batch 2.
- `batch1_complete.rs`: `assert_eq!(names, BATCH1)` becomes a prefix assertion,
  `assert_eq!(names[..BATCH1.len()], BATCH1)`. Batch 1 stays first in `BATCHES`, so the test keeps
  its full strength for batch 1.

These two one-line edits are the only writes outside the workflow's listed write paths. They are
recorded for operator confirmation in `design-docs/user-qa/pending-airwindows-batch3.md`. If the operator
rejects them, the final `cargo nextest run --workspace --all-features` acceptance cannot be met, and
the integration plan must report it as blocked instead of weakening any batch-3 test.

## Plans, write ownership and order

Plan ids use the `aw-b3-` prefix. The evidence root is `tmp/airwindows-batch3/<planId>/`, which is
gitignored through `**/tmp`. Every plan:

- declares that evidence directory in both `writePaths` and `artifactRoots`, plus `target/` as an
  artifact root
- sets `CARGO_TARGET_DIR=tmp/airwindows-batch3/<planId>/target` and `CARGO_TERM_QUIET=true`
- states the completeness rule above, verbatim
- uses the verification record shape
  `{command, exitStatus: 0, testsRun, testsPassed, failureCount: 0, outcome: "passed", log, notes}`.
  Negative controls go in `mutationEvidence`, and repaired findings go only in `addressedFeedback`.

The dispatch manifest is `impl-plans/active/airwindows-batch3-dispatch.json`. The leftover
`impl-plans/active/behringer-oss-ports-dispatch.json` belongs to batch 1 and is not touched.

| Plan | Depends on | Owns (writePaths, besides its evidence dir) |
|------|------------|---------------------------------------------|
| `aw-b3-00-bootstrap` | - | `src/plugins/batch3/mod.rs`, the 14 `src/plugins/batch3/gNN/mod.rs` (each with `ENTRIES = &[]`), `tools/airwindows-harness/generate_batch3.sh`, `tools/vst-shim/audioeffectx.{h,cpp}` (additive, only if needed), the directory `tests/fixtures/batch3/` (all 218 files, generated), `tests/fixture_budget.rs` and `tests/batch1_complete.rs` (R1 edits only), `tests/fixture_budget_batch3.rs` |
| `aw-b3-01`..`aw-b3-14` group gNN | 00 | the directory `src/plugins/batch3/gNN/` (its `mod.rs` `ENTRIES` and plugin files), `tests/parity_batch3_gNN.rs`, and the directory `tests/fixtures/batch3/gNN/` (regeneration after a bootstrap harness fix only) |
| `aw-b3-15-integration` | 01-14 | `tests/catalog_batch3.rs`, this design doc and `design-docs/user-qa/pending-airwindows-batch3.md` (record the measured max-abs summary and any divergence). Plan archiving is done by the workflow's plan-archive node, not by a plan |

Waves: W1 = 00. W2 = 01-14 in parallel. W3 = 15. No plan writes `batch1/`, `batch2/`, the framework
files (`lib.rs`, `effect.rs`, `params.rs`, `dither.rs`, `rand.rs`, `display.rs`, `registry.rs`,
`plugins/mod.rs`), `tests/catalog.rs`, `tests/common/`, the root `README.md`, `THIRD_PARTY_NOTICES.md`
or `Cargo.toml`/`Cargo.lock`. Batch 3 adds no dependency.

Bootstrap completion requires:

- every one of the 109 harness executables builds and runs, so shim gaps surface here and not in
  group plans
- `tests/fixtures/batch3/` holds exactly 218 files
- the R1 edits are in place
- the workspace compiles with empty groups

## Verification

Group plan gates (no crate-wide clippy, which avoids the batch-1 shared-crate deadlock):

```sh
export CARGO_TERM_QUIET=true CARGO_TARGET_DIR=tmp/airwindows-batch3/<planId>/target
cargo check -p music-things-airwindows --all-targets
cargo nextest run -p music-things-airwindows --test parity_batch3_gNN   # 8 tests (g14: 5), 0 skipped
rustfmt --edition 2021 --check <own files>
{ cargo clippy -p music-things-airwindows --all-targets --message-format short -- -D warnings 2>&1 \
  | grep -E "src/plugins/batch3/gNN/|tests/parity_batch3_gNN" ; } ; test $? -eq 1   # prints nothing
find crates/airwindows/src/plugins/batch3/gNN -name '*.rs' -exec wc -l {} + # every file < 1000
```

Bootstrap adds `tools/airwindows-harness/generate_batch3.sh --work-dir tmp/airwindows-batch3/aw-b3-00/harness --out-root crates/airwindows/tests/fixtures`
(requires `$MUSIC_THINGS_REFS` and clang++). If the shim changed, it also adds the batch-1
reproduction `cmp`, plus `cargo nextest run -p music-things-airwindows --test fixture_budget --test fixture_budget_batch3 --test batch1_complete`.

Integration (final tree):

```sh
export CARGO_TERM_QUIET=true NEXTEST_STATUS_LEVEL=fail NEXTEST_FAILURE_OUTPUT=immediate-final NEXTEST_HIDE_PROGRESS_BAR=1
cargo nextest run --workspace --all-features
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo fmt --check
cargo build --workspace --all-features --target wasm32-unknown-unknown
```

It also checks that `grep -c '#\[test\]' tests/parity_batch3_g*.rs` totals 109, that there is no
`#[ignore]`, `todo!`, `unimplemented!` or `stub` under `src/plugins/batch3/`, and that no `.rs` file
under batch 3 reaches 1000 lines.

## Intentional divergences

None beyond the baseline's D1, D5 and D6. A plugin that ends up `Custom(x)` with x > 1e-6 is listed
here with its reason before the integration plan completes.

## Risks

- **Largest ports skipped again**: Righteous4 and Reverb are both in g04, PurestSquish is in g02 and
  ToVinyl4 is in g09. Each of those plans names them as completion criteria, and `catalog_batch3.rs`
  and the parity count catch any omission.
- **Libm drift**: the reverb, tape and vinyl families (Reverb, Verbity/Verbity2, Tape, ToTape5/6,
  ToVinyl4) call transcendentals heavily. The expected value is 0 on the generation host. A nonzero
  result is investigated first, usually as a wrong `f32`/`f64` overload, and is never loosened silently.
- **Batch-2 merge conflicts**: batch 2 may make the analogous R1 edits to `fixture_budget.rs` and
  `batch1_complete.rs`. The operator resolves that at merge by excluding both batch directories and
  keeping the prefix assertion. All other batch-3 files are disjoint from batch 2.
- **Shim extension**: an additive shim change could alter batch-1 output. The batch-1 `cmp` check
  guards against this.
