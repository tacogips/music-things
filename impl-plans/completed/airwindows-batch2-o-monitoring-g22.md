# Airwindows Batch 2 / o-monitoring: Monitoring (oversized; group g22, kick line 188)

**Status**: Completed
**Plan ID**: ab2-o-monitoring
**Wave**: W2. Depends on ab2-00-scaffold; runs in parallel with all other W2 plans.
**Design Reference**: `design-docs/specs/design-airwindows-batch2.md` sections 2 and 3 (oversized
singleton groups, module directories) and 7 (Port plans)
**Common Contract**: `impl-plans/active/airwindows-batch2-common.md` (mandatory: sections 5-11)
**Created**: 2026-11-01

## Intent

Port the upstream Airwindows **Monitoring** plugin. Its upstream `MonitoringProc.cpp` is 1042
lines; only the `processReplacing` float path is ported.

- Register it in `g22/mod.rs`.
- Prove parity with `tests/parity_batch2_g22.rs`.

Monitoring must be fully ported, registered and parity-tested. A stub, a partial port (including
any missing mode) or an unregistered module is a failure of this plan, not a deferral.

## Non-goals

No other plugin or group (Monitoring2 and Monitoring3 belong to ab2-n12 and ab2-o-monitoring3), no
framework/shared-test/tools/fixture edits, no `processDoubleReplacing`, no code sharing with sibling
plans.

## Write Ownership

- **writePaths**:
  - `crates/airwindows/src/plugins/batch2/g22`
  - `crates/airwindows/tests/parity_batch2_g22.rs`
  - `crates/airwindows/tests/fixtures/batch2/g22`
  - `impl-plans/active/airwindows-batch2-o-monitoring-g22.md`
  - `target`
  - `tmp/airwindows-batch2/ab2-o-monitoring`
- **sharedPaths**: none.
- **artifactRoots**: `target`, `tmp/airwindows-batch2/ab2-o-monitoring`.

## Module Layout (required: module directory, every file under 1000 lines, target 900 or fewer)

- `g22/mod.rs`: `mod monitoring;` and `ENTRIES = &[monitoring::ENTRY]`.
- `g22/monitoring/mod.rs`: header, narrow allow list, params, `INFO`/`ENTRY`/`construct`, the
  struct, the constructor, the trait impl, and the mode dispatch.
- `g22/monitoring/<modes>.rs`: the per-mode branches (for example the dither/wordlength modes, the
  analysis modes and the headphone/"cans" modes), split so each file is at most 900 lines.
  - The shared pre- and post-mode code stays in the same order around the dispatch.

## Ordered Subtasks (declare `#[allow(dead_code)] mod <module>;` in S1 so cmd 1 compiles each subtask; "register" = add the ENTRY and remove the allow, only at the end; common section 5 item 6)

1. **S1**: state (`.h` arrays, including the large dither/`bynL` arrays and any delay buffers,
   `Box`ed if over 4096 elements), constructor, seed replay, params and the mode-name display
   strings.
2. **S2**: every mode branch, ported in order.
3. **S3**: `reset`.
4. **S4**: register, then the parity test **plus the required mode-sweep test** covering every
   discrete mode value.
5. **S5**: measure, record and run the gates.

## Pitfalls

- Fixtures R0/R1/R2 reach at most two or three modes. Unreached modes are covered only by the
  mode-sweep test (no panic, finite output) and by review, so port them with the same care.
- Derive the mode index exactly as upstream does (an integer cast of a scaled param). The
  mode-name display must match the fixture's display strings.
- Some modes zero one channel or sum to mono. Keep the exact channel assignments.

## Verification

Use common section 9 with P = `ab2-o-monitoring` and G = `g22`. Logs go to
`tmp/airwindows-batch2/ab2-o-monitoring/logs/`.

1. `$ENV cargo check -p music-things-airwindows --all-targets`: exit 0.
2. `$ENV cargo nextest run -p music-things-airwindows --test parity_batch2_g22`: exit 0,
   testsRun >= 2 (parity plus mode sweep), failures 0.
3. `find crates/airwindows/src/plugins/batch2/g22 crates/airwindows/tests/parity_batch2_g22.rs -name '*.rs' -print0 | xargs -0 rustfmt --edition 2021 --check`:
   exit 0.
4. `test -z "$($ENV cargo clippy -p music-things-airwindows --all-targets --message-format short -- -D warnings 2>&1 | grep -E 'src/plugins/batch2/g22/|tests/parity_batch2_g22\.rs')"`:
   exit 0.
5. `test -z "$(find crates/airwindows/src/plugins/batch2/g22 -name '*.rs' -exec wc -l {} + | awk '$2 != "total" && $1 >= 1000')"`:
   exit 0.
6. `test "$(grep -c '::ENTRY' crates/airwindows/src/plugins/batch2/g22/mod.rs)" -eq 1 && test -f crates/airwindows/src/plugins/batch2/g22/monitoring/mod.rs`:
   exit 0.
7. `! grep -rnE 'todo!|unimplemented!|#\[ignore' crates/airwindows/src/plugins/batch2/g22 crates/airwindows/tests/parity_batch2_g22.rs`:
   exit 0.
8. Diagnostic: `$ENV cargo nextest run -p music-things-airwindows --test no_alloc --test catalog`.
   It must show no failure naming Monitoring.

## Completion Criteria

- [x] Monitoring fully ported (all modes) as a module directory, registered (cmd 6) and passing
  parity and the mode sweep (cmd 2).
- [x] The `ParityRecord` and the header record the class and measured max-abs.
- [x] Commands 1-7 exit 0, with logs. Command 8 shows no Monitoring failure.
- [x] Only writePaths changed (pre/post hashes). Progress Log records subtasks and the mode list.

## Progress Log

### Session: 2026-11-01
**Tasks Completed**: Plan created (not started).

### Session: 2026-11-01 implementation
**Tasks Completed**: S1–S4: state, all 17 modes, reset, registration, parity and mode sweep. S5: measured Libm max-abs 0.0 for all three fixture renders.
**Modes**: Out24, Out16, Peaks, Slew, Subs, Mono, Side, Vinyl, Aurat, MonoRat, MonoLat, Phone, Cans A–D, V Trick.
**Notes**: Upstream float path uses NJAD rather than floating-point dither. Final gates pass (logs `tmp/airwindows-batch2/ab2-o-monitoring/logs/final3-*`); formal review remains pending. Scope evidence: `tmp/airwindows-batch2/ab2-o-monitoring/scope-hashes.json` and per-edit intents.

### Session: 2026-11-02 verification
**Tasks Completed**: Rechecked the existing Monitoring port and all 17 modes against the assigned design. Libm measured max-abs remains 0.0 across the three recorded renders; no plugin remains outstanding.
**Gates**: Commands 1 and 2 reran after concurrent changes: exit 0, with 2/2 parity and mode-sweep tests passing (`logs/agent-final-check.log`, `logs/agent-final-parity.log`). Commands 3, 4, 5, 6, 7 and 7b each exited 0 (`logs/agent-rustfmt.log`, `logs/agent-clippy.log`, `logs/agent-lines.log`, `logs/agent-entry.log`, `logs/agent-stubs.log`, `logs/agent-dead-code.log`). Diagnostic command 8 exited 0 with 4/4 tests passing (`logs/agent-final-diagnostic.log`). All log paths are relative to `tmp/airwindows-batch2/ab2-o-monitoring/`.
**Moving tree**: Diagnostic initially failed twice before tests due to a concurrent borrow error in sibling `g14/leadamp/mod.rs:107` (`logs/agent-diagnostic.log`, `logs/agent-diagnostic-retry.log`); the sibling was repaired, and fresh check, parity and diagnostic reruns passed. Formal review remains pending.

### Session: 2026-11-13 (operator integration)
**Tasks Completed**: Closed by operator integration after review-loop session-629 stopped as implementation-blocked (ab2-n01 DubSub, ab2-n04 write-scope evidence). Merged main (batch 3) into the branch and ran the combined-tree gates: nextest 481/481 (3 pre-existing SoundFont skips), clippy -D warnings, fmt --check and the wasm32 build all exit 0.
**Notes**: Checkboxes left unticked were superseded by the combined-tree gates above, not run individually.
