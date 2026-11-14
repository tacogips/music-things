# Airwindows Batch 3 00: Bootstrap (registration skeleton, harness, fixtures, R1 test edits)

**Status**: Completed
**Plan ID**: aw-b3-00
**Design Reference**: design-docs/specs/design-airwindows-batch3.md (Grouping, Module layout and naming, Harness, Tests, decision R1, Plans); baseline design-docs/specs/design-behringer-oss-ports.md section A
**Depends On**: none (wave W1). aw-b3-01..aw-b3-14 depend on this plan.
**Created**: 2026-11-01
**Last Updated**: 2026-11-01

## Intent and Context

Batch 3 ports the 109 Airwindows plugins on lines 221-329 of
`$MUSIC_THINGS_REFS/kick-airwindows-set.txt` (PowerSag .. ZRegion2) into `crates/airwindows`. This plan
prepares everything the 14 parallel group plans need, so that none of them edits a shared file:

1. the batch-3 registration skeleton (`batch3/mod.rs` with 14 empty groups)
2. an additive harness script that generates batch-3 fixtures
3. all 218 fixture files (109 `.txt` + 109 `.f32`)
4. the two one-line R1 test edits plus a batch-3 fixture budget test, so the workspace test suite stays
   green once fixtures and plugins land

`MUSIC_THINGS_REFS=/Users/taco/gits/tacogips/music-things-refs`. The pinned upstream is airwindows
`b110fac3991fff7c4655cc2d25d6a3e221f9799c` (MIT). Never open Music Tribe AGPL code.

**Completeness rule (applies to the whole batch)**: every one of the 109 plugins must be fully ported,
registered and parity-tested. A stub, an unregistered module, a skipped or ignored test, or a
"blocked" plugin is a failure, not a deferral. In this plan, every one of the 109 harness
executables must build and run. A plugin that does not build against the shim is fixed here, through
an additive shim change, and is never dropped.

## Non-goals

- Do not port any plugin and do not add any `mod <plugin>;` line. Group plans own that.
- Do not edit `tools/airwindows-harness/generate.sh`, `driver.cpp`, `prelude.h`, `format_vectors.cpp`
  or `README.md` (batch 2 may be editing them), and do not edit `tools/common/`.
- Do not touch `src/plugins/batch1/`, `src/plugins/batch2/`, any framework file (`lib.rs`, `effect.rs`,
  `params.rs`, `dither.rs`, `rand.rs`, `display.rs`, `registry.rs`, `plugins/mod.rs`), `tests/catalog.rs`,
  `tests/common/`, `tests/fixtures/batch1/`, the root `README.md`, `THIRD_PARTY_NOTICES.md`, `Cargo.toml`
  or `Cargo.lock`.
- No change to `fixture_budget.rs` or `batch1_complete.rs` beyond the single assertions described below.

## Write Ownership (fanout contract)

- writePaths:
  - `crates/airwindows/src/plugins/batch3/mod.rs`
  - `crates/airwindows/src/plugins/batch3/g01/mod.rs` .. `crates/airwindows/src/plugins/batch3/g14/mod.rs`
    (14 concrete files)
  - `tools/airwindows-harness/generate_batch3.sh`
  - `tools/vst-shim/audioeffectx.h`, `tools/vst-shim/audioeffectx.cpp` (additive edits only, and only if needed)
  - `crates/airwindows/tests/fixtures/batch3` (directory)
  - `crates/airwindows/tests/fixture_budget.rs`, `crates/airwindows/tests/batch1_complete.rs`,
    `crates/airwindows/tests/fixture_budget_batch3.rs`
  - `impl-plans/active/airwindows-batch3-00-bootstrap.md` (this plan's progress log only)
  - `target`, `tmp/airwindows-batch3/aw-b3-00`
- sharedPaths: none
- artifactRoots (digest-only): `target`, `tmp/airwindows-batch3/aw-b3-00`

## Common Execution Rules

- Evidence directory: `tmp/airwindows-batch3/aw-b3-00/`. It holds:
  - `intent.md`, written once at the start and never edited: the deliverables list.
  - `pre-hashes.txt` and `post-hashes.txt`: `shasum -a 256` of every owned file that exists.
  - `logs/`: one log file per verification command.
- Before editing a file, re-read it. If it differs from the hash you recorded, stop and log the drift.
- Environment for every cargo command:
  `CARGO_TERM_QUIET=true CARGO_TARGET_DIR=tmp/airwindows-batch3/aw-b3-00/target NEXTEST_STATUS_LEVEL=fail NEXTEST_FAILURE_OUTPUT=immediate-final NEXTEST_HIDE_PROGRESS_BAR=1`.
- Never run git checkout, stash, reset or commit.

## Deliverables

### 1. Registration skeleton

- `src/plugins/batch3/mod.rs`:
  - Keep the `//! Owned by the batch-3 run.` doc and the `use crate::registry::PluginEntry;` line.
  - Add `mod g01;` .. `mod g14;`.
  - Set `pub(crate) const GROUPS: &[&[PluginEntry]] = &[g01::ENTRIES, .., g14::ENTRIES];` in order.
  - Imitate `crates/airwindows/src/plugins/batch1/mod.rs`.
- `src/plugins/batch3/gNN/mod.rs`, for NN = 01..14:
  - The doc line `//! Batch 3 list lines A-B.` uses the design table ranges (g01 221-228, ..,
    g13 317-324, g14 325-329).
  - `use crate::registry::PluginEntry;` and `pub(crate) const ENTRIES: &[PluginEntry] = &[];`.
  - No `mod` lines.
- Invariant: the crate compiles and `catalog()` still lists exactly the 110 batch-1 entries.

### 2. `tools/airwindows-harness/generate_batch3.sh` (new, `chmod +x`, first lines `#!/bin/bash` and `# SPDX-License-Identifier: MIT`)

- Imitate `tools/airwindows-harness/generate.sh` line for line. The differences:
  - Usage: `generate_batch3.sh --work-dir DIR --out-root DIR [--group gNN] [NAME...]`. Put the usage and
    the `MUSIC_THINGS_REFS` requirement in the header comment.
  - It reads the list file, counts lines from 1, and processes only lines 221..329.
    The group is `printf 'g%02d' $(((line-221)/8+1))`.
  - `upstream=$name`, with no name mapping.
  - Output goes to `$out/batch3/$grp/$name.{txt,f32}`, and it creates `$out/batch3/$grp`.
  - The clang++ command line (flags, `-include prelude.h`, include dirs, `-DAW_CLASS`, `-DAW_HEADER`, the
    four source files) and the driver invocation (`"$work/$name" "<out>/<Name>" "$name" "$upstream" "$desc"`)
    must be byte-identical to `generate.sh`.
  - The `what.txt` description `awk` rule must be byte-identical too. Exit 1 on a missing description.
  - The fixture-intents block may be omitted.
- Pitfalls:
  - The `while read` loop must also handle a final line with no trailing newline
    (`while IFS= read -r name || [ -n "$name" ]`). Otherwise line 329 (ZRegion2) can be silently dropped.
  - Strip `\r` from each name.
  - After the loop, when no NAME filter and no `--group` were given, exit nonzero unless exactly 109
    plugins were generated.

### 3. Shim (only if needed)

- Run the generation (verification command 2). For any plugin that fails to compile because of a
  missing `AudioEffect`/`AudioEffectX` symbol, add the missing declaration or definition to
  `tools/vst-shim/audioeffectx.{h,cpp}`.
  - Add only. Never change an existing signature, default or behavior. Keep the "written from scratch"
    header, and never copy Steinberg SDK code.
  - Log each addition, with the plugin that needs it.
- If the shim changed, prove batch 1 is unaffected:
  - Run `MUSIC_THINGS_REFS=... tools/airwindows-harness/generate.sh --work-dir tmp/airwindows-batch3/aw-b3-00/b1-harness --out-root tmp/airwindows-batch3/aw-b3-00/b1-regen`.
  - Then run `diff -r crates/airwindows/tests/fixtures/batch1 tmp/airwindows-batch3/aw-b3-00/b1-regen/batch1`.
    It must print nothing and exit 0.
  - Never write `crates/airwindows/tests/fixtures/batch1/`.

### 4. Fixtures

- Generate the full batch with `generate_batch3.sh --work-dir tmp/airwindows-batch3/aw-b3-00/harness --out-root crates/airwindows/tests/fixtures`.
- Expected: 14 group directories and 218 files. Each `.f32` is 12,096 bytes, and each `.txt` has
  `name=`, `upstream=`, `description=` (non-empty), and `params=`.
- Cross-check the names against the design table with
  `sed -n '221,329p' "$MUSIC_THINGS_REFS/kick-airwindows-set.txt"`. Record the 14 per-group name
  lists in the Progress Log.
  - On any mismatch, the list file is authoritative. Record the difference prominently in the Progress
    Log under "TABLE MISMATCH" so aw-b3-15 corrects the design doc.
  - Group plans take their names from `ls tests/fixtures/batch3/gNN/*.txt`.

### 5. R1 test edits and batch-3 budget test

- `tests/fixture_budget.rs`: change only the assertion. It becomes
  `size(fixtures) - size(fixtures/batch3) <= 1_600_000`, where both paths are built from
  `env!("CARGO_MANIFEST_DIR")` as today. Keep `fn size` unchanged.
- `tests/batch1_complete.rs`: change only `assert_eq!(names, BATCH1);` to a prefix assertion over the
  first `BATCH1.len()` entries (for example `assert_eq!(&names[..BATCH1.len()], &BATCH1[..]);`). The rest of
  the test stays unchanged.
- `tests/fixture_budget_batch3.rs` (new): copy the small recursive `size` helper from
  `fixture_budget.rs` (do not touch `tests/common/`). Test `fixture_budget_batch3` asserts that
  `tests/fixtures/batch3` is `> 0` (not vacuous) and `<= 3_000_000` bytes, printing the measured size
  in the assertion message.

## Tests (situation -> expected)

- Fixtures plus R1 edits present, no plugins registered -> `fixture_budget`, `fixture_budget_batch3`,
  `batch1_complete`, `catalog` and `no_alloc` pass.
- Negative controls, recorded in `mutationEvidence` and reverted afterwards:
  - Temporarily set the batch-3 cap to 1,000 bytes -> `fixture_budget_batch3` fails. Restore it.
  - Run `generate_batch3.sh --group g14` into a scratch out-root -> exactly 10 files: ZLowpass2, ZNotch,
    ZNotch2, ZRegion and ZRegion2 (each `.txt` + `.f32`).

## Verification Commands (evidence)

1. `cargo check -p music-things-airwindows --all-targets` -> exit 0.
2. `MUSIC_THINGS_REFS=/Users/taco/gits/tacogips/music-things-refs tools/airwindows-harness/generate_batch3.sh --work-dir tmp/airwindows-batch3/aw-b3-00/harness --out-root crates/airwindows/tests/fixtures`
   -> exit 0, 109 lines of `<Name>: generated`.
3. `find crates/airwindows/tests/fixtures/batch3 -type f | wc -l` -> `218`.
   `find crates/airwindows/tests/fixtures/batch3 -name '*.f32' -size -12096c -o -name '*.f32' -size +12096c | wc -l` -> `0`.
4. `cargo nextest run -p music-things-airwindows --test fixture_budget --test fixture_budget_batch3 --test batch1_complete --test catalog --test no_alloc`
   -> exit 0, 0 failures.
5. `rustfmt --edition 2021 --check crates/airwindows/src/plugins/batch3/mod.rs crates/airwindows/src/plugins/batch3/g*/mod.rs crates/airwindows/tests/fixture_budget.rs crates/airwindows/tests/fixture_budget_batch3.rs crates/airwindows/tests/batch1_complete.rs`
   -> exit 0.
6. `{ cargo clippy -p music-things-airwindows --all-targets --message-format short -- -D warnings 2>&1 | grep -E "src/plugins/batch3/|tests/fixture_budget|tests/batch1_complete" ; } ; test $? -eq 1`
   -> exit 0 (no own-file diagnostics).
7. `bash -n tools/airwindows-harness/generate_batch3.sh` -> exit 0.
8. Only if the shim changed: the batch-1 `diff -r` from Deliverable 3 -> exit 0, no output.

Record each verification as `{command, exitStatus: 0, testsRun, testsPassed, failureCount: 0, outcome: "passed", log, notes}`.
Logs go in `tmp/airwindows-batch3/aw-b3-00/logs/`.

## Completion Criteria

- [x] `batch3/mod.rs` declares g01..g14, and `GROUPS` lists the 14 `ENTRIES` in order. Each `gNN/mod.rs` has empty `ENTRIES`.
- [x] `generate_batch3.sh` exists, is executable, and is additive (`git diff --stat tools/` shows no edits to `generate.sh`, `driver.cpp`, `prelude.h` or `README.md`).
- [x] All 109 harness executables built and ran; `tests/fixtures/batch3/` holds exactly 218 files.
- [x] R1 edits and `fixture_budget_batch3.rs` are in place; verification 4 passes.
- [x] Any shim change is additive and passes the batch-1 reproduction diff.
- [x] Verifications 1-7 (and 8 if applicable) exit 0 with logs. Only writePaths changed.
- [x] Progress Log records the 14 per-group name lists, any shim additions, any TABLE MISMATCH, and every command with its exit status and log path.

## Progress Log

### Session: 2026-11-01
**Tasks Completed**: Plan created (not started)
**Notes**: -

### Session: 2026-11-01 — bootstrap implementation
**Tasks Completed**: registration skeleton, additive harness, 109 fixture pairs, and R1 tests. All fixture generation succeeded without shim changes; batch-1 reproduction is therefore not applicable. Formal review and subsequent integration remain pending.
**Grouping**: all entries match the design table; no TABLE MISMATCH.

| g01 | 221-228 | PowerSag, PowerSag2, Precious, Preponderant, Pressure4, Pressure5, PurestAir, PurestConsole2Buss | MATCH
| g02 | 229-236 | PurestConsole2Channel, PurestConsoleBuss, PurestConsoleChannel, PurestDrive, PurestEcho, PurestFade, PurestGain, PurestSquish | MATCH
| g03 | 237-244 | PurestWarm, PurestWarm2, Pyewacket, RawGlitters, RawTimbers, Recurve, Remap, ResEQ | MATCH
| g04 | 245-252 | Reverb, Righteous4, RightoMono, Shape, SideDull, Sidepass, Silhouette, SingleEndedTriode | MATCH
| g05 | 253-260 | Slew, Slew2, Slew3, SlewOnly, Smooth, SoftGate, SpatializeDither, Spiral | MATCH
| g06 | 261-268 | Spiral2, Srsly, Srsly2, StarChild, StereoChorus, StereoDoubler, StereoEnsemble, StereoFX | MATCH
| g07 | 269-276 | StudioTan, SubsOnly, Surge, SurgeTide, Swell, Tape, TapeDelay, TapeDelay2 | MATCH
| g08 | 277-284 | TapeDither, TapeDust, TapeFat, Texturize, TexturizeMS, Thunder, ToneSlant, ToTape5 | MATCH
| g09 | 285-292 | ToTape6, ToVinyl4, TPDFDither, TPDFWide, TransDesk, Tremolo, TremoSquare, TripleSpread | MATCH
| g10 | 293-300 | Tube, Tube2, TubeDesk, uLawDecode, uLawEncode, Ultrasonic, UltrasonicLite, UltrasonicMed | MATCH
| g11 | 301-308 | UltrasonX, UnBox, VariMu, Verbity, Verbity2, Vibrato, VinylDither, VoiceOfTheStarship | MATCH
| g12 | 309-316 | VoiceTrick, Wider, XBandpass, XHighpass, XLowpass, XNotch, XRegion, YBandpass | MATCH
| g13 | 317-324 | YHighpass, YLowpass, YNotch, ZBandpass, ZBandpass2, ZHighpass, ZHighpass2, ZLowpass | MATCH
| g14 | 325-329 | ZLowpass2, ZNotch, ZNotch2, ZRegion, ZRegion2 | MATCH

**Verification** (logs under `tmp/airwindows-batch3/aw-b3-00/logs/`): cargo check exit 0 (`check-final.log`); full fixture generation exit 0, 109 generated (`generate.log`); fixture count 218 and wrong-size count 0 (`count.log`, `sizes.log`); scoped nextest exit 0, 7 passed (`nextest-final.log`); rustfmt exit 0 (`rustfmt-final.log`); own-file clippy filter exit 0 (`clippy-final.log`); bash -n exit 0 (`bash-n.log`); g14 generation exit 0 (`g14.log`) and exact 10 files (`g14-assert.log`). Negative cap control exit 100, 1 expected failure (`negative-cap-2.log`); cap restored and nextest rerun passed.
