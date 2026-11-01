# Airwindows Batch 2 / o-monitoring3: Monitoring3 (oversized; group g24, kick line 190)

**Status**: Ready
**Plan ID**: ab2-o-monitoring3
**Wave**: W2. Depends on ab2-00-scaffold; runs in parallel with all other W2 plans.
**Design Reference**: `design-docs/specs/design-airwindows-batch2.md` sections 2 and 3 (oversized
singleton groups, module directories) and 7 (Port plans)
**Common Contract**: `impl-plans/active/airwindows-batch2-common.md` (mandatory: sections 5-11)
**Created**: 2026-11-01

## Intent

Port the upstream Airwindows **Monitoring3** plugin. Its upstream `Monitoring3Proc.cpp` is 1050
lines; only the `processReplacing` float path is ported.

- Register it in `g24/mod.rs`.
- Prove parity with `tests/parity_batch2_g24.rs`.

Monitoring3 must be fully ported, registered and parity-tested. A stub, a partial port (including
any missing mode) or an unregistered module is a failure of this plan, not a deferral.

## Non-goals

No other plugin or group, no framework/shared-test/tools/fixture edits, no `processDoubleReplacing`,
no code sharing with the Monitoring and Monitoring2 plans.

## Write Ownership

- **writePaths**:
  - `crates/airwindows/src/plugins/batch2/g24`
  - `crates/airwindows/tests/parity_batch2_g24.rs`
  - `crates/airwindows/tests/fixtures/batch2/g24`
  - `impl-plans/active/airwindows-batch2-o-monitoring3-g24.md`
  - `target`
  - `tmp/airwindows-batch2/ab2-o-monitoring3`
- **sharedPaths**: none.
- **artifactRoots**: `target`, `tmp/airwindows-batch2/ab2-o-monitoring3`.

## Module Layout (required: module directory, every file under 1000 lines, target 900 or fewer)

- `g24/mod.rs`: `mod monitoring3;` and `ENTRIES = &[monitoring3::ENTRY]`.
- `g24/monitoring3/mod.rs`: header, narrow allow list, params, `INFO`/`ENTRY`/`construct`, the
  struct, the constructor, the trait impl, and the mode dispatch.
- `g24/monitoring3/<modes>.rs`: the per-mode branches, split so each file is at most 900 lines.

## Ordered Subtasks (declare `#[allow(dead_code)] mod <module>;` in S1 so cmd 1 compiles each subtask; "register" = add the ENTRY and remove the allow, only at the end; common section 5 item 6)

1. **S1**: state (arrays `Box`ed if over 4096 elements), constructor, seed replay, params and the
   mode-name display.
2. **S2**: every mode branch, ported in order.
3. **S3**: `reset`.
4. **S4**: register, then the parity test **plus the required mode-sweep test** over every discrete
   mode.
5. **S5**: measure, record and run the gates.

## Pitfalls

- Unreached modes are covered only by the mode sweep and review, so port them with full care.
- Derive the mode index exactly as upstream does. Display strings must match the fixture.
- Keep the exact channel assignments per mode.

## Verification

Use common section 9 with P = `ab2-o-monitoring3` and G = `g24`. Logs go to
`tmp/airwindows-batch2/ab2-o-monitoring3/logs/`.

1. `$ENV cargo check -p music-things-airwindows --all-targets`: exit 0.
2. `$ENV cargo nextest run -p music-things-airwindows --test parity_batch2_g24`: exit 0,
   testsRun >= 2, failures 0.
3. `find crates/airwindows/src/plugins/batch2/g24 crates/airwindows/tests/parity_batch2_g24.rs -name '*.rs' -print0 | xargs -0 rustfmt --edition 2021 --check`:
   exit 0.
4. `test -z "$($ENV cargo clippy -p music-things-airwindows --all-targets --message-format short -- -D warnings 2>&1 | grep -E 'src/plugins/batch2/g24/|tests/parity_batch2_g24\.rs')"`:
   exit 0.
5. `test -z "$(find crates/airwindows/src/plugins/batch2/g24 -name '*.rs' -exec wc -l {} + | awk '$2 != "total" && $1 >= 1000')"`:
   exit 0.
6. `test "$(grep -c '::ENTRY' crates/airwindows/src/plugins/batch2/g24/mod.rs)" -eq 1 && test -f crates/airwindows/src/plugins/batch2/g24/monitoring3/mod.rs`:
   exit 0.
7. `! grep -rnE 'todo!|unimplemented!|#\[ignore' crates/airwindows/src/plugins/batch2/g24 crates/airwindows/tests/parity_batch2_g24.rs`:
   exit 0.
8. Diagnostic: `$ENV cargo nextest run -p music-things-airwindows --test no_alloc --test catalog`.
   It must show no failure naming Monitoring3.

## Completion Criteria

- [ ] Monitoring3 fully ported (all modes) as a module directory, registered (cmd 6) and passing
  parity and the mode sweep (cmd 2).
- [ ] The `ParityRecord` and the header record the class and measured max-abs.
- [ ] Commands 1-7 exit 0, with logs. Command 8 shows no Monitoring3 failure.
- [ ] Only writePaths changed (pre/post hashes). Progress Log records subtasks and the mode list.

## Progress Log

### Session: 2026-11-01
**Tasks Completed**: Plan created (not started).
