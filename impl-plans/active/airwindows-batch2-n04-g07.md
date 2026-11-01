# Airwindows Batch 2 / n04: Group g07 (kick lines 135-139)

**Status**: Ready
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

- [ ] All 5 plugins ported, registered in order (cmd 6) and passing parity (cmd 2).
- [ ] Each `ParityRecord` and module header records the class and measured max-abs.
- [ ] Commands 1-7 exit 0, with logs. Command 8 shows no g07 failure.
- [ ] Only writePaths changed (pre/post hashes). Progress Log is updated per common section 11.

## Progress Log

### Session: 2026-11-01
**Tasks Completed**: Plan created (not started).
