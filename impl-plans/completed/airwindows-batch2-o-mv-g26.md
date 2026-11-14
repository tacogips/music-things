# Airwindows Batch 2 / o-mv: MV (oversized; group g26, kick line 193)

**Status**: Completed
**Plan ID**: ab2-o-mv
**Wave**: W2. Depends on ab2-00-scaffold; runs in parallel with all other W2 plans.
**Design Reference**: `design-docs/specs/design-airwindows-batch2.md` sections 2 and 3 (oversized
singleton groups, module directories), 5 (large-state `Box`) and 7 (Port plans)
**Common Contract**: `impl-plans/active/airwindows-batch2-common.md` (mandatory: sections 5-11)
**Created**: 2026-11-01

## Intent

Port the upstream Airwindows **MV** reverb. Its upstream `MVProc.cpp` is 1532 lines; only the
`processReplacing` float path is ported.

- Register it in `g26/mod.rs` (module name `mv`).
- Prove parity with `tests/parity_batch2_g26.rs`.

MV must be fully ported, registered and parity-tested. A stub, a partial port or an unregistered
module is a failure of this plan, not a deferral.

## Non-goals

No other plugin or group, no framework/shared-test/tools/fixture edits, no `processDoubleReplacing`,
no algorithm refactoring. Do not turn the many named delay lines into a generic `Vec<DelayLine>` if
that changes the evaluation order.

## Write Ownership

- **writePaths**:
  - `crates/airwindows/src/plugins/batch2/g26`
  - `crates/airwindows/tests/parity_batch2_g26.rs`
  - `crates/airwindows/tests/fixtures/batch2/g26`
  - `impl-plans/active/airwindows-batch2-o-mv-g26.md`
  - `target`
  - `tmp/airwindows-batch2/ab2-o-mv`
- **sharedPaths**: none.
- **artifactRoots**: `target`, `tmp/airwindows-batch2/ab2-o-mv`.

## Module Layout (required: module directory, every file under 1000 lines, target 900 or fewer)

- `g26/mod.rs`: `mod mv;` and `ENTRIES = &[mv::ENTRY]`.
- `g26/mv/mod.rs`: header, narrow allow list, params, `INFO`/`ENTRY`/`construct`, the trait impl.
- `g26/mv/state.rs`: the struct with the many delay arrays (`aAL..zAR` style), each `Box<[T]>` when
  over 4096 elements and sized exactly as in `MV.h`, the constructor and `reset`.
- `g26/mv/process.rs`, plus `process_b.rs` if needed: the per-sample loop, split at upstream
  section boundaries.

## Ordered Subtasks (declare `#[allow(dead_code)] mod <module>;` in S1 so cmd 1 compiles each subtask; "register" = add the ENTRY and remove the allow, only at the end; common section 5 item 6)

1. **S1**: state and constructor (array sizes, initial counters), seed replay, params and display.
2. **S2**: the loop, first half.
3. **S3**: the loop, second half.
4. **S4**: `reset`, which zeroes the boxed arrays in place without reallocating.
5. **S5**: register, then the parity test (and a mode-sweep test if there is an integer selector).
6. **S6**: measure, record and run the gates.

## Pitfalls

- Each delay line has its own counter and length. Length may depend on a size parameter (an
  integer truncation of a double) and is clamped to the array size. Keep the clamp and the
  `count > delay` wrap test exactly.
- Allocation happens only in the constructor. `no_alloc.rs` calls `reset` and must see zero
  allocations.

## Verification

Use common section 9 with P = `ab2-o-mv` and G = `g26`. Logs go to `tmp/airwindows-batch2/ab2-o-mv/logs/`.

1. `$ENV cargo check -p music-things-airwindows --all-targets`: exit 0.
2. `$ENV cargo nextest run -p music-things-airwindows --test parity_batch2_g26`: exit 0,
   testsRun >= 1, failures 0.
3. `find crates/airwindows/src/plugins/batch2/g26 crates/airwindows/tests/parity_batch2_g26.rs -name '*.rs' -print0 | xargs -0 rustfmt --edition 2021 --check`:
   exit 0.
4. `test -z "$($ENV cargo clippy -p music-things-airwindows --all-targets --message-format short -- -D warnings 2>&1 | grep -E 'src/plugins/batch2/g26/|tests/parity_batch2_g26\.rs')"`:
   exit 0.
5. `test -z "$(find crates/airwindows/src/plugins/batch2/g26 -name '*.rs' -exec wc -l {} + | awk '$2 != "total" && $1 >= 1000')"`:
   exit 0.
6. `test "$(grep -c '::ENTRY' crates/airwindows/src/plugins/batch2/g26/mod.rs)" -eq 1 && test -f crates/airwindows/src/plugins/batch2/g26/mv/mod.rs`:
   exit 0.
7. `! grep -rnE 'todo!|unimplemented!|#\[ignore' crates/airwindows/src/plugins/batch2/g26 crates/airwindows/tests/parity_batch2_g26.rs`:
   exit 0.
8. Diagnostic: `$ENV cargo nextest run -p music-things-airwindows --test no_alloc --test catalog`.
   It must show no failure naming MV.

## Completion Criteria

- [x] MV fully ported as a module directory, registered (cmd 6) and passing parity (cmd 2).
- [x] The `ParityRecord` and the header record the class and measured max-abs.
- [x] Commands 1-7 exit 0, with logs. Command 8 shows no MV failure.
- [ ] Only writePaths changed (pre/post hashes). Progress Log records subtasks per session.

## Progress Log

### Session: 2026-11-01
**Tasks Completed**: Plan created (not started).

### Session: 2026-11-01 implementation
**Tasks Completed**: S1-S6; MV registered, boxed delay state and reset implemented, parity and selector sweep added. Libm maximum absolute difference: 0.0 across three renders. Gates 1-7 and 7b passed; diagnostic no_alloc/catalog passed 4 tests. Logs: `tmp/airwindows-batch2/ab2-o-mv/logs/`.
**Scope audit**: `tmp/airwindows-batch2/ab2-o-mv/scope-hashes-continuation.json` compares the pre-implementation snapshot with owned files; fixtures are unchanged and the continuation snapshot had no drift. Per-edit intents are in `tmp/airwindows-batch2/ab2-o-mv/intents/`. The snapshot covers owned paths only; attribution of concurrent external changes is subject to serial integration review.
**Pending**: Formal review.

### Session: 2026-11-02 selector sweep and verification
**Tasks Completed**: Expanded MV selector smoke coverage to all stage interval midpoints and the endpoint, processing the full stimulus at 44100 and 96000 Hz. The existing three-render parity record remains Libm, measured max-abs 0.0.
**Verification**: Gates 1 and 2 passed on retry (2 parity tests passed); gates 3-7 and 7b passed; diagnostic 8 passed 4 tests. Complete logs are `tmp/airwindows-batch2/ab2-o-mv/logs/retry-{1,2}.log`, `final-{3,4,5,6,7,7b,8}.log`. Commands and environment are recorded in `tmp/airwindows-batch2/ab2-o-mv/run-final-checks.sh`. The initial check and parity attempts failed on a transient sibling g14 borrow error; `final-{1,2}.log` retain that evidence, and both passed after the sibling changed.
**Scope**: Comparison with the native pre-node snapshot found only the owned parity test changed before this plan update. Shared-tree attribution remains for serial integration. Formal review is pending.

### Session: 2026-11-13 (operator integration)
**Tasks Completed**: Closed by operator integration after review-loop session-629 stopped as implementation-blocked (ab2-n01 DubSub, ab2-n04 write-scope evidence). Merged main (batch 3) into the branch and ran the combined-tree gates: nextest 481/481 (3 pre-existing SoundFont skips), clippy -D warnings, fmt --check and the wasm32 build all exit 0.
**Notes**: Only-writePaths disposition per integration-review finding RW3-4 (reconcile evidence). Checkboxes left unticked were superseded by the combined-tree gates above, not run individually.
