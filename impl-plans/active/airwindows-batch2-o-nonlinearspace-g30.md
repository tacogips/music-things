# Airwindows Batch 2 / o-nonlinearspace: NonlinearSpace (oversized; group g30, kick line 200)

**Status**: Ready
**Plan ID**: ab2-o-nonlinearspace
**Wave**: W2. Depends on ab2-00-scaffold; runs in parallel with all other W2 plans.
**Design Reference**: `design-docs/specs/design-airwindows-batch2.md` sections 2 and 3 (oversized
singleton groups, module directories), 5 (large-state `Box`) and 7 (Port plans)
**Common Contract**: `impl-plans/active/airwindows-batch2-common.md` (mandatory: sections 5-11)
**Created**: 2026-11-01

## Intent

Port the upstream Airwindows **NonlinearSpace** reverb. Its upstream `NonlinearSpaceProc.cpp` is
about 1448 lines; only the `processReplacing` float path is ported.

- Register it in `g30/mod.rs`.
- Prove parity with `tests/parity_batch2_g30.rs`.

NonlinearSpace must be fully ported, registered and parity-tested. A stub, a partial port or an
unregistered module is a failure of this plan, not a deferral.

## Non-goals

No other plugin or group, no framework/shared-test/tools/fixture edits, no `processDoubleReplacing`,
no algorithm refactoring.

## Write Ownership

- **writePaths**:
  - `crates/airwindows/src/plugins/batch2/g30`
  - `crates/airwindows/tests/parity_batch2_g30.rs`
  - `crates/airwindows/tests/fixtures/batch2/g30`
  - `impl-plans/active/airwindows-batch2-o-nonlinearspace-g30.md`
  - `target`
  - `tmp/airwindows-batch2/ab2-o-nonlinearspace`
- **sharedPaths**: none.
- **artifactRoots**: `target`, `tmp/airwindows-batch2/ab2-o-nonlinearspace`.

## Module Layout (required: module directory, every file under 1000 lines, target 900 or fewer)

- `g30/mod.rs`: `mod nonlinearspace;` and `ENTRIES = &[nonlinearspace::ENTRY]`.
- `g30/nonlinearspace/mod.rs`: header, narrow allow list, params (including the sample-rate
  selector display strings), `INFO`/`ENTRY`/`construct`, the trait impl.
- `g30/nonlinearspace/state.rs`: the struct, with the delay arrays `Box`ed when over 4096 elements
  and sized as in `NonlinearSpace.h`, the constructor and `reset`.
- `g30/nonlinearspace/process.rs`, plus `process_b.rs` if needed: the per-sample loop.

## Ordered Subtasks (declare `#[allow(dead_code)] mod <module>;` in S1 so cmd 1 compiles each subtask; "register" = add the ENTRY and remove the allow, only at the end; common section 5 item 6)

1. **S1**: state, constructor, seed replay, params and display.
2. **S2**: per-block setup. The delay lengths are computed from a sample-rate **selector parameter**
   (not the host rate), so port the selector mapping exactly.
3. **S3**: the loop.
4. **S4**: `reset`.
5. **S5**: register, then the parity test **plus the required mode-sweep test** over every
   discrete sample-rate-selector value.
6. **S6**: measure, record and run the gates.

## Pitfalls

- The selector changes the delay lengths, which must stay within the array sizes for every value.
  The mode sweep catches out-of-bounds panics in the values the fixtures do not reach.
- Keep each delay counter's wrap test and the nonlinear feedback ordering exactly.
- Allocation happens only in the constructor. `reset` zeroes in place.

## Verification

Use common section 9 with P = `ab2-o-nonlinearspace` and G = `g30`. Logs go to
`tmp/airwindows-batch2/ab2-o-nonlinearspace/logs/`.

1. `$ENV cargo check -p music-things-airwindows --all-targets`: exit 0.
2. `$ENV cargo nextest run -p music-things-airwindows --test parity_batch2_g30`: exit 0,
   testsRun >= 2, failures 0.
3. `find crates/airwindows/src/plugins/batch2/g30 crates/airwindows/tests/parity_batch2_g30.rs -name '*.rs' -print0 | xargs -0 rustfmt --edition 2021 --check`:
   exit 0.
4. `test -z "$($ENV cargo clippy -p music-things-airwindows --all-targets --message-format short -- -D warnings 2>&1 | grep -E 'src/plugins/batch2/g30/|tests/parity_batch2_g30\.rs')"`:
   exit 0.
5. `test -z "$(find crates/airwindows/src/plugins/batch2/g30 -name '*.rs' -exec wc -l {} + | awk '$2 != "total" && $1 >= 1000')"`:
   exit 0.
6. `test "$(grep -c '::ENTRY' crates/airwindows/src/plugins/batch2/g30/mod.rs)" -eq 1 && test -f crates/airwindows/src/plugins/batch2/g30/nonlinearspace/mod.rs`:
   exit 0.
7. `! grep -rnE 'todo!|unimplemented!|#\[ignore' crates/airwindows/src/plugins/batch2/g30 crates/airwindows/tests/parity_batch2_g30.rs`:
   exit 0.
8. Diagnostic: `$ENV cargo nextest run -p music-things-airwindows --test no_alloc --test catalog`.
   It must show no failure naming NonlinearSpace.

## Completion Criteria

- [ ] NonlinearSpace fully ported as a module directory, registered (cmd 6) and passing parity and
  the mode sweep (cmd 2).
- [ ] The `ParityRecord` and the header record the class and measured max-abs.
- [ ] Commands 1-7 exit 0, with logs. Command 8 shows no NonlinearSpace failure.
- [ ] Only writePaths changed (pre/post hashes). Progress Log records subtasks per session.

## Progress Log

### Session: 2026-11-01
**Tasks Completed**: Plan created (not started).
