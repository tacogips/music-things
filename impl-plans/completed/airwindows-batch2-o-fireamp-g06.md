# Airwindows Batch 2 / o-fireamp: FireAmp (oversized; group g06, kick line 134)

**Status**: Completed
**Plan ID**: ab2-o-fireamp
**Wave**: W2. Depends on ab2-00-scaffold; runs in parallel with all other W2 plans.
**Design Reference**: `design-docs/specs/design-airwindows-batch2.md` sections 2 and 3 (oversized
singleton groups, module directories) and 7 (Port plans)
**Common Contract**: `impl-plans/active/airwindows-batch2-common.md` (mandatory: sections 5-11)
**Created**: 2026-11-01

## Intent

Port the upstream Airwindows **FireAmp** amp simulator. Its upstream `FireAmpProc.cpp` is 1707
lines; only the `processReplacing` float path is ported.

- Register it in `g06/mod.rs`.
- Prove parity with `tests/parity_batch2_g06.rs`.

FireAmp must be fully ported, registered and parity-tested. A stub, a partial port or an
unregistered module is a failure of this plan, not a deferral. The batch-1 sibling BigAmp, in the
same amp family, was left as a stub; do not repeat that.

## Non-goals

No other plugin or group, no framework/shared-test/tools/fixture edits, no `processDoubleReplacing`,
no algorithm refactoring. Do not import code from `batch1/g03/bigamp` (read it only as a pattern).

## Write Ownership

- **writePaths**:
  - `crates/airwindows/src/plugins/batch2/g06`
  - `crates/airwindows/tests/parity_batch2_g06.rs`
  - `crates/airwindows/tests/fixtures/batch2/g06`
  - `impl-plans/active/airwindows-batch2-o-fireamp-g06.md`
  - `target`
  - `tmp/airwindows-batch2/ab2-o-fireamp`
- **sharedPaths**: none.
- **artifactRoots**: `target`, `tmp/airwindows-batch2/ab2-o-fireamp`.

## Module Layout (required: module directory, every file under 1000 lines, target 900 or fewer)

- `g06/mod.rs`: `mod fireamp;` and `ENTRIES = &[fireamp::ENTRY]`.
- `g06/fireamp/mod.rs`: header, narrow allow list, params, `INFO`/`ENTRY`/`construct`, the struct,
  the constructor and the trait impl.
- `g06/fireamp/cabinet.rs`: the cabinet-emulation section (the long `b[]` delay/convolution taps
  and their smoothing), split like `batch1/g03/bigamp/cabinet.rs`.
- Further `g06/fireamp/<stage>.rs` files for the gain/tone/tube stages if `mod.rs` would exceed
  900 lines.

## Ordered Subtasks (declare `#[allow(dead_code)] mod <module>;` in S1 so cmd 1 compiles each subtask; "register" = add the ENTRY and remove the allow, only at the end; common section 5 item 6)

1. **S1**: state from `FireAmp.h` (including the `b[]` arrays, `Box`ed if over 4096 elements),
   constructor and seed replay, params and display.
2. **S2**: the input, gain and tone stages.
3. **S3**: the cabinet stage, including the sample-rate-dependent cycle/`cycleEnd` logic (an
   integer from `overallscale`).
4. **S4**: `reset`.
5. **S5**: register, then the parity test (and a mode-sweep test if there is an integer selector).
6. **S6**: measure, record and run the gates.

## Pitfalls

- The `cycleEnd` integer derivation and the `lastRef[]` interpolation for high sample rates are
  exercised by R2 (96 kHz). Port them exactly.
- The many `b[n]` tap coefficients are literals. Copy every digit; do not round.
- Pass crossing locals explicitly between submodules.

## Verification

Use common section 9 with P = `ab2-o-fireamp` and G = `g06`. Logs go to
`tmp/airwindows-batch2/ab2-o-fireamp/logs/`.

1. `$ENV cargo check -p music-things-airwindows --all-targets`: exit 0.
2. `$ENV cargo nextest run -p music-things-airwindows --test parity_batch2_g06`: exit 0,
   testsRun >= 1, failures 0.
3. `find crates/airwindows/src/plugins/batch2/g06 crates/airwindows/tests/parity_batch2_g06.rs -name '*.rs' -print0 | xargs -0 rustfmt --edition 2021 --check`:
   exit 0.
4. `test -z "$($ENV cargo clippy -p music-things-airwindows --all-targets --message-format short -- -D warnings 2>&1 | grep -E 'src/plugins/batch2/g06/|tests/parity_batch2_g06\.rs')"`:
   exit 0.
5. `test -z "$(find crates/airwindows/src/plugins/batch2/g06 -name '*.rs' -exec wc -l {} + | awk '$2 != "total" && $1 >= 1000')"`:
   exit 0.
6. `test "$(grep -c '::ENTRY' crates/airwindows/src/plugins/batch2/g06/mod.rs)" -eq 1 && test -f crates/airwindows/src/plugins/batch2/g06/fireamp/mod.rs`:
   exit 0.
7. `! grep -rnE 'todo!|unimplemented!|#\[ignore' crates/airwindows/src/plugins/batch2/g06 crates/airwindows/tests/parity_batch2_g06.rs`:
   exit 0.
8. Diagnostic: `$ENV cargo nextest run -p music-things-airwindows --test no_alloc --test catalog`.
   It must show no failure naming FireAmp.

## Completion Criteria

- [x] FireAmp fully ported as a module directory, registered and passing initial parity.
- [x] The `ParityRecord` and the header record Libm and measured max-abs 0.0 on the acceptance host.
- [x] Commands 1-7 and the dead-code gate exit 0; diagnostic command 8 passes all 4 tests.
- [x] Author edits are within writePaths; pre/post owned hashes and per-edit intentions are recorded. Shared-tree sibling changes require serial-join reconciliation.

## Progress Log

### Session: 2026-11-01
**Tasks Completed**: Plan created (not started).

### Session: 2026-11-01 (implementation)
**Tasks Completed**: S1-S5: state, parameters, float gain/tone processing, cabinet and cycle interpolation, reset and registration; S6 measurement and initial parity.
**Evidence**: `tmp/airwindows-batch2/ab2-o-fireamp/logs/parity-measure.log` (2 passing tests before the reset-preservation addition); `logs/measured-error.log` records maximum 0.0 from three renders. Final-source logs: `logs/check-agent-final2.log`, `logs/parity-agent-final2.log` (3 passed), `logs/rustfmt-agent-final2.log`, `logs/clippy-agent-final2.log`, `logs/lines-agent-final2.log`, `logs/entry-agent-final2.log`, `logs/stub-agent-final2.log`, `logs/dead-code-agent-final2.log`, and `logs/diagnostic-agent-final2.log` (4 passed). Formal review remains downstream.

**Scope**: `tmp/airwindows-batch2/ab2-o-fireamp/scope-hashes.json` records the pre-node snapshot and post-edit owned hashes; `intents/001-cabinet.txt` through `intents/013-plan-scope.txt` record edits. The shared tree contains concurrent sibling work and is not a single-worker global diff.

### Session: 2026-11-02 (verification)
**Tasks Completed**: Rechecked the existing implementation and reran the plan-local gates. Owned source hashes match `scope-hashes.json`; no Rust source was changed in this session.
**Evidence**: `logs/redispatch-check-retry.log` passed; `logs/redispatch-parity-retry.log` passed 3 tests; `logs/redispatch-diagnostic-retry.log` passed 4 tests. The format, own-file clippy, line count, entry, stub and dead-code checks passed in `logs/redispatch-{fmt,clippy,lines,entry,stub,dead}.log`. Initial parity and diagnostic attempts encountered a transient sibling g14 compile error; the bounded retries passed after that sibling changed. These checks ran on a moving shared tree; formal review and serial integration remain downstream.

### Session: 2026-11-13 (operator integration)
**Tasks Completed**: Closed by operator integration after review-loop session-629 stopped as implementation-blocked (ab2-n01 DubSub, ab2-n04 write-scope evidence). Merged main (batch 3) into the branch and ran the combined-tree gates: nextest 481/481 (3 pre-existing SoundFont skips), clippy -D warnings, fmt --check and the wasm32 build all exit 0.
**Notes**: Checkboxes left unticked were superseded by the combined-tree gates above, not run individually.
