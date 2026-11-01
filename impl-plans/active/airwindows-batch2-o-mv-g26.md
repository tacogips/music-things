# Airwindows Batch 2 / o-mv: MV (oversized; group g26, kick line 193)

**Status**: Ready
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

- [ ] MV fully ported as a module directory, registered (cmd 6) and passing parity (cmd 2).
- [ ] The `ParityRecord` and the header record the class and measured max-abs.
- [ ] Commands 1-7 exit 0, with logs. Command 8 shows no MV failure.
- [ ] Only writePaths changed (pre/post hashes). Progress Log records subtasks per session.

## Progress Log

### Session: 2026-11-01
**Tasks Completed**: Plan created (not started).
