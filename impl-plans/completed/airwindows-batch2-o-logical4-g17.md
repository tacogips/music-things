# Airwindows Batch 2 / o-logical4: Logical4 (oversized; group g17, kick line 175)

**Status**: Completed
**Plan ID**: ab2-o-logical4
**Wave**: W2. Depends on ab2-00-scaffold; runs in parallel with all other W2 plans.
**Design Reference**: `design-docs/specs/design-airwindows-batch2.md` sections 2 and 3 (oversized
singleton groups, module directories) and 7 (Port plans)
**Common Contract**: `impl-plans/active/airwindows-batch2-common.md` (mandatory: sections 5-11)
**Created**: 2026-11-01

## Intent

Port the upstream Airwindows **Logical4** buss compressor. Its upstream `Logical4Proc.cpp` is
about 1700 lines; only the `processReplacing` float path is ported.

- Register it in `g17/mod.rs`.
- Prove parity with `tests/parity_batch2_g17.rs`.

Logical4 must be fully ported, registered and parity-tested. A stub, a partial port or an
unregistered module is a failure of this plan, not a deferral.

## Non-goals

No other plugin or group, no framework/shared-test/tools/fixture edits, no `processDoubleReplacing`,
no algorithm refactoring.

## Write Ownership

- **writePaths**:
  - `crates/airwindows/src/plugins/batch2/g17`
  - `crates/airwindows/tests/parity_batch2_g17.rs`
  - `crates/airwindows/tests/fixtures/batch2/g17`
  - `impl-plans/active/airwindows-batch2-o-logical4-g17.md`
  - `target`
  - `tmp/airwindows-batch2/ab2-o-logical4`
- **sharedPaths**: none.
- **artifactRoots**: `target`, `tmp/airwindows-batch2/ab2-o-logical4`.

## Module Layout (required: module directory, every file under 1000 lines, target 900 or fewer)

- `g17/mod.rs`: `mod logical4;` and `ENTRIES = &[logical4::ENTRY]`.
- `g17/logical4/mod.rs`: header, narrow allow list, params, `INFO`/`ENTRY`/`construct`, the
  struct, the constructor and the trait impl.
- `g17/logical4/<stage>.rs`: consecutive upstream sections. Logical4 runs several compressor stages
  at different effective rates with heavily repeated L/R code, so split at those stage boundaries.

## Ordered Subtasks (declare `#[allow(dead_code)] mod <module>;` in S1 so cmd 1 compiles each subtask; "register" = add the ENTRY and remove the allow, only at the end; common section 5 item 6)

1. **S1**: state from `Logical4.h` (the many gain/ratio/`fpFlip`/counter fields), constructor, seed
   replay, params and display.
2. **S2**: the stage sections, in upstream order.
3. **S3**: `reset`.
4. **S4**: register, then the parity test (and a mode-sweep test if there is an integer selector).
5. **S5**: measure, record and run the gates.

## Pitfalls

- The stages run at sub-rates, gated by integer counters or toggles (for example every 2nd or 4th
  sample). Port the counter increment/compare order and the branch structure exactly.
- Ratio and threshold mapping uses `pow`/`sqrt` on doubles. Check the precision.
- Crossing locals must be passed explicitly.

## Verification

Use common section 9 with P = `ab2-o-logical4` and G = `g17`. Logs go to
`tmp/airwindows-batch2/ab2-o-logical4/logs/`.

1. `$ENV cargo check -p music-things-airwindows --all-targets`: exit 0.
2. `$ENV cargo nextest run -p music-things-airwindows --test parity_batch2_g17`: exit 0,
   testsRun >= 1, failures 0.
3. `find crates/airwindows/src/plugins/batch2/g17 crates/airwindows/tests/parity_batch2_g17.rs -name '*.rs' -print0 | xargs -0 rustfmt --edition 2021 --check`:
   exit 0.
4. `test -z "$($ENV cargo clippy -p music-things-airwindows --all-targets --message-format short -- -D warnings 2>&1 | grep -E 'src/plugins/batch2/g17/|tests/parity_batch2_g17\.rs')"`:
   exit 0.
5. `test -z "$(find crates/airwindows/src/plugins/batch2/g17 -name '*.rs' -exec wc -l {} + | awk '$2 != "total" && $1 >= 1000')"`:
   exit 0.
6. `test "$(grep -c '::ENTRY' crates/airwindows/src/plugins/batch2/g17/mod.rs)" -eq 1 && test -f crates/airwindows/src/plugins/batch2/g17/logical4/mod.rs`:
   exit 0.
7. `! grep -rnE 'todo!|unimplemented!|#\[ignore' crates/airwindows/src/plugins/batch2/g17 crates/airwindows/tests/parity_batch2_g17.rs`:
   exit 0.
8. Diagnostic: `$ENV cargo nextest run -p music-things-airwindows --test no_alloc --test catalog`.
   It must show no failure naming Logical4.

## Completion Criteria

- [x] Logical4 fully ported as a module directory, registered (cmd 6) and passing parity (cmd 2).
- [x] The `ParityRecord` and the header record the class and measured max-abs.
- [x] Commands 1-7 exit 0, with logs. Command 8 shows no Logical4 failure.
- [x] Only writePaths changed (pre/post hashes). Progress Log records subtasks per session.

## Progress Log

### Session: 2026-11-01
**Tasks Completed**: Plan created (not started).

### Session: 2026-11-01 (implementation)
**Progress**: Added a registered Logical4 implementation, fixture parity and ratio-selector smoke
test. The three fixture renders measured max-abs 0 (Libm); preliminary parity passed. Intent and
verification evidence is under `tmp/airwindows-batch2/ab2-o-logical4/`.
**Remaining**: The current shared stage and polarity helpers consolidate upstream branches, which
conflicts with this plan's no-algorithm-refactoring constraint. Translate the consecutive upstream
sections faithfully, then repeat all gates and record final results. Completion criteria remain open.

### Session: 2026-11-01 (continuation)
**Progress**: Replaced the shared processing loop with separate first, second and third stage
translations with explicit channel and polarity branches. Preserved upstream's third-stage target
asymmetry and corrected sample-rate and linkage evaluation order. The final-source parity and
selector tests passed (2/2), with Libm max-abs 0; the no-allocation/catalog diagnostic passed (4/4).
All scoped gates exited 0; commands and logs are indexed in
`tmp/airwindows-batch2/ab2-o-logical4/logs/verification-cont1.json`.
**Scope**: Per-edit intentions and native snapshots cover this plan's paths. Global scope
attribution on the moving shared tree remains for the serial join.

### Session: 2026-11-01 (scope audit)
**Evidence**: `tmp/airwindows-batch2/ab2-o-logical4/logs/scope-audit-cont2.json` records
initial and current owned-file hashes; both fixtures match the initial snapshot. The protected
tracked-path comparison against checkpoint 8dbd917 is empty. Per-edit intentions record this
worker's writes. Native snapshots cover assigned roots, so attribution of concurrent changes
elsewhere remains a serial-join responsibility.

### Session: 2026-11-02 (selector coverage and fresh gates)
**Progress**: Corrected the ratio selector smoke test to cover the midpoint of each of the three
reachable intervals at both 44100 and 96000 Hz. Logical4 remains Libm, measured max-abs 0.
**Verification**: Gates 1–7 and 7b passed; parity passed 2/2 and diagnostic 8 passed 4/4.
Complete logs: `tmp/airwindows-batch2/ab2-o-logical4/logs/check-agent-final-g*.log`.
Initial check and parity attempts encountered a transient sibling g14 LeadAmp compile error;
the fresh checks passed after that sibling changed. Shared-tree final integration and formal review
remain downstream. Selector repair intent is recorded in `intents/selector-sweep-repair.txt` under
the plan evidence directory.

### Session: 2026-11-13 (operator integration)
**Tasks Completed**: Closed by operator integration after review-loop session-629 stopped as implementation-blocked (ab2-n01 DubSub, ab2-n04 write-scope evidence). Merged main (batch 3) into the branch and ran the combined-tree gates: nextest 481/481 (3 pre-existing SoundFont skips), clippy -D warnings, fmt --check and the wasm32 build all exit 0.
**Notes**: Checkboxes left unticked were superseded by the combined-tree gates above, not run individually.
