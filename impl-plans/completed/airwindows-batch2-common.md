# Airwindows Batch 2: Common Execution Contract (read before any ab2-* plan)

This file is mandatory reading for every `ab2-*` plan. It is not a plan of its own and owns no files.
Each plan file adds its own plugin list, paths, commands and done criteria. If a plan file and this
file disagree, the plan file wins for its own paths and this file wins for everything else.

**Design**: `design-docs/specs/design-airwindows-batch2.md`, the batch-2 design. It extends
`design-docs/specs/design-behringer-oss-ports.md`, the base design: section A "Public API",
"Framework internals", "Additive registration", "Porting rules", "Parity method (Airwindows)",
"Tolerances (Airwindows)" and "Additive procedure for batches 2 and 3".
**Issue**: workflow input "Port Airwindows batch 2 (110 plugins, DitherMeTimbers .. Pop2) into
music-things-airwindows". No GitHub number. Workflow execution:
opus-terra-design-and-implement-review-loop-session-627.

## 1. User intent

Add faithful Rust ports of the 110 Airwindows plugins on lines 111-220 of
`$MUSIC_THINGS_REFS/kick-airwindows-set.txt` to `crates/airwindows` as batch 2. Each port must
match the pinned upstream C++ in a fixture parity test. Batch 1 (lines 1-110) is already merged
and is the pattern to imitate. A concurrent run ports batch 3 on another branch and must never be
disturbed.

**No stubs, no skips.** Every plugin assigned to a plan must be fully ported, registered in its
group's `ENTRIES`, and covered by a passing parity test. A stub, an unregistered module, an omitted
plugin, a no-op or "TODO" branch where upstream has code, or an ignored or weakened test is a
**failure of the plan, not a deferral**. A plan whose plugins are not all registered and passing
is never `Completed`. In batch 1 this went wrong twice: BigAmp was left as an unregistered stub and
Ditherbox was never written.

## 2. Upstream source boundary

- `MUSIC_THINGS_REFS=/Users/taco/gits/tacogips/music-things-refs`. Read only
  `airwindows/plugins/LinuxVST/src/<Name>/{<Name>.h,<Name>.cpp,<Name>Proc.cpp}`,
  `airwindows/what.txt` and `kick-airwindows-set.txt`. The pinned revision is
  `b110fac3991fff7c4655cc2d25d6a3e221f9799c` (MIT).
- Never open Music Tribe AGPL code (the KICK/IGNITE `AudIO`, `Kick` and `Source` trees). Never
  copy upstream C/C++ into the repository; the harness compiles it in place.
- For every batch-2 plugin, `upstream_name == name`. The batch-1 alias `Console8BusOut` does not
  occur in batch 2.

## 3. Repository context (frozen; read-only for every ab2 plan)

Framework APIs (`crates/airwindows/src/`):

- `effect.rs:AirwindowsEffect`
- `params.rs:{ParamInfo, PluginInfo, sanitize_param}`
- `registry.rs:PluginEntry { info, construct: fn(f32, u32) -> Box<dyn AirwindowsEffect> }`
- `dither.rs:{fpd_from_seed, xorshift, frexpf_exponent, pow2, fpd_dither}`
- `rand.rs:{AwRand, RAND_MAX, replay_fpd_seed_loop}`
- `display.rs:{float2string, db2string, int2string}`
- `plugins/mod.rs`: `BATCHES = [batch1::GROUPS, batch2::GROUPS, batch3::GROUPS]`. The module
  contract is that each plugin module defines `const INFO`, `const ENTRY` and
  `fn construct(sample_rate, seed)`.

Patterns to imitate:

- `src/plugins/batch1/g03/beam.rs` for a single-file plugin (header, `PARAMS`, `INFO`, `ENTRY`,
  `construct`, struct, constructor with seed replay, trait impl).
- `src/plugins/batch1/g03/bigamp/{mod.rs,cabinet.rs}` for a split module directory, including the
  `#![allow(non_snake_case, unused_parens, clippy::...)]` list used for line-by-line ports.
- `src/plugins/batch1/g03/mod.rs` for group registration (`mod` lines and `ENTRIES` in kick-line
  order).
- `tests/parity_batch1_g03.rs` for a group parity test (`ParityRecord` consts,
  `check_reset_preserves_params`, fixture parse, R0/R1/R2 render at 128-frame blocks, stored-frame
  compare).

Shared test helpers live in `tests/common/{fixture,parity,stimulus,fnv}.rs` and are used through
`mod common;`. Do not edit them.

These shared tests are not owned by any ab2 plan: `tests/catalog.rs`, `tests/no_alloc.rs`,
`tests/batch1_complete.rs`, `tests/fixture_budget.rs` and `tests/stimulus_checksum.rs`.
`no_alloc.rs` and `catalog.rs` iterate every catalog entry, so they cover batch-2 plugins
automatically.

Never touch:

- `src/plugins/batch1/**`, `src/plugins/batch3/**`
- framework files: `lib.rs`, `effect.rs`, `params.rs`, `dither.rs`, `rand.rs`, `display.rs`,
  `registry.rs`, `plugins/mod.rs`
- `tests/common/**`, `tests/fixtures/batch1/**`, the shared tests above
- `tools/airwindows-harness/{generate.sh,driver.cpp,prelude.h,README.md}`
- root `README.md`, `THIRD_PARTY_NOTICES.md`, any `Cargo.toml`, `Cargo.lock`, `mise.toml`
- `design-docs/specs/design-behringer-oss-ports.md`, `design-docs/user-qa/README.md`
- `impl-plans/active/behringer-oss-ports-dispatch.json`

## 4. Grouping (fixed; design section 2)

Each group has a directory `src/plugins/batch2/gNN/`, a test file `tests/parity_batch2_gNN.rs` and
a fixture directory `tests/fixtures/batch2/gNN/`. Oversized groups (marked `*`) hold one plugin
each.

| Group | Lines | Plugins | Plan |
|-------|-------|---------|------|
| g01 | 111-118 | DitherMeTimbers, Doublelay, DoublePaul, Drive, DrumSlam, DubCenter, Dubly, DubSub | ab2-n01 |
| g02 | 119-125 | DustBunny, Dynamics, Dyno, Edge, EdIsDim, Elation, ElectroHat | ab2-n02 |
| g03* | 126 | Energy | ab2-o-energy |
| g04* | 127 | Energy2 | ab2-o-energy2 |
| g05 | 128-133 | Ensemble, EQ, EveryTrim, Exciter, Facet, FathomFive | ab2-n03 |
| g06* | 134 | FireAmp | ab2-o-fireamp |
| g07 | 135-139 | Flipity, Floor, Flutter, Focus, Fracture | ab2-n04 |
| g08 | 140-144 | FromTape, Galactic, Gatelope, GlitchShifter, Golem | ab2-n05 |
| g09* | 145 | GrindAmp | ab2-o-grindamp |
| g10 | 146-152 | Gringer, GrooveWear, GuitarConditioner, HardVacuum, Hermepass, HermeTrim, HighGlossDither | ab2-n06 |
| g11 | 153-159 | HighImpact, Highpass, Highpass2, Holt, Holt2, Hombre, Hull | ab2-n07 |
| g12 | 160-165 | Hype, Hypersonic, HypersonX, Infinity, Infinity2, Infrasonic | ab2-n08 |
| g13 | 166-171 | Interstage, IronOxide5, IronOxideClassic, IronOxideClassic2, Isolator, Isolator2 | ab2-n09 |
| g14* | 172 | LeadAmp | ab2-o-leadamp |
| g15 | 173 | LeftoMono | ab2-n10 |
| g16* | 174 | LilAmp | ab2-o-lilamp |
| g17* | 175 | Logical4 | ab2-o-logical4 |
| g18 | 176-180 | Loud, Lowpass, Lowpass2, LRFlipTimer, Luxor | ab2-n10 |
| g19 | 181-184 | MackEQ, Mackity, MatrixVerb, Melt | ab2-n11 |
| g20* | 185 | MidAmp | ab2-o-midamp |
| g21 | 186-187 | MidSide, Mojo | ab2-n11 |
| g22* | 188 | Monitoring | ab2-o-monitoring |
| g23 | 189 | Monitoring2 | ab2-n12 |
| g24* | 190 | Monitoring3 | ab2-o-monitoring3 |
| g25 | 191-192 | MoNoam, MultiBandDistortion | ab2-n12 |
| g26* | 193 | MV | ab2-o-mv |
| g27 | 194-196 | NaturalizeDither, NCSeventeen, Neverland | ab2-n12 |
| g28* | 197 | Nikola | ab2-o-nikola |
| g29 | 198-199 | NodeDither, Noise | ab2-n12 |
| g30* | 200 | NonlinearSpace | ab2-o-nonlinearspace |
| g31 | 201-206 | NotJustAnotherCD, NotJustAnotherDither, OneCornerClip, Pafnuty, PaulDither, PaulWide | ab2-n13 |
| g32 | 207-212 | PDBuss, PDChannel, PeaksOnly, PhaseNudge, PitchDelay, PitchNasty | ab2-n14 |
| g33* | 213 | PocketVerbs | ab2-o-pocketverbs |
| g34 | 214-220 | Pockey, Pockey2, Podcast, PodcastDeluxe, Point, Pop, Pop2 | ab2-n15 |

Module name = the KICK name lowercased (for example `dithermetimbers`, `eq`, `lrfliptimer`,
`pocketverbs`). Struct name = the upstream class name.

## 5. Per-plugin deliverable (port plans)

Each plugin module (`gNN/<module>.rs`, or `gNN/<module>/mod.rs` plus submodules) contains:

1. **Header.** The upstream copyright line verbatim from `<Name>.h`, then
   `Upstream: airwindows/plugins/LinuxVST/src/<Name>, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.`,
   then `Parity: <Class>, measured max-abs <value>.` (plus the reason when Custom). Every submodule
   file of a split plugin repeats the copyright and Upstream lines.
2. **`PARAMS`, `INFO`, `ENTRY`, `construct`** exactly as in `beam.rs`.
   - `INFO.description` is the unescaped `description=` value from
     `tests/fixtures/batch2/gNN/<Name>.txt`.
   - Param name, label and default come from that fixture and from `<Name>.cpp`.
3. **Struct and constructor.**
   - The struct holds `[f32; N]` params, `sample_rate: f32`, `seed: u32`, the `.h` state in
     snake_case, `fpd_l`/`fpd_r` when upstream has them, and an `AwRand` when upstream calls
     `rand()` outside the fpd loop.
   - The constructor reproduces upstream defaults and replays every constructor `rand()` call in
     order (the fpd loop through `replay_fpd_seed_loop`). It then sets
     `fpd_l = fpd_from_seed(seed, 0)` and `fpd_r = fpd_from_seed(seed, 1)`.
   - Arrays longer than 4096 elements are `Box<[T]>`, allocated only here.
4. **`param_display`/`param_label`** reproduce `getParameterDisplay`/`getParameterLabel` case by
   case, using the `display.rs` helpers.
5. **`process`** ports `processReplacing` (the float path) statement by statement over
   `min(len)` frames. **`reset`** restores the post-construction DSP state in place: no allocation,
   params and sample rate kept, fpd and rand stream re-derived from `seed`. Imitate how batch-1
   files save and restore params around the state reset.
6. **`gNN/mod.rs`: declaration is separate from registration** (S5-M1).
   - **Declare early.** Add `mod <module>;` to your own `gNN/mod.rs` as soon as you create the
     module, so that gate 1 (`cargo check`) really compiles it. An undeclared file is never
     compiled, so checking it before declaration proves nothing.
   - While the module is declared but not yet registered, the declaration may carry a narrow
     `#[allow(dead_code)]` (on the `mod` line only, nowhere else).
   - **Register last.** Add `<module>::ENTRY` to `ENTRIES`, in kick-line order, only when the
     plugin is complete (fully ported, `reset` done, parity test written). Remove that plugin's
     `#[allow(dead_code)]` in the same edit.
   - **Registered** in every plan means "its `ENTRY` is in `ENTRIES`". Gate 6 counts `ENTRY` items,
     and gate 7b proves that no `dead_code` allow is left.

## 6. Porting rules (common failure sources)

- **Precision.** `float` becomes `f32`; `double` and `long double` become `f64`.
  - Mixed float/double expressions are evaluated in f64 and rounded only where C++ assigns to a
    float.
  - A libm call on a `float` argument is the f32 function (`sinf` -> `f32::sin`); on a `double`
    argument it is the f64 one. Check every call's argument type, including literals: `0.5f` is
    float, `0.5` is double.
- **Order.** Keep operation order. Never rewrite `pow(x,2)` as `x*x`, never factor or reorder, and
  never use `mul_add`. `(int)`/`(VstInt32)` casts truncate toward zero. Unsigned arithmetic wraps
  (`wrapping_*`), and `uint32_t` stays `u32`.
- **Copy exactly.** Copy the near-zero input replacement, the `fpd` xorshift order and the dither
  block exactly. Use `fpd_dither` only when the block matches the canonical form byte for byte;
  otherwise inline it. The output cast is `as f32`.
- **Sample rate.** `getSampleRate()` is `self.sample_rate` (f32). `overallscale` follows upstream's
  expression and evaluation order.
- **State.** Statics and function-local statics become fields.
- **Indexing.** Modulo and circular indices follow upstream: check `count < 0` / `count > N` wrap
  logic, and the `+1` offsets.
- **Branches.** Port every branch, including mode/type selector branches that the R0-R2 fixtures may
  never reach. A missing branch is a stub.
- **No new dependencies, no I/O, no `unsafe`** in library code. Never add
  `#[allow(clippy::all)]`-style blanket allows; use the narrow allow list in `bigamp/mod.rs`.
- **File size.** Every `.rs` file stays under 1000 lines. Aim for at most 900, leaving headroom for
  rustfmt.

**Parity class**:

- `Exact`: the per-sample path uses only IEEE basic ops plus sqrt, fabs, floor, frexp and pow2.
- `Libm`: the path calls a transcendental.
- `Custom(x)`: only with a written reason in `note` and in the module header, after investigation;
  a bound above 1e-6 must be justified in the design doc first.

The measured max-abs is the value seen on aarch64-apple-darwin. Record it in the `ParityRecord`
and in the module header.

**About the "Plugin Pitfalls" in plan files.** They are hints drawn from general knowledge of the
Airwindows codebase. They were not verified line by line against the pinned source when the plans
were written. The pinned upstream `.h`/`.cpp`/`Proc.cpp` always wins:

- If a hint is wrong, ignore it and note that in the Progress Log.
- If a plugin has no integer mode selector, it needs no mode-sweep test.
- If the hint omits a selector that the source has, the test is required.

## 7. Parity test file (`tests/parity_batch2_gNN.rs`, one per group)

Imitate `tests/parity_batch1_g03.rs` exactly:

- `mod common;`
- One `const RECORD_<NAME>: ParityRecord = ParityRecord { name: "<Name>", class, measured_max_abs, note };`
  per plugin. The literal `name: "<Name>"` is grepped by `tests/catalog_batch2.rs`.
- One `#[test] fn <module>()` per plugin. It does:
  - `check_reset_preserves_params`
  - parse `include_str!("fixtures/batch2/gNN/<Name>.txt")` and
    `include_bytes!("fixtures/batch2/gNN/<Name>.f32")`
  - construct via `music_things_airwindows::create_with_seed("<Name>", rate, 0x00c0_ffee)`
  - set R1/R2 params to `ALT[i % 9]` for every parameter index; the count comes from the fixture
    or `INFO` and is not hard-coded short
  - render in 128-frame blocks and compare the 504 stored frames per render
  - check metadata and description as `parity_batch1_g03.rs` does, and assert the max-abs bound
    of the class
- No `#[ignore]`, no early `return`, no loosened bound.
- **Mode-sweep smoke test** (required only for plugins whose parameter selects discrete
  modes/types through an integer cast in `processReplacing`): one extra `#[test]` per such plugin.
  For every discrete value of that parameter (computed from the upstream cast expression, using the
  midpoint of each value's parameter interval), render the full stimulus at 44100 and 96000 Hz with
  the other params at default. Assert that nothing panics and every output sample is finite. This
  covers branches the fixtures do not reach. It is not a parity claim.

## 8. Shared working directory rules (all W2 plans run at once)

- Evidence directory: `tmp/airwindows-batch2/<planId>/`, containing:
  - `intent.md`: written once at start, then immutable. It holds the plugin list, the owned paths
    and the deliverables.
  - `pre-hashes.txt` / `post-hashes.txt`: `shasum -a 256` of every owned file at start and at end.
  - `logs/`
- Re-read a file immediately before each edit. If its content differs from your last write
  (drift), stop editing it, log the drift in the Progress Log, and repair serially after re-reading.
- Edit only your plan's `writePaths`. Never fix a sibling's file.
- **Keep the shared crate compiling.** All 29 W2 plans share one lib crate, so one broken
  declared module breaks every sibling's gates 1 and 2.
  - Run gate 1 immediately after every edit to a declared module of yours.
  - If it fails in your own files and the fix is not immediate (a few minutes), comment out your
    own `mod <module>;` line until the module compiles again, and log that.
  - Never leave your own `mod` line producing a compile error while idle, between subtasks, or at
    session end. Before ending any session, gate 1 must show no error in your own files.
- **Bounded wait on siblings.** If gate 1 or gate 2 fails because of a file you do not own:
  - Retry once after a short interval (a few minutes).
  - If it still fails, record a blocker in your Progress Log (the file path and its owning plan id
    from section 4) and continue porting your own files. Do not keep waiting, and never edit that
    file.
  - Before setting `Completed`, rerun all your gates. A blocker that still prevents your gates from
    passing leaves your plan `Blocked (sibling <planId>)` with the evidence, and the finding is
    routed to the owner.
- Never run `git checkout`, `stash`, `reset`, `commit`, `rebase` or any tree-rewriting git command.
  Never create worktrees or branches.
- Format only owned files: `rustfmt --edition 2021 <files>`. Never run `cargo fmt` (workspace-wide)
  in a port plan.
- Use your own target dir: `CARGO_TARGET_DIR=tmp/airwindows-batch2/<planId>/target`. Cargo env
  prefix, referred to as `$ENV` below:
  `CARGO_TERM_QUIET=true NEXTEST_STATUS_LEVEL=fail NEXTEST_FAILURE_OUTPUT=immediate-final NEXTEST_HIDE_PROGRESS_BAR=1 CARGO_TARGET_DIR=tmp/airwindows-batch2/<planId>/target`.
  You may delete your own `target` once evidence is recorded.
- Run every command in the foreground from the repository root. Pipe the complete output to
  `tmp/airwindows-batch2/<planId>/logs/<name>.log` (`2>&1 | tee`), and record the exit status of
  the command itself, not of `tee`. Use `set -o pipefail` or `${PIPESTATUS[0]}`.

## 9. Port-plan gate template (substitute G = your groups, P = your planId)

1. `$ENV cargo check -p music-things-airwindows --all-targets`: exit 0.
2. `$ENV cargo nextest run -p music-things-airwindows --test parity_batch2_gNN` (one `--test` per
   owned group): exit 0, testsRun = number of plugin tests plus mode-sweep tests, failures 0.
3. `find <each owned group dir> <each owned test file> -name '*.rs' -print0 | xargs -0 rustfmt --edition 2021 --check`:
   exit 0.
4. `test -z "$($ENV cargo clippy -p music-things-airwindows --all-targets --message-format short -- -D warnings 2>&1 | grep -E 'src/plugins/batch2/(gAA|gBB)/|tests/parity_batch2_(gAA|gBB)\.rs')"`:
   exit 0. This checks own files only; diagnostics in sibling files are logged, not fixed.
5. `test -z "$(find <owned group dirs> -name '*.rs' -exec wc -l {} + | awk '$2 != "total" && $1 >= 1000')"`:
   exit 0.
6. `test "$(grep -c '::ENTRY' crates/airwindows/src/plugins/batch2/gNN/mod.rs)" -eq <plugins in group>`
   for each owned group: exit 0. This proves registration.
7. `! grep -rnE 'todo!|unimplemented!|#\[ignore' <owned group dirs> <owned test files>`: exit 0.
   This proves there are no stubs.
7b. `! grep -rn 'dead_code' <owned group dirs>`: exit 0. Every module is registered and no
    temporary `#[allow(dead_code)]` remains (section 5 item 6). It is mandatory for every port plan
    in addition to the commands listed in the plan file.
8. Diagnostic, not a gate: `$ENV cargo nextest run -p music-things-airwindows --test no_alloc --test catalog`.
   A failure naming one of your plugins is your defect and must be fixed. A failure naming a
   sibling's plugin is logged with that plugin's owner.

Port plans never gate on crate-wide clippy, the full test suite, `cargo fmt --check` or the wasm32
build. Only `ab2-99-integration` does.

Verification record format (Progress Log and node output):
`{command, exitStatus: 0, testsRun, testsPassed, failureCount: 0, outcome: "passed", log, notes}`.
Intentional negative controls (for example, a deliberately wrong constant showing that the parity
test fails) go in `mutationEvidence`, never as a passing record. Repaired review findings go only in
`addressedFeedback`.

## 10. Harness-defect escalation

If parity fails and investigation shows that the harness or shim is wrong (not the port):

- Record the evidence (the C++ symbol, the compile error or the wrong value) in your Progress Log.
- Mark the plugin `BLOCKED-HARNESS`, keep it out of `ENTRIES`, and report the finding against
  `ab2-00-scaffold`, the owner of `tools/` for batch 2.
- Do not edit `tools/`.

After ab2-00 lands an additive fix, regenerate only your own fixtures:

`MUSIC_THINGS_REFS=/Users/taco/gits/tacogips/music-things-refs tools/airwindows-harness/generate_batch2.sh --work-dir tmp/airwindows-batch2/<planId>/harness --out-root crates/airwindows/tests/fixtures --group gNN`

A blocked plugin still keeps the plan from `Completed`.

## 11. Progress Log expectations

Append one session entry per session to your own plan file only. Each entry records:

- the plugins done, with class and measured max-abs
- the plugins remaining or blocked, with evidence
- each gate command with exit status, testsRun/testsPassed and the log path
- any drift or sibling-blocker notes

Never edit another plan's file or `impl-plans/README.md`.
