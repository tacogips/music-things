# Airwindows Batch 2 / o-nikola: Nikola (oversized; group g28, kick line 197)

**Status**: Ready
**Plan ID**: ab2-o-nikola
**Wave**: W2. Depends on ab2-00-scaffold; runs in parallel with all other W2 plans.
**Design Reference**: `design-docs/specs/design-airwindows-batch2.md` sections 2 and 3 (oversized
singleton groups, module directories) and 7 (Port plans)
**Common Contract**: `impl-plans/active/airwindows-batch2-common.md` (mandatory: sections 5-11)
**Created**: 2026-11-01

## Intent

Port the upstream Airwindows **Nikola** plugin. Its upstream `NikolaProc.cpp` is 2047 lines, the
second-largest in batch 2; only the `processReplacing` float path is ported.

- Register it in `g28/mod.rs`.
- Prove parity with `tests/parity_batch2_g28.rs`.

Nikola must be fully ported, registered and parity-tested. A stub, a partial port or an
unregistered module is a failure of this plan, not a deferral.

## Non-goals

No other plugin or group, no framework/shared-test/tools/fixture edits, no `processDoubleReplacing`,
no algorithm refactoring.

## Write Ownership

- **writePaths**:
  - `crates/airwindows/src/plugins/batch2/g28`
  - `crates/airwindows/tests/parity_batch2_g28.rs`
  - `crates/airwindows/tests/fixtures/batch2/g28`
  - `impl-plans/active/airwindows-batch2-o-nikola-g28.md`
  - `target`
  - `tmp/airwindows-batch2/ab2-o-nikola`
- **sharedPaths**: none.
- **artifactRoots**: `target`, `tmp/airwindows-batch2/ab2-o-nikola`.

## Module Layout (required: module directory, every file under 1000 lines, target 900 or fewer)

- `g28/mod.rs`: `mod nikola;` and `ENTRIES = &[nikola::ENTRY]`.
- `g28/nikola/mod.rs`: header, narrow allow list, params, `INFO`/`ENTRY`/`construct`, the struct,
  the constructor and the trait impl.
- `g28/nikola/<section>.rs`: at least two submodules holding consecutive sections of the
  per-sample loop, split at upstream section boundaries. First read `NikolaProc.cpp` and record in
  the Progress Log where `processReplacing` ends and how the sections divide.

## Ordered Subtasks (declare `#[allow(dead_code)] mod <module>;` in S1 so cmd 1 compiles each subtask; "register" = add the ENTRY and remove the allow, only at the end; common section 5 item 6)

1. **S0**: read and map the upstream file (the `processReplacing` line range, sections, statics,
   `rand()` use). Record the map in the Progress Log.
2. **S1**: state, constructor, seed replay, params and display.
3. **S2**: loop sections, ported in order (one session per about 600 upstream lines if needed).
4. **S3**: `reset`.
5. **S4**: register, then the parity test (and a mode-sweep test if there is an integer selector).
6. **S5**: measure, record and run the gates.

## Pitfalls

- Long repeated per-stage code: keep every statement and its order. Do not fold it into loops that
  change the order.
- Pass crossing locals explicitly. Per-sample locals stay per-sample.
- Keep the `.h` float/double member types.

## Verification

Use common section 9 with P = `ab2-o-nikola` and G = `g28`. Logs go to
`tmp/airwindows-batch2/ab2-o-nikola/logs/`.

1. `$ENV cargo check -p music-things-airwindows --all-targets`: exit 0.
2. `$ENV cargo nextest run -p music-things-airwindows --test parity_batch2_g28`: exit 0,
   testsRun >= 1, failures 0.
3. `find crates/airwindows/src/plugins/batch2/g28 crates/airwindows/tests/parity_batch2_g28.rs -name '*.rs' -print0 | xargs -0 rustfmt --edition 2021 --check`:
   exit 0.
4. `test -z "$($ENV cargo clippy -p music-things-airwindows --all-targets --message-format short -- -D warnings 2>&1 | grep -E 'src/plugins/batch2/g28/|tests/parity_batch2_g28\.rs')"`:
   exit 0.
5. `test -z "$(find crates/airwindows/src/plugins/batch2/g28 -name '*.rs' -exec wc -l {} + | awk '$2 != "total" && $1 >= 1000')"`:
   exit 0.
6. `test "$(grep -c '::ENTRY' crates/airwindows/src/plugins/batch2/g28/mod.rs)" -eq 1 && test -f crates/airwindows/src/plugins/batch2/g28/nikola/mod.rs`:
   exit 0.
7. `! grep -rnE 'todo!|unimplemented!|#\[ignore' crates/airwindows/src/plugins/batch2/g28 crates/airwindows/tests/parity_batch2_g28.rs`:
   exit 0.
8. Diagnostic: `$ENV cargo nextest run -p music-things-airwindows --test no_alloc --test catalog`.
   It must show no failure naming Nikola.

## Completion Criteria

- [ ] Nikola fully ported as a module directory, registered (cmd 6) and passing parity (cmd 2).
- [ ] The `ParityRecord` and the header record the class and measured max-abs.
- [ ] Commands 1-7 exit 0, with logs. Command 8 shows no Nikola failure.
- [ ] Only writePaths changed (pre/post hashes). Progress Log records the S0 map and subtasks.

## Progress Log

### Session: 2026-11-01
**Tasks Completed**: Plan created (not started).
