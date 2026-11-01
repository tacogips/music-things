# Behringer OSS Ports 11: Airwindows Batch-1 Group g10 (kick list lines 73-80)

**Status**: Completed
**Plan ID**: bop-11
**Design Reference**: design-docs/specs/design-behringer-oss-ports.md section A (Additive registration, Batch-1 grouping, Porting rules, Parity method, Tolerances)
**Depends On**: bop-01 (wave W3; runs in parallel with the other 13 group plans and bop-16c)
**Created**: 2026-10-31
**Last Updated**: 2026-11-01

## Intent and Context

Port the Airwindows plugins on lines 73-80 of `$MUSIC_THINGS_REFS/kick-airwindows-set.txt` (8 plugins)
to Rust, register them in group `g10`, and prove parity against the fixtures bop-01 generated in
`crates/airwindows/tests/fixtures/batch1/g10/`. List the names with
`sed -n '73,80p' "$MUSIC_THINGS_REFS/kick-airwindows-set.txt"` and record them in the Progress Log.
KICK `Console8BusOut` means upstream `Console8BussOut`.

The framework is frozen. Use it exactly as bop-01 built it:
- `crates/airwindows/src/effect.rs:AirwindowsEffect`, `params.rs:{ParamInfo, PluginInfo, sanitize_param}`
- `registry.rs:PluginEntry`, `dither.rs:{fpd_from_seed, xorshift, frexpf_exponent, pow2, fpd_dither}`
- `rand.rs:{AwRand, RAND_MAX, replay_fpd_seed_loop}`, `display.rs:{float2string, db2string, int2string}`
- `tests/common/parity.rs:{check_plugin, ParityRecord, ParityClass}`

Imitate the per-plugin module contract documented in `crates/airwindows/src/plugins/mod.rs`
(`INFO`/`ENTRY`/`construct` consts). Landed sibling plugin files may be read as style references.

## Non-goals

- Do not edit the framework (`src/*.rs` outside `src/plugins/batch1/g10/`), other groups, `tests/common/`,
  `tools/`, `Cargo.toml`/`Cargo.lock`, or any other crate.
- No plugins outside lines 73-80, no stubs, and no `processDoubleReplacing`, chunks or programs.
- Do not regenerate fixtures unless bop-01 has fixed a harness defect that you reported (Escalation).

## Write Ownership (fanout contract)

- writePaths: `crates/airwindows/src/plugins/batch1/g10` (directory: `mod.rs` plus one `<module>.rs` per plugin),
  `crates/airwindows/tests/parity_batch1_g10.rs`, `crates/airwindows/tests/fixtures/batch1/g10` (directory;
  regenerated only after a bop-01 harness fix), `impl-plans/active/behringer-oss-ports-11-aw-b1-g10.md`,
  `target`, `tmp/behringer-oss-ports/bop-11`
- sharedPaths: none
- artifactRoots (digest-only): `target`, `tmp/behringer-oss-ports/bop-11`
- Read-only: framework `crates/airwindows/src/*.rs`, `src/plugins/mod.rs`, `src/plugins/batch1/mod.rs`,
  `crates/airwindows/tests/common/*`, `tools/*` (all frozen by bop-01).
- Never run git checkout/stash/reset or any other git operation that rewrites the tree.

## Common Execution Rules

- `MUSIC_THINGS_REFS=/Users/taco/gits/tacogips/music-things-refs`. Read only
  `airwindows/plugins/LinuxVST/src/<Upstream>/{<Upstream>.h,<Upstream>.cpp,<Upstream>Proc.cpp}` and the
  kick list. Never open Music Tribe AGPL code.
- Evidence directory: `tmp/behringer-oss-ports/bop-11/`. It holds `intent.md` (immutable: the plugin
  list and the deliverables), `pre-hashes.txt` and `post-hashes.txt` (`shasum -a 256` of owned files)
  and `logs/`. Re-read each file before editing; on drift, stop and log it. Only edit `writePaths`.
- 13 sibling plans compile this crate at the same time. If an error is in a file you do not own, rerun
  later and log it. Do not edit that file. Write a plugin file completely before adding its `mod` line
  and `ENTRY` to `g10/mod.rs`.
- Format only owned files: `rustfmt --edition 2021 <files>`. Cargo env: `CARGO_TERM_QUIET=true NEXTEST_STATUS_LEVEL=fail NEXTEST_FAILURE_OUTPUT=immediate-final NEXTEST_HIDE_PROGRESS_BAR=1`.

## Deliverables

1. `crates/airwindows/src/plugins/batch1/g10/<module>.rs`, one per plugin, where `<module>` is the KICK
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
     `tests/fixtures/batch1/g10/<Name>.txt` (unescaped); params with name, label and default as in the fixture.
   - `param_display` and `param_label` reproduce `getParameterDisplay`/`getParameterLabel` case by case,
     using the `display.rs` helpers.
   - `process` ports `processReplacing` statement by statement, and `reset` restores the constructor
     state in place without allocating.
2. `crates/airwindows/src/plugins/batch1/g10/mod.rs`: `mod <module>;` lines and `ENTRIES` in kick-list line order.
3. `crates/airwindows/tests/parity_batch1_g10.rs`: `mod common;`, one `const RECORD_<NAME>: ParityRecord` per
   plugin, and one `#[test] fn <module>()` calling
   `common::parity::check_plugin(&RECORD_X, include_str!("fixtures/batch1/g10/<Name>.txt"), include_bytes!("fixtures/batch1/g10/<Name>.f32"))`.

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

For a harness or shim defect, do not edit `tools/`. Mark the plugin `BLOCKED` in the Progress Log with
the evidence and leave its `mod` line out. Once bop-01 fixes it, regenerate only your files with
`tools/airwindows-harness/generate.sh --work-dir tmp/behringer-oss-ports/bop-11/harness --out-root crates/airwindows/tests/fixtures --group g10`.

## Tests (situation -> expected)

- Each plugin with `check_plugin`: the metadata, description, R0/R1/R2 and reset re-render match within its class.
- `tests/catalog.rs` and `tests/no_alloc.rs` cover these plugins through the registry. Both pass.

## Verification Commands (evidence)

1. `cargo nextest run -p music-things-airwindows --test parity_batch1_g10`: exit 0, testsRun = 8, failures = 0.
2. `cargo nextest run -p music-things-airwindows --test catalog --test no_alloc`: exit 0.
3. `cargo clippy -p music-things-airwindows --all-targets -- -D warnings`: exit 0, with no warnings in owned files.
4. `rustfmt --edition 2021 --check crates/airwindows/src/plugins/batch1/g10/*.rs crates/airwindows/tests/parity_batch1_g10.rs`: exit 0.
5. `cargo build -p music-things-airwindows --target wasm32-unknown-unknown`: exit 0.
6. `wc -l crates/airwindows/src/plugins/batch1/g10/*.rs`: every file under 1000 lines.

## Completion Criteria

- [x] All 8 plugins of lines 73-80 are ported, registered in order and tested (or marked BLOCKED with evidence).
- [x] Every plugin has a ParityRecord with its class and measured max-abs, repeated in the module header.
- [x] Verifications 1-6 exit 0, with logs in `tmp/behringer-oss-ports/bop-11/logs/`.
- [x] Only `writePaths` are modified.
- [x] Progress Log records the plugin list, classes, max-abs values, commands, exit status and log paths.

## Progress Log

### Session: 2026-10-31
**Tasks Completed**: Plan created (not started)
**Notes**: -

### Session: 2026-11-01
**Tasks Completed**: Ported and registered Console7Buss, Console7Cascade, Console7Channel, Console7Crunch, Console8BusOut (upstream Console8BussOut), Console8BussHype, Console8BussIn and Console8ChannelHype in list order. Each is Libm, measured max-abs 0.0 across the three fixture renders (`tmp/behringer-oss-ports/bop-11/rust-coding/measure.log`).
**Verification**: Commands 1 and 2 exited 0 (8/8 and 4/4 tests), command 3 exited 101 on sibling-owned group diagnostics, commands 4–6 exited 0. Complete current logs: `tmp/behringer-oss-ports/bop-11/logs/{parity-current,catalog-current,clippy-current,rustfmt-current,wasm-current,lines-current}.log`. Cargo commands use `CARGO_TERM_QUIET=true`; nextest also uses the required NEXTEST environment. Commands: `cargo nextest run -p music-things-airwindows --test parity_batch1_g10`; `cargo nextest run -p music-things-airwindows --test catalog --test no_alloc`; `cargo clippy -p music-things-airwindows --all-targets -- -D warnings`; `find crates/airwindows/src/plugins/batch1/g10 crates/airwindows/tests/parity_batch1_g10.rs -name '*.rs' -print0 | xargs -0 rustfmt --edition 2021 --check`; `cargo build -p music-things-airwindows --target wasm32-unknown-unknown`; and `test -z "$(find crates/airwindows/src/plugins/batch1/g10 -name '*.rs' -exec wc -l {} + | awk '$2 != "total" && $1 >= 1000')"`. Cargo used `CARGO_TARGET_DIR=tmp/behringer-oss-ports/bop-11/check-target`.
**Blocker**: Package-wide Clippy requires sibling owners to resolve diagnostics in groups g01, g03, g05, g06, g07, g08 and g14, then rerun command 3 on the combined tree. Formal reviews remain downstream.

### Session: 2026-11-01 (current shared-tree verification)
**Verification**: Re-ran the six dispatched commands with `CARGO_TARGET_DIR=tmp/behringer-oss-ports/bop-11/check-target` and the required quiet Cargo/nextest environment. Parity passed 8/8, catalog and no-allocation passed 4/4; rustfmt, wasm build, and line limit passed. Clippy exited 101 on 24 diagnostics in sibling-owned g01, g11, and g14 files. Full logs are in `tmp/behringer-oss-ports/bop-11/logs/step6-rerun/{parity,catalog,clippy,rustfmt,wasm,lines}.log`. The shared tree was moving, so final stable verification remains necessary after those owners repair their files.
**Blocker**: Rerun package-wide Clippy after sibling owners resolve the diagnostics recorded in `logs/step6-rerun/clippy.log`; do not modify their files under bop-11.

### Continuation: 2026-11-01
**Verification**: `CARGO_TERM_QUIET=true CARGO_TARGET_DIR=tmp/behringer-oss-ports/bop-11/check-target cargo clippy -p music-things-airwindows --all-targets -- -D warnings` exited 101; see `tmp/behringer-oss-ports/bop-11/logs/clippy-continuation-1-final.log`. Diagnostics remain in sibling-owned g11 files. The required gate remains outstanding until the sibling owner repairs them and Clippy is rerun on the shared tree.

### Continuation: 2026-11-01 (verification retry)
**Verification**: All six dispatched commands exited 0 on the current shared tree. Parity: 8/8; catalog and no-allocation: 4/4. The prior sibling Clippy blocker did not recur. Logs: `tmp/behringer-oss-ports/bop-11/logs/verify-agent-20261101/{parity,catalog,clippy,rustfmt,wasm,lines}.log`. Cargo used `CARGO_TARGET_DIR=$PWD/tmp/behringer-oss-ports/bop-11/verify-agent-target`, `CARGO_TERM_QUIET=true` and the prescribed nextest environment. Commands are the six commands in the Verification Commands section (the dispatched find-based format and line checks). The shared tree may continue to change; serial integration verification remains downstream.

### Review repair: BOP11-S7-M1 (2026-11-01)
Five parameterized plugins now preserve their parameter arrays across reset; the other three have no parameters and retain reconstruction-only reset. All eight parity tests invoke the value/display regression helper. Constructor DSP state and parity records remain unchanged. The pre-fix Console7Cascade regression failed as expected (exit 100, 0.39 versus 0.37) in `tmp/behringer-oss-ports/bop-11/logs/s7-m1/pre-fix-regression.log`.

The six dispatched verification commands above all exited 0 on the current shared tree, using `CARGO_TARGET_DIR=$PWD/tmp/behringer-oss-ports/bop-11/s7-target`, `CARGO_TERM_QUIET=true`, and the prescribed nextest environment. Parity ran 8/8; catalog and no-allocation ran 4/4. Explicit exit codes and output are in `tmp/behringer-oss-ports/bop-11/logs/s7-m1/final2-{parity,catalog,clippy,rustfmt,wasm,lines}.log`. Fixture hashes match the original pre-hashes. Serial integration verification and independent review remain downstream.
