# Airwindows Batch 2 / o-logical4: Logical4 (oversized; group g17, kick line 175)

**Status**: Ready
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

- [ ] Logical4 fully ported as a module directory, registered (cmd 6) and passing parity (cmd 2).
- [ ] The `ParityRecord` and the header record the class and measured max-abs.
- [ ] Commands 1-7 exit 0, with logs. Command 8 shows no Logical4 failure.
- [ ] Only writePaths changed (pre/post hashes). Progress Log records subtasks per session.

## Progress Log

### Session: 2026-11-01
**Tasks Completed**: Plan created (not started).
