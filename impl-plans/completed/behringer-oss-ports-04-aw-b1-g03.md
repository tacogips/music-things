# Behringer OSS Ports 04: Airwindows Batch-1 Group g03 (kick list lines 17-24)

**Status**: Completed
**Plan ID**: bop-04
**Design Reference**: design-docs/specs/design-behringer-oss-ports.md section A (Additive registration, Batch-1 grouping, Porting rules, Parity method, Tolerances)
**Depends On**: bop-01 (wave W3; runs in parallel with the other 13 group plans and bop-16c)
**Created**: 2026-10-31
**Last Updated**: 2026-11-01

## Intent and Context

Port the Airwindows plugins on lines 17-24 of `$MUSIC_THINGS_REFS/kick-airwindows-set.txt` (8 plugins)
to Rust, register them in group `g03`, and prove parity against the fixtures bop-01 generated in
`crates/airwindows/tests/fixtures/batch1/g03/`. List the names with
`sed -n '17,24p' "$MUSIC_THINGS_REFS/kick-airwindows-set.txt"` and record them in the Progress Log.
KICK `Console8BusOut` means upstream `Console8BussOut`.

The framework is frozen. Use it exactly as bop-01 built it:
- `crates/airwindows/src/effect.rs:AirwindowsEffect`, `params.rs:{ParamInfo, PluginInfo, sanitize_param}`
- `registry.rs:PluginEntry`, `dither.rs:{fpd_from_seed, xorshift, frexpf_exponent, pow2, fpd_dither}`
- `rand.rs:{AwRand, RAND_MAX, replay_fpd_seed_loop}`, `display.rs:{float2string, db2string, int2string}`
- `tests/common/parity.rs:{check_plugin, ParityRecord, ParityClass}`

Imitate the per-plugin module contract documented in `crates/airwindows/src/plugins/mod.rs`
(`INFO`/`ENTRY`/`construct` consts). Landed sibling plugin files may be read as style references.

## Non-goals

- Do not edit the framework (`src/*.rs` outside `src/plugins/batch1/g03/`), other groups, `tests/common/`,
  `tools/`, `Cargo.toml`/`Cargo.lock`, or any other crate.
- No plugins outside lines 17-24, no stubs, and no `processDoubleReplacing`, chunks or programs.
- Do not regenerate fixtures unless bop-01 has fixed a harness defect that you reported (Escalation).

## Write Ownership (fanout contract)

- writePaths: `crates/airwindows/src/plugins/batch1/g03` (directory: `mod.rs` plus one `<module>.rs` per plugin),
  `crates/airwindows/tests/parity_batch1_g03.rs`, `crates/airwindows/tests/fixtures/batch1/g03` (directory;
  regenerated only after a bop-01 harness fix), `impl-plans/active/behringer-oss-ports-04-aw-b1-g03.md`,
  `target`, `tmp/behringer-oss-ports/bop-04`
- sharedPaths: none
- artifactRoots (digest-only): `target`, `tmp/behringer-oss-ports/bop-04`
- Read-only: framework `crates/airwindows/src/*.rs`, `src/plugins/mod.rs`, `src/plugins/batch1/mod.rs`,
  `crates/airwindows/tests/common/*`, `tools/*` (all frozen by bop-01).
- Never run git checkout/stash/reset or any other git operation that rewrites the tree.

## Common Execution Rules

- `MUSIC_THINGS_REFS=/Users/taco/gits/tacogips/music-things-refs`. Read only
  `airwindows/plugins/LinuxVST/src/<Upstream>/{<Upstream>.h,<Upstream>.cpp,<Upstream>Proc.cpp}` and the
  kick list. Never open Music Tribe AGPL code.
- Evidence directory: `tmp/behringer-oss-ports/bop-04/`. It holds `intent.md` (immutable: the plugin
  list and the deliverables), `pre-hashes.txt` and `post-hashes.txt` (`shasum -a 256` of owned files)
  and `logs/`. Re-read each file before editing; on drift, stop and log it. Only edit `writePaths`.
- 13 sibling plans compile this crate at the same time. If an error is in a file you do not own, rerun
  later and log it. Do not edit that file. Write a plugin file completely before adding its `mod` line
  and `ENTRY` to `g03/mod.rs`.
- Format only owned files: `rustfmt --edition 2021 <files>`. Cargo env: `CARGO_TERM_QUIET=true NEXTEST_STATUS_LEVEL=fail NEXTEST_FAILURE_OUTPUT=immediate-final NEXTEST_HIDE_PROGRESS_BAR=1`.

## Deliverables

1. `crates/airwindows/src/plugins/batch1/g03/<module>.rs`, one per plugin, where `<module>` is the KICK
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
     `tests/fixtures/batch1/g03/<Name>.txt` (unescaped); params with name, label and default as in the fixture.
   - `param_display` and `param_label` reproduce `getParameterDisplay`/`getParameterLabel` case by case,
     using the `display.rs` helpers.
   - `process` ports `processReplacing` statement by statement, and `reset` restores the constructor
     state in place without allocating.
2. `crates/airwindows/src/plugins/batch1/g03/mod.rs`: `mod <module>;` lines and `ENTRIES` in kick-list line order.
3. `crates/airwindows/tests/parity_batch1_g03.rs`: `mod common;`, one `const RECORD_<NAME>: ParityRecord` per
   plugin, and one `#[test] fn <module>()` calling
   `common::parity::check_plugin(&RECORD_X, include_str!("fixtures/batch1/g03/<Name>.txt"), include_bytes!("fixtures/batch1/g03/<Name>.f32"))`.

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
`tools/airwindows-harness/generate.sh --work-dir tmp/behringer-oss-ports/bop-04/harness --out-root crates/airwindows/tests/fixtures --group g03`.

## Tests (situation -> expected)

- Each plugin with `check_plugin`: the metadata, description, R0/R1/R2 and reset re-render match within its class.
- `tests/catalog.rs` and `tests/no_alloc.rs` cover these plugins through the registry. Both pass.

## Verification Commands (evidence)

1. `cargo nextest run -p music-things-airwindows --test parity_batch1_g03`: exit 0, testsRun = 8, failures = 0.
2. `cargo nextest run -p music-things-airwindows --test catalog --test no_alloc`: exit 0.
3. `cargo clippy -p music-things-airwindows --all-targets -- -D warnings`: exit 0, with no warnings in owned files.
4. `rustfmt --edition 2021 --check crates/airwindows/src/plugins/batch1/g03/*.rs crates/airwindows/tests/parity_batch1_g03.rs`: exit 0.
5. `cargo build -p music-things-airwindows --target wasm32-unknown-unknown`: exit 0.
6. `wc -l crates/airwindows/src/plugins/batch1/g03/*.rs`: every file under 1000 lines.

## Completion Criteria

- [x] All 8 plugins of lines 17-24 are ported, registered in order and tested (or marked BLOCKED with evidence).
- [x] Every plugin has a ParityRecord with its class and measured max-abs, repeated in the module header.
- [x] Verifications 1-6 exit 0, with logs in `tmp/behringer-oss-ports/bop-04/logs/`.
- [x] Only `writePaths` are modified.
- [x] Progress Log records the plugin list, classes, max-abs values, commands, exit status and log paths.

## Progress Log

### Session: 2026-10-31
**Tasks Completed**: Plan created (not started)
**Notes**: -

### Session: 2026-11-01
**Tasks Completed**: Provisional ports and registration for BassKit, Baxandall, Baxandall2, Beam, Biquad and Biquad2. Each is Libm with measured max-abs 0 (logs/measurement-retry.log).
**Tasks In Progress**: BassDrive and BigAmp are unimplemented and their parity records are unmeasured. No harness defect has been established.
**Verification**: Isolated target `tmp/behringer-oss-ports/bop-04/check-final-target`; logs under `tmp/behringer-oss-ports/bop-04/logs/`. Parity command 1 exit 100, 8 run, 6 passed, 2 failed (check-final2-parity.log); catalog/no_alloc command 2 exit 0, 4 passed (check-final2-catalog.log); clippy command 3 exit 101 on sibling files, no g03 diagnostics (check-final2-clippy.log); rustfmt command 4 exit 0 (check-final2-rustfmt.log); wasm command 5 exit 0 (check-final2-wasm.log); line limit command 6 exit 0 (check-final2-lines.log). Commands are listed in Verification Commands above with the dispatched environment; the shared tree was moving.
**Blockers**: Package-wide clippy currently reports diagnostics in sibling-owned files; their owners must repair them before this gate can pass. Complete the two missing ports, measure parity, and rerun all gates on current source.

### Session: 2026-11-01 (continued)
**Tasks Completed**: BassDrive implemented and registered; Libm, measured max-abs 0.0 (`logs/bassdrive-measure.log`). BassKit, Baxandall, Baxandall2, Beam, Biquad and Biquad2 remain Libm, max-abs 0.0 as recorded above.
**Tasks In Progress**: BigAmp has an unregistered scaffold; its process path and parity measurement remain incomplete. No harness defect has been established. Group order: BassDrive, BassKit, Baxandall, Baxandall2, Beam, BigAmp, Biquad, Biquad2.
**Verification**: Commands 1–6 above used the dispatched cargo environment and `CARGO_TARGET_DIR=tmp/behringer-oss-ports/bop-04/final-20261101-target`; current-source logs `logs/turn3-{parity,catalog,clippy,rustfmt,wasm,lines}.log` respectively. Exits: 100 (8 run, 7 passed, BigAmp missing), 0 (4 passed), 101 (sibling g11 diagnostics), 0, 0, 0. The shared tree was moving; stable verification remains pending.
**Blockers**: Package clippy requires repairs to sibling-owned g11 files `console8channelin.rs`, `console8subhype.rs`, `console8subin.rs`, and `crystal.rs`. Complete BigAmp and repeat all gates after sibling repairs.

### Session: 2026-11-01 (redispatch 2)
**Status**: BigAmp remains an unregistered scaffold; processing and measured parity are incomplete. No harness defect was established. The seven other plugins retain their previously recorded Libm, max-abs 0.0 measurements. The rust-coding agent investigated BigAmp and made no edits.
**Verification**: The check-and-test-after-modify agent ran commands 1–6 above using the dispatched environment and `CARGO_TARGET_DIR=tmp/behringer-oss-ports/bop-04/recheck-target`. Logs are `tmp/behringer-oss-ports/bop-04/logs/redispatch2-{parity,catalog,clippy,rustfmt,wasm,lines}.log`. Exits respectively: 100 (8 run, 7 passed, BigAmp missing), 0 (4 passed), 0, 0, 0, 0. The shared tree may change; repeat all gates after completing BigAmp.
**Remaining work**: Implement and register BigAmp, measure and record parity, then repeat all six gates. No sibling Clippy blocker was observed in this run.

### Session: 2026-11-01 (RW5 redispatch)
**Progress**: BassDrive, BassKit, Baxandall, Baxandall2, Beam, Biquad and Biquad2 now preserve parameters during reset; all seven remain Libm, measured max-abs 0.0 as previously recorded. Reset assertions cover all eight group tests. The BigAmp scaffold gained filter coefficient setup and parameter-preserving reset, but its float processing and registration remain incomplete; no harness defect was established. Group order remains BassDrive, BassKit, Baxandall, Baxandall2, Beam, BigAmp, Biquad, Biquad2. BigAmp parity is unmeasured.
**Verification**: Six dispatched commands above, with `CARGO_TARGET_DIR=tmp/behringer-oss-ports/bop-04/verification-redispatch-target` and prescribed quiet environment, produced exits 100 (8 run, 7 passed, BigAmp unregistered), 0 (4 passed), 0, 0, 0, 0 respectively. Complete logs: `tmp/behringer-oss-ports/bop-04/logs/redispatch3-final-{parity,catalog,clippy,rustfmt,wasm,lines}.log`. Seven-plugin focused parity passed (7/7), `tmp/behringer-oss-ports/bop-04/reset/logs/seven-plugins.log`; the pre-fix BassDrive reset negative control failed as expected, `tmp/behringer-oss-ports/bop-04/reset/logs/negative-control.log`. Shared tree verification may need repetition at integration.
**Remaining**: Translate BigAmp float processing, register it, measure parity and verify reset, then rerun all six gates. Formal review and integration remain downstream.

### Session: 2026-11-01 (BigAmp completion)
**Progress**: Completed and registered the BigAmp float port, including parameter-preserving reset and upstream seed replay. BigAmp is Libm with measured max-abs 0.0 across the existing fixture renders.
**Verification**: With `CARGO_TARGET_DIR=target/bop-04-direct`, group parity (8 tests), catalog and no-allocation (4 tests), Clippy, rustfmt on the touched Rust files, and the wasm32 build all exited 0.

## Operator resume note (2026-11-01)

The port that blocked this plan in session 578 waves 5 and 6 was completed by a
direct gpt-6.1-terra pass and committed at 49052ec (fixture parity exact,
measured max-abs 0.0; group parity, catalog/no_alloc, clippy -D warnings,
rustfmt and wasm32 all exit 0). Re-verify the plugin against the plan and the
pinned upstream source, fix anything a review finds, and re-run every gate
with fresh logs. Sibling plans are committed dependencies (see the manifest's
acceptedDependencies), so the shared crate is stable.

### Session: 2026-11-01 (final source verification)
**Progress**: Reviewed BigAmp against the pinned upstream float path and added the upstream copyright and revision to its split cabinet source. Group order: BassDrive, BassKit, Baxandall, Baxandall2, Beam, BigAmp, Biquad, Biquad2. All eight records are Libm, measured max-abs 0.0.
**Verification**: The following commands ran with `CARGO_TARGET_DIR=target/bop-04-current` and `CARGO_TERM_QUIET=true NEXTEST_STATUS_LEVEL=fail NEXTEST_FAILURE_OUTPUT=immediate-final NEXTEST_HIDE_PROGRESS_BAR=1`; all exited 0. Logs are under `tmp/behringer-oss-ports/bop-04/logs/`.

1. `cargo nextest run -p music-things-airwindows --test parity_batch1_g03`: 8 passed, 0 failed; `final-parity.log`.
2. `cargo nextest run -p music-things-airwindows --test catalog --test no_alloc`: 4 passed, 0 failed; `final-catalog.log`.
3. `cargo clippy -p music-things-airwindows --all-targets -- -D warnings`: `final-clippy.log`.
4. `find crates/airwindows/src/plugins/batch1/g03 crates/airwindows/tests/parity_batch1_g03.rs -name '*.rs' -print0 | xargs -0 rustfmt --edition 2021 --check`: `final-rustfmt.log`.
5. `cargo build -p music-things-airwindows --target wasm32-unknown-unknown`: `final-wasm.log`.
6. `test -z "$(find crates/airwindows/src/plugins/batch1/g03 -name '*.rs' -exec wc -l {} + | awk '$2 != "total" && $1 >= 1000')"`: `final-lines.log`.

Formal review and integration remain downstream.

### Session: 2026-11-01 (completion)
**Status**: Completed. Review-loop session-624 accepted it: test integrity (comm-012081), adversarial review (comm-012082) and integration review (comm-012087), all with no findings. The bop-18 combined-tree gates passed after this plan's last edit: workspace nextest 150 run, 150 passed (`tmp/behringer-oss-ports/bop-18/logs/workspace-nextest-final.log`); workspace clippy, wasm32 and `cargo fmt --check` exit 0 (`workspace-clippy-final.log`, `workspace-wasm-final.log`, `fmt-final.log`). Archived to `impl-plans/completed/`.
