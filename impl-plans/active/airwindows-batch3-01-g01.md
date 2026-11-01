# Airwindows Batch 3 01: Group g01 (kick list lines 221-228)

**Status**: Ready
**Plan ID**: aw-b3-01
**Design Reference**: design-docs/specs/design-airwindows-batch3.md (Grouping, Module layout and naming, Tests, Verification); baseline design-docs/specs/design-behringer-oss-ports.md section A (Porting rules, Parity method, Tolerances)
**Depends On**: aw-b3-00 (wave W2; runs in parallel with the other 13 group plans). aw-b3-15 depends on this plan.
**Created**: 2026-11-01
**Last Updated**: 2026-11-01

## Intent and Context

Port the 8 Airwindows plugins on lines 221-228 of `$MUSIC_THINGS_REFS/kick-airwindows-set.txt` to Rust.
Register them in `batch3/g01` and prove parity against the fixtures that aw-b3-00 generated in
`crates/airwindows/tests/fixtures/batch3/g01/`.

Plugins, in line order: PowerSag, PowerSag2, Precious, Preponderant, Pressure4, Pressure5, PurestAir, PurestConsole2Buss.

First confirm the list with `sed -n '221,228p' "$MUSIC_THINGS_REFS/kick-airwindows-set.txt"` and
`ls crates/airwindows/tests/fixtures/batch3/g01/*.txt`. If either disagrees with the list above, the list
file and the fixtures win. Record any difference in the Progress Log.

`MUSIC_THINGS_REFS=/Users/taco/gits/tacogips/music-things-refs`. Read only
`airwindows/plugins/LinuxVST/src/<Name>/{<Name>.h,<Name>.cpp,<Name>Proc.cpp}` (airwindows @
`b110fac3991fff7c4655cc2d25d6a3e221f9799c`, MIT). Never open Music Tribe AGPL code.

**Completeness rule**: each of the 8 plugins above must be fully ported, registered in `g01/mod.rs` and
parity-tested in `tests/parity_batch3_g01.rs`. Any of the following is a failure of this plan, not a
deferral:
- a stub, a `todo!`/`unimplemented!`, or an empty `process`
- an unregistered module
- an `#[ignore]`d or missing test
- a plugin marked "blocked"

There is no "mark BLOCKED" exit here. In batch 1 the two largest plugins ended up as a stub and as
never written. Do not repeat that.

The framework is frozen. Use it as it is:
- `crates/airwindows/src/effect.rs:AirwindowsEffect` (`info`, `get_param`, `set_param`, `param_display`,
  `param_label`, `reset`, `process`)
- `params.rs:{ParamInfo, PluginInfo, sanitize_param}`
- `registry.rs:PluginEntry { info, construct: fn(f32, u32) -> Box<dyn AirwindowsEffect> }`
- `dither.rs:{fpd_from_seed, xorshift, frexpf_exponent, pow2, fpd_dither}`
- `rand.rs:{AwRand, RAND_MAX, replay_fpd_seed_loop}`
- `display.rs:{float2string, db2string, int2string}`
- `tests/common/parity.rs:{check_plugin, ParityRecord, ParityClass}`

Patterns to imitate:
- `crates/airwindows/src/plugins/batch1/g03/beam.rs`: a single-file plugin (header, `PARAMS`, `INFO`,
  `ENTRY`, `construct`, struct, `impl AirwindowsEffect`).
- `crates/airwindows/src/plugins/batch1/g03/bigamp/mod.rs` and `bigamp/cabinet.rs`: a split module
  (`mod cabinet;` with `pub(super) fn` helpers).
- `crates/airwindows/src/plugins/batch1/g03/mod.rs`: the group's `mod` lines and `ENTRIES`.
- `crates/airwindows/tests/parity_batch1_g03.rs`: the local `check_reset_preserves_params` helper, and
  one `RECORD_<NAME>` const plus one `#[test]` per plugin.

## Group-specific notes

- Before porting each plugin, run `grep -n 'rand()' <Name>Proc.cpp <Name>.cpp`. Any per-sample `rand()`
  must draw from the plugin's `AwRand` in upstream order, and `reset` must replay it.
- Before porting each plugin, check every array member size in `<Name>.h`. Arrays longer than 4096
  elements become `Box<[T]>`.

## Non-goals

- Do not edit anything outside writePaths. That includes the framework files, `src/plugins/mod.rs`,
  `src/plugins/batch3/mod.rs`, other groups, `batch1/`, `batch2/`, `tests/common/`, `tests/catalog.rs`,
  other tests, `tools/`, `Cargo.toml` and `Cargo.lock`.
- Port no plugin outside lines 221-228. Do not port `processDoubleReplacing`, chunks or programs.
- Do not regenerate fixtures unless aw-b3-00 has fixed a harness defect that you reported (see Escalation).
- Do not gate on crate-wide clippy and do not run workspace `cargo fmt`. Sibling plans edit the same
  crate concurrently.

## Write Ownership (fanout contract)

- writePaths:
  - `crates/airwindows/src/plugins/batch3/g01` (a directory: `mod.rs`, plus `<module>.rs` or `<module>/` per plugin)
  - `crates/airwindows/tests/parity_batch3_g01.rs`
  - `crates/airwindows/tests/fixtures/batch3/g01` (a directory; regenerate only after an aw-b3-00 harness fix)
  - `impl-plans/active/airwindows-batch3-01-g01.md` (this plan's progress log only)
  - `target`
  - `tmp/airwindows-batch3/aw-b3-01`
- sharedPaths: none.
- artifactRoots (digest-only): `target`, `tmp/airwindows-batch3/aw-b3-01`.
- Read-only: `crates/airwindows/src/*.rs`, `src/plugins/mod.rs`, `src/plugins/batch3/mod.rs`,
  `crates/airwindows/tests/common/*`, `tools/*`, and `batch1` files (reference only).

## Common Execution Rules

- The evidence directory is `tmp/airwindows-batch3/aw-b3-01/`. It holds:
  - `intent.md`: the plugin list and the deliverables. Write it once at the start and never change it.
  - `pre-hashes.txt` and `post-hashes.txt`: `shasum -a 256` of every owned file.
  - `logs/`: one log per verification command.
- Re-read each file just before you edit it. If it drifted unexpectedly, stop and log the drift.
- Set this environment for every cargo command:
  `CARGO_TERM_QUIET=true CARGO_TARGET_DIR=tmp/airwindows-batch3/aw-b3-01/target NEXTEST_STATUS_LEVEL=fail NEXTEST_FAILURE_OUTPUT=immediate-final NEXTEST_HIDE_PROGRESS_BAR=1`.
- 13 sibling plans compile this crate at the same time.
  - If a compile error is in a file you do not own, log it, wait, and rerun. Never edit that file.
  - Finish a plugin file so it compiles before adding its `mod` line and `ENTRY` to `g01/mod.rs`.
- Format only the files you own, with `rustfmt --edition 2021`.
- Never run git checkout, stash, reset or commit.

## Deliverables

1. One module per plugin: `crates/airwindows/src/plugins/batch3/g01/<module>.rs`, where `<module>` is
   the KICK name lowercased. If the port would reach 1000 lines, use `<module>/mod.rs` plus submodules
   split by concern instead.
   - Header:
     - the upstream copyright line, verbatim from `<Name>.h`
     - `Upstream: airwindows/plugins/LinuxVST/src/<Name>, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.`
     - `Parity: <Class>, measured max-abs <value>.`, plus the reason when the class is `Custom`
   - Struct, named for the upstream class in UpperCamelCase. Fields:
     - params `[f32; N]`, `sample_rate: f32`, `seed: u32`
     - snake_case state mirroring the members in `.h`
     - `fpd_l`/`fpd_r`, only if upstream has them
     - the `AwRand`, when `processReplacing` calls `rand()`
   - Constructor:
     - Set the defaults exactly as upstream does.
     - Call `AwRand::new(seed)`, then replay every constructor `rand()` call in upstream order. This
       includes the fpd loop, through `replay_fpd_seed_loop`.
     - Then set `fpd_l = fpd_from_seed(seed, 0)` and `fpd_r = fpd_from_seed(seed, 1)`.
   - `INFO`:
     - `name` and `upstream_name` are both the KICK name.
     - `description` is exactly the unescaped `description=` value from
       `tests/fixtures/batch3/g01/<Name>.txt`.
     - Param names, labels and defaults are as in that fixture.
   - `param_display`/`param_label` reproduce `getParameterDisplay`/`getParameterLabel` case by case,
     using the `display.rs` helpers.
   - `process` ports `processReplacing` statement by statement.
   - `reset` restores the constructor state in place, without allocating:
     - zero the buffers
     - re-derive fpd
     - recreate and replay `AwRand`
     - keep the params and the sample rate
2. `crates/airwindows/src/plugins/batch3/g01/mod.rs`: keep the doc line and the `use`. Add the
   `mod <module>;` lines and set `ENTRIES = &[<module>::ENTRY, ..]`, both in kick-list line order.
3. `crates/airwindows/tests/parity_batch3_g01.rs`:
   - Start with `mod common;` and `use common::parity::{check_plugin, ParityClass, ParityRecord};`.
   - Copy the local `check_reset_preserves_params(name)` from `parity_batch1_g03.rs`.
   - For each plugin, add `const RECORD_<NAME>: ParityRecord` and a `#[test] fn <module>()`. The test
     calls `check_reset_preserves_params("<Name>")` and then
     `check_plugin(&RECORD_<NAME>, include_str!("fixtures/batch3/g01/<Name>.txt"), include_bytes!("fixtures/batch3/g01/<Name>.f32"))`.

## Porting Rules (common failure sources)

- **Precision.**
  - `float` becomes `f32`, and `double`/`long double` become `f64`.
  - Mixed expressions are computed in f64 and rounded only where C++ assigns to a float.
  - A libm call with a float argument uses the f32 function; with a double argument it uses the f64
    function. Check every call's argument type.
- **Operation order.** Keep it exactly.
  - Never rewrite `pow(x,2)` as `x*x`.
  - Never reorder or factor expressions, and never use `mul_add`.
- **Integers.**
  - Unsigned math wraps (`wrapping_*`), and `uint32_t` shifts are `u32`.
  - Integer `/` and `%` truncate, the same as in C. Keep signedness identical.
- **Dither.**
  - Copy the near-zero input replacement exactly.
  - Use `fpd_dither` only when the dither block matches the canonical form exactly. Otherwise inline it.
  - Convert the output with `as f32`.
- **`rand()` in `processReplacing`** uses the same `AwRand`, in upstream call order (`next()`, `RAND_MAX`).
- **`overallscale`** uses `self.sample_rate as f64`, in upstream order.
- **Allocation.**
  - Function-local statics become fields.
  - Arrays longer than 4096 elements become `Box<[T]>`, allocated in the constructor.
  - `process`, `set_param`, `get_param` and `reset` never allocate. `tests/no_alloc.rs` checks this.
- **Casts and indices.** Float-to-int casts and array indices only see values that upstream keeps in
  range. Mirror the upstream wrap and clamp logic exactly.
- **Parity class.**
  - `Exact`: no libm transcendental in the per-sample path (sqrt, fabs, floor, frexp and pow2 are allowed).
  - `Libm` otherwise, with max-abs <= 1e-6.
  - `Custom(x)` only after investigation, with the reason in `note` and in the header.
  - The expected measured max-abs on aarch64-apple-darwin is 0. If it is not 0, investigate first
    (usually a wrong f32/f64 overload or operation order). Never loosen the tolerance silently.

## Escalation

If you find a harness or shim defect (a provably wrong fixture or a missing shim symbol):
1. Do not edit `tools/`.
2. Log the evidence and report it to aw-b3-00.
3. After aw-b3-00 fixes it, regenerate only this group:
   `MUSIC_THINGS_REFS=... tools/airwindows-harness/generate_batch3.sh --work-dir tmp/airwindows-batch3/aw-b3-01/harness --out-root crates/airwindows/tests/fixtures --group g01`.

The plan stays In Progress until every plugin passes.

## Tests (situation -> expected)

- Each plugin:
  - `check_plugin` passes: metadata, description, and the R0/R1/R2 renders plus the reset re-render are
    within the class bound.
  - `check_reset_preserves_params` passes.
- Negative control (record in `mutationEvidence`):
  - Temporarily perturb one constant in one plugin. Its parity test must fail and report the plugin,
    render, frame and channel.
  - Restore the constant and confirm the file hash equals its pre-mutation hash.
- `tests/catalog.rs` and `tests/no_alloc.rs` pass with these plugins registered.

## Verification Commands (evidence)

Use the environment above. Write logs to `tmp/airwindows-batch3/aw-b3-01/logs/`.

1. `cargo check -p music-things-airwindows --all-targets`: exit 0.
2. `cargo nextest run -p music-things-airwindows --test parity_batch3_g01`: exit 0, testsRun = 8,
   testsPassed = 8, 0 skipped.
3. `cargo nextest run -p music-things-airwindows --test catalog --test no_alloc`: exit 0. If a failure
   names a sibling plan's plugin, log it and rerun later. Failures in your own plugins must be fixed.
4. `find crates/airwindows/src/plugins/batch3/g01 crates/airwindows/tests/parity_batch3_g01.rs -name '*.rs' -print0 | xargs -0 rustfmt --edition 2021 --check`:
   exit 0.
5. `{ cargo clippy -p music-things-airwindows --all-targets --message-format short -- -D warnings 2>&1 | grep -E "src/plugins/batch3/g01/|tests/parity_batch3_g01" ; } ; test $? -eq 1`:
   exit 0, meaning no diagnostics in your own files. Run it only after command 1 has passed in the
   same session.
6. `find crates/airwindows/src/plugins/batch3/g01 crates/airwindows/tests/parity_batch3_g01.rs -name '*.rs' -exec wc -l {} + | awk '$2!="total" && $1>=1000 {bad=1; print} END {exit bad}'`:
   exit 0.
7. Completeness:
   - `grep -c '^#\[test\]' crates/airwindows/tests/parity_batch3_g01.rs` prints `8`.
   - `grep -c '::ENTRY' crates/airwindows/src/plugins/batch3/g01/mod.rs` prints `8`.
   - `grep -rnE 'todo!|unimplemented!|#\[ignore' crates/airwindows/src/plugins/batch3/g01 crates/airwindows/tests/parity_batch3_g01.rs`
     prints nothing.

Record each command as `{command, exitStatus: 0, testsRun, testsPassed, failureCount: 0, outcome: "passed", log, notes}`.

## Completion Criteria

- [ ] All 8 plugins listed under Intent and Context are ported, registered in line order and
  parity-tested. None is stubbed, skipped or blocked.
- [ ] Every plugin has a `ParityRecord` with its class and measured max-abs, and the module header repeats it.
- [ ] Verifications 1-7 pass (`parity_batch3_g01`: 8 run, 8 passed), with logs.
- [ ] Every `.rs` file is under 1000 lines, and only writePaths changed (pre- and post-hashes recorded).
- [ ] The Progress Log records the plugin list, each class and max-abs, and every command with its exit
  status and log path.

## Progress Log

### Session: 2026-11-01
**Tasks Completed**: Plan created (not started)
**Notes**: -
