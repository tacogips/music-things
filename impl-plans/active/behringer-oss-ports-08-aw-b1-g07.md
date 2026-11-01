# Behringer OSS Ports 08: Airwindows Batch-1 Group g07 (kick list lines 49-56)

**Status**: In Progress
**Plan ID**: bop-08
**Design Reference**: design-docs/specs/design-behringer-oss-ports.md section A (Additive registration, Batch-1 grouping, Porting rules, Parity method, Tolerances)
**Depends On**: bop-01 (wave W3; runs in parallel with the other 13 group plans and bop-16c)
**Created**: 2026-10-31
**Last Updated**: 2026-10-31

## Intent and Context

Port the Airwindows plugins on lines 49-56 of `$MUSIC_THINGS_REFS/kick-airwindows-set.txt` (8 plugins)
to Rust, register them in group `g07`, and prove parity against the fixtures bop-01 generated in
`crates/airwindows/tests/fixtures/batch1/g07/`. List the names with
`sed -n '49,56p' "$MUSIC_THINGS_REFS/kick-airwindows-set.txt"` and record them in the Progress Log.
KICK `Console8BusOut` means upstream `Console8BussOut`.

The framework is frozen. Use it exactly as bop-01 built it:
- `crates/airwindows/src/effect.rs:AirwindowsEffect`, `params.rs:{ParamInfo, PluginInfo, sanitize_param}`
- `registry.rs:PluginEntry`, `dither.rs:{fpd_from_seed, xorshift, frexpf_exponent, pow2, fpd_dither}`
- `rand.rs:{AwRand, RAND_MAX, replay_fpd_seed_loop}`, `display.rs:{float2string, db2string, int2string}`
- `tests/common/parity.rs:{check_plugin, ParityRecord, ParityClass}`

Imitate the per-plugin module contract documented in `crates/airwindows/src/plugins/mod.rs`
(`INFO`/`ENTRY`/`construct` consts). Landed sibling plugin files may be read as style references.

## Non-goals

- Do not edit the framework (`src/*.rs` outside `src/plugins/batch1/g07/`), other groups, `tests/common/`,
  `tools/`, `Cargo.toml`/`Cargo.lock`, or any other crate.
- No plugins outside lines 49-56, no stubs, and no `processDoubleReplacing`, chunks or programs.
- Do not regenerate fixtures unless bop-01 has fixed a harness defect that you reported (Escalation).

## Write Ownership (fanout contract)

- writePaths: `crates/airwindows/src/plugins/batch1/g07` (directory: `mod.rs` plus one `<module>.rs` per plugin),
  `crates/airwindows/tests/parity_batch1_g07.rs`, `crates/airwindows/tests/fixtures/batch1/g07` (directory;
  regenerated only after a bop-01 harness fix), `impl-plans/active/behringer-oss-ports-08-aw-b1-g07.md`,
  `target`, `tmp/behringer-oss-ports/bop-08`
- sharedPaths: none
- artifactRoots (digest-only): `target`, `tmp/behringer-oss-ports/bop-08`
- Read-only: framework `crates/airwindows/src/*.rs`, `src/plugins/mod.rs`, `src/plugins/batch1/mod.rs`,
  `crates/airwindows/tests/common/*`, `tools/*` (all frozen by bop-01).
- Never run git checkout/stash/reset or any other git operation that rewrites the tree.

## Common Execution Rules

- `MUSIC_THINGS_REFS=/Users/taco/gits/tacogips/music-things-refs`. Read only
  `airwindows/plugins/LinuxVST/src/<Upstream>/{<Upstream>.h,<Upstream>.cpp,<Upstream>Proc.cpp}` and the
  kick list. Never open Music Tribe AGPL code.
- Evidence directory: `tmp/behringer-oss-ports/bop-08/`. It holds `intent.md` (immutable: the plugin
  list and the deliverables), `pre-hashes.txt` and `post-hashes.txt` (`shasum -a 256` of owned files)
  and `logs/`. Re-read each file before editing; on drift, stop and log it. Only edit `writePaths`.
- 13 sibling plans compile this crate at the same time. If an error is in a file you do not own, rerun
  later and log it. Do not edit that file. Write a plugin file completely before adding its `mod` line
  and `ENTRY` to `g07/mod.rs`.
- Format only owned files: `rustfmt --edition 2021 <files>`. Cargo env: `CARGO_TERM_QUIET=true NEXTEST_STATUS_LEVEL=fail NEXTEST_FAILURE_OUTPUT=immediate-final NEXTEST_HIDE_PROGRESS_BAR=1`.

## Deliverables

1. `crates/airwindows/src/plugins/batch1/g07/<module>.rs`, one per plugin, where `<module>` is the KICK
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
     `tests/fixtures/batch1/g07/<Name>.txt` (unescaped); params with name, label and default as in the fixture.
   - `param_display` and `param_label` reproduce `getParameterDisplay`/`getParameterLabel` case by case,
     using the `display.rs` helpers.
   - `process` ports `processReplacing` statement by statement, and `reset` restores the constructor
     state in place without allocating.
2. `crates/airwindows/src/plugins/batch1/g07/mod.rs`: `mod <module>;` lines and `ENTRIES` in kick-list line order.
3. `crates/airwindows/tests/parity_batch1_g07.rs`: `mod common;`, one `const RECORD_<NAME>: ParityRecord` per
   plugin, and one `#[test] fn <module>()` calling
   `common::parity::check_plugin(&RECORD_X, include_str!("fixtures/batch1/g07/<Name>.txt"), include_bytes!("fixtures/batch1/g07/<Name>.f32"))`.

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
`tools/airwindows-harness/generate.sh --work-dir tmp/behringer-oss-ports/bop-08/harness --out-root crates/airwindows/tests/fixtures --group g07`.

## Tests (situation -> expected)

- Each plugin with `check_plugin`: the metadata, description, R0/R1/R2 and reset re-render match within its class.
- `tests/catalog.rs` and `tests/no_alloc.rs` cover these plugins through the registry. Both pass.

## Verification Commands (evidence)

1. `cargo nextest run -p music-things-airwindows --test parity_batch1_g07`: exit 0, testsRun = 8, failures = 0.
2. `cargo nextest run -p music-things-airwindows --test catalog --test no_alloc`: exit 0.
3. `cargo clippy -p music-things-airwindows --all-targets -- -D warnings`: exit 0, with no warnings in owned files.
4. `rustfmt --edition 2021 --check crates/airwindows/src/plugins/batch1/g07/*.rs crates/airwindows/tests/parity_batch1_g07.rs`: exit 0.
5. `cargo build -p music-things-airwindows --target wasm32-unknown-unknown`: exit 0.
6. `wc -l crates/airwindows/src/plugins/batch1/g07/*.rs`: every file under 1000 lines.

## Completion Criteria

- [x] All 8 plugins of lines 49-56 are ported, registered in order and tested (or marked BLOCKED with evidence).
- [x] Every plugin has a ParityRecord with its class and measured max-abs, repeated in the module header.
- [x] Verifications 1-6 exit 0, with logs in `tmp/behringer-oss-ports/bop-08/logs/`.
- [x] Only `writePaths` are modified.
- [x] Progress Log records the plugin list, classes, max-abs values, commands, exit status and log paths.

## Progress Log

### Session: 2026-10-31
**Tasks Completed**: Plan created (not started)
**Notes**: -

### Session: 2026-11-01
**Tasks Completed**: Channel4, Channel5, Channel6, Channel7, Channel8, Channel9, Chorus, ChorusEnsemble registered in order and tested. All are Libm, measured max-abs 0.0 on the fixture host; see `tmp/behringer-oss-ports/bop-08/measure-final.log`.
**Blockers**: Package-wide clippy fails in sibling-owned g01, g08 and g14 files; no g07 diagnostics remain. Rerun after sibling repairs.

| Command | Exit | Tests passed/run | Log |
|---|---:|---:|---|
| `CARGO_TARGET_DIR=tmp/behringer-oss-ports/bop-08/check-target CARGO_TERM_QUIET=true NEXTEST_STATUS_LEVEL=fail NEXTEST_FAILURE_OUTPUT=immediate-final NEXTEST_HIDE_PROGRESS_BAR=1 cargo nextest run -p music-things-airwindows --test parity_batch1_g07` | 0 | 8/8 | `tmp/behringer-oss-ports/bop-08/logs/final2-parity.log` |
| `CARGO_TARGET_DIR=tmp/behringer-oss-ports/bop-08/check-target CARGO_TERM_QUIET=true NEXTEST_STATUS_LEVEL=fail NEXTEST_FAILURE_OUTPUT=immediate-final NEXTEST_HIDE_PROGRESS_BAR=1 cargo nextest run -p music-things-airwindows --test catalog --test no_alloc` | 0 | 4/4 | `tmp/behringer-oss-ports/bop-08/logs/final2-catalog.log` |
| `CARGO_TARGET_DIR=tmp/behringer-oss-ports/bop-08/check-target CARGO_TERM_QUIET=true cargo clippy -p music-things-airwindows --all-targets -- -D warnings` | 101 | - | `tmp/behringer-oss-ports/bop-08/logs/final2-clippy.log` |
| `find crates/airwindows/src/plugins/batch1/g07 crates/airwindows/tests/parity_batch1_g07.rs -name '*.rs' -print0 | xargs -0 rustfmt --edition 2021 --check` | 0 | - | `tmp/behringer-oss-ports/bop-08/logs/final2-fmt.log` |
| `CARGO_TARGET_DIR=tmp/behringer-oss-ports/bop-08/check-target CARGO_TERM_QUIET=true cargo build -p music-things-airwindows --target wasm32-unknown-unknown` | 0 | - | `tmp/behringer-oss-ports/bop-08/logs/final2-wasm.log` |
| `test -z "$(find crates/airwindows/src/plugins/batch1/g07 -name '*.rs' -exec wc -l {} + | awk '$2 != "total" && $1 >= 1000')"` | 0 | - | `tmp/behringer-oss-ports/bop-08/logs/final2-lines.log` |

### Continuation 1: 2026-11-01
`CARGO_TARGET_DIR=tmp/behringer-oss-ports/bop-08/check-target CARGO_TERM_QUIET=true cargo clippy -p music-things-airwindows --all-targets -- -D warnings` exited 101; `tmp/behringer-oss-ports/bop-08/logs/continuation1-clippy.log` records diagnostics in sibling g01, g06 and g14 files only. No g07 diagnostics. The required gate remains blocked pending sibling repairs. The pre-continuation owned-file hashes all matched `post-hashes.txt` (`logs/continuation1-hashes.log`).

### Continuation 2: 2026-11-01
`CARGO_TARGET_DIR=tmp/behringer-oss-ports/bop-08/check-target CARGO_TERM_QUIET=true cargo clippy -p music-things-airwindows --all-targets -- -D warnings` exited 101; `tmp/behringer-oss-ports/bop-08/logs/continuation2-clippy.log` records diagnostics in sibling g01 and g14 files only. No g07 diagnostics. The gate remains blocked pending sibling repairs. The pre-continuation owned files matched `continuation1-post-hashes.txt` (`logs/continuation2-hashes.log`).

### Step 6 recheck: 2026-11-01
The current shared tree passed parity (8/8), catalog/no_alloc (4/4), owned-file rustfmt, wasm build and the under-1000-line check. Package-wide clippy remains blocked by 15 diagnostics in sibling-owned `g01` and `g14`; no g07 diagnostics appeared. No Rust files were changed in this recheck. All commands used the plan-local `CARGO_TARGET_DIR=tmp/behringer-oss-ports/bop-08/check-target` and the required quiet Cargo environment.

| Command | Exit | Tests passed/run | Log |
|---|---:|---:|---|
| `cargo nextest run -p music-things-airwindows --test parity_batch1_g07` | 0 | 8/8 | `tmp/behringer-oss-ports/bop-08/logs/step6-current-parity.log` |
| `cargo nextest run -p music-things-airwindows --test catalog --test no_alloc` | 0 | 4/4 | `tmp/behringer-oss-ports/bop-08/logs/step6-current-catalog.log` |
| `cargo clippy -p music-things-airwindows --all-targets -- -D warnings` | 101 | - | `tmp/behringer-oss-ports/bop-08/logs/step6-current-clippy.log` |
| `find crates/airwindows/src/plugins/batch1/g07 crates/airwindows/tests/parity_batch1_g07.rs -name '*.rs' -print0 | xargs -0 rustfmt --edition 2021 --check` | 0 | - | `tmp/behringer-oss-ports/bop-08/logs/step6-current-fmt.log` |
| `cargo build -p music-things-airwindows --target wasm32-unknown-unknown` | 0 | - | `tmp/behringer-oss-ports/bop-08/logs/step6-current-wasm.log` |
| `test -z "$(find crates/airwindows/src/plugins/batch1/g07 -name '*.rs' -exec wc -l {} + | awk '$2 != "total" && $1 >= 1000')"` | 0 | - | `tmp/behringer-oss-ports/bop-08/logs/step6-current-lines.log` |

### Continuation 3: 2026-11-01
`CARGO_TARGET_DIR=tmp/behringer-oss-ports/bop-08/check-target CARGO_TERM_QUIET=true cargo clippy -p music-things-airwindows --all-targets -- -D warnings` exited 101. `tmp/behringer-oss-ports/bop-08/logs/continuation3-retry-clippy.log` records 12 diagnostics in sibling-owned g11 files and none in g07. The required gate remains blocked until those files are repaired. The initial attempt's exit capture failed because zsh reserves `status`; its output is preserved in `logs/continuation3-clippy.log`.

### Redispatch 4: 2026-11-01
All six required checks passed on the current shared tree: parity 8/8, catalog/no_alloc 4/4, clippy, rustfmt, wasm build and line limit each exited 0. Complete command output and exit statuses are in `tmp/behringer-oss-ports/bop-08/logs/redispatch4-{parity,catalog,clippy,fmt,wasm,lines}.log`. Cargo used `CARGO_TARGET_DIR=tmp/behringer-oss-ports/bop-08/check-target CARGO_TERM_QUIET=true NEXTEST_STATUS_LEVEL=fail NEXTEST_FAILURE_OUTPUT=immediate-final NEXTEST_HIDE_PROGRESS_BAR=1`. The earlier sibling Clippy blocker is resolved in this run. Formal reviews remain downstream.
