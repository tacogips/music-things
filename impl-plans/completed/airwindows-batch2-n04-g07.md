# Airwindows Batch 2 / n04: Group g07 (kick lines 135-139)

**Status**: Completed
**Plan ID**: ab2-n04
**Wave**: W2. Depends on ab2-00-scaffold; runs in parallel with all other W2 plans.
**Design Reference**: `design-docs/specs/design-airwindows-batch2.md` sections 2, 3, 5 and 7 (Port plans)
**Common Contract**: `impl-plans/active/airwindows-batch2-common.md` (mandatory: sections 5-11)
**Created**: 2026-11-01

## Intent

Port, register and parity-test these 5 plugins in kick-line order (modules in parentheses):

Flipity (`flipity`), Floor (`floor`), Flutter (`flutter`), Focus (`focus`), Fracture (`fracture`).

Every listed plugin must be fully ported, registered in `g07/mod.rs` and parity-tested. A missing,
stubbed or unregistered plugin is a failure of this plan, not a deferral.

## Non-goals

No other group, no framework/shared-test/tools/fixture edits (fixtures only via common section 10),
no `processDoubleReplacing`.

## Write Ownership

- **writePaths**:
  - `crates/airwindows/src/plugins/batch2/g07`
  - `crates/airwindows/tests/parity_batch2_g07.rs`
  - `crates/airwindows/tests/fixtures/batch2/g07`
  - `impl-plans/active/airwindows-batch2-n04-g07.md`
  - `target`
  - `tmp/airwindows-batch2/ab2-n04`
- **sharedPaths**: none.
- **artifactRoots**: `target`, `tmp/airwindows-batch2/ab2-n04`.

## File-level Changes

- `g07/mod.rs`: 5 `mod` lines and 5 `ENTRY` items in the order above.
- `g07/<module>.rs` per common section 5. Imitate `batch1/g03/beam.rs`.
- `tests/parity_batch2_g07.rs`: 5 records and tests, plus mode-sweep tests where required.
  Imitate `tests/parity_batch1_g03.rs`.

## Plugin Pitfalls

- **Flipity**: a channel flip/invert mode selector. Port every mode and add a mode-sweep test.
- **Flutter**: a modulated delay driven by `rand()`/fpd-based wander.
  - Draw `rand()` from `AwRand` in order.
  - The buffer index wraps exactly as upstream.
- **Focus**: a mode selector (several saturation modes) plus an output-level display. Port every
  mode and add a mode-sweep test.
- **Fracture**: a multi-fold distortion using `sin` on doubles. Keep the precision.
- **Floor**: sub-bass shaping with sample-rate scaled IIRs.

## Verification

Use common section 9 with P = `ab2-n04` and G = `g07`. Logs go to `tmp/airwindows-batch2/ab2-n04/logs/`.

1. `$ENV cargo check -p music-things-airwindows --all-targets`: exit 0.
2. `$ENV cargo nextest run -p music-things-airwindows --test parity_batch2_g07`: exit 0,
   testsRun >= 5, failures 0.
3. `find crates/airwindows/src/plugins/batch2/g07 crates/airwindows/tests/parity_batch2_g07.rs -name '*.rs' -print0 | xargs -0 rustfmt --edition 2021 --check`:
   exit 0.
4. `test -z "$($ENV cargo clippy -p music-things-airwindows --all-targets --message-format short -- -D warnings 2>&1 | grep -E 'src/plugins/batch2/g07/|tests/parity_batch2_g07\.rs')"`:
   exit 0.
5. `test -z "$(find crates/airwindows/src/plugins/batch2/g07 -name '*.rs' -exec wc -l {} + | awk '$2 != "total" && $1 >= 1000')"`:
   exit 0.
6. `test "$(grep -c '::ENTRY' crates/airwindows/src/plugins/batch2/g07/mod.rs)" -eq 5`: exit 0.
7. `! grep -rnE 'todo!|unimplemented!|#\[ignore' crates/airwindows/src/plugins/batch2/g07 crates/airwindows/tests/parity_batch2_g07.rs`:
   exit 0.
8. Diagnostic: `$ENV cargo nextest run -p music-things-airwindows --test no_alloc --test catalog`.
   It must show no failure naming a g07 plugin.

## Completion Criteria

- [x] All 5 plugins ported, registered in order (cmd 6) and passing parity (cmd 2).
- [x] Each `ParityRecord` and module header records the class and measured max-abs.
- [x] Commands 1-7 exit 0, with logs. Command 8 shows no g07 failure.
- [ ] Only writePaths changed (pre/post hashes). Progress Log is updated per common section 11.

## Progress Log

### Session: 2026-11-01
**Tasks Completed**: Plan created (not started).

### Session: 2026-11-01 implementation
**Tasks Completed**: Five ports, registrations, fixture parity and Flipity/Focus selector sweeps. On arm64 Darwin, each measured max-abs was 0; Flipity is Exact and the other four are Libm. Final scoped gates 1-7 and 7b passed, parity 8/8 and no_alloc/catalog diagnostic 4/4 passed; see `tmp/airwindows-batch2/ab2-n04/logs/recheck-*.log`.
**Notes**: Flipity upstream float path has no dither; Flutter modulation uses fpd. Initial pre-edit hashes were not recorded before source creation; `pre-hashes.txt` was reconstructed from the native pre-node snapshot, and a complete outside-scope pre/post audit remains for reconciliation. Formal reviews remain downstream.

### Session: 2026-11-01 scope-audit continuation
**Finding**: The native pre-node and continuation snapshots cover only the four assigned roots. They do not capture outside-scope pre-node state, so they cannot prove or attribute outside-scope changes on the concurrent shared tree. The source identity check against `post-hashes.txt` passed before this plan update (`logs/continuation-source-identity.log`).
**Blocked**: Complete scope verification requires a broader pre-node baseline or an explicit workflow disposition of the missing evidence. No outside-scope files were edited in this continuation.

### Session: 2026-11-02 verification continuation
**Tasks Completed**: Rechecked existing five ports and selector sweeps; no source edits. Fresh cargo check passed after a transient sibling g31 syntax error (initial log `logs/current-check.log`, retry `logs/final-check.log`). Parity passed 8/8 (`logs/current-parity.log`); format, own-file clippy, line counts, ENTRY count, stub and dead-code checks passed (`logs/current-fmt.log`, `logs/final-clippy.log`, `logs/current-lines.log`, `logs/current-entries.log`, `logs/current-stub.log`, `logs/current-deadcode.log`). Catalog/no-allocation diagnostic passed 4/4 (`logs/current-diagnostic.log`). All exits were 0.
**Notes**: The earlier missing outside-scope baseline remains an attribution limitation for serial reconciliation; this continuation edited only this plan and plan-local evidence. Formal reviews remain downstream.

### Session: 2026-11-02 current-source recheck
**Tasks Completed**: Cargo check, parity (8/8), own-file formatting and clippy diagnostic, line count, registration, stub and dead-code checks all passed. Catalog/no-allocation diagnostic passed 4/4. Logs: `tmp/airwindows-batch2/ab2-n04/logs/run-20261102-*.log`.
**Scope limitation**: The original outside-scope pre-node baseline remains unavailable; serial reconciliation or workflow disposition is required to establish the only-writePaths criterion. No source files were edited in this recheck.

### Session: 2026-11-02 scope redispatch
**Tasks Completed**: Verified g07 source, tests and fixtures match the current native pre-node snapshot; fresh parity passed 8/8 (`logs/redispatch-20261102-parity.log`). No Rust or fixture edits.
**Blocked**: The historical pre-node snapshot does not cover outside-scope paths. A broader baseline or explicit serial reconciliation disposition is still required for the only-writePaths criterion.

### Session: 2026-11-13 (operator integration)
**Tasks Completed**: Closed by operator integration after review-loop session-629 stopped as implementation-blocked (ab2-n01 DubSub, ab2-n04 write-scope evidence). Merged main (batch 3) into the branch and ran the combined-tree gates: nextest 481/481 (3 pre-existing SoundFont skips), clippy -D warnings, fmt --check and the wasm32 build all exit 0.
**Notes**: Write scope checked by hand: the batch-2 diff against main touches g07 only under crates/airwindows/src/plugins/batch2/g07, and no file outside batch-2-owned paths except the batch-owned catalog/design/harness files and the fixture_budget.rs Q1 edit. Parity 8/8. Checkboxes left unticked were superseded by the combined-tree gates above, not run individually.
