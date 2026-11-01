# Behringer OSS Ports 07: Airwindows Batch-1 Group g06 (kick list lines 41-48)

**Status**: In Progress
**Plan ID**: bop-07
**Design Reference**: design-docs/specs/design-behringer-oss-ports.md section A (Additive registration, Batch-1 grouping, Porting rules, Parity method, Tolerances)
**Depends On**: bop-01 (wave W3; runs in parallel with the other 13 group plans and bop-16c)
**Created**: 2026-10-31
**Last Updated**: 2026-10-31

## Intent and Context

Port the Airwindows plugins on lines 41-48 of `$MUSIC_THINGS_REFS/kick-airwindows-set.txt` (8 plugins)
to Rust, register them in group `g06`, and prove parity against the fixtures bop-01 generated in
`crates/airwindows/tests/fixtures/batch1/g06/`. List the names with
`sed -n '41,48p' "$MUSIC_THINGS_REFS/kick-airwindows-set.txt"` and record them in the Progress Log.
KICK `Console8BusOut` means upstream `Console8BussOut`.

The framework is frozen. Use it exactly as bop-01 built it:
- `crates/airwindows/src/effect.rs:AirwindowsEffect`, `params.rs:{ParamInfo, PluginInfo, sanitize_param}`
- `registry.rs:PluginEntry`, `dither.rs:{fpd_from_seed, xorshift, frexpf_exponent, pow2, fpd_dither}`
- `rand.rs:{AwRand, RAND_MAX, replay_fpd_seed_loop}`, `display.rs:{float2string, db2string, int2string}`
- `tests/common/parity.rs:{check_plugin, ParityRecord, ParityClass}`

Imitate the per-plugin module contract documented in `crates/airwindows/src/plugins/mod.rs`
(`INFO`/`ENTRY`/`construct` consts). Landed sibling plugin files may be read as style references.

## Non-goals

- Do not edit the framework (`src/*.rs` outside `src/plugins/batch1/g06/`), other groups, `tests/common/`,
  `tools/`, `Cargo.toml`/`Cargo.lock`, or any other crate.
- No plugins outside lines 41-48, no stubs, and no `processDoubleReplacing`, chunks or programs.
- Do not regenerate fixtures unless bop-01 has fixed a harness defect that you reported (Escalation).

## Write Ownership (fanout contract)

- writePaths: `crates/airwindows/src/plugins/batch1/g06` (directory: `mod.rs` plus one `<module>.rs` per plugin),
  `crates/airwindows/tests/parity_batch1_g06.rs`, `crates/airwindows/tests/fixtures/batch1/g06` (directory;
  regenerated only after a bop-01 harness fix), `impl-plans/active/behringer-oss-ports-07-aw-b1-g06.md`,
  `target`, `tmp/behringer-oss-ports/bop-07`
- sharedPaths: none
- artifactRoots (digest-only): `target`, `tmp/behringer-oss-ports/bop-07`
- Read-only: framework `crates/airwindows/src/*.rs`, `src/plugins/mod.rs`, `src/plugins/batch1/mod.rs`,
  `crates/airwindows/tests/common/*`, `tools/*` (all frozen by bop-01).
- Never run git checkout/stash/reset or any other git operation that rewrites the tree.

## Common Execution Rules

- `MUSIC_THINGS_REFS=/Users/taco/gits/tacogips/music-things-refs`. Read only
  `airwindows/plugins/LinuxVST/src/<Upstream>/{<Upstream>.h,<Upstream>.cpp,<Upstream>Proc.cpp}` and the
  kick list. Never open Music Tribe AGPL code.
- Evidence directory: `tmp/behringer-oss-ports/bop-07/`. It holds `intent.md` (immutable: the plugin
  list and the deliverables), `pre-hashes.txt` and `post-hashes.txt` (`shasum -a 256` of owned files)
  and `logs/`. Re-read each file before editing; on drift, stop and log it. Only edit `writePaths`.
- 13 sibling plans compile this crate at the same time. If an error is in a file you do not own, rerun
  later and log it. Do not edit that file. Write a plugin file completely before adding its `mod` line
  and `ENTRY` to `g06/mod.rs`.
- Format only owned files: `rustfmt --edition 2021 <files>`. Cargo env: `CARGO_TERM_QUIET=true NEXTEST_STATUS_LEVEL=fail NEXTEST_FAILURE_OUTPUT=immediate-final NEXTEST_HIDE_PROGRESS_BAR=1`.

## Deliverables

1. `crates/airwindows/src/plugins/batch1/g06/<module>.rs`, one per plugin, where `<module>` is the KICK
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
     `tests/fixtures/batch1/g06/<Name>.txt` (unescaped); params with name, label and default as in the fixture.
   - `param_display` and `param_label` reproduce `getParameterDisplay`/`getParameterLabel` case by case,
     using the `display.rs` helpers.
   - `process` ports `processReplacing` statement by statement, and `reset` restores the constructor
     state in place without allocating.
2. `crates/airwindows/src/plugins/batch1/g06/mod.rs`: `mod <module>;` lines and `ENTRIES` in kick-list line order.
3. `crates/airwindows/tests/parity_batch1_g06.rs`: `mod common;`, one `const RECORD_<NAME>: ParityRecord` per
   plugin, and one `#[test] fn <module>()` calling
   `common::parity::check_plugin(&RECORD_X, include_str!("fixtures/batch1/g06/<Name>.txt"), include_bytes!("fixtures/batch1/g06/<Name>.f32"))`.

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
`tools/airwindows-harness/generate.sh --work-dir tmp/behringer-oss-ports/bop-07/harness --out-root crates/airwindows/tests/fixtures --group g06`.

## Tests (situation -> expected)

- Each plugin with `check_plugin`: the metadata, description, R0/R1/R2 and reset re-render match within its class.
- `tests/catalog.rs` and `tests/no_alloc.rs` cover these plugins through the registry. Both pass.

## Verification Commands (evidence)

1. `cargo nextest run -p music-things-airwindows --test parity_batch1_g06`: exit 0, testsRun = 8, failures = 0.
2. `cargo nextest run -p music-things-airwindows --test catalog --test no_alloc`: exit 0.
3. `cargo clippy -p music-things-airwindows --all-targets -- -D warnings`: exit 0, with no warnings in owned files.
4. `rustfmt --edition 2021 --check crates/airwindows/src/plugins/batch1/g06/*.rs crates/airwindows/tests/parity_batch1_g06.rs`: exit 0.
5. `cargo build -p music-things-airwindows --target wasm32-unknown-unknown`: exit 0.
6. `wc -l crates/airwindows/src/plugins/batch1/g06/*.rs`: every file under 1000 lines.

## Completion Criteria

- [x] All 8 plugins of lines 41-48 are ported, registered in order and tested (or marked BLOCKED with evidence).
- [x] Every plugin has a ParityRecord with its class and measured max-abs, repeated in the module header.
- [x] Verifications 1-6 exit 0, with logs in `tmp/behringer-oss-ports/bop-07/logs/`.
- [ ] Only `writePaths` are modified.
- [x] Progress Log records the plugin list, classes, max-abs values, commands, exit status and log paths.

## Progress Log

### Session: 2026-10-31
**Tasks Completed**: Plan created (not started)
**Notes**: -

### Session: 2026-11-01
**Tasks Completed**: C5RawBuss, C5RawChannel, Cabs, Calibre, Capacitor, Capacitor2, Chamber and Chamber2; each Libm, measured max-abs 0.0. Registered in list order.
**Verification**: With `CARGO_TARGET_DIR=$PWD/tmp/behringer-oss-ports/bop-07/check-target` and required quiet environment: `cargo nextest run -p music-things-airwindows --test parity_batch1_g06` exit 0, 8/8 (`logs/check-parity.log`); `cargo nextest run -p music-things-airwindows --test catalog --test no_alloc` exit 0, 4/4 (`logs/check-catalog-noalloc.log`); `cargo clippy -p music-things-airwindows --all-targets -- -D warnings` exit 101, sibling g01 and g14 diagnostics (`logs/check-clippy.log`); owned `rustfmt --edition 2021 --check` exit 0 (`logs/check-rustfmt.log`); `cargo build -p music-things-airwindows --target wasm32-unknown-unknown` exit 0 (`logs/check-wasm.log`); line limit check exit 0 (`logs/check-lines.log`). Logs are under `tmp/behringer-oss-ports/bop-07/`.
**Blockers**: Package-wide clippy requires sibling owners to resolve diagnostics, then rerun. Shared-tree checks require serial integration review.

**Continuation 1**: `CARGO_TARGET_DIR=$PWD/tmp/behringer-oss-ports/bop-07/check-target CARGO_TERM_QUIET=true cargo clippy -p music-things-airwindows --all-targets -- -D warnings` exited 101. The 15 diagnostics remain in sibling-owned g01 and g14 files; complete log: `tmp/behringer-oss-ports/bop-07/logs/clippy-continuation-1.log`. The gate remains blocked pending sibling repairs.

**Continuation 2**: Package-wide `CARGO_TERM_QUIET=true cargo clippy -p music-things-airwindows --all-targets -- -D warnings` exited 101 with the same 15 sibling-owned g01/g14 diagnostics. Log: `tmp/behringer-oss-ports/bop-07/logs/clippy-continuation-2.log`. The required gate remains blocked pending sibling repairs.

**Current verification rerun (2026-11-01)**: With `CARGO_TARGET_DIR=$PWD/tmp/behringer-oss-ports/bop-07/check-target`, `CARGO_TERM_QUIET=true` and the prescribed nextest environment: `cargo nextest run -p music-things-airwindows --test parity_batch1_g06` exit 0, 8/8 (`logs/final-parity.log`); `cargo nextest run -p music-things-airwindows --test catalog --test no_alloc` exit 0, 4/4 (`logs/final-catalog-noalloc.log`); `cargo clippy -p music-things-airwindows --all-targets -- -D warnings` exit 101, 15 diagnostics in sibling-owned g01 and g14 (`logs/final-clippy.log`); the dispatched owned-file rustfmt check exit 0 (`logs/final-rustfmt.log`); `cargo build -p music-things-airwindows --target wasm32-unknown-unknown` exit 0 (`logs/final-wasm.log`); dispatched line-limit check exit 0 (`logs/final-lines.log`). All logs are under `tmp/behringer-oss-ports/bop-07/`. The shared package Clippy gate remains blocked until the owners repair those files; rerun against the integrated tree.

**Redispatch continuation (2026-11-01)**: `CARGO_TARGET_DIR=$PWD/tmp/behringer-oss-ports/bop-07/check-target CARGO_TERM_QUIET=true cargo clippy -p music-things-airwindows --all-targets -- -D warnings` exited 101 with 31 diagnostics in sibling-owned g01, g03, g11 and g14 files. Complete log: `tmp/behringer-oss-ports/bop-07/logs/clippy-redispatch-1.log`. The gate remains blocked until those owners repair their files and the command is rerun.


**Redispatch verification (2026-11-01)**: The eight plugins listed above remain Libm, max-abs 0.0. With `CARGO_TARGET_DIR=$PWD/tmp/behringer-oss-ports/bop-07/check-target`, `CARGO_TERM_QUIET=true`, `NEXTEST_STATUS_LEVEL=fail`, `NEXTEST_FAILURE_OUTPUT=immediate-final`, and `NEXTEST_HIDE_PROGRESS_BAR=1`: `cargo nextest run -p music-things-airwindows --test parity_batch1_g06` exit 0, 8/8 (`logs/turn4-parity.log`); `cargo nextest run -p music-things-airwindows --test catalog --test no_alloc` exit 0, 4/4 (`logs/turn4-catalog.log`); `cargo clippy -p music-things-airwindows --all-targets -- -D warnings` exit 0 (`logs/turn4-clippy.log`); `find crates/airwindows/src/plugins/batch1/g06 crates/airwindows/tests/parity_batch1_g06.rs -name '*.rs' -print0 | xargs -0 rustfmt --edition 2021 --check` exit 0 (`logs/turn4-rustfmt.log`); `cargo build -p music-things-airwindows --target wasm32-unknown-unknown` exit 0 (`logs/turn4-wasm.log`); `test -z "$(find crates/airwindows/src/plugins/batch1/g06 -name '*.rs' -exec wc -l {} + | awk '$2 != "total" && $1 >= 1000')"` exit 0 (`logs/turn4-lines.log`). Logs are under `tmp/behringer-oss-ports/bop-07/`. This was a shared-tree run; integrated review remains downstream.

**Review repair BOP07-RESET-PARAMS (2026-11-01)**: All eight resets now retain parameters while resetting DSP state; Chamber and Chamber2 retain in-place buffer clearing. The regression in `parity_batch1_g06.rs` checks every parameter at 0.37 for all eight. Pre-fix focused test exited 100 (`logs/reset-negative.log`). With `CARGO_TARGET_DIR=$PWD/tmp/behringer-oss-ports/bop-07/repair-target CARGO_TERM_QUIET=true NEXTEST_STATUS_LEVEL=fail NEXTEST_FAILURE_OUTPUT=immediate-final NEXTEST_HIDE_PROGRESS_BAR=1`, `cargo nextest run -p music-things-airwindows --test parity_batch1_g06` exited 0, 9/9 (`logs/repair-parity.log`); `cargo nextest run -p music-things-airwindows --test catalog --test no_alloc` exited 0, 4/4 (`logs/repair-catalog.log`); `cargo clippy -p music-things-airwindows --all-targets -- -D warnings` exited 0 (`logs/repair-clippy.log`); dispatched rustfmt check exited 0 (`logs/repair-rustfmt.log`); `cargo build -p music-things-airwindows --target wasm32-unknown-unknown` exited 0 (`logs/repair-wasm.log`); dispatched line-limit check exited 0 (`logs/repair-lines.log`). Fixture hash check against bop-01 exited 0, 16/16 (`logs/repair-fixtures.log`). Logs are under `tmp/behringer-oss-ports/bop-07/`; final integration review remains downstream.
