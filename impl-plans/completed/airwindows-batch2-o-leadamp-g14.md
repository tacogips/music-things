# Airwindows Batch 2 / o-leadamp: LeadAmp (oversized; group g14, kick line 172)

**Status**: Completed
**Plan ID**: ab2-o-leadamp
**Wave**: W2. Depends on ab2-00-scaffold; runs in parallel with all other W2 plans.
**Design Reference**: `design-docs/specs/design-airwindows-batch2.md` sections 2 and 3 (oversized
singleton groups, module directories) and 7 (Port plans)
**Common Contract**: `impl-plans/active/airwindows-batch2-common.md` (mandatory: sections 5-11)
**Created**: 2026-11-01

## Intent

Port the upstream Airwindows **LeadAmp** amp simulator. Its upstream `LeadAmpProc.cpp` is 1770
lines; only the `processReplacing` float path is ported.

- Register it in `g14/mod.rs`.
- Prove parity with `tests/parity_batch2_g14.rs`.

LeadAmp must be fully ported, registered and parity-tested. A stub, a partial port or an
unregistered module is a failure of this plan, not a deferral.

## Non-goals

No other plugin or group, no framework/shared-test/tools/fixture edits, no `processDoubleReplacing`,
no algorithm refactoring, no code sharing with the parallel amp plans.

## Write Ownership

- **writePaths**:
  - `crates/airwindows/src/plugins/batch2/g14`
  - `crates/airwindows/tests/parity_batch2_g14.rs`
  - `crates/airwindows/tests/fixtures/batch2/g14`
  - `impl-plans/active/airwindows-batch2-o-leadamp-g14.md`
  - `target`
  - `tmp/airwindows-batch2/ab2-o-leadamp`
- **sharedPaths**: none.
- **artifactRoots**: `target`, `tmp/airwindows-batch2/ab2-o-leadamp`.

## Module Layout (required: module directory, every file under 1000 lines, target 900 or fewer)

- `g14/mod.rs`: `mod leadamp;` and `ENTRIES = &[leadamp::ENTRY]`.
- `g14/leadamp/mod.rs`: header, narrow allow list, params, `INFO`/`ENTRY`/`construct`, the struct,
  the constructor and the trait impl.
- `g14/leadamp/cabinet.rs`: the cabinet section, as in `batch1/g03/bigamp/cabinet.rs`.
- `g14/leadamp/<stage>.rs`: the gain/tube stages as needed for the size limit.

## Ordered Subtasks (declare `#[allow(dead_code)] mod <module>;` in S1 so cmd 1 compiles each subtask; "register" = add the ENTRY and remove the allow, only at the end; common section 5 item 6)

1. **S1**: state, constructor, seed replay, params and display.
2. **S2**: the gain and tone stages.
3. **S3**: the cabinet stage and the `cycleEnd`/`lastRef[]` high-sample-rate logic.
4. **S4**: `reset`.
5. **S5**: register, then the parity test (and a mode-sweep test if there is an integer selector).
6. **S6**: measure, record and run the gates.

## Pitfalls

- R2 (96 kHz) exercises the `cycleEnd` interpolation. Port it exactly.
- Copy every literal coefficient exactly.
- Pass crossing locals explicitly. This is the largest amp port (1770 lines), so plan for at least
  three source files.

## Verification

Use common section 9 with P = `ab2-o-leadamp` and G = `g14`. Logs go to
`tmp/airwindows-batch2/ab2-o-leadamp/logs/`.

1. `$ENV cargo check -p music-things-airwindows --all-targets`: exit 0.
2. `$ENV cargo nextest run -p music-things-airwindows --test parity_batch2_g14`: exit 0,
   testsRun >= 1, failures 0.
3. `find crates/airwindows/src/plugins/batch2/g14 crates/airwindows/tests/parity_batch2_g14.rs -name '*.rs' -print0 | xargs -0 rustfmt --edition 2021 --check`:
   exit 0.
4. `test -z "$($ENV cargo clippy -p music-things-airwindows --all-targets --message-format short -- -D warnings 2>&1 | grep -E 'src/plugins/batch2/g14/|tests/parity_batch2_g14\.rs')"`:
   exit 0.
5. `test -z "$(find crates/airwindows/src/plugins/batch2/g14 -name '*.rs' -exec wc -l {} + | awk '$2 != "total" && $1 >= 1000')"`:
   exit 0.
6. `test "$(grep -c '::ENTRY' crates/airwindows/src/plugins/batch2/g14/mod.rs)" -eq 1 && test -f crates/airwindows/src/plugins/batch2/g14/leadamp/mod.rs`:
   exit 0.
7. `! grep -rnE 'todo!|unimplemented!|#\[ignore' crates/airwindows/src/plugins/batch2/g14 crates/airwindows/tests/parity_batch2_g14.rs`:
   exit 0.
8. Diagnostic: `$ENV cargo nextest run -p music-things-airwindows --test no_alloc --test catalog`.
   It must show no failure naming LeadAmp.

## Completion Criteria

- [x] LeadAmp fully ported as a module directory, registered (cmd 6) and passing parity (cmd 2).
- [x] The `ParityRecord` and the header record the class and measured max-abs.
- [x] Commands 1-7 exit 0, with logs. Command 8 shows no LeadAmp failure.
- [x] Only writePaths changed (pre/post hashes). Progress Log records subtasks per session.

## Progress Log

### Session: 2026-11-01
**Tasks Completed**: Plan created (not started).

### Session: 2026-11-01 (implementation attempt)
**Tasks Completed**: Partial S1/S3 helpers only: state, cabinet convolution and interpolation were drafted in `g14/leadamp/`. They are unconnected and unverified as an effect.
**Remaining**: Complete the gain and tone stages, processing loop, trait implementation, reset, registration, parity test and all required gates. No parity measurement has been made. The plan remains incomplete.

### Session: 2026-11-02 (implementation)
**Tasks Completed**: S1-S6: completed LeadAmp state, gain/tone and cabinet processing, interpolation, reset, registration and fixture parity. The Libm max-abs was 0.0 across all three renders on the acceptance host.
**Verification**: Gates 1-7 and 7b passed; parity 1/1 and diagnostic no_alloc/catalog 4/4 passed. Complete logs are under `tmp/airwindows-batch2/ab2-o-leadamp/logs/final-*.log`. Earlier metadata and lint issues were corrected; prior logs are retained. Edit intentions and snapshots are under `tmp/airwindows-batch2/ab2-o-leadamp/intent/` and `edit-intents/`.
**Remaining**: Formal review and workflow finalization are downstream.

### Session: 2026-11-13 (operator integration)
**Tasks Completed**: Closed by operator integration after review-loop session-629 stopped as implementation-blocked (ab2-n01 DubSub, ab2-n04 write-scope evidence). Merged main (batch 3) into the branch and ran the combined-tree gates: nextest 481/481 (3 pre-existing SoundFont skips), clippy -D warnings, fmt --check and the wasm32 build all exit 0.
**Notes**: Checkboxes left unticked were superseded by the combined-tree gates above, not run individually.
