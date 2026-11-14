# Airwindows Batch 2 / o-lilamp: LilAmp (oversized; group g16, kick line 174)

**Status**: Completed
**Plan ID**: ab2-o-lilamp
**Wave**: W2. Depends on ab2-00-scaffold; runs in parallel with all other W2 plans.
**Design Reference**: `design-docs/specs/design-airwindows-batch2.md` sections 2 and 3 (oversized
singleton groups, module directories) and 7 (Port plans)
**Common Contract**: `impl-plans/active/airwindows-batch2-common.md` (mandatory: sections 5-11)
**Created**: 2026-11-01

## Intent

Port the upstream Airwindows **LilAmp** amp simulator. Its upstream `LilAmpProc.cpp` is 1204
lines; only the `processReplacing` float path is ported.

- Register it in `g16/mod.rs`.
- Prove parity with `tests/parity_batch2_g16.rs`.

LilAmp must be fully ported, registered and parity-tested. A stub, a partial port or an
unregistered module is a failure of this plan, not a deferral.

## Non-goals

No other plugin or group, no framework/shared-test/tools/fixture edits, no `processDoubleReplacing`,
no algorithm refactoring, no code sharing with the parallel amp plans.

## Write Ownership

- **writePaths**:
  - `crates/airwindows/src/plugins/batch2/g16`
  - `crates/airwindows/tests/parity_batch2_g16.rs`
  - `crates/airwindows/tests/fixtures/batch2/g16`
  - `impl-plans/active/airwindows-batch2-o-lilamp-g16.md`
  - `target`
  - `tmp/airwindows-batch2/ab2-o-lilamp`
- **sharedPaths**: none.
- **artifactRoots**: `target`, `tmp/airwindows-batch2/ab2-o-lilamp`.

## Module Layout (required: module directory, every file under 1000 lines, target 900 or fewer)

- `g16/mod.rs`: `mod lilamp;` and `ENTRIES = &[lilamp::ENTRY]`.
- `g16/lilamp/mod.rs`: header, narrow allow list, params, `INFO`/`ENTRY`/`construct`, the struct,
  the constructor and the trait impl.
- `g16/lilamp/cabinet.rs`: the cabinet section, as in `batch1/g03/bigamp/cabinet.rs`.

## Ordered Subtasks (declare `#[allow(dead_code)] mod <module>;` in S1 so cmd 1 compiles each subtask; "register" = add the ENTRY and remove the allow, only at the end; common section 5 item 6)

1. **S1**: state, constructor, seed replay, params and display.
2. **S2**: the gain and tone stages.
3. **S3**: the cabinet stage and the high-sample-rate `cycleEnd` logic.
4. **S4**: `reset`.
5. **S5**: register, then the parity test (and a mode-sweep test if there is an integer selector).
6. **S6**: measure, record and run the gates.

## Pitfalls

- R2 (96 kHz) exercises the `cycleEnd` path.
- Copy every literal coefficient exactly.
- Pass crossing locals explicitly.

## Verification

Use common section 9 with P = `ab2-o-lilamp` and G = `g16`. Logs go to
`tmp/airwindows-batch2/ab2-o-lilamp/logs/`.

1. `$ENV cargo check -p music-things-airwindows --all-targets`: exit 0.
2. `$ENV cargo nextest run -p music-things-airwindows --test parity_batch2_g16`: exit 0,
   testsRun >= 1, failures 0.
3. `find crates/airwindows/src/plugins/batch2/g16 crates/airwindows/tests/parity_batch2_g16.rs -name '*.rs' -print0 | xargs -0 rustfmt --edition 2021 --check`:
   exit 0.
4. `test -z "$($ENV cargo clippy -p music-things-airwindows --all-targets --message-format short -- -D warnings 2>&1 | grep -E 'src/plugins/batch2/g16/|tests/parity_batch2_g16\.rs')"`:
   exit 0.
5. `test -z "$(find crates/airwindows/src/plugins/batch2/g16 -name '*.rs' -exec wc -l {} + | awk '$2 != "total" && $1 >= 1000')"`:
   exit 0.
6. `test "$(grep -c '::ENTRY' crates/airwindows/src/plugins/batch2/g16/mod.rs)" -eq 1 && test -f crates/airwindows/src/plugins/batch2/g16/lilamp/mod.rs`:
   exit 0.
7. `! grep -rnE 'todo!|unimplemented!|#\[ignore' crates/airwindows/src/plugins/batch2/g16 crates/airwindows/tests/parity_batch2_g16.rs`:
   exit 0.
8. Diagnostic: `$ENV cargo nextest run -p music-things-airwindows --test no_alloc --test catalog`.
   It must show no failure naming LilAmp.

## Completion Criteria

- [x] LilAmp fully ported as a module directory, registered (cmd 6) and passing parity (cmd 2).
- [x] The `ParityRecord` and the header record the class and measured max-abs.
- [x] Commands 1-7 exit 0, with logs. Command 8 shows no LilAmp failure.
- [x] Only writePaths changed (pre/post hashes). Progress Log records subtasks per session.

## Progress Log

### Session: 2026-11-01
**Tasks Completed**: Plan created (not started).

### Session: 2026-11-01 — implementation
**Tasks Completed**: S1–S6. Ported and registered LilAmp; float fixture parity measured Libm max-abs 0.0 on arm64. Final scoped gates and no_alloc/catalog diagnostic passed; logs are under `tmp/airwindows-batch2/ab2-o-lilamp/logs/`. Pre-node and post hashes are recorded in `logs/scope.log`; serial shared-tree reconciliation and formal review remain pending.

### Session: 2026-11-02 — verification and provenance
Added the pinned upstream revision to the cabinet submodule header. LilAmp remains registered and
its Libm measured max-abs remains 0.0; no plugins remain in this plan. Fresh gates 1, 3–7 and 7b
exited 0 (`logs/fresh-{1,3,4,5,6,7,7b}.log`); gate 2 exited 0 with 3/3 tests passing
(`logs/fresh-2-retry.log`). Diagnostic gate 8 exited 0 with 4/4 tests passing
(`logs/fresh-8-retry.log`). Initial gates 2 and 8 failed to compile a concurrently edited sibling
`g14/leadamp/mod.rs:107` (logs `fresh-2.log`, `fresh-8.log`); both passed after the sibling fix.
All log paths are under `tmp/airwindows-batch2/ab2-o-lilamp/`. Formal review and serial
shared-tree reconciliation remain downstream.

### Session: 2026-11-13 (operator integration)
**Tasks Completed**: Closed by operator integration after review-loop session-629 stopped as implementation-blocked (ab2-n01 DubSub, ab2-n04 write-scope evidence). Merged main (batch 3) into the branch and ran the combined-tree gates: nextest 481/481 (3 pre-existing SoundFont skips), clippy -D warnings, fmt --check and the wasm32 build all exit 0.
**Notes**: Checkboxes left unticked were superseded by the combined-tree gates above, not run individually.
