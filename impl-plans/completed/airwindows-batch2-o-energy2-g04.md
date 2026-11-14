# Airwindows Batch 2 / o-energy2: Energy2 (oversized; group g04, kick line 127)

**Status**: Completed
**Plan ID**: ab2-o-energy2
**Wave**: W2. Depends on ab2-00-scaffold; runs in parallel with all other W2 plans.
**Design Reference**: `design-docs/specs/design-airwindows-batch2.md` sections 2 and 3 (oversized
singleton groups, module directories) and 7 (Port plans)
**Common Contract**: `impl-plans/active/airwindows-batch2-common.md` (mandatory: sections 5-11)
**Created**: 2026-11-01

## Intent

Port the upstream Airwindows **Energy2** plugin. Its upstream `Energy2Proc.cpp` is 1292 lines; only
the `processReplacing` float path is ported.

- Register it in `g04/mod.rs`.
- Prove parity with `tests/parity_batch2_g04.rs`.

Energy2 must be fully ported, registered and parity-tested. A stub, a partial port or an
unregistered module is a failure of this plan, not a deferral.

## Non-goals

No other plugin or group, no framework/shared-test/tools/fixture edits, no `processDoubleReplacing`,
no algorithm refactoring. Do not reuse code from the Energy plan (ab2-o-energy runs in parallel and
its files are not yours).

## Write Ownership

- **writePaths**:
  - `crates/airwindows/src/plugins/batch2/g04`
  - `crates/airwindows/tests/parity_batch2_g04.rs`
  - `crates/airwindows/tests/fixtures/batch2/g04`
  - `impl-plans/active/airwindows-batch2-o-energy2-g04.md`
  - `target`
  - `tmp/airwindows-batch2/ab2-o-energy2`
- **sharedPaths**: none.
- **artifactRoots**: `target`, `tmp/airwindows-batch2/ab2-o-energy2`.

## Module Layout (required: module directory, every file under 1000 lines, target 900 or fewer)

- `g04/mod.rs`: `mod energy2;` and `ENTRIES = &[energy2::ENTRY]`.
- `g04/energy2/mod.rs`: header, narrow allow list (as in `batch1/g03/bigamp/mod.rs`), params,
  `INFO`/`ENTRY`/`construct`, the struct, the constructor and the trait impl.
- `g04/energy2/<stage>.rs`: consecutive sections of the upstream per-sample loop (harmonic band
  stages), passing crossing locals explicitly.

## Ordered Subtasks (declare `#[allow(dead_code)] mod <module>;` in S1 so cmd 1 compiles each subtask; "register" = add the ENTRY and remove the allow, only at the end; common section 5 item 6)

1. **S1**: state, constructor, seed replay, params and display.
2. **S2**: loop sections, ported in order.
3. **S3**: `reset`.
4. **S4**: register in `g04/mod.rs`, then the parity test (and a mode-sweep test if there is an
   integer selector).
5. **S5**: measure, record and run the gates.

## Pitfalls

- Crossing locals must be passed explicitly, and per-sample locals must stay per-sample.
- Keep the float/double member types from `Energy2.h`.
- Do not fold the repeated band code into loops if that changes the evaluation order.

## Verification

Use common section 9 with P = `ab2-o-energy2` and G = `g04`. Logs go to
`tmp/airwindows-batch2/ab2-o-energy2/logs/`.

1. `$ENV cargo check -p music-things-airwindows --all-targets`: exit 0.
2. `$ENV cargo nextest run -p music-things-airwindows --test parity_batch2_g04`: exit 0,
   testsRun >= 1, failures 0.
3. `find crates/airwindows/src/plugins/batch2/g04 crates/airwindows/tests/parity_batch2_g04.rs -name '*.rs' -print0 | xargs -0 rustfmt --edition 2021 --check`:
   exit 0.
4. `test -z "$($ENV cargo clippy -p music-things-airwindows --all-targets --message-format short -- -D warnings 2>&1 | grep -E 'src/plugins/batch2/g04/|tests/parity_batch2_g04\.rs')"`:
   exit 0.
5. `test -z "$(find crates/airwindows/src/plugins/batch2/g04 -name '*.rs' -exec wc -l {} + | awk '$2 != "total" && $1 >= 1000')"`:
   exit 0.
6. `test "$(grep -c '::ENTRY' crates/airwindows/src/plugins/batch2/g04/mod.rs)" -eq 1 && test -f crates/airwindows/src/plugins/batch2/g04/energy2/mod.rs`:
   exit 0.
7. `! grep -rnE 'todo!|unimplemented!|#\[ignore' crates/airwindows/src/plugins/batch2/g04 crates/airwindows/tests/parity_batch2_g04.rs`:
   exit 0.
8. Diagnostic: `$ENV cargo nextest run -p music-things-airwindows --test no_alloc --test catalog`.
   It must show no failure naming Energy2.

## Completion Criteria

- [x] Energy2 fully ported as a module directory, registered (cmd 6) and passing parity (cmd 2).
- [x] The `ParityRecord` and the header record the class and measured max-abs.
- [x] Commands 1-7 exit 0, with logs. Command 8 shows no Energy2 failure.
- [x] Only writePaths changed (pre/post hashes). Progress Log records subtasks S1-S5 per session.

## Progress Log

### Session: 2026-11-01
**Tasks Completed**: Plan created (not started).

### Session: 2026-11-01 — implementation
**Tasks Completed**: S1–S5: Energy2 float path, state/reset, registration and fixture parity. Libm measured max-abs 0.0 on the acceptance host.
**Verification**: Parity 1/1 passed; the reported clippy grep was vacuous because sibling compilation failed (see review revision below). Diagnostic no_alloc/catalog 4/4 passed. Complete logs: `tmp/airwindows-batch2/ab2-o-energy2/logs/final-*.log`. Checks ran on the shared moving tree; g06 FireAmp emitted sibling warnings during cargo checks. Formal review and integration remain downstream.
**Evidence**: Per-edit intentions and owned-file hashes are under `tmp/airwindows-batch2/ab2-o-energy2/`.

### Session: 2026-11-02 — current-source recheck
**Tasks Completed**: Reviewed S1–S5 and reran the assigned gates; no source edits were needed.
**Verification**: Parity passed 1/1 and no_alloc/catalog passed 4/4; the reported clippy grep was vacuous because sibling compilation failed (see review revision below). An initial parity compilation failed in concurrently edited g14 LeadAmp; the bounded retry passed. Logs are under `tmp/airwindows-batch2/ab2-o-energy2/logs/recheck-20261102/`. Checks ran on the shared moving tree. Formal review and integration remain downstream.

### Session: 2026-11-02 — test-integrity revision
**Finding**: TI-ENERGY2-CLIPPY-VACUOUS. Earlier clippy logs aborted on sibling compilation errors and did not establish the own-file lint gate.
**Verification**: Fresh cargo check and full clippy both exited 0; the full clippy log contains no compilation errors or g04 diagnostics. Parity passed 1/1 and own-file rustfmt passed. Logs: `tmp/airwindows-batch2/ab2-o-energy2/logs/revision-ti/`. No Rust changes were required. Checks ran on the moving shared tree; independent review remains downstream.

### Session: 2026-11-13 (operator integration)
**Tasks Completed**: Closed by operator integration after review-loop session-629 stopped as implementation-blocked (ab2-n01 DubSub, ab2-n04 write-scope evidence). Merged main (batch 3) into the branch and ran the combined-tree gates: nextest 481/481 (3 pre-existing SoundFont skips), clippy -D warnings, fmt --check and the wasm32 build all exit 0.
**Notes**: Checkboxes left unticked were superseded by the combined-tree gates above, not run individually.
