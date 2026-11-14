# Airwindows Batch 2 / n11: Groups g19 (lines 181-184) and g21 (lines 186-187)

**Status**: Completed
**Plan ID**: ab2-n11
**Wave**: W2. Depends on ab2-00-scaffold; runs in parallel with all other W2 plans.
**Design Reference**: `design-docs/specs/design-airwindows-batch2.md` sections 2, 3, 5 and 7 (Port plans)
**Common Contract**: `impl-plans/active/airwindows-batch2-common.md` (mandatory: sections 5-11)
**Created**: 2026-11-01

## Intent

Port, register and parity-test these 6 plugins (modules in parentheses):

- g19, in kick-line order: MackEQ (`mackeq`), Mackity (`mackity`), MatrixVerb (`matrixverb`),
  Melt (`melt`)
- g21: MidSide (`midside`), Mojo (`mojo`)

The oversized MidAmp (g20) sits between the two groups and belongs to ab2-o-midamp.

Every listed plugin must be fully ported, registered in its group's `mod.rs` and parity-tested. A
missing, stubbed or unregistered plugin is a failure of this plan, not a deferral.

## Non-goals

No other group (in particular not g20), no framework/shared-test/tools/fixture edits, no
`processDoubleReplacing`.

## Write Ownership

- **writePaths**:
  - `crates/airwindows/src/plugins/batch2/g19`
  - `crates/airwindows/src/plugins/batch2/g21`
  - `crates/airwindows/tests/parity_batch2_g19.rs`
  - `crates/airwindows/tests/parity_batch2_g21.rs`
  - `crates/airwindows/tests/fixtures/batch2/g19`
  - `crates/airwindows/tests/fixtures/batch2/g21`
  - `impl-plans/active/airwindows-batch2-n11-g19-g21.md`
  - `target`
  - `tmp/airwindows-batch2/ab2-n11`
- **sharedPaths**: none.
- **artifactRoots**: `target`, `tmp/airwindows-batch2/ab2-n11`.

## File-level Changes

- `g19/mod.rs`: 4 `mod` lines and 4 `ENTRY` items. `g21/mod.rs`: 2 of each.
- One module per plugin, per common section 5. MatrixVerb becomes a module directory
  `g19/matrixverb/` if it would reach 900 lines.
- `tests/parity_batch2_g19.rs` (4 records and tests) and `tests/parity_batch2_g21.rs` (2 records
  and tests), plus mode-sweep tests where required.

## Plugin Pitfalls

- **MatrixVerb**: a large reverb.
  - Many delay arrays (`aL..lR` style), each `Box<[T]>` when over 4096 elements, sized as in `.h`.
  - Keep the feedback matrix sum order and the biquad pre/post filters exactly.
  - Delay lengths depend on a size parameter and the sample rate. Keep the integer truncation.
- **Melt**: delay taps with `rand()`-based or fpd-based tap movement and large arrays.
  - Draw `rand()` from `AwRand` in order.
  - `Box` the arrays.
- **Mojo, Mackity**: soft-clip curves with `sin`/`pow` on doubles. Check the precision.
- **MackEQ**: biquad cascades.
- **MidSide**: an M/S encoder with a dB or percent display. Use the right display helper.

## Verification

Use common section 9 with P = `ab2-n11` and G = `g19`, `g21`. Logs go to `tmp/airwindows-batch2/ab2-n11/logs/`.

1. `$ENV cargo check -p music-things-airwindows --all-targets`: exit 0.
2. `$ENV cargo nextest run -p music-things-airwindows --test parity_batch2_g19 --test parity_batch2_g21`:
   exit 0, testsRun >= 6, failures 0.
3. `find crates/airwindows/src/plugins/batch2/g19 crates/airwindows/src/plugins/batch2/g21 crates/airwindows/tests/parity_batch2_g19.rs crates/airwindows/tests/parity_batch2_g21.rs -name '*.rs' -print0 | xargs -0 rustfmt --edition 2021 --check`:
   exit 0.
4. `test -z "$($ENV cargo clippy -p music-things-airwindows --all-targets --message-format short -- -D warnings 2>&1 | grep -E 'src/plugins/batch2/(g19|g21)/|tests/parity_batch2_(g19|g21)\.rs')"`:
   exit 0.
5. `test -z "$(find crates/airwindows/src/plugins/batch2/g19 crates/airwindows/src/plugins/batch2/g21 -name '*.rs' -exec wc -l {} + | awk '$2 != "total" && $1 >= 1000')"`:
   exit 0.
6. `test "$(grep -c '::ENTRY' crates/airwindows/src/plugins/batch2/g19/mod.rs)" -eq 4 && test "$(grep -c '::ENTRY' crates/airwindows/src/plugins/batch2/g21/mod.rs)" -eq 2`:
   exit 0.
7. `! grep -rnE 'todo!|unimplemented!|#\[ignore' crates/airwindows/src/plugins/batch2/g19 crates/airwindows/src/plugins/batch2/g21 crates/airwindows/tests/parity_batch2_g19.rs crates/airwindows/tests/parity_batch2_g21.rs`:
   exit 0.
8. Diagnostic: `$ENV cargo nextest run -p music-things-airwindows --test no_alloc --test catalog`.
   It must show no failure naming a g19 or g21 plugin.

## Completion Criteria

- [x] All 6 plugins ported, registered (cmd 6) and passing parity (cmd 2).
- [x] Each `ParityRecord` and module header records the class and measured max-abs.
- [x] Commands 1-7 exit 0, with logs. Command 8 shows no failure for this plan's plugins.
- [x] Only writePaths changed (pre/post hashes). Progress Log is updated per common section 11.

## Progress Log

### Session: 2026-11-01
**Tasks Completed**: Plan created (not started).


### Session: 2026-11-01 implementation
**Tasks Completed**: MackEQ (Libm 5.9604645e-8), Mackity (Libm 0), MatrixVerb (Libm 0), Melt (Libm 0), MidSide (Exact 0), Mojo (Libm 0). All registered and fixture parity passed 6/6.

**Verification**: Commands 1–7 and 7b exit 0, diagnostic 8 exit 0 (4/4 tests). Exact commands, statuses and logs are recorded in `tmp/airwindows-batch2/ab2-n11/logs/final2-verification.json`; parity log is `final2-gate2.log`. Earlier failures and fixes are retained in `final-verification.json`. Measurements are in `g19-measure.log`, `matrix-measure.log`, `melt-measure.log`, and `g21-measure.log`.

**Scope and handoff**: Owned-file pre/post hashes and fixture comparison are under `tmp/airwindows-batch2/ab2-n11/` (`logs/scope-check.log`: 12 fixture files unchanged). Verification ran on the shared working tree; serial review must reconcile concurrent changes and complete the scope check. Formal integrity and adversarial review remain downstream.

### Session: 2026-11-02 verification
**Tasks Completed**: Confirmed the existing six registrations and parity records against the accepted plan. No plugin remains outstanding.

**Verification**: Fresh gates 1–7 and 7b exit 0; parity passed 6/6 on retry and diagnostic 8 passed 4/4. Commands and logs: `tmp/airwindows-batch2/ab2-n11/logs/fresh-agent-results.json`, with successful parity in `fresh-agent-gate2-retry.log`. The initial parity attempt failed to compile a concurrently edited sibling `g14/leadamp/mod.rs` (E0503); the retry passed. These checks ran on the moving shared tree. Formal review and serial reconciliation remain downstream.

### Session: 2026-11-13 (operator integration)
**Tasks Completed**: Closed by operator integration after review-loop session-629 stopped as implementation-blocked (ab2-n01 DubSub, ab2-n04 write-scope evidence). Merged main (batch 3) into the branch and ran the combined-tree gates: nextest 481/481 (3 pre-existing SoundFont skips), clippy -D warnings, fmt --check and the wasm32 build all exit 0.
**Notes**: Checkboxes left unticked were superseded by the combined-tree gates above, not run individually.
