# Behringer OSS Ports 05: Airwindows Batch-1 Group g04 (kick list lines 25-32)

**Status**: In Progress
**Plan ID**: bop-05
**Design Reference**: design-docs/specs/design-behringer-oss-ports.md section A (Additive registration, Batch-1 grouping, Porting rules, Parity method, Tolerances)
**Depends On**: bop-01 (wave W3; runs in parallel with the other 13 group plans and bop-16c)
**Created**: 2026-10-31
**Last Updated**: 2026-10-31

## Intent and Context

Port the Airwindows plugins on lines 25-32 of `$MUSIC_THINGS_REFS/kick-airwindows-set.txt` (8 plugins)
to Rust, register them in group `g04`, and prove parity against the fixtures bop-01 generated in
`crates/airwindows/tests/fixtures/batch1/g04/`. List the names with
`sed -n '25,32p' "$MUSIC_THINGS_REFS/kick-airwindows-set.txt"` and record them in the Progress Log.
KICK `Console8BusOut` means upstream `Console8BussOut`.

The framework is frozen. Use it exactly as bop-01 built it:
- `crates/airwindows/src/effect.rs:AirwindowsEffect`, `params.rs:{ParamInfo, PluginInfo, sanitize_param}`
- `registry.rs:PluginEntry`, `dither.rs:{fpd_from_seed, xorshift, frexpf_exponent, pow2, fpd_dither}`
- `rand.rs:{AwRand, RAND_MAX, replay_fpd_seed_loop}`, `display.rs:{float2string, db2string, int2string}`
- `tests/common/parity.rs:{check_plugin, ParityRecord, ParityClass}`

Imitate the per-plugin module contract documented in `crates/airwindows/src/plugins/mod.rs`
(`INFO`/`ENTRY`/`construct` consts). Landed sibling plugin files may be read as style references.

## Non-goals

- Do not edit the framework (`src/*.rs` outside `src/plugins/batch1/g04/`), other groups, `tests/common/`,
  `tools/`, `Cargo.toml`/`Cargo.lock`, or any other crate.
- No plugins outside lines 25-32, no stubs, and no `processDoubleReplacing`, chunks or programs.
- Do not regenerate fixtures unless bop-01 has fixed a harness defect that you reported (Escalation).

## Write Ownership (fanout contract)

- writePaths: `crates/airwindows/src/plugins/batch1/g04` (directory: `mod.rs` plus one `<module>.rs` per plugin),
  `crates/airwindows/tests/parity_batch1_g04.rs`, `crates/airwindows/tests/fixtures/batch1/g04` (directory;
  regenerated only after a bop-01 harness fix), `impl-plans/active/behringer-oss-ports-05-aw-b1-g04.md`,
  `target`, `tmp/behringer-oss-ports/bop-05`
- sharedPaths: none
- artifactRoots (digest-only): `target`, `tmp/behringer-oss-ports/bop-05`
- Read-only: framework `crates/airwindows/src/*.rs`, `src/plugins/mod.rs`, `src/plugins/batch1/mod.rs`,
  `crates/airwindows/tests/common/*`, `tools/*` (all frozen by bop-01).
- Never run git checkout/stash/reset or any other git operation that rewrites the tree.

## Common Execution Rules

- `MUSIC_THINGS_REFS=/Users/taco/gits/tacogips/music-things-refs`. Read only
  `airwindows/plugins/LinuxVST/src/<Upstream>/{<Upstream>.h,<Upstream>.cpp,<Upstream>Proc.cpp}` and the
  kick list. Never open Music Tribe AGPL code.
- Evidence directory: `tmp/behringer-oss-ports/bop-05/`. It holds `intent.md` (immutable: the plugin
  list and the deliverables), `pre-hashes.txt` and `post-hashes.txt` (`shasum -a 256` of owned files)
  and `logs/`. Re-read each file before editing; on drift, stop and log it. Only edit `writePaths`.
- 13 sibling plans compile this crate at the same time. If an error is in a file you do not own, rerun
  later and log it. Do not edit that file. Write a plugin file completely before adding its `mod` line
  and `ENTRY` to `g04/mod.rs`.
- Format only owned files: `rustfmt --edition 2021 <files>`. Cargo env: `CARGO_TERM_QUIET=true NEXTEST_STATUS_LEVEL=fail NEXTEST_FAILURE_OUTPUT=immediate-final NEXTEST_HIDE_PROGRESS_BAR=1`.

## Deliverables

1. `crates/airwindows/src/plugins/batch1/g04/<module>.rs`, one per plugin, where `<module>` is the KICK
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
     `tests/fixtures/batch1/g04/<Name>.txt` (unescaped); params with name, label and default as in the fixture.
   - `param_display` and `param_label` reproduce `getParameterDisplay`/`getParameterLabel` case by case,
     using the `display.rs` helpers.
   - `process` ports `processReplacing` statement by statement, and `reset` restores the constructor
     state in place without allocating.
2. `crates/airwindows/src/plugins/batch1/g04/mod.rs`: `mod <module>;` lines and `ENTRIES` in kick-list line order.
3. `crates/airwindows/tests/parity_batch1_g04.rs`: `mod common;`, one `const RECORD_<NAME>: ParityRecord` per
   plugin, and one `#[test] fn <module>()` calling
   `common::parity::check_plugin(&RECORD_X, include_str!("fixtures/batch1/g04/<Name>.txt"), include_bytes!("fixtures/batch1/g04/<Name>.f32"))`.

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
`tools/airwindows-harness/generate.sh --work-dir tmp/behringer-oss-ports/bop-05/harness --out-root crates/airwindows/tests/fixtures --group g04`.

## Tests (situation -> expected)

- Each plugin with `check_plugin`: the metadata, description, R0/R1/R2 and reset re-render match within its class.
- `tests/catalog.rs` and `tests/no_alloc.rs` cover these plugins through the registry. Both pass.

## Verification Commands (evidence)

1. `cargo nextest run -p music-things-airwindows --test parity_batch1_g04`: exit 0, testsRun = 8, failures = 0.
2. `cargo nextest run -p music-things-airwindows --test catalog --test no_alloc`: exit 0.
3. `cargo clippy -p music-things-airwindows --all-targets -- -D warnings`: exit 0, with no warnings in owned files.
4. `rustfmt --edition 2021 --check crates/airwindows/src/plugins/batch1/g04/*.rs crates/airwindows/tests/parity_batch1_g04.rs`: exit 0.
5. `cargo build -p music-things-airwindows --target wasm32-unknown-unknown`: exit 0.
6. `wc -l crates/airwindows/src/plugins/batch1/g04/*.rs`: every file under 1000 lines.

## Completion Criteria

- [x] All 8 plugins of lines 25-32 are ported, registered in order and tested (or marked BLOCKED with evidence).
- [x] Every plugin has a ParityRecord with its class and measured max-abs, repeated in the module header.
- [x] Verifications 1-6 exit 0, with logs in `tmp/behringer-oss-ports/bop-05/logs/`.
- [ ] Only `writePaths` are modified.
- [x] Progress Log records the plugin list, classes, max-abs values, commands, exit status and log paths.

## Progress Log

### Session: 2026-10-31
**Tasks Completed**: Plan created (not started)
**Notes**: -

### Session: 2026-11-01
**Tasks Completed**: Draft ports, registration and eight parity tests. Parity maxima measured on aarch64-apple-darwin (all max-abs 0): BiquadDouble, BiquadOneHalf, BiquadPlus, BiquadTriple, Bite, BitGlitter and BlockParty are Libm; BitShiftGain is Exact. Measurement log: `tmp/behringer-oss-ports/bop-05/logs/measurement.log`.

**Verification** (isolated `CARGO_TARGET_DIR=target/bop-05-check`; logs under `tmp/behringer-oss-ports/bop-05/logs/`):

| Command | Exit | Tests run/passed | Log |
| --- | ---: | ---: | --- |
| `CARGO_TERM_QUIET=true NEXTEST_STATUS_LEVEL=fail NEXTEST_FAILURE_OUTPUT=immediate-final NEXTEST_HIDE_PROGRESS_BAR=1 cargo nextest run -p music-things-airwindows --test parity_batch1_g04` | 0 | 8/8 | `check-agent-final2-parity.log` |
| `CARGO_TERM_QUIET=true NEXTEST_STATUS_LEVEL=fail NEXTEST_FAILURE_OUTPUT=immediate-final NEXTEST_HIDE_PROGRESS_BAR=1 cargo nextest run -p music-things-airwindows --test catalog --test no_alloc` | 0 | 4/4 | `check-agent-final2-catalog.log` |
| `CARGO_TERM_QUIET=true cargo clippy -p music-things-airwindows --all-targets -- -D warnings` | 101 | 0/0 | `check-agent-final2-clippy.log` |
| `find crates/airwindows/src/plugins/batch1/g04 crates/airwindows/tests/parity_batch1_g04.rs -name '*.rs' -print0 | xargs -0 rustfmt --edition 2021 --check` | 0 | 0/0 | `check-agent-final2-rustfmt.log` |
| `CARGO_TERM_QUIET=true cargo build -p music-things-airwindows --target wasm32-unknown-unknown` | 0 | 0/0 | `check-agent-final2-wasm.log` |
| `test -z "$(find crates/airwindows/src/plugins/batch1/g04 -name '*.rs' -exec wc -l {} + | awk '$2 != "total" && $1 >= 1000')"` | 0 | 0/0 | `check-agent-final2-lines.log` |

**Blocker**: The required package-wide Clippy gate reports 31 diagnostics in sibling-owned g01, g06, g07, g08 and g14 source files, none in g04. Those owners must fix their diagnostics before the gate is rerun. Formal reviews remain downstream.

### Continuation 1
Retried `CARGO_TARGET_DIR=target/bop-05-check CARGO_TERM_QUIET=true cargo clippy -p music-things-airwindows --all-targets -- -D warnings`: exit 101, 0 tests run. The complete log is `tmp/behringer-oss-ports/bop-05/logs/continuation1-clippy.log`. Nineteen diagnostics remain in sibling-owned g01, g06 and g14 files; no g04 diagnostic appeared. Those owners must resolve the errors before the required gate can pass.

### Continuation 2
Retried `CARGO_TARGET_DIR=target/bop-05-check CARGO_TERM_QUIET=true cargo clippy -p music-things-airwindows --all-targets -- -D warnings`: exit 101, 0 tests run. Complete log: `tmp/behringer-oss-ports/bop-05/logs/continuation2-clippy.log`. The 25 diagnostics are in sibling-owned g01, g06 and g14 files; none are in g04. The required gate remains blocked until those owners fix the errors.

### Redispatch verification
With `CARGO_TARGET_DIR=target/bop-05-check` and the configured quiet Cargo/nextest environment, parity passed 8/8 (`logs/redispatch-parity.log`), catalog and no-allocation passed 4/4 (`logs/redispatch-catalog.log`), rustfmt passed (`logs/redispatch-rustfmt.log`), wasm build passed (`logs/redispatch-wasm.log`), and the line limit passed (`logs/redispatch-lines.log`). Each exited 0. The exact commands are the six dispatch verification commands.

The required package-wide Clippy command exited 101 (`logs/redispatch-clippy.log`), reporting 24 errors in sibling-owned g01, g11 and g14 files and none in g04. The respective owners must resolve those diagnostics before this gate can pass; rerun the command against the stable shared tree. Formal reviews remain downstream.

### Progress-gate continuation
`CARGO_TARGET_DIR=target/bop-05-check CARGO_TERM_QUIET=true cargo clippy -p music-things-airwindows --all-targets -- -D warnings` exited 101; `tmp/behringer-oss-ports/bop-05/logs/progress-continuation-clippy-final.log` records 12 diagnostics in sibling-owned g11 `console8channelin.rs`, `console8subhype.rs`, `console8subin.rs`, and `crystal.rs`. The g11 owner must resolve these before the required gate can pass on the shared tree.

### Fresh shared-tree verification
All six dispatch commands exited 0 with `CARGO_TARGET_DIR=target/bop-05-check` and the configured quiet Cargo and nextest environment. Parity passed 8/8 (`logs/fresh-parity.log`); catalog and no-allocation passed 4/4 (`logs/fresh-catalog.log`); Clippy (`logs/fresh-clippy.log`), rustfmt (`logs/fresh-rustfmt.log`), wasm build (`logs/fresh-wasm.log`), and the line-limit check (`logs/fresh-lines.log`) passed. The prior sibling Clippy blocker is resolved in this shared-tree run. Final integrated-tree review remains downstream.

### RW5-1 reset repair
BiquadDouble, BiquadOneHalf, BiquadPlus and BiquadTriple now retain parameters across reset; all eight group tests assert this contract. The pre-fix BiquadPlus negative control exited 101 as expected (`tmp/behringer-oss-ports/bop-05/logs/rw5-negative-confirm.log`). Parity classes and measured max-abs remain as recorded above.

On the shared tree, all six dispatch commands exited 0 using `CARGO_TARGET_DIR=target/bop-05-check` and the specified quiet environment. Parity: 8/8 (`logs/rw5-final-parity.log`); catalog/no_alloc: 4/4 (`logs/rw5-final-catalog.log`); clippy (`logs/rw5-final-clippy.log`), rustfmt (`logs/rw5-final-rustfmt.log`), wasm (`logs/rw5-final-wasm.log`), and line limit (`logs/rw5-final-lines.log`) passed. Log paths are relative to `tmp/behringer-oss-ports/bop-05/`. Formal review remains downstream.
