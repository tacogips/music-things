# Airwindows Batch 2 / n10: Groups g15 (line 173) and g18 (lines 176-180)

**Status**: Completed
**Plan ID**: ab2-n10
**Wave**: W2. Depends on ab2-00-scaffold; runs in parallel with all other W2 plans.
**Design Reference**: `design-docs/specs/design-airwindows-batch2.md` sections 2, 3, 5 and 7 (Port plans)
**Common Contract**: `impl-plans/active/airwindows-batch2-common.md` (mandatory: sections 5-11)
**Created**: 2026-11-01

## Intent

Port, register and parity-test these 6 plugins (modules in parentheses):

- g15: LeftoMono (`leftomono`)
- g18, in kick-line order: Loud (`loud`), Lowpass (`lowpass`), Lowpass2 (`lowpass2`),
  LRFlipTimer (`lrfliptimer`), Luxor (`luxor`)

The two groups are separate registration units. The oversized LilAmp (g16) and Logical4 (g17) sit
between them in kick order and belong to other plans.

Every listed plugin must be fully ported, registered in its group's `mod.rs` and parity-tested. A
missing, stubbed or unregistered plugin is a failure of this plan, not a deferral.

## Non-goals

No other group (in particular not g16 or g17), no framework/shared-test/tools/fixture edits, no
`processDoubleReplacing`.

## Write Ownership

- **writePaths**:
  - `crates/airwindows/src/plugins/batch2/g15`
  - `crates/airwindows/src/plugins/batch2/g18`
  - `crates/airwindows/tests/parity_batch2_g15.rs`
  - `crates/airwindows/tests/parity_batch2_g18.rs`
  - `crates/airwindows/tests/fixtures/batch2/g15`
  - `crates/airwindows/tests/fixtures/batch2/g18`
  - `impl-plans/active/airwindows-batch2-n10-g15-g18.md`
  - `target`
  - `tmp/airwindows-batch2/ab2-n10`
- **sharedPaths**: none.
- **artifactRoots**: `target`, `tmp/airwindows-batch2/ab2-n10`.

## File-level Changes

- `g15/mod.rs`: 1 `mod` line and 1 `ENTRY`. `g18/mod.rs`: 5 of each, in the order above.
- One `<module>.rs` per plugin in its group directory, per common section 5.
- `tests/parity_batch2_g15.rs` (1 record and test) and `tests/parity_batch2_g18.rs` (5 records and
  tests), plus mode-sweep tests where required. Each test file reads only its own group's fixtures.

## Plugin Pitfalls

- **LRFlipTimer**: swaps L/R on a timer measured in samples from the sample rate and a minutes
  parameter.
  - Keep the integer counter type and the comparison exactly.
  - At the fixture length, the flip may never trigger. Still port it fully.
- **Loud**: a soft-clip loudness stage with `sin` on doubles. Check the precision.
- **Lowpass, Lowpass2**: IIR lowpass with alternating-sample (`fpFlip`) state. Keep the toggle order.
- **Luxor**: biquad/IIR coloring with sample-rate-scaled coefficients.
- **LeftoMono**: a trivial channel copy. It still needs fpd handling only if upstream has it.

## Verification

Use common section 9 with P = `ab2-n10` and G = `g15`, `g18`. Logs go to `tmp/airwindows-batch2/ab2-n10/logs/`.

1. `$ENV cargo check -p music-things-airwindows --all-targets`: exit 0.
2. `$ENV cargo nextest run -p music-things-airwindows --test parity_batch2_g15 --test parity_batch2_g18`:
   exit 0, testsRun >= 6, failures 0.
3. `find crates/airwindows/src/plugins/batch2/g15 crates/airwindows/src/plugins/batch2/g18 crates/airwindows/tests/parity_batch2_g15.rs crates/airwindows/tests/parity_batch2_g18.rs -name '*.rs' -print0 | xargs -0 rustfmt --edition 2021 --check`:
   exit 0.
4. `test -z "$($ENV cargo clippy -p music-things-airwindows --all-targets --message-format short -- -D warnings 2>&1 | grep -E 'src/plugins/batch2/(g15|g18)/|tests/parity_batch2_(g15|g18)\.rs')"`:
   exit 0.
5. `test -z "$(find crates/airwindows/src/plugins/batch2/g15 crates/airwindows/src/plugins/batch2/g18 -name '*.rs' -exec wc -l {} + | awk '$2 != "total" && $1 >= 1000')"`:
   exit 0.
6. `test "$(grep -c '::ENTRY' crates/airwindows/src/plugins/batch2/g15/mod.rs)" -eq 1 && test "$(grep -c '::ENTRY' crates/airwindows/src/plugins/batch2/g18/mod.rs)" -eq 5`:
   exit 0.
7. `! grep -rnE 'todo!|unimplemented!|#\[ignore' crates/airwindows/src/plugins/batch2/g15 crates/airwindows/src/plugins/batch2/g18 crates/airwindows/tests/parity_batch2_g15.rs crates/airwindows/tests/parity_batch2_g18.rs`:
   exit 0.
8. Diagnostic: `$ENV cargo nextest run -p music-things-airwindows --test no_alloc --test catalog`.
   It must show no failure naming a g15 or g18 plugin.

## Completion Criteria

- [x] All 6 plugins ported, registered (cmd 6) and passing parity (cmd 2).
- [x] Each `ParityRecord` and module header records the class and measured max-abs.
- [x] Commands 1-7 exit 0, with logs. Command 8 shows no failure for this plan's plugins.
- [x] Only writePaths changed (pre/post hashes). Progress Log is updated per common section 11.

## Progress Log

### Session: 2026-11-01
**Tasks Completed**: Plan created (not started).

### Session: 2026-11-01 (implementation)
**Tasks Completed**: Six ports and fixture parity tests; measured max-abs 0.0 for each (Libm). Scoped gates and no_alloc/catalog diagnostic passed; logs under `tmp/airwindows-batch2/ab2-n10/logs/`.
**Scope evidence**: `tmp/airwindows-batch2/ab2-n10/scope-hashes.json` compares the native pre-node snapshot with owned paths. Independent review remains pending.

### Session: 2026-11-02 (verification and tolerance correction)
**Tasks completed**: LeftoMono, Lowpass2 and LRFlipTimer use Exact; Loud, Lowpass and Luxor use Libm. All six measured max-abs values remain 0.0. All six parity tests pass. No plugins remain blocked.
**Gates**: Commands 1 and 2 exited 0 after a bounded retry (`logs/retry-check.log`, `logs/retry-parity.log`; 6 tests passed). Commands 3, 5, 6, 7 and 7b exited 0 (`logs/final-rustfmt.log`, `logs/final-lines.log`, `logs/final-entries.log`, `logs/final-nostub.log`, `logs/final-deadcode.log`). Scoped clippy gate 4 exited 0 (`logs/retry-clippy-gate.log`); raw clippy reported sibling g14 diagnostics (`logs/retry-clippy-raw.log`). Diagnostic 8 exited 0 with 4 tests passed (`logs/final-diagnostic.log`). All log paths are under `tmp/airwindows-batch2/ab2-n10/`. Exact commands are recorded in `logs/final-results.json` and `logs/retry-results.json`.
**Sibling drift**: Initial check and parity failed on a transient g14 LeadAmp borrow error (`logs/final-check.log`, `logs/final-parity.log`); both passed on retry after the sibling changed. Formal review and integration remain downstream.

### Session: 2026-11-13 (operator integration)
**Tasks Completed**: Closed by operator integration after review-loop session-629 stopped as implementation-blocked (ab2-n01 DubSub, ab2-n04 write-scope evidence). Merged main (batch 3) into the branch and ran the combined-tree gates: nextest 481/481 (3 pre-existing SoundFont skips), clippy -D warnings, fmt --check and the wasm32 build all exit 0.
**Notes**: Checkboxes left unticked were superseded by the combined-tree gates above, not run individually.
