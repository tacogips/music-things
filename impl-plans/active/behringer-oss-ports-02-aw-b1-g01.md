# Behringer OSS Ports 02: Airwindows Batch-1 Group g01 (kick list lines 1-8)

**Status**: Ready
**Plan ID**: bop-02
**Design Reference**: design-docs/specs/design-behringer-oss-ports.md section A (Additive registration, Batch-1 grouping, Porting rules, Parity method, Tolerances)
**Depends On**: bop-01 (wave W3; runs in parallel with the other 13 group plans and bop-16c)
**Created**: 2026-10-31
**Last Updated**: 2026-10-31

## Intent and Context

Port the Airwindows plugins on lines 1-8 of `$MUSIC_THINGS_REFS/kick-airwindows-set.txt` (8 plugins;
line 1 is `Acceleration`) to Rust, register them in group `g01`, and prove parity against the fixtures
bop-01 already generated in `crates/airwindows/tests/fixtures/batch1/g01/`. List the names with
`sed -n '1,8p' "$MUSIC_THINGS_REFS/kick-airwindows-set.txt"` and record them in the Progress Log.
KICK `Console8BusOut` means upstream `Console8BussOut`.

The framework is frozen. Use it exactly as bop-01 built it:
- `crates/airwindows/src/effect.rs:AirwindowsEffect`, `params.rs:{ParamInfo, PluginInfo, sanitize_param}`
- `registry.rs:PluginEntry`, `dither.rs:{fpd_from_seed, xorshift, frexpf_exponent, pow2, fpd_dither}`
- `rand.rs:{AwRand, RAND_MAX, replay_fpd_seed_loop}`, `display.rs:{float2string, db2string, int2string}`
- `tests/common/parity.rs:{check_plugin, ParityRecord, ParityClass}`

Imitate the per-plugin module contract documented in `crates/airwindows/src/plugins/mod.rs`
(`INFO`/`ENTRY`/`construct` consts).

## Non-goals

- Do not edit the framework (`src/*.rs` outside `src/plugins/batch1/g01/`), other groups, `tests/common/`,
  `tools/`, `Cargo.toml`/`Cargo.lock`, or any other crate.
- No plugins outside lines 1-8, no stubs, and no `processDoubleReplacing`, chunks or programs.
- Do not regenerate fixtures unless bop-01 has fixed a harness defect that you reported (Escalation).

## Write Ownership (fanout contract)

- writePaths: `crates/airwindows/src/plugins/batch1/g01` (directory: `mod.rs` plus one `<module>.rs` per plugin),
  `crates/airwindows/tests/parity_batch1_g01.rs`, `crates/airwindows/tests/fixtures/batch1/g01` (directory;
  regenerated only after a bop-01 harness fix), `impl-plans/active/behringer-oss-ports-02-aw-b1-g01.md`,
  `target`, `tmp/behringer-oss-ports/bop-02`
- sharedPaths: none
- artifactRoots (digest-only): `target`, `tmp/behringer-oss-ports/bop-02`
- Read-only: framework `crates/airwindows/src/*.rs`, `src/plugins/mod.rs`, `src/plugins/batch1/mod.rs`,
  `crates/airwindows/tests/common/*`, `tools/*` (all frozen by bop-01).
- Never run git checkout/stash/reset or any other git operation that rewrites the tree.

## Common Execution Rules

- `MUSIC_THINGS_REFS=/Users/taco/gits/tacogips/music-things-refs`. Read only
  `airwindows/plugins/LinuxVST/src/<Upstream>/{<Upstream>.h,<Upstream>.cpp,<Upstream>Proc.cpp}` and the
  kick list. Never open Music Tribe AGPL code.
- Evidence directory: `tmp/behringer-oss-ports/bop-02/`. It holds `intent.md` (immutable: the plugin
  list and the deliverables), `pre-hashes.txt` and `post-hashes.txt` (`shasum -a 256` of owned files)
  and `logs/`. Re-read each file before editing; on drift, stop and log it. Only edit `writePaths`.
- 13 sibling plans compile this crate at the same time. If an error is in a file you do not own, rerun
  later and log it. Do not edit that file. Write a plugin file completely before adding its `mod` line
  and `ENTRY` to `g01/mod.rs`.
- Format only owned files: `rustfmt --edition 2021 <files>`. Cargo env: `CARGO_TERM_QUIET=true NEXTEST_STATUS_LEVEL=fail NEXTEST_FAILURE_OUTPUT=immediate-final NEXTEST_HIDE_PROGRESS_BAR=1`.

## Deliverables

1. `crates/airwindows/src/plugins/batch1/g01/<module>.rs`, one per plugin, where `<module>` is the KICK
   name lowercased. Each file has:
   - A header: the upstream copyright line verbatim from `<Upstream>.h`, the upstream path, the
     revision `b110fac3991fff7c4655cc2d25d6a3e221f9799c`, and a `Parity:` line giving the class and the
     measured max-abs (plus the reason when Custom).
   - A struct named after the upstream class. It holds the params `[f32; N]`, `sample_rate: f32`,
     `seed: u32`, the state mirroring the `.h` members (snake_case), and `fpd_l`/`fpd_r` when upstream
     has them.
   - A constructor that sets defaults exactly as upstream does. It creates `AwRand::new(seed)` and
     replays every upstream constructor `rand()` call in order (including the fpd loop, through
     `replay_fpd_seed_loop`), then sets `fpd_l = fpd_from_seed(seed, 0)` and
     `fpd_r = fpd_from_seed(seed, 1)`. Any `rand()` in `processReplacing` uses the same `AwRand`.
   - `INFO`: name = the KICK name; upstream_name; `description` = the exact `description=` value of
     `tests/fixtures/batch1/g01/<Name>.txt` (unescaped); params with name, label and default as in the fixture.
   - `param_display` and `param_label` reproduce `getParameterDisplay`/`getParameterLabel` case by case,
     using the `display.rs` helpers.
   - `process` ports `processReplacing` statement by statement, and `reset` restores the constructor
     state in place without allocating.
2. `crates/airwindows/src/plugins/batch1/g01/mod.rs`: `mod <module>;` lines and `ENTRIES` in kick-list line order.
3. `crates/airwindows/tests/parity_batch1_g01.rs`: `mod common;`, one `const RECORD_<NAME>: ParityRecord` per
   plugin, and one `#[test] fn <module>()` calling
   `common::parity::check_plugin(&RECORD_X, include_str!("fixtures/batch1/g01/<Name>.txt"), include_bytes!("fixtures/batch1/g01/<Name>.f32"))`.

## Porting Rules (common failure sources)

- Precision: `float` becomes `f32`, and `double`/`long double` become `f64`. Mixed expressions are
  computed in f64 and rounded only where C++ assigns to a float. A libm call on a float argument is the
  f32 function (`sinf`); on a double it is the f64 one. Check each call's argument type.
- Keep the operation order. Never rewrite `pow(x,2)` as `x*x`, never reorder or factor, and never use
  `mul_add`. Unsigned math wraps, and `uint32_t` shifts are `u32`.
- Copy the near-zero input replacement exactly. Use `fpd_dither` only when the plugin's dither block
  matches the canonical form exactly; otherwise inline it. The output cast is `as f32`.
- `overallscale` uses `self.sample_rate as f64`, in the same order as upstream.
- Statics become fields. Arrays longer than 4096 elements are `Box<[T]>`.
- Class: use `Exact` when the per-sample path has no libm transcendental (sqrt, fabs, floor, frexp and
  pow2 are allowed), otherwise `Libm`. Use `Custom(x)` only with a written reason in `note` and in the header.

## Escalation

For a harness or shim defect (a wrong description extraction, a missing shim behavior, a seeding
mismatch), do not edit `tools/`. Mark the plugin `BLOCKED` in the Progress Log with the evidence and
leave its `mod` line out. Once bop-01 fixes it, regenerate only your files with
`tools/airwindows-harness/generate.sh --work-dir tmp/behringer-oss-ports/bop-02/harness --out-root crates/airwindows/tests/fixtures --group g01`.

## Tests (situation -> expected)

- Each plugin with `check_plugin`: the metadata, description, R0/R1/R2 and reset re-render match within its class.
- `tests/catalog.rs` and `tests/no_alloc.rs` iterate the registry and now cover these plugins. Both pass
  (0 allocations during process).

## Verification Commands (evidence)

1. `cargo nextest run -p music-things-airwindows --test parity_batch1_g01`: exit 0, testsRun = 8, failures = 0.
2. `cargo nextest run -p music-things-airwindows --test catalog --test no_alloc`: exit 0.
3. `cargo clippy -p music-things-airwindows --all-targets -- -D warnings`: exit 0, with no warnings in owned files.
4. `rustfmt --edition 2021 --check crates/airwindows/src/plugins/batch1/g01/*.rs crates/airwindows/tests/parity_batch1_g01.rs`: exit 0.
5. `cargo build -p music-things-airwindows --target wasm32-unknown-unknown`: exit 0.
6. `wc -l crates/airwindows/src/plugins/batch1/g01/*.rs`: every file under 1000 lines.

## Completion Criteria

- [x] All 8 plugins of lines 1-8 are ported, registered in order and tested (or marked BLOCKED with evidence).
- [x] Every plugin has a ParityRecord with its class and measured max-abs, repeated in the module header.
- [x] Verifications 1-6 exit 0, with logs in `tmp/behringer-oss-ports/bop-02/logs/`.
- [x] Only `writePaths` are modified.
- [x] Progress Log records the plugin list, classes, max-abs values, commands, exit status and log paths.

## Progress Log

### Session: 2026-10-31
**Tasks Completed**: Plan created (not started)
**Notes**: -

### Session: 2026-11-01
**Status**: Blocked; implementation incomplete.
**Plugin list**: Acceleration, Acceleration2, ADClip7, ADT, Air, Air2, Apicolypse, AQuickVoiceClip.
**Progress**: Seven provisional ports and parity tests are present; ADClip7 remains unimplemented. All provisional records currently say Libm with measurement pending; the recorded zero values are placeholders, not measurements. Acceleration passed one earlier focused parity run (`logs/acceleration-parity.log`, 1/1), before subsequent shared-tree changes.
**Blocker**: Required cargo gates fail before testing with E0507 in sibling-owned `g02/mod.rs` and `g12/mod.rs`; see `tmp/behringer-oss-ports/bop-02/logs/check-agent-parity.log`. The owners must repair their registrations before this plan can complete and rerun its gates. No harness defect has been established.
**Checks on moving tree**: `cargo nextest run -p music-things-airwindows --test parity_batch1_g01` exit 101, 0 tests (`logs/check-agent-parity.log`); `cargo nextest run -p music-things-airwindows --test catalog --test no_alloc` exit 101, 0 tests (`logs/check-agent-catalog.log`); `cargo clippy -p music-things-airwindows --all-targets -- -D warnings` exit 101 (`logs/check-agent-clippy.log`); scoped rustfmt check exit 1 (`logs/check-agent-rustfmt.log`); `cargo build -p music-things-airwindows --target wasm32-unknown-unknown` exit 101 (`logs/check-agent-wasm.log`); file-size check exit 0 (`logs/check-agent-lines.log`). Log paths are relative to `tmp/behringer-oss-ports/bop-02/`.
**Remaining**: Finish ADClip7, verify and correct every provisional port, measure maxima, format owned files, rerun all six gates and update the criteria. Formal reviews follow implementation.


### Session: 2026-11-01 (continued)
**Status**: Blocked on shared-tree Clippy; implementation verification incomplete.
**Plugins and parity**: Acceleration, Acceleration2, ADClip7, ADT, Air, Air2, Apicolypse, AQuickVoiceClip: each Libm, measured max-abs 0.0 on aarch64-apple-darwin across all three fixture renders (`tmp/behringer-oss-ports/bop-02/logs/measure-01.log`, exit 0). ADClip7 is now implemented and registered; g01 Clippy diagnostics repaired.
**Checks on moving shared tree**: `CARGO_TERM_QUIET=true CARGO_TARGET_DIR=target/bop-02 cargo nextest run -p music-things-airwindows --test parity_batch1_g01` exit 0, 8/8 (`logs/final2-parity.log`); `cargo nextest run -p music-things-airwindows --test catalog --test no_alloc` exit 0, 4/4 (`logs/final2-catalog-no-alloc.log`); `cargo clippy -p music-things-airwindows --all-targets -- -D warnings` exit 101 (`logs/final2-clippy.log`), exclusively sibling g11 diagnostics; scoped rustfmt check exit 0 (`logs/final2-rustfmt.log`); wasm build exit 0 (`logs/final2-wasm.log`); line-count check exit 0 (`logs/final2-lines.log`). Cargo runs use the required quiet and nextest environment and isolated target directory. Logs are relative to `tmp/behringer-oss-ports/bop-02/`.
**Resume criterion**: g11 owner repairs Clippy errors in console8channelin.rs, console8subhype.rs, console8subin.rs and crystal.rs; rerun all required gates on a stable shared tree. Formal reviews and workflow finalization remain downstream.

### Continuation 1: 2026-11-01
**Status**: Blocked on shared-tree Clippy; the prior g11 diagnostics no longer appear. All eight plugins remain Libm with measured max-abs 0.0 as recorded above.
**Moving-tree verification**: `CARGO_TARGET_DIR=target/bop-02 CARGO_TERM_QUIET=true NEXTEST_STATUS_LEVEL=fail NEXTEST_FAILURE_OUTPUT=immediate-final NEXTEST_HIDE_PROGRESS_BAR=1 cargo nextest run -p music-things-airwindows --test parity_batch1_g01` exit 0, 8/8 (`logs/continuation1-parity.log`); same environment, `cargo nextest run -p music-things-airwindows --test catalog --test no_alloc` exit 0, 4/4 (`logs/continuation1-catalog-no-alloc.log`); `CARGO_TARGET_DIR=target/bop-02 CARGO_TERM_QUIET=true cargo clippy -p music-things-airwindows --all-targets -- -D warnings` exit 101 (`logs/continuation1-clippy.log`); scoped rustfmt exit 0 (`logs/continuation1-rustfmt.log`); wasm build exit 0 (`logs/continuation1-wasm.log`); line check exit 0 (`logs/continuation1-lines.log`). Logs are relative to `tmp/behringer-oss-ports/bop-02/`.
**Resume criterion**: g05 owner repairs `crates/airwindows/tests/parity_batch1_g05.rs:50` excessive-precision Clippy error; rerun required checks on the shared tree. No g01 files were changed in this continuation.

### Continuation 2: 2026-11-01
**Status**: Implementation gates passed on the current shared tree; formal review remains downstream. No Rust files changed in this continuation.
**Plugins and parity**: Acceleration, Acceleration2, ADClip7, ADT, Air, Air2, Apicolypse, AQuickVoiceClip: Libm, measured max-abs 0.0 each (`logs/measure-01.log`).
**Verification**: With `CARGO_TARGET_DIR=target/bop-02 CARGO_TERM_QUIET=true NEXTEST_STATUS_LEVEL=fail NEXTEST_FAILURE_OUTPUT=immediate-final NEXTEST_HIDE_PROGRESS_BAR=1`, `cargo nextest run -p music-things-airwindows --test parity_batch1_g01` exit 0, 8/8 (`logs/redispatch2-parity.log`); `cargo nextest run -p music-things-airwindows --test catalog --test no_alloc` exit 0, 4/4 (`logs/redispatch2-catalog.log`); `cargo clippy -p music-things-airwindows --all-targets -- -D warnings` exit 0 (`logs/redispatch2-clippy.log`); `find crates/airwindows/src/plugins/batch1/g01 crates/airwindows/tests/parity_batch1_g01.rs -name '*.rs' -print0 | xargs -0 rustfmt --edition 2021 --check` exit 0 (`logs/redispatch2-rustfmt.log`); `cargo build -p music-things-airwindows --target wasm32-unknown-unknown` exit 0 (`logs/redispatch2-wasm.log`); `test -z "$(find crates/airwindows/src/plugins/batch1/g01 -name '*.rs' -exec wc -l {} + | awk '$2 != "total" && $1 >= 1000')"` exit 0 (`logs/redispatch2-lines.log`). Logs relative to `tmp/behringer-oss-ports/bop-02/`; quiet checks have empty logs and their exits were observed. Shared-tree integration verification remains downstream.

### Review revision: BOP02-RESET-PARAMS
The rust-coding agent preserved parameters across reset in Acceleration, Acceleration2, ADClip7, ADT, Apicolypse and AQuickVoiceClip. The existing eight parity tests now also assert every parameter remains 0.37 after reset; Air and Air2 already preserved them. Parity classes and fixtures remain unchanged; previously measured maxima are 0.0 for each plugin (`logs/measure-01.log`).

On the current shared tree, with `CARGO_TARGET_DIR=target/bop-02-review CARGO_TERM_QUIET=true NEXTEST_STATUS_LEVEL=fail NEXTEST_FAILURE_OUTPUT=immediate-final NEXTEST_HIDE_PROGRESS_BAR=1`, the check-and-test-after-modify agent recorded explicit exits in `tmp/behringer-oss-ports/bop-02/logs/`:
- `cargo nextest run -p music-things-airwindows --test parity_batch1_g01`: exit 0, 8/8 (`review-reset-parity.log`).
- `cargo nextest run -p music-things-airwindows --test catalog --test no_alloc`: exit 0, 4/4 (`review-reset-catalog.log`).
- `cargo clippy -p music-things-airwindows --all-targets -- -D warnings`: exit 0 (`review-reset-clippy.log`).
- `find crates/airwindows/src/plugins/batch1/g01 crates/airwindows/tests/parity_batch1_g01.rs -name '*.rs' -print0 | xargs -0 rustfmt --edition 2021 --check`: exit 0 (`review-reset-rustfmt.log`).
- `cargo build -p music-things-airwindows --target wasm32-unknown-unknown`: exit 0 (`review-reset-wasm.log`).
- `test -z "$(find crates/airwindows/src/plugins/batch1/g01 -name '*.rs' -exec wc -l {} + | awk '$2 != "total" && $1 >= 1000')"`: exit 0 (`review-reset-lines.log`).
Independent re-review and stable-tree integration remain downstream. The pre-fix defect was confirmed by source inspection, not a behavioral run.
