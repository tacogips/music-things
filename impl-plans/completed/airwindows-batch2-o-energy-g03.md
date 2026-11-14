# Airwindows Batch 2 / o-energy: Energy (oversized; group g03, kick line 126)

**Status**: Completed
**Plan ID**: ab2-o-energy
**Wave**: W2. Depends on ab2-00-scaffold; runs in parallel with all other W2 plans.
**Design Reference**: `design-docs/specs/design-airwindows-batch2.md` sections 2 and 3 (oversized
singleton groups, module directories) and 7 (Port plans)
**Common Contract**: `impl-plans/active/airwindows-batch2-common.md` (mandatory: sections 5-11)
**Created**: 2026-11-01

## Intent

Port the upstream Airwindows **Energy** plugin. Its upstream `EnergyProc.cpp` is 1448 lines; only
the `processReplacing` float path is ported, roughly the first half.

- Register it in `g03/mod.rs`.
- Prove parity with `tests/parity_batch2_g03.rs`.

Energy must be fully ported, registered and parity-tested. A stub, a partial port with missing
stages, or an unregistered module is a failure of this plan, not a deferral. This plan exists
because batch 1 left its largest plugin (BigAmp) as an unregistered stub.

## Non-goals

No other plugin or group, no framework/shared-test/tools/fixture edits (fixtures only via common
section 10), no `processDoubleReplacing`, no refactoring of the upstream algorithm.

## Write Ownership

- **writePaths**:
  - `crates/airwindows/src/plugins/batch2/g03`
  - `crates/airwindows/tests/parity_batch2_g03.rs`
  - `crates/airwindows/tests/fixtures/batch2/g03`
  - `impl-plans/active/airwindows-batch2-o-energy-g03.md`
  - `target`
  - `tmp/airwindows-batch2/ab2-o-energy`
- **sharedPaths**: none.
- **artifactRoots**: `target`, `tmp/airwindows-batch2/ab2-o-energy`.

## Module Layout (required: module directory, every file under 1000 lines, target 900 or fewer)

- `g03/mod.rs`: `mod energy;` and `ENTRIES = &[energy::ENTRY]`.
- `g03/energy/mod.rs`: header, the narrow `#![allow(...)]` list copied from
  `batch1/g03/bigamp/mod.rs`, `PARAMS`/`INFO`/`ENTRY`/`construct`, the `Energy` struct, the
  constructor and the `AirwindowsEffect` impl (display, label, reset, and `process` delegating to
  the submodules).
- One or more `g03/energy/<stage>.rs` submodules holding consecutive sections of the upstream
  per-sample loop. Energy processes several harmonic bands in sequence, so split at those upstream
  section boundaries.
  - Each section function takes `&mut Energy` (or the struct's state fields) plus the per-sample
    locals it reads and writes, and returns the updated locals.
  - The upstream order of sections and statements is kept exactly.
- Imitate the split in `batch1/g03/bigamp/{mod.rs,cabinet.rs}`.

## Ordered Subtasks (may span several sessions; registration only at the end)

1. **S1**: struct fields mirroring `Energy.h` (types per common section 6), constructor defaults,
   seed replay, `PARAMS`/`INFO` from fixture `g03/Energy.txt`, and `param_display`/`param_label`.
   Declare `#[allow(dead_code)] mod energy;` in `g03/mod.rs` now (common section 5 item 6), with
   `ENTRIES` still empty. Cmd 1 must exit 0.
2. **S2**: port the per-sample loop section by section into submodules, keeping each file at most
   900 lines.
3. **S3**: `reset`, preserving params and not allocating.
4. **S4**: `mod energy;` was declared at S1 with a temporary `#[allow(dead_code)]`, so cmd 1
   compiled every section as it landed. Add `energy::ENTRY` to `ENTRIES` only now and remove the
   allow, then write
   `tests/parity_batch2_g03.rs`: 1 `ParityRecord` plus the test, plus a mode-sweep test if Energy
   has an integer selector.
5. **S5**: measure max-abs, record it in the record and the header, and run all gates.

## Pitfalls

- When splitting the loop across functions, every local that crosses a section boundary must be
  passed and returned explicitly. A local re-initialized per sample upstream must not become a
  persistent field, and the reverse also holds.
- Do not merge repeated per-band code into a loop or generic helper if that changes evaluation
  order or precision. Identical-op helpers are allowed.
- Check every `.h` member's type (float vs double) before use.

## Verification

Use common section 9 with P = `ab2-o-energy` and G = `g03`. Logs go to
`tmp/airwindows-batch2/ab2-o-energy/logs/`.

1. `$ENV cargo check -p music-things-airwindows --all-targets`: exit 0.
2. `$ENV cargo nextest run -p music-things-airwindows --test parity_batch2_g03`: exit 0,
   testsRun >= 1, failures 0.
3. `find crates/airwindows/src/plugins/batch2/g03 crates/airwindows/tests/parity_batch2_g03.rs -name '*.rs' -print0 | xargs -0 rustfmt --edition 2021 --check`:
   exit 0.
4. `test -z "$($ENV cargo clippy -p music-things-airwindows --all-targets --message-format short -- -D warnings 2>&1 | grep -E 'src/plugins/batch2/g03/|tests/parity_batch2_g03\.rs')"`:
   exit 0.
5. `test -z "$(find crates/airwindows/src/plugins/batch2/g03 -name '*.rs' -exec wc -l {} + | awk '$2 != "total" && $1 >= 1000')"`:
   exit 0.
6. `test "$(grep -c '::ENTRY' crates/airwindows/src/plugins/batch2/g03/mod.rs)" -eq 1 && test -f crates/airwindows/src/plugins/batch2/g03/energy/mod.rs`:
   exit 0.
7. `! grep -rnE 'todo!|unimplemented!|#\[ignore' crates/airwindows/src/plugins/batch2/g03 crates/airwindows/tests/parity_batch2_g03.rs`:
   exit 0.
8. Diagnostic: `$ENV cargo nextest run -p music-things-airwindows --test no_alloc --test catalog`.
   It must show no failure naming Energy.

## Completion Criteria

- [x] Energy fully ported as a module directory, registered (cmd 6) and passing parity (cmd 2).
- [x] The `ParityRecord` and the header record the class and measured max-abs.
- [x] Commands 1-7 exit 0, with logs. Command 8 shows no Energy failure.
- [x] Only writePaths changed (pre/post hashes). Progress Log records subtasks S1-S5 per session.

## Progress Log

### Session: 2026-11-01
**Tasks Completed**: Plan created (not started).


### Session: 2026-11-01 (implementation)
**Tasks Completed**: S1-S5. Energy is registered with eight ordered band modules; Libm measured max-abs 0.0 on the acceptance host. Parity covers three renders and all nine parameters.
**Verification**: Gates 1-7 and 7b exit 0; parity gate 2 ran 1 test, passed 1. Diagnostic 8 exits 0, 4 tests passed. Complete logs: `tmp/airwindows-batch2/ab2-o-energy/logs/final-gate{1,2,3,4,5,6,7,7b}.log` and `final-diagnostic8.log`. Gate 4 scoped grep is empty; full clippy reports sibling g34 diagnostics in `final-clippy-full.log`.
**Scope and review**: Source and test edits are within this plan's write paths; native fanout snapshots provide the shared-tree baseline. Pre-edit hashes were recovered from the native pre-node snapshot into `pre-hashes.txt`; `post-hashes.txt` records owned files. Formal review and combined-tree reconciliation remain downstream.

### Session: 2026-11-02 (implementation recheck)
**Tasks Completed**: Rechecked S1-S5 and the existing Energy implementation against the accepted design; no source changes were needed. Parity passed 1 test covering all three renders; the catalog and no-allocation diagnostic passed 4 tests.
**Verification**: Gates 1-7 and 7b passed on the moving shared tree; complete logs are under `tmp/airwindows-batch2/ab2-o-energy/logs/recheck-20261102/`. The gate 4 scoped grep found no Energy diagnostics. Its first full clippy invocation encountered an in-progress sibling g14 compile error; the retry compiled and reported only sibling g14 lint errors in `clippy-retry-full.log`. Formal review and stable combined-tree verification remain downstream.

### Session: 2026-11-13 (operator integration)
**Tasks Completed**: Closed by operator integration after review-loop session-629 stopped as implementation-blocked (ab2-n01 DubSub, ab2-n04 write-scope evidence). Merged main (batch 3) into the branch and ran the combined-tree gates: nextest 481/481 (3 pre-existing SoundFont skips), clippy -D warnings, fmt --check and the wasm32 build all exit 0.
**Notes**: Checkboxes left unticked were superseded by the combined-tree gates above, not run individually.
