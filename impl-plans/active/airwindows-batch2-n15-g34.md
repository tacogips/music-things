# Airwindows Batch 2 / n15: Group g34 (kick lines 214-220)

**Status**: Ready
**Plan ID**: ab2-n15
**Wave**: W2. Depends on ab2-00-scaffold; runs in parallel with all other W2 plans.
**Design Reference**: `design-docs/specs/design-airwindows-batch2.md` sections 2, 3, 5 and 7 (Port plans)
**Common Contract**: `impl-plans/active/airwindows-batch2-common.md` (mandatory: sections 5-11)
**Created**: 2026-11-01

## Intent

Port, register and parity-test these 7 plugins in kick-line order (modules in parentheses):

Pockey (`pockey`), Pockey2 (`pockey2`), Podcast (`podcast`), PodcastDeluxe (`podcastdeluxe`),
Point (`point`), Pop (`pop`), Pop2 (`pop2`).

Pop2 (line 220) is the last batch-2 plugin.

Every listed plugin must be fully ported, registered in `g34/mod.rs` and parity-tested. A missing,
stubbed or unregistered plugin is a failure of this plan, not a deferral.

## Non-goals

No other group, no framework/shared-test/tools/fixture edits (fixtures only via common section 10),
no `processDoubleReplacing`.

## Write Ownership

- **writePaths**:
  - `crates/airwindows/src/plugins/batch2/g34`
  - `crates/airwindows/tests/parity_batch2_g34.rs`
  - `crates/airwindows/tests/fixtures/batch2/g34`
  - `impl-plans/active/airwindows-batch2-n15-g34.md`
  - `target`
  - `tmp/airwindows-batch2/ab2-n15`
- **sharedPaths**: none.
- **artifactRoots**: `target`, `tmp/airwindows-batch2/ab2-n15`.

## File-level Changes

- `g34/mod.rs`: 7 `mod` lines and 7 `ENTRY` items in the order above.
- `g34/<module>.rs` per common section 5. Imitate `batch1/g03/beam.rs`.
- `tests/parity_batch2_g34.rs`: 7 records and tests, plus mode-sweep tests where required.

## Plugin Pitfalls

- **Pockey, Pockey2**: bit-crush/decimation with `floor`, integer hold counters and a DeRez-style
  rate. Keep the integer/float mix and the counter wrap exactly.
- **PodcastDeluxe**: several cascaded delay-tap compressors with fixed arrays. Keep the tap indices
  and the counter decrement order.
- **Podcast**: a two-stage compressor. Keep the state update order.
- **Point**: a transient designer with nibble/nobble state. Keep the alternating-sample (`fpFlip`)
  logic.
- **Pop, Pop2**: compressors with `muVary`/`muAttack` style state and sample-rate-scaled attack. The
  comparison operators must be exact.

## Verification

Use common section 9 with P = `ab2-n15` and G = `g34`. Logs go to `tmp/airwindows-batch2/ab2-n15/logs/`.

1. `$ENV cargo check -p music-things-airwindows --all-targets`: exit 0.
2. `$ENV cargo nextest run -p music-things-airwindows --test parity_batch2_g34`: exit 0,
   testsRun >= 7, failures 0.
3. `find crates/airwindows/src/plugins/batch2/g34 crates/airwindows/tests/parity_batch2_g34.rs -name '*.rs' -print0 | xargs -0 rustfmt --edition 2021 --check`:
   exit 0.
4. `test -z "$($ENV cargo clippy -p music-things-airwindows --all-targets --message-format short -- -D warnings 2>&1 | grep -E 'src/plugins/batch2/g34/|tests/parity_batch2_g34\.rs')"`:
   exit 0.
5. `test -z "$(find crates/airwindows/src/plugins/batch2/g34 -name '*.rs' -exec wc -l {} + | awk '$2 != "total" && $1 >= 1000')"`:
   exit 0.
6. `test "$(grep -c '::ENTRY' crates/airwindows/src/plugins/batch2/g34/mod.rs)" -eq 7`: exit 0.
7. `! grep -rnE 'todo!|unimplemented!|#\[ignore' crates/airwindows/src/plugins/batch2/g34 crates/airwindows/tests/parity_batch2_g34.rs`:
   exit 0.
8. Diagnostic: `$ENV cargo nextest run -p music-things-airwindows --test no_alloc --test catalog`.
   It must show no failure naming a g34 plugin.

## Completion Criteria

- [ ] All 7 plugins ported, registered in order (cmd 6) and passing parity (cmd 2).
- [ ] Each `ParityRecord` and module header records the class and measured max-abs.
- [ ] Commands 1-7 exit 0, with logs. Command 8 shows no g34 failure.
- [ ] Only writePaths changed (pre/post hashes). Progress Log is updated per common section 11.

## Progress Log

### Session: 2026-11-01
**Tasks Completed**: Plan created (not started).
