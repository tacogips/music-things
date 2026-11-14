# Behringer OSS Ports 13: Airwindows Batch-1 Group g12 (kick list lines 89-96)

**Status**: Completed
**Plan ID**: bop-13
**Design Reference**: design-docs/specs/design-behringer-oss-ports.md section A (Additive registration, Batch-1 grouping, Porting rules, Parity method, Tolerances)
**Depends On**: bop-01 (wave W3; runs in parallel with the other 13 group plans and bop-16c)
**Created**: 2026-10-31
**Last Updated**: 2026-11-01

## Intent and Context

Port the Airwindows plugins on lines 89-96 of `$MUSIC_THINGS_REFS/kick-airwindows-set.txt` (8 plugins)
to Rust, register them in group `g12`, and prove parity against the fixtures bop-01 generated in
`crates/airwindows/tests/fixtures/batch1/g12/`. List the names with
`sed -n '89,96p' "$MUSIC_THINGS_REFS/kick-airwindows-set.txt"` and record them in the Progress Log.
KICK `Console8BusOut` means upstream `Console8BussOut`.

The framework is frozen. Use it exactly as bop-01 built it:
- `crates/airwindows/src/effect.rs:AirwindowsEffect`, `params.rs:{ParamInfo, PluginInfo, sanitize_param}`
- `registry.rs:PluginEntry`, `dither.rs:{fpd_from_seed, xorshift, frexpf_exponent, pow2, fpd_dither}`
- `rand.rs:{AwRand, RAND_MAX, replay_fpd_seed_loop}`, `display.rs:{float2string, db2string, int2string}`
- `tests/common/parity.rs:{check_plugin, ParityRecord, ParityClass}`

Imitate the per-plugin module contract documented in `crates/airwindows/src/plugins/mod.rs`
(`INFO`/`ENTRY`/`construct` consts). Landed sibling plugin files may be read as style references.

## Non-goals

- Do not edit the framework (`src/*.rs` outside `src/plugins/batch1/g12/`), other groups, `tests/common/`,
  `tools/`, `Cargo.toml`/`Cargo.lock`, or any other crate.
- No plugins outside lines 89-96, no stubs, and no `processDoubleReplacing`, chunks or programs.
- Do not regenerate fixtures unless bop-01 has fixed a harness defect that you reported (Escalation).

## Write Ownership (fanout contract)

- writePaths: `crates/airwindows/src/plugins/batch1/g12` (directory: `mod.rs` plus one `<module>.rs` per plugin),
  `crates/airwindows/tests/parity_batch1_g12.rs`, `crates/airwindows/tests/fixtures/batch1/g12` (directory;
  regenerated only after a bop-01 harness fix), `impl-plans/active/behringer-oss-ports-13-aw-b1-g12.md`,
  `target`, `tmp/behringer-oss-ports/bop-13`
- sharedPaths: none
- artifactRoots (digest-only): `target`, `tmp/behringer-oss-ports/bop-13`
- Read-only: framework `crates/airwindows/src/*.rs`, `src/plugins/mod.rs`, `src/plugins/batch1/mod.rs`,
  `crates/airwindows/tests/common/*`, `tools/*` (all frozen by bop-01).
- Never run git checkout/stash/reset or any other git operation that rewrites the tree.

## Common Execution Rules

- `MUSIC_THINGS_REFS=/Users/taco/gits/tacogips/music-things-refs`. Read only
  `airwindows/plugins/LinuxVST/src/<Upstream>/{<Upstream>.h,<Upstream>.cpp,<Upstream>Proc.cpp}` and the
  kick list. Never open Music Tribe AGPL code.
- Evidence directory: `tmp/behringer-oss-ports/bop-13/`. It holds `intent.md` (immutable: the plugin
  list and the deliverables), `pre-hashes.txt` and `post-hashes.txt` (`shasum -a 256` of owned files)
  and `logs/`. Re-read each file before editing; on drift, stop and log it. Only edit `writePaths`.
- 13 sibling plans compile this crate at the same time. If an error is in a file you do not own, rerun
  later and log it. Do not edit that file. Write a plugin file completely before adding its `mod` line
  and `ENTRY` to `g12/mod.rs`.
- Format only owned files: `rustfmt --edition 2021 <files>`. Cargo env: `CARGO_TERM_QUIET=true NEXTEST_STATUS_LEVEL=fail NEXTEST_FAILURE_OUTPUT=immediate-final NEXTEST_HIDE_PROGRESS_BAR=1`.

## Deliverables

1. `crates/airwindows/src/plugins/batch1/g12/<module>.rs`, one per plugin, where `<module>` is the KICK
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
     `tests/fixtures/batch1/g12/<Name>.txt` (unescaped); params with name, label and default as in the fixture.
   - `param_display` and `param_label` reproduce `getParameterDisplay`/`getParameterLabel` case by case,
     using the `display.rs` helpers.
   - `process` ports `processReplacing` statement by statement, and `reset` restores the constructor
     state in place without allocating.
2. `crates/airwindows/src/plugins/batch1/g12/mod.rs`: `mod <module>;` lines and `ENTRIES` in kick-list line order.
3. `crates/airwindows/tests/parity_batch1_g12.rs`: `mod common;`, one `const RECORD_<NAME>: ParityRecord` per
   plugin, and one `#[test] fn <module>()` calling
   `common::parity::check_plugin(&RECORD_X, include_str!("fixtures/batch1/g12/<Name>.txt"), include_bytes!("fixtures/batch1/g12/<Name>.f32"))`.

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
`tools/airwindows-harness/generate.sh --work-dir tmp/behringer-oss-ports/bop-13/harness --out-root crates/airwindows/tests/fixtures --group g12`.

## Tests (situation -> expected)

- Each plugin with `check_plugin`: the metadata, description, R0/R1/R2 and reset re-render match within its class.
- `tests/catalog.rs` and `tests/no_alloc.rs` cover these plugins through the registry. Both pass.

## Verification Commands (evidence)

1. `cargo nextest run -p music-things-airwindows --test parity_batch1_g12`: exit 0, testsRun = 8, failures = 0.
2. `cargo nextest run -p music-things-airwindows --test catalog --test no_alloc`: exit 0.
3. `cargo clippy -p music-things-airwindows --all-targets -- -D warnings`: exit 0, with no warnings in owned files.
4. `rustfmt --edition 2021 --check crates/airwindows/src/plugins/batch1/g12/*.rs crates/airwindows/tests/parity_batch1_g12.rs`: exit 0.
5. `cargo build -p music-things-airwindows --target wasm32-unknown-unknown`: exit 0.
6. `wc -l crates/airwindows/src/plugins/batch1/g12/*.rs`: every file under 1000 lines.

## Completion Criteria

- [x] All 8 plugins of lines 89-96 are ported, registered in order and tested (or marked BLOCKED with evidence).
- [x] Every plugin has a ParityRecord with its class and measured max-abs, repeated in the module header.
- [x] Verifications 1-6 exit 0, with logs in `tmp/behringer-oss-ports/bop-13/logs/`.
- [x] Only `writePaths` are modified.
- [x] Progress Log records the plugin list, classes, max-abs values, commands, exit status and log paths.

## Progress Log

### Session: 2026-10-31
**Tasks Completed**: Plan created (not started)
**Notes**: -

### Session: 2026-11-01 — implementation in progress
**Plugins and measured maximum absolute error**: curve (Libm, 0), Dark (Libm, 0), DarkNoise (Libm, 0), DCVoltage (Exact, 0), DeBess (Libm, 0), Deckwrecka (Libm, 0), DeEss (Libm, 5.1315766e-8), DeHiss (Libm, 0). Measurement: `tmp/behringer-oss-ports/bop-13/logs/measurement.log`. All are registered in list order; measurements and records are provisional pending review.

**Verification** (isolated `CARGO_TARGET_DIR=tmp/behringer-oss-ports/bop-13/target`; cargo quiet and nextest environment as specified above; shared tree may change):

| Command | Exit | Tests passed/run | Log under `tmp/behringer-oss-ports/bop-13/logs/` |
|---|---:|---:|---|
| `cargo nextest run -p music-things-airwindows --test parity_batch1_g12` | 0 | 8/8 | `final3-parity.log` |
| `cargo nextest run -p music-things-airwindows --test catalog --test no_alloc` | 0 | 4/4 | `final3-catalog.log` |
| `cargo clippy -p music-things-airwindows --all-targets -- -D warnings` | 101 | 0/0 | `final3-clippy.log` |
| `find crates/airwindows/src/plugins/batch1/g12 crates/airwindows/tests/parity_batch1_g12.rs -name '*.rs' -print0 \| xargs -0 rustfmt --edition 2021 --check` | 0 | 0/0 | `final3-rustfmt.log` |
| `cargo build -p music-things-airwindows --target wasm32-unknown-unknown` | 0 | 0/0 | `final3-wasm.log` |
| `test -z "$(find crates/airwindows/src/plugins/batch1/g12 -name '*.rs' -exec wc -l {} + \| awk '$2 != "total" && $1 >= 1000')"` | 0 | 0/0 | `final3-lines.log` |

**Blocker**: Clippy reports sibling-owned g01, g08 and g14 files; no g12 diagnostic remains. The sibling owners must resolve their diagnostics before rerunning the package-wide gate. The earlier six-plugin parity attempt failed during concurrent sibling edits (`logs/parity-provisional.log`); the final-source parity run passed.

### Continuation: package Clippy retry
`CARGO_TARGET_DIR=tmp/behringer-oss-ports/bop-13/target CARGO_TERM_QUIET=true cargo clippy -p music-things-airwindows --all-targets -- -D warnings` exited 101; `tmp/behringer-oss-ports/bop-13/logs/continuation-clippy.log` records 19 diagnostics in sibling-owned g01, g06 and g14 files, none in g12. The package gate remains blocked until those owners repair their files. The g12 sources, fixtures and parity test match the previous post-hashes (`logs/continuation-hashes.log`, exit 0); previous focused test results therefore remain applicable to those owned sources on the shared tree.

### Continuation 2: package Clippy retry

The required `CARGO_TARGET_DIR=tmp/behringer-oss-ports/bop-13/target CARGO_TERM_QUIET=true cargo clippy -p music-things-airwindows --all-targets -- -D warnings` exited 101; `tmp/behringer-oss-ports/bop-13/logs/continuation2-clippy.log` records 15 diagnostics in sibling-owned g01 and g14, none in g12. The gate remains blocked pending sibling repairs.

### Continuation: current shared-tree verification

The isolated target `tmp/behringer-oss-ports/bop-13/target` was used with the prescribed quiet Cargo and nextest environment. `recheck-parity.log` passed 8/8; `recheck-catalog.log` passed 4/4; `recheck-rustfmt.log`, `recheck-wasm.log`, and `recheck-lines.log` exited 0. The required package-wide Clippy command exited 101 (`logs/recheck-clippy.log`), with 27 diagnostics in sibling-owned g01, g03, g11 and g14 files and none in g12. The package gate remains blocked pending those owners' repairs and a fresh rerun. These results were collected on the moving shared tree.

### Continuation: sibling gate retry

`CARGO_TARGET_DIR=tmp/behringer-oss-ports/bop-13/target CARGO_TERM_QUIET=true cargo clippy -p music-things-airwindows --all-targets -- -D warnings` exited 101. `tmp/behringer-oss-ports/bop-13/logs/continuation3-clippy-retry.log` contains 23 diagnostics in sibling-owned g11 files, none in g12. The package gate requires sibling repair and a fresh rerun. The first retry log is preserved as `logs/continuation3-clippy.log`; its shell exit capture failed because `status` is read-only in zsh.

### Continuation: current verification

All six required commands passed on the shared tree using `CARGO_TARGET_DIR=target/bop-13-current`, `CARGO_TERM_QUIET=true`, `NEXTEST_STATUS_LEVEL=fail`, `NEXTEST_FAILURE_OUTPUT=immediate-final`, and `NEXTEST_HIDE_PROGRESS_BAR=1`. The prior sibling Clippy blocker is cleared for this run. The shared tree may change before integration.

| Command | Exit | Passed/run | Log under `tmp/behringer-oss-ports/bop-13/logs/` |
|---|---:|---:|---|
| `cargo nextest run -p music-things-airwindows --test parity_batch1_g12` | 0 | 8/8 | `agent-current-parity.log` |
| `cargo nextest run -p music-things-airwindows --test catalog --test no_alloc` | 0 | 4/4 | `agent-current-catalog.log` |
| `cargo clippy -p music-things-airwindows --all-targets -- -D warnings` | 0 | 0/0 | `agent-current-clippy.log` |
| `find crates/airwindows/src/plugins/batch1/g12 crates/airwindows/tests/parity_batch1_g12.rs -name '*.rs' -print0 \| xargs -0 rustfmt --edition 2021 --check` | 0 | 0/0 | `agent-current-rustfmt.log` |
| `cargo build -p music-things-airwindows --target wasm32-unknown-unknown` | 0 | 0/0 | `agent-current-wasm.log` |
| `test -z "$(find crates/airwindows/src/plugins/batch1/g12 -name '*.rs' -exec wc -l {} + \| awk '$2 != "total" && $1 >= 1000')"` | 0 | 0/0 | `agent-current-lines.log` |

Formal review and integrated-tree verification remain downstream.

### Review revision: reset preserves parameters

Addressed `bop13-F1-reset-keeps-params`: Dark, DarkNoise, DCVoltage, DeBess, Deckwrecka, DeEss and DeHiss retain parameters through reset; curve has no parameters and continues to reset its DSP state. DarkNoise clears its boxed buffers in place. All eight parity tests exercise parameter preservation. Pre-fix DCVoltage regression failed as expected (`logs/review-reset-negative.log`, exit 100). Protected fixture and framework hashes match (`logs/review-final-protected.log`, exit 0). The initial repair introduced invalid curve field references; the corrected source supersedes the failed `review-fix-*` logs.

Final verification used `CARGO_TARGET_DIR=target/bop-13-review-fix CARGO_TERM_QUIET=true NEXTEST_STATUS_LEVEL=fail NEXTEST_FAILURE_OUTPUT=immediate-final NEXTEST_HIDE_PROGRESS_BAR=1` on the shared tree:

| Command | Exit | Passed/run | Log under `tmp/behringer-oss-ports/bop-13/logs/` |
|---|---:|---:|---|
| `cargo nextest run -p music-things-airwindows --test parity_batch1_g12` | 0 | 8/8 | `review-final-parity.log` |
| `cargo nextest run -p music-things-airwindows --test catalog --test no_alloc` | 0 | 4/4 | `review-final-catalog.log` |
| `cargo clippy -p music-things-airwindows --all-targets -- -D warnings` | 0 | 0/0 | `review-final-clippy.log` |
| `find crates/airwindows/src/plugins/batch1/g12 crates/airwindows/tests/parity_batch1_g12.rs -name '*.rs' -print0 \| xargs -0 rustfmt --edition 2021 --check` | 0 | 0/0 | `review-final-rustfmt.log` |
| `cargo build -p music-things-airwindows --target wasm32-unknown-unknown` | 0 | 0/0 | `review-final-wasm.log` |
| `test -z "$(find crates/airwindows/src/plugins/batch1/g12 -name '*.rs' -exec wc -l {} + \| awk '$2 != "total" && $1 >= 1000')"` | 0 | 0/0 | `review-final-lines.log` |

Independent review and combined-tree verification remain downstream.
