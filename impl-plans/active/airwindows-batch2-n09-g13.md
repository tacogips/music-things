# Airwindows Batch 2 / n09: Group g13 (kick lines 166-171)

**Status**: Ready
**Plan ID**: ab2-n09
**Wave**: W2. Depends on ab2-00-scaffold; runs in parallel with all other W2 plans.
**Design Reference**: `design-docs/specs/design-airwindows-batch2.md` sections 2, 3, 5 and 7 (Port plans)
**Common Contract**: `impl-plans/active/airwindows-batch2-common.md` (mandatory: sections 5-11)
**Created**: 2026-11-01

## Intent

Port, register and parity-test these 6 plugins in kick-line order (modules in parentheses):

Interstage (`interstage`), IronOxide5 (`ironoxide5`), IronOxideClassic (`ironoxideclassic`),
IronOxideClassic2 (`ironoxideclassic2`), Isolator (`isolator`), Isolator2 (`isolator2`).

Every listed plugin must be fully ported, registered in `g13/mod.rs` and parity-tested. A missing,
stubbed or unregistered plugin is a failure of this plan, not a deferral.

## Non-goals

No other group, no framework/shared-test/tools/fixture edits (fixtures only via common section 10),
no `processDoubleReplacing`.

## Write Ownership

- **writePaths**:
  - `crates/airwindows/src/plugins/batch2/g13`
  - `crates/airwindows/tests/parity_batch2_g13.rs`
  - `crates/airwindows/tests/fixtures/batch2/g13`
  - `impl-plans/active/airwindows-batch2-n09-g13.md`
  - `target`
  - `tmp/airwindows-batch2/ab2-n09`
- **sharedPaths**: none.
- **artifactRoots**: `target`, `tmp/airwindows-batch2/ab2-n09`.

## File-level Changes

- `g13/mod.rs`: 6 `mod` lines and 6 `ENTRY` items in the order above.
- `g13/<module>.rs` per common section 5. A port that would reach 900 lines becomes a module
  directory.
- `tests/parity_batch2_g13.rs`: 6 records and tests, plus mode-sweep tests where required.

## Plugin Pitfalls

- **IronOxide5, IronOxideClassic, IronOxideClassic2**: tape emulation with flutter.
  - The `d[]` delay arrays and `gcount` wrap.
  - Flutter uses `sin`/`cos` on doubles, plus `rand()` or fpd in some versions; draw `rand()` from
    `AwRand` in order.
  - Sample-rate-dependent IIR constants.
  - Keep the `fastIIR`/`slowIIR` alternating-sample (`fpFlip`) state.
- **Isolator, Isolator2**: biquad cascades with coefficients from params. Keep the biquad array
  layout indices.
- **Interstage**: alternating-sample state and fixed filters.

## Verification

Use common section 9 with P = `ab2-n09` and G = `g13`. Logs go to `tmp/airwindows-batch2/ab2-n09/logs/`.

1. `$ENV cargo check -p music-things-airwindows --all-targets`: exit 0.
2. `$ENV cargo nextest run -p music-things-airwindows --test parity_batch2_g13`: exit 0,
   testsRun >= 6, failures 0.
3. `find crates/airwindows/src/plugins/batch2/g13 crates/airwindows/tests/parity_batch2_g13.rs -name '*.rs' -print0 | xargs -0 rustfmt --edition 2021 --check`:
   exit 0.
4. `test -z "$($ENV cargo clippy -p music-things-airwindows --all-targets --message-format short -- -D warnings 2>&1 | grep -E 'src/plugins/batch2/g13/|tests/parity_batch2_g13\.rs')"`:
   exit 0.
5. `test -z "$(find crates/airwindows/src/plugins/batch2/g13 -name '*.rs' -exec wc -l {} + | awk '$2 != "total" && $1 >= 1000')"`:
   exit 0.
6. `test "$(grep -c '::ENTRY' crates/airwindows/src/plugins/batch2/g13/mod.rs)" -eq 6`: exit 0.
7. `! grep -rnE 'todo!|unimplemented!|#\[ignore' crates/airwindows/src/plugins/batch2/g13 crates/airwindows/tests/parity_batch2_g13.rs`:
   exit 0.
8. Diagnostic: `$ENV cargo nextest run -p music-things-airwindows --test no_alloc --test catalog`.
   It must show no failure naming a g13 plugin.

## Completion Criteria

- [ ] All 6 plugins ported, registered in order (cmd 6) and passing parity (cmd 2).
- [ ] Each `ParityRecord` and module header records the class and measured max-abs.
- [ ] Commands 1-7 exit 0, with logs. Command 8 shows no g13 failure.
- [ ] Only writePaths changed (pre/post hashes). Progress Log is updated per common section 11.

## Progress Log

### Session: 2026-11-01
**Tasks Completed**: Plan created (not started).
