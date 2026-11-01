# Behringer OSS Ports 15: Airwindows Batch-1 Group g14 (kick list lines 105-110)

**Status**: Ready
**Plan ID**: bop-15
**Design Reference**: design-docs/specs/design-behringer-oss-ports.md section A (Additive registration, Batch-1 grouping, Porting rules, Parity method, Tolerances)
**Depends On**: bop-01 (wave W3; runs in parallel with the other 13 group plans and bop-16c)
**Created**: 2026-10-31
**Last Updated**: 2026-10-31

## Intent and Context

Port the Airwindows plugins on lines 105-110 of `$MUSIC_THINGS_REFS/kick-airwindows-set.txt`
(6 plugins; line 110 is `DitherMeDiskers`, and line 111 `DitherMeTimbers` belongs to batch 2 and must
NOT be ported) to Rust. Register them in group `g14` and prove parity against the fixtures bop-01
generated in `crates/airwindows/tests/fixtures/batch1/g14/`. List the names with
`sed -n '105,110p' "$MUSIC_THINGS_REFS/kick-airwindows-set.txt"` and record them in the Progress Log.

The framework is frozen. Use it exactly as bop-01 built it:
- `crates/airwindows/src/effect.rs:AirwindowsEffect`, `params.rs:{ParamInfo, PluginInfo, sanitize_param}`
- `registry.rs:PluginEntry`, `dither.rs:{fpd_from_seed, xorshift, frexpf_exponent, pow2, fpd_dither}`
- `rand.rs:{AwRand, RAND_MAX, replay_fpd_seed_loop}`, `display.rs:{float2string, db2string, int2string}`
- `tests/common/parity.rs:{check_plugin, ParityRecord, ParityClass}`

Imitate the per-plugin module contract documented in `crates/airwindows/src/plugins/mod.rs`
(`INFO`/`ENTRY`/`construct` consts). Landed sibling plugin files may be read as style references.

## Non-goals

- Do not edit the framework (`src/*.rs` outside `src/plugins/batch1/g14/`), other groups, `tests/common/`,
  `tools/`, `Cargo.toml`/`Cargo.lock`, or any other crate.
- No plugins outside lines 105-110, no stubs, and no `processDoubleReplacing`, chunks or programs.
- Do not regenerate fixtures unless bop-01 has fixed a harness defect that you reported (Escalation).

## Write Ownership (fanout contract)

- writePaths: `crates/airwindows/src/plugins/batch1/g14` (directory: `mod.rs` plus one `<module>.rs` per plugin),
  `crates/airwindows/tests/parity_batch1_g14.rs`, `crates/airwindows/tests/fixtures/batch1/g14` (directory;
  regenerated only after a bop-01 harness fix), `impl-plans/active/behringer-oss-ports-15-aw-b1-g14.md`,
  `target`, `tmp/behringer-oss-ports/bop-15`
- sharedPaths: none
- artifactRoots (digest-only): `target`, `tmp/behringer-oss-ports/bop-15`
- Read-only: framework `crates/airwindows/src/*.rs`, `src/plugins/mod.rs`, `src/plugins/batch1/mod.rs`,
  `crates/airwindows/tests/common/*`, `tools/*` (all frozen by bop-01).
- Never run git checkout/stash/reset or any other git operation that rewrites the tree.

## Common Execution Rules

- `MUSIC_THINGS_REFS=/Users/taco/gits/tacogips/music-things-refs`. Read only
  `airwindows/plugins/LinuxVST/src/<Upstream>/{<Upstream>.h,<Upstream>.cpp,<Upstream>Proc.cpp}` and the
  kick list. Never open Music Tribe AGPL code.
- Evidence directory: `tmp/behringer-oss-ports/bop-15/`. It holds `intent.md` (immutable: the plugin
  list and the deliverables), `pre-hashes.txt` and `post-hashes.txt` (`shasum -a 256` of owned files)
  and `logs/`. Re-read each file before editing; on drift, stop and log it. Only edit `writePaths`.
- 13 sibling plans compile this crate at the same time. If an error is in a file you do not own, rerun
  later and log it. Do not edit that file. Write a plugin file completely before adding its `mod` line
  and `ENTRY` to `g14/mod.rs`.
- Format only owned files: `rustfmt --edition 2021 <files>`. Cargo env: `CARGO_TERM_QUIET=true NEXTEST_STATUS_LEVEL=fail NEXTEST_FAILURE_OUTPUT=immediate-final NEXTEST_HIDE_PROGRESS_BAR=1`.

## Deliverables

1. `crates/airwindows/src/plugins/batch1/g14/<module>.rs`, one per plugin, where `<module>` is the KICK
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
     `tests/fixtures/batch1/g14/<Name>.txt` (unescaped); params with name, label and default as in the fixture.
   - `param_display` and `param_label` reproduce `getParameterDisplay`/`getParameterLabel` case by case,
     using the `display.rs` helpers.
   - `process` ports `processReplacing` statement by statement, and `reset` restores the constructor
     state in place without allocating.
2. `crates/airwindows/src/plugins/batch1/g14/mod.rs`: `mod <module>;` lines and `ENTRIES` in kick-list line order.
3. `crates/airwindows/tests/parity_batch1_g14.rs`: `mod common;`, one `const RECORD_<NAME>: ParityRecord` per
   plugin, and one `#[test] fn <module>()` calling
   `common::parity::check_plugin(&RECORD_X, include_str!("fixtures/batch1/g14/<Name>.txt"), include_bytes!("fixtures/batch1/g14/<Name>.f32"))`.

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
`tools/airwindows-harness/generate.sh --work-dir tmp/behringer-oss-ports/bop-15/harness --out-root crates/airwindows/tests/fixtures --group g14`.

## Tests (situation -> expected)

- Each plugin with `check_plugin`: the metadata, description, R0/R1/R2 and reset re-render match within its class.
- `tests/catalog.rs` and `tests/no_alloc.rs` cover these plugins through the registry. Both pass.

## Verification Commands (evidence)

1. `cargo nextest run -p music-things-airwindows --test parity_batch1_g14`: exit 0, testsRun = 6, failures = 0.
2. `cargo nextest run -p music-things-airwindows --test catalog --test no_alloc`: exit 0.
3. `cargo clippy -p music-things-airwindows --all-targets -- -D warnings`: exit 0, with no warnings in owned files.
4. `rustfmt --edition 2021 --check crates/airwindows/src/plugins/batch1/g14/*.rs crates/airwindows/tests/parity_batch1_g14.rs`: exit 0.
5. `cargo build -p music-things-airwindows --target wasm32-unknown-unknown`: exit 0.
6. `wc -l crates/airwindows/src/plugins/batch1/g14/*.rs`: every file under 1000 lines.

## Completion Criteria

- [ ] All 6 plugins of lines 105-110 are ported, registered in order and tested (or marked BLOCKED with evidence).
- [ ] Every plugin has a ParityRecord with its class and measured max-abs, repeated in the module header.
- [ ] Verifications 1-6 exit 0, with logs in `tmp/behringer-oss-ports/bop-15/logs/`.
- [ ] Only `writePaths` are modified.
- [ ] Progress Log records the plugin list, classes, max-abs values, commands, exit status and log paths.

## Progress Log

### Session: 2026-10-31
**Tasks Completed**: Plan created (not started)
**Notes**: -

### Session: 2026-11-01
**Tasks Completed**: Candidate ports and registration for Distance, Distance2, Distortion, DitherFloat and DitherMeDiskers; six provisional parity tests.
**Tasks In Progress**: Ditherbox is not implemented. Parity classes in the test are provisional and all max-abs values are unmeasured; headers explicitly say pending. No completion criterion is met yet.
**Blockers**: Focused five-plugin nextest exited 101 before running tests: sibling-owned `crates/airwindows/src/plugins/batch1/g12/mod.rs:16-19` moves static entries (E0507). Log: `tmp/behringer-oss-ports/bop-15/logs/focused-five.log`. Repair by the g12 owner, then finish Ditherbox, measure parity and rerun all six required gates.
**Command**: `CARGO_TARGET_DIR=tmp/behringer-oss-ports/bop-15/check-target CARGO_TERM_QUIET=true NEXTEST_STATUS_LEVEL=fail NEXTEST_FAILURE_OUTPUT=immediate-final NEXTEST_HIDE_PROGRESS_BAR=1 cargo nextest run -p music-things-airwindows --test parity_batch1_g14 -E 'not test(ditherbox)'` (exit 101; 0 tests run).

### Session: 2026-11-01 (continued)
**Tasks Completed**: Fixed Distance display scaling and parameter-preserving reset for Distance, Distance2, Distortion and DitherFloat. Added reset regression assertions and scoped Clippy allowances for upstream constants and signatures. The prior g12 compile error no longer occurs.
**Tasks In Progress**: Ditherbox remains unimplemented and unregistered. Parity classes and max-abs measurements remain provisional and unmeasured for Distance, Distance2, Distortion, Ditherbox, DitherFloat and DitherMeDiskers. Formal review and subsequent workflow finalization remain downstream.
**Verification**: With `CARGO_TARGET_DIR=tmp/behringer-oss-ports/bop-15/check-target` and the plan's quiet Cargo environment:
- `cargo nextest run -p music-things-airwindows --test parity_batch1_g14`: exit 100, 6 run, 5 passed, 1 failed (Ditherbox missing); `tmp/behringer-oss-ports/bop-15/logs/final-2-parity.log`.
- `cargo nextest run -p music-things-airwindows --test catalog --test no_alloc`: exit 0, 4 passed; `tmp/behringer-oss-ports/bop-15/logs/final-2-catalog.log`.
- `cargo clippy -p music-things-airwindows --all-targets -- -D warnings`: exit 101; sibling-owned g11 Clippy diagnostics, `tmp/behringer-oss-ports/bop-15/logs/final-2-clippy.log`.
- `find crates/airwindows/src/plugins/batch1/g14 crates/airwindows/tests/parity_batch1_g14.rs -name '*.rs' -print0 | xargs -0 rustfmt --edition 2021 --check`: exit 0; `tmp/behringer-oss-ports/bop-15/logs/final-2-rustfmt.log`.
- `cargo build -p music-things-airwindows --target wasm32-unknown-unknown`: exit 0; `tmp/behringer-oss-ports/bop-15/logs/final-2-wasm.log`.
- `test -z "$(find crates/airwindows/src/plugins/batch1/g14 -name '*.rs' -exec wc -l {} + | awk '$2 != "total" && $1 >= 1000')"`: exit 0; `tmp/behringer-oss-ports/bop-15/logs/final-2-lines.log`.
**Next**: Complete Ditherbox, measure and record parity, resolve g11 ownership issue with its owner, and rerun all six gates on the stable shared tree.

### Session: 2026-11-01 (current continuation)
**Investigation**: The rust-coding agent inspected the pinned Ditherbox processor: its 25 modes require a complete port; no reusable local implementation was found. The parity helper does not expose measured maxima. No Rust source was changed in this continuation. Ditherbox and all six measured maxima remain outstanding.
**Verification**: The check-and-test-after-modify agent ran the six plan commands with `CARGO_TARGET_DIR=tmp/behringer-oss-ports/bop-15/turn-current-target` and the specified quiet environment. Parity exited 100 (5/6 passed; Ditherbox missing); catalog/no-alloc exited 0 (4/4 passed); Clippy, rustfmt, wasm build and line limit each exited 0. Complete logs: `tmp/behringer-oss-ports/bop-15/logs/turn-current-{parity,catalog,clippy,rustfmt,wasm,lines}.log`. These results reflect the current shared tree; rerun after completing the port.
**Next**: Implement and register Ditherbox, measure and record each plugin's parity class and maximum, and rerun all six gates.

### Session: 2026-11-01 (RW5-3 redispatch)
**Investigation**: Ditherbox remains absent. Its upstream float path includes 14 processing cases; upstream `flip` is not initialized by the constructor and is read in case 7. This needs investigation during the full port. All six measured maxima remain outstanding; zero values in provisional records are not measurements. No Rust source was changed.
**Verification**: On the shared tree, using `CARGO_TARGET_DIR=tmp/behringer-oss-ports/bop-15/rw5-target` and the plan's quiet environment, the six requested gates produced: parity exit 100 (6 run, 5 passed, 1 failed: Ditherbox missing); catalog/no-alloc exit 0 (4 passed); clippy, rustfmt, wasm build and line limit exit 0. Complete logs are `tmp/behringer-oss-ports/bop-15/logs/rw5-{parity,catalog,clippy,rustfmt,wasm,lines}.log`.
**Remaining**: Implement and register all Ditherbox modes, measure all six maxima, update records and headers, and rerun the gates. RW5-3 remains unresolved; formal review remains downstream.

### Session: 2026-11-01 (Ditherbox completion)
**Tasks Completed**: Ported and registered Ditherbox's FLOAT processing modes. Initialized upstream's uninitialized `flip` to false, recorded the divergence, and verified parameter-preserving reset. The committed Ditherbox fixture matches exactly, with measured max-abs `0.0`.
**Verification**: With `CARGO_TARGET_DIR=target/bop-15-direct` and quiet Cargo/nextest settings, parity (6 tests), catalog and no-alloc (4 tests), Clippy, rustfmt on the touched Rust files, and the wasm32 build all exited 0.
**Remaining**: The other plugins' provisional measurements and broader plan completion remain outside this session's scope.
