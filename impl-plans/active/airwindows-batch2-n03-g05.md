# Airwindows Batch 2 / n03: Group g05 (kick lines 128-133)

**Status**: Ready
**Plan ID**: ab2-n03
**Wave**: W2. Depends on ab2-00-scaffold; runs in parallel with all other W2 plans.
**Design Reference**: `design-docs/specs/design-airwindows-batch2.md` sections 2, 3, 5 and 7 (Port plans)
**Common Contract**: `impl-plans/active/airwindows-batch2-common.md` (mandatory: sections 5-11)
**Created**: 2026-11-01

## Intent

Port, register and parity-test these 6 plugins in kick-line order (modules in parentheses):

Ensemble (`ensemble`), EQ (`eq`), EveryTrim (`everytrim`), Exciter (`exciter`), Facet (`facet`),
FathomFive (`fathomfive`).

Every listed plugin must be fully ported, registered in `g05/mod.rs` and parity-tested. A missing,
stubbed or unregistered plugin is a failure of this plan, not a deferral.

## Non-goals

No other group, no framework/shared-test/tools/fixture edits (fixtures only via common section 10),
no `processDoubleReplacing`.

## Write Ownership

- **writePaths**:
  - `crates/airwindows/src/plugins/batch2/g05`
  - `crates/airwindows/tests/parity_batch2_g05.rs`
  - `crates/airwindows/tests/fixtures/batch2/g05`
  - `impl-plans/active/airwindows-batch2-n03-g05.md`
  - `target`
  - `tmp/airwindows-batch2/ab2-n03`
- **sharedPaths**: none.
- **artifactRoots**: `target`, `tmp/airwindows-batch2/ab2-n03`.

## File-level Changes

- `g05/mod.rs`: 6 `mod` lines and 6 `ENTRY` items in the order above.
- `g05/<module>.rs` per common section 5. Imitate `batch1/g03/beam.rs`.
- `tests/parity_batch2_g05.rs`: 6 records and tests, plus mode-sweep tests where required.
  Imitate `tests/parity_batch1_g03.rs`.

## Plugin Pitfalls

- **Ensemble**: a multi-voice modulated delay.
  - Large delay arrays become `Box<[T]>`.
  - Keep the per-voice `sin`/`cos` precision (float vs double) and the interpolation order.
- **EQ**: many band filters with sample-rate-dependent coefficients computed per block. Keep
  per-block coefficient updates at the same point as upstream.
- **EveryTrim**: dB-style trims. Use the right `display.rs` helper (`db2string` vs `float2string`)
  per parameter.
- **FathomFive**: a sub-bass generator with state flags. Port the boolean/int state transitions
  exactly.

## Verification

Use common section 9 with P = `ab2-n03` and G = `g05`. Logs go to `tmp/airwindows-batch2/ab2-n03/logs/`.

1. `$ENV cargo check -p music-things-airwindows --all-targets`: exit 0.
2. `$ENV cargo nextest run -p music-things-airwindows --test parity_batch2_g05`: exit 0,
   testsRun >= 6, failures 0.
3. `find crates/airwindows/src/plugins/batch2/g05 crates/airwindows/tests/parity_batch2_g05.rs -name '*.rs' -print0 | xargs -0 rustfmt --edition 2021 --check`:
   exit 0.
4. `test -z "$($ENV cargo clippy -p music-things-airwindows --all-targets --message-format short -- -D warnings 2>&1 | grep -E 'src/plugins/batch2/g05/|tests/parity_batch2_g05\.rs')"`:
   exit 0.
5. `test -z "$(find crates/airwindows/src/plugins/batch2/g05 -name '*.rs' -exec wc -l {} + | awk '$2 != "total" && $1 >= 1000')"`:
   exit 0.
6. `test "$(grep -c '::ENTRY' crates/airwindows/src/plugins/batch2/g05/mod.rs)" -eq 6`: exit 0.
7. `! grep -rnE 'todo!|unimplemented!|#\[ignore' crates/airwindows/src/plugins/batch2/g05 crates/airwindows/tests/parity_batch2_g05.rs`:
   exit 0.
8. Diagnostic: `$ENV cargo nextest run -p music-things-airwindows --test no_alloc --test catalog`.
   It must show no failure naming a g05 plugin.

## Completion Criteria

- [ ] All 6 plugins ported, registered in order (cmd 6) and passing parity (cmd 2).
- [ ] Each `ParityRecord` and module header records the class and measured max-abs.
- [ ] Commands 1-7 exit 0, with logs. Command 8 shows no g05 failure.
- [ ] Only writePaths changed (pre/post hashes). Progress Log is updated per common section 11.

## Progress Log

### Session: 2026-11-01
**Tasks Completed**: Plan created (not started).
