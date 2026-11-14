# Airwindows Batch 2 / o-grindamp: GrindAmp (oversized; group g09, kick line 145)

**Status**: Completed
**Plan ID**: ab2-o-grindamp
**Wave**: W2. Depends on ab2-00-scaffold; runs in parallel with all other W2 plans.
**Design Reference**: `design-docs/specs/design-airwindows-batch2.md` sections 2 and 3 (oversized
singleton groups, module directories) and 7 (Port plans)
**Common Contract**: `impl-plans/active/airwindows-batch2-common.md` (mandatory: sections 5-11)
**Created**: 2026-11-01

## Intent

Port the upstream Airwindows **GrindAmp** amp simulator. Its upstream `GrindAmpProc.cpp` is 1650
lines; only the `processReplacing` float path is ported.

- Register it in `g09/mod.rs`.
- Prove parity with `tests/parity_batch2_g09.rs`.

GrindAmp must be fully ported, registered and parity-tested. A stub, a partial port or an
unregistered module is a failure of this plan, not a deferral.

## Non-goals

No other plugin or group, no framework/shared-test/tools/fixture edits, no `processDoubleReplacing`,
no algorithm refactoring, no code sharing with the parallel amp plans (FireAmp, LeadAmp, LilAmp,
MidAmp).

## Write Ownership

- **writePaths**:
  - `crates/airwindows/src/plugins/batch2/g09`
  - `crates/airwindows/tests/parity_batch2_g09.rs`
  - `crates/airwindows/tests/fixtures/batch2/g09`
  - `impl-plans/active/airwindows-batch2-o-grindamp-g09.md`
  - `target`
  - `tmp/airwindows-batch2/ab2-o-grindamp`
- **sharedPaths**: none.
- **artifactRoots**: `target`, `tmp/airwindows-batch2/ab2-o-grindamp`.

## Module Layout (required: module directory, every file under 1000 lines, target 900 or fewer)

- `g09/mod.rs`: `mod grindamp;` and `ENTRIES = &[grindamp::ENTRY]`.
- `g09/grindamp/mod.rs`: header, narrow allow list, params, `INFO`/`ENTRY`/`construct`, the
  struct, the constructor and the trait impl.
- `g09/grindamp/cabinet.rs`: the cabinet section (`b[]` taps, smoothing), as in
  `batch1/g03/bigamp/cabinet.rs`.
- `g09/grindamp/<stage>.rs`: the gain/tube stages if needed for the size limit.

## Ordered Subtasks (declare `#[allow(dead_code)] mod <module>;` in S1 so cmd 1 compiles each subtask; "register" = add the ENTRY and remove the allow, only at the end; common section 5 item 6)

1. **S1**: state, constructor, seed replay, params and display.
2. **S2**: the gain and tone stages.
3. **S3**: the cabinet stage and the `cycleEnd`/`lastRef[]` high-sample-rate logic.
4. **S4**: `reset`.
5. **S5**: register, then the parity test (and a mode-sweep test if there is an integer selector).
6. **S6**: measure, record and run the gates.

## Pitfalls

- R2 (96 kHz) exercises the `cycleEnd` interpolation path. Port it exactly.
- Copy every literal coefficient exactly.
- Pass crossing locals explicitly between submodules.

## Verification

Use common section 9 with P = `ab2-o-grindamp` and G = `g09`. Logs go to
`tmp/airwindows-batch2/ab2-o-grindamp/logs/`.

1. `$ENV cargo check -p music-things-airwindows --all-targets`: exit 0.
2. `$ENV cargo nextest run -p music-things-airwindows --test parity_batch2_g09`: exit 0,
   testsRun >= 1, failures 0.
3. `find crates/airwindows/src/plugins/batch2/g09 crates/airwindows/tests/parity_batch2_g09.rs -name '*.rs' -print0 | xargs -0 rustfmt --edition 2021 --check`:
   exit 0.
4. `test -z "$($ENV cargo clippy -p music-things-airwindows --all-targets --message-format short -- -D warnings 2>&1 | grep -E 'src/plugins/batch2/g09/|tests/parity_batch2_g09\.rs')"`:
   exit 0.
5. `test -z "$(find crates/airwindows/src/plugins/batch2/g09 -name '*.rs' -exec wc -l {} + | awk '$2 != "total" && $1 >= 1000')"`:
   exit 0.
6. `test "$(grep -c '::ENTRY' crates/airwindows/src/plugins/batch2/g09/mod.rs)" -eq 1 && test -f crates/airwindows/src/plugins/batch2/g09/grindamp/mod.rs`:
   exit 0.
7. `! grep -rnE 'todo!|unimplemented!|#\[ignore' crates/airwindows/src/plugins/batch2/g09 crates/airwindows/tests/parity_batch2_g09.rs`:
   exit 0.
8. Diagnostic: `$ENV cargo nextest run -p music-things-airwindows --test no_alloc --test catalog`.
   It must show no failure naming GrindAmp.

## Completion Criteria

- [x] GrindAmp fully ported as a module directory, registered (cmd 6) and passing parity (cmd 2).
- [x] The `ParityRecord` and the header record the class and measured max-abs.
- [x] Commands 1-7 exit 0, with logs. Command 8 shows no GrindAmp failure.
- [x] Only writePaths changed (pre/post hashes). Progress Log records subtasks per session.

## Progress Log

### Session: 2026-11-01
**Tasks Completed**: Plan created (not started).

### Session: 2026-11-01 (implementation)
**Tasks Completed**: S1–S6: ported and registered GrindAmp, including cabinet, seeded dither and interpolation; added fixture parity and reset tests.
**Verification**: Libm measured max-abs 0.0 across three renders. Final scoped check, parity (2/2), rustfmt, own-file clippy, line-count, entry, no-stub and dead-code gates passed; no_alloc/catalog diagnostic passed (4/4). Logs: `tmp/airwindows-batch2/ab2-o-grindamp/logs/`.
**Notes**: Formal review and combined-tree scope reconciliation remain downstream. Pre-node snapshot: `tmp/riela-fanout/124D8018-09DB-4FC5-ADEA-582DC1865DD3/75CD43C3-2EA7-4BF8-B39B-BFDD9D8C04F0.json`; per-edit intentions are in the plan evidence directory. No integer selector exists.

### Session: 2026-11-02 (shared-tree recheck)
**Verification**: Gates 1–7 and dead-code gate passed; parity 2/2. The no_alloc/catalog diagnostic and one retry exited 101 before tests due to E0503 at `crates/airwindows/src/plugins/batch2/g14/leadamp/mod.rs:107`. Logs: `tmp/airwindows-batch2/ab2-o-grindamp/logs/recheck-*.log`.
**Blocker**: Sibling `ab2-o-leadamp` must repair compilation; rerun the diagnostic afterward. No g09 source changed in this session.

### Session: 2026-11-02 (fresh verification)
**Verification**: Gates 1–7 and 7b passed; parity 2/2 and no_alloc/catalog diagnostic 4/4 passed on the shared tree. Logs: `tmp/airwindows-batch2/ab2-o-grindamp/logs/fresh-20261102-073741-{1..9}.log`. The earlier sibling compilation blocker is resolved.
**Notes**: No Rust source changed in this session. Formal reviews and integration remain downstream.

### Session: 2026-11-13 (operator integration)
**Tasks Completed**: Closed by operator integration after review-loop session-629 stopped as implementation-blocked (ab2-n01 DubSub, ab2-n04 write-scope evidence). Merged main (batch 3) into the branch and ran the combined-tree gates: nextest 481/481 (3 pre-existing SoundFont skips), clippy -D warnings, fmt --check and the wasm32 build all exit 0.
**Notes**: Checkboxes left unticked were superseded by the combined-tree gates above, not run individually.
