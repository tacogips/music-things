# Behringer OSS Ports 03: Airwindows Batch-1 Group g02 (kick list lines 9-16)

**Status**: Completed
**Plan ID**: bop-03
**Design Reference**: design-docs/specs/design-behringer-oss-ports.md section A (Additive registration, Batch-1 grouping, Porting rules, Parity method, Tolerances)
**Depends On**: bop-01 (wave W3; runs in parallel with the other 13 group plans and bop-16c)
**Created**: 2026-10-31
**Last Updated**: 2026-11-01

## Intent and Context

Port the Airwindows plugins on lines 9-16 of `$MUSIC_THINGS_REFS/kick-airwindows-set.txt` (8 plugins)
to Rust, register them in group `g02`, and prove parity against the fixtures bop-01 generated in
`crates/airwindows/tests/fixtures/batch1/g02/`. List the names with
`sed -n '9,16p' "$MUSIC_THINGS_REFS/kick-airwindows-set.txt"` and record them in the Progress Log.
KICK `Console8BusOut` means upstream `Console8BussOut`.

The framework is frozen. Use it exactly as bop-01 built it:
- `crates/airwindows/src/effect.rs:AirwindowsEffect`, `params.rs:{ParamInfo, PluginInfo, sanitize_param}`
- `registry.rs:PluginEntry`, `dither.rs:{fpd_from_seed, xorshift, frexpf_exponent, pow2, fpd_dither}`
- `rand.rs:{AwRand, RAND_MAX, replay_fpd_seed_loop}`, `display.rs:{float2string, db2string, int2string}`
- `tests/common/parity.rs:{check_plugin, ParityRecord, ParityClass}`

Imitate the per-plugin module contract documented in `crates/airwindows/src/plugins/mod.rs`
(`INFO`/`ENTRY`/`construct` consts). Landed sibling plugin files may be read as style references.

## Non-goals

- Do not edit the framework (`src/*.rs` outside `src/plugins/batch1/g02/`), other groups, `tests/common/`,
  `tools/`, `Cargo.toml`/`Cargo.lock`, or any other crate.
- No plugins outside lines 9-16, no stubs, and no `processDoubleReplacing`, chunks or programs.
- Do not regenerate fixtures unless bop-01 has fixed a harness defect that you reported (Escalation).

## Write Ownership (fanout contract)

- writePaths: `crates/airwindows/src/plugins/batch1/g02` (directory: `mod.rs` plus one `<module>.rs` per plugin),
  `crates/airwindows/tests/parity_batch1_g02.rs`, `crates/airwindows/tests/fixtures/batch1/g02` (directory;
  regenerated only after a bop-01 harness fix), `impl-plans/active/behringer-oss-ports-03-aw-b1-g02.md`,
  `target`, `tmp/behringer-oss-ports/bop-03`
- sharedPaths: none
- artifactRoots (digest-only): `target`, `tmp/behringer-oss-ports/bop-03`
- Read-only: framework `crates/airwindows/src/*.rs`, `src/plugins/mod.rs`, `src/plugins/batch1/mod.rs`,
  `crates/airwindows/tests/common/*`, `tools/*` (all frozen by bop-01).
- Never run git checkout/stash/reset or any other git operation that rewrites the tree.

## Common Execution Rules

- `MUSIC_THINGS_REFS=/Users/taco/gits/tacogips/music-things-refs`. Read only
  `airwindows/plugins/LinuxVST/src/<Upstream>/{<Upstream>.h,<Upstream>.cpp,<Upstream>Proc.cpp}` and the
  kick list. Never open Music Tribe AGPL code.
- Evidence directory: `tmp/behringer-oss-ports/bop-03/`. It holds `intent.md` (immutable: the plugin
  list and the deliverables), `pre-hashes.txt` and `post-hashes.txt` (`shasum -a 256` of owned files)
  and `logs/`. Re-read each file before editing; on drift, stop and log it. Only edit `writePaths`.
- 13 sibling plans compile this crate at the same time. If an error is in a file you do not own, rerun
  later and log it. Do not edit that file. Write a plugin file completely before adding its `mod` line
  and `ENTRY` to `g02/mod.rs`.
- Format only owned files: `rustfmt --edition 2021 <files>`. Cargo env: `CARGO_TERM_QUIET=true NEXTEST_STATUS_LEVEL=fail NEXTEST_FAILURE_OUTPUT=immediate-final NEXTEST_HIDE_PROGRESS_BAR=1`.

## Deliverables

1. `crates/airwindows/src/plugins/batch1/g02/<module>.rs`, one per plugin, where `<module>` is the KICK
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
     `tests/fixtures/batch1/g02/<Name>.txt` (unescaped); params with name, label and default as in the fixture.
   - `param_display` and `param_label` reproduce `getParameterDisplay`/`getParameterLabel` case by case,
     using the `display.rs` helpers.
   - `process` ports `processReplacing` statement by statement, and `reset` restores the constructor
     state in place without allocating.
2. `crates/airwindows/src/plugins/batch1/g02/mod.rs`: `mod <module>;` lines and `ENTRIES` in kick-list line order.
3. `crates/airwindows/tests/parity_batch1_g02.rs`: `mod common;`, one `const RECORD_<NAME>: ParityRecord` per
   plugin, and one `#[test] fn <module>()` calling
   `common::parity::check_plugin(&RECORD_X, include_str!("fixtures/batch1/g02/<Name>.txt"), include_bytes!("fixtures/batch1/g02/<Name>.f32"))`.

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
`tools/airwindows-harness/generate.sh --work-dir tmp/behringer-oss-ports/bop-03/harness --out-root crates/airwindows/tests/fixtures --group g02`.

## Tests (situation -> expected)

- Each plugin with `check_plugin`: the metadata, description, R0/R1/R2 and reset re-render match within its class.
- `tests/catalog.rs` and `tests/no_alloc.rs` cover these plugins through the registry. Both pass.

## Verification Commands (evidence)

1. `cargo nextest run -p music-things-airwindows --test parity_batch1_g02`: exit 0, testsRun = 8, failures = 0.
2. `cargo nextest run -p music-things-airwindows --test catalog --test no_alloc`: exit 0.
3. `cargo clippy -p music-things-airwindows --all-targets -- -D warnings`: exit 0, with no warnings in owned files.
4. `rustfmt --edition 2021 --check crates/airwindows/src/plugins/batch1/g02/*.rs crates/airwindows/tests/parity_batch1_g02.rs`: exit 0.
5. `cargo build -p music-things-airwindows --target wasm32-unknown-unknown`: exit 0.
6. `wc -l crates/airwindows/src/plugins/batch1/g02/*.rs`: every file under 1000 lines.

## Completion Criteria

- [x] All 8 plugins of lines 9-16 are ported, registered in order and tested (or marked BLOCKED with evidence).
- [x] Every plugin has a ParityRecord with its class and measured max-abs, repeated in the module header.
- [x] Verifications 1-6 exit 0, with logs in `tmp/behringer-oss-ports/bop-03/logs/`.
- [x] Only `writePaths` are modified.
- [x] Progress Log records the plugin list, classes, max-abs values, commands, exit status and log paths.

## Progress Log

### Session: 2026-10-31
**Tasks Completed**: Plan created (not started)
**Notes**: -

### Session: 2026-11-01
**Status**: In Progress; implementation incomplete.
**Plugins**: AtmosphereBuss, AtmosphereChannel, Aura, AutoPan (Libm, max-abs 0); Average, AverMatrix, Balanced (Exact, max-abs 0). BassAmp remains unimplemented and unregistered. Measurements cover the three sampled fixture renders; see `tmp/behringer-oss-ports/bop-03/logs/measure.log`.
**Verification**: Cargo used `CARGO_TARGET_DIR=tmp/behringer-oss-ports/bop-03/target CARGO_TERM_QUIET=true`; nextest additionally used `NEXTEST_STATUS_LEVEL=fail NEXTEST_FAILURE_OUTPUT=immediate-final NEXTEST_HIDE_PROGRESS_BAR=1`.

| Command | Exit | Tests | Log |
|---|---:|---:|---|
| `cargo nextest run -p music-things-airwindows --test parity_batch1_g02` | 0 | 7/7; eighth missing | `tmp/behringer-oss-ports/bop-03/logs/final2-parity.log` |
| `cargo nextest run -p music-things-airwindows --test catalog --test no_alloc` | 0 | 4/4 | `tmp/behringer-oss-ports/bop-03/logs/final2-catalog-noalloc.log` |
| `cargo clippy -p music-things-airwindows --all-targets -- -D warnings` | 101 | 0 | `tmp/behringer-oss-ports/bop-03/logs/final2-clippy.log` |
| `find crates/airwindows/src/plugins/batch1/g02 crates/airwindows/tests/parity_batch1_g02.rs -name '*.rs' -print0 \| xargs -0 rustfmt --edition 2021 --check` | 0 | 0 | `tmp/behringer-oss-ports/bop-03/logs/final2-rustfmt.log` |
| `cargo build -p music-things-airwindows --target wasm32-unknown-unknown` | 0 | 0 | `tmp/behringer-oss-ports/bop-03/logs/final2-wasm.log` |
| `test -z "$(find crates/airwindows/src/plugins/batch1/g02 -name '*.rs' -exec wc -l {} + \| awk '$2 != "total" && $1 >= 1000')"` | 0 | 0 | `tmp/behringer-oss-ports/bop-03/logs/final2-lines.log` |

**Blocker**: Clippy reports errors in sibling-owned g01, g08 and g14 files, outside this plan's write paths. Their owners must repair them before the package gate can pass. BassAmp still requires implementation and parity verification; no harness defect was established.

### Continuation: 2026-11-01
**Completed**: BassAmp ported and registered after Balanced; Exact, measured max-abs 0 (`tmp/behringer-oss-ports/bop-03/logs/bassamp-measure.log`). The first parity attempt exposed the upstream left/right filter coupling; the port was corrected and the eighth test passes.
**Verification**: Same six commands and environment as the preceding table, with logs prefixed `tmp/behringer-oss-ports/bop-03/logs/continuation-`: parity exit 0 (8/8), catalog/no_alloc exit 0 (4/4), clippy exit 101 (0 tests), rustfmt exit 0, wasm exit 0, lines exit 0. The clippy gate remains blocked by sibling-owned g01 and g14 files listed in `continuation-clippy.log`; no g02 diagnostic appears. All-gates completion remains unchecked.

### Continuation 2: 2026-11-01
`CARGO_TARGET_DIR=tmp/behringer-oss-ports/bop-03/target CARGO_TERM_QUIET=true cargo clippy -p music-things-airwindows --all-targets -- -D warnings` exited 101; `tmp/behringer-oss-ports/bop-03/logs/continuation2-clippy.log` records 15 diagnostics in sibling-owned g01 and g14 files, and none in g02. Their repair is required before this gate can pass. The owned files and fixtures matched `post-hashes-continuation.txt` before this plan update (`logs/continuation2-hashes.log`); the earlier 8/8 parity evidence therefore remains source-matched.

### Redispatch: 2026-11-01
The required package clippy gate remains blocked: `CARGO_TARGET_DIR=tmp/behringer-oss-ports/bop-03/target CARGO_TERM_QUIET=true cargo clippy -p music-things-airwindows --all-targets -- -D warnings` exited 101. The complete log is `tmp/behringer-oss-ports/bop-03/logs/redispatch-clippy-final.log`; it reports 24 diagnostics in sibling-owned g01, g11 and g14 files, with none in g02. Their owners must repair the diagnostics before this gate can be rerun. The shared tree was moving during fanout; the prior passing gates require fresh stable-tree verification after repair.

### Step 6 retry: 2026-11-01
All eight plugins remain registered in list order. AtmosphereBuss, AtmosphereChannel, Aura and AutoPan: Libm, max-abs 0; Average, AverMatrix, Balanced and BassAmp: Exact, max-abs 0. Prior measurements are in `logs/measure.log` and `logs/bassamp-measure.log`. No Rust files were changed in this retry. The shared tree may still change; integration review must verify the combined tree.

Cargo commands used `CARGO_TARGET_DIR=tmp/behringer-oss-ports/bop-03/target CARGO_TERM_QUIET=true NEXTEST_STATUS_LEVEL=fail NEXTEST_FAILURE_OUTPUT=immediate-final NEXTEST_HIDE_PROGRESS_BAR=1`. All logs below are under `tmp/behringer-oss-ports/bop-03/logs/`.

| Command | Exit | Tests | Log |
|---|---:|---:|---|
| `cargo nextest run -p music-things-airwindows --test parity_batch1_g02` | 0 | 8/8 | `step6-retry-parity.log` |
| `cargo nextest run -p music-things-airwindows --test catalog --test no_alloc` | 0 | 4/4 | `step6-retry-catalog.log` |
| `cargo clippy -p music-things-airwindows --all-targets -- -D warnings` | 0 | 0 | `step6-retry-clippy.log` |
| `find crates/airwindows/src/plugins/batch1/g02 crates/airwindows/tests/parity_batch1_g02.rs -name '*.rs' -print0 \| xargs -0 rustfmt --edition 2021 --check` | 0 | 0 | `step6-retry-format.log` |
| `cargo build -p music-things-airwindows --target wasm32-unknown-unknown` | 0 | 0 | `step6-retry-wasm.log` |
| `test -z "$(find crates/airwindows/src/plugins/batch1/g02 -name '*.rs' -exec wc -l {} + \| awk '$2 != "total" && $1 >= 1000')"` | 0 | 0 | `step6-retry-lines.log` |

### Review repair RW5-1/bop-03: 2026-11-01
Average, AverMatrix and Balanced now preserve parameters during reset; all eight plugins have reset parameter assertions. AtmosphereBuss, AtmosphereChannel, Aura and AutoPan remain Libm, max-abs 0; Average, AverMatrix, Balanced and BassAmp remain Exact, max-abs 0 (measurements in `logs/measure.log` and `logs/bassamp-measure.log`). The pre-fix regression negative control failed as expected (3/3 failures, exit 100; `logs/rw5-negative-agent.log`).

Cargo commands used `CARGO_TARGET_DIR=tmp/behringer-oss-ports/bop-03/target CARGO_TERM_QUIET=true NEXTEST_STATUS_LEVEL=fail NEXTEST_FAILURE_OUTPUT=immediate-final NEXTEST_HIDE_PROGRESS_BAR=1`. Logs below are under `tmp/behringer-oss-ports/bop-03/logs/`; verification ran on the shared tree.

| Command | Exit | Tests | Log |
|---|---:|---:|---|
| `cargo nextest run -p music-things-airwindows --test parity_batch1_g02` | 0 | 8/8 | `rw5-final-parity.log` |
| `cargo nextest run -p music-things-airwindows --test catalog --test no_alloc` | 0 | 4/4 | `rw5-final-catalog.log` |
| `cargo clippy -p music-things-airwindows --all-targets -- -D warnings` | 0 | 0 | `rw5-final-clippy.log` |
| `find crates/airwindows/src/plugins/batch1/g02 crates/airwindows/tests/parity_batch1_g02.rs -name '*.rs' -print0 \| xargs -0 rustfmt --edition 2021 --check` | 0 | 0 | `rw5-final-format.log` |
| `cargo build -p music-things-airwindows --target wasm32-unknown-unknown` | 0 | 0 | `rw5-final-wasm.log` |
| `test -z "$(find crates/airwindows/src/plugins/batch1/g02 -name '*.rs' -exec wc -l {} + \| awk '$2 != "total" && $1 >= 1000')"` | 0 | 0 | `rw5-final-lines.log` |

Formal review and stable-tree integration verification remain downstream.
