# Airwindows Batch 2 / n13: Group g31 (kick lines 201-206)

**Status**: Ready
**Plan ID**: ab2-n13
**Wave**: W2. Depends on ab2-00-scaffold; runs in parallel with all other W2 plans.
**Design Reference**: `design-docs/specs/design-airwindows-batch2.md` sections 2, 3, 5 and 7 (Port plans)
**Common Contract**: `impl-plans/active/airwindows-batch2-common.md` (mandatory: sections 5-11)
**Created**: 2026-11-01

## Intent

Port, register and parity-test these 6 plugins in kick-line order (modules in parentheses):

NotJustAnotherCD (`notjustanothercd`), NotJustAnotherDither (`notjustanotherdither`),
OneCornerClip (`onecornerclip`), Pafnuty (`pafnuty`), PaulDither (`pauldither`),
PaulWide (`paulwide`).

Every listed plugin must be fully ported, registered in `g31/mod.rs` and parity-tested. A missing,
stubbed or unregistered plugin is a failure of this plan, not a deferral.

## Non-goals

No other group, no framework/shared-test/tools/fixture edits (fixtures only via common section 10),
no `processDoubleReplacing`.

## Write Ownership

- **writePaths**:
  - `crates/airwindows/src/plugins/batch2/g31`
  - `crates/airwindows/tests/parity_batch2_g31.rs`
  - `crates/airwindows/tests/fixtures/batch2/g31`
  - `impl-plans/active/airwindows-batch2-n13-g31.md`
  - `target`
  - `tmp/airwindows-batch2/ab2-n13`
- **sharedPaths**: none.
- **artifactRoots**: `target`, `tmp/airwindows-batch2/ab2-n13`.

## File-level Changes

- `g31/mod.rs`: 6 `mod` lines and 6 `ENTRY` items in the order above.
- `g31/<module>.rs` per common section 5. Imitate `batch1/g03/beam.rs`, which is also a
  wordlength reducer.
- `tests/parity_batch2_g31.rs`: 6 records and tests, plus mode-sweep tests where required.

## Plugin Pitfalls

- **NotJustAnotherCD, NotJustAnotherDither**: Benford-style noise-shaped quantizers.
  - Large per-digit `bynL[13]` state and `log10`/`pow` calls on doubles.
  - Keep the integer digit extraction and `floor` exactly.
- **PaulDither, PaulWide**: TPDF-like dither from `rand()` or fpd.
  - Draw `rand()` from `AwRand` in exact call order.
  - Keep the previous-sample state.
- **Pafnuty**: Chebyshev polynomial harmonics with about 13 params.
  - R1 sets every param to `ALT[i % 9]`, so loop over all params.
  - Keep the polynomial evaluation order; do not refactor it into Horner form.
- **OneCornerClip**: clip with sample-rate-scaled constants and an output display in dB.

## Verification

Use common section 9 with P = `ab2-n13` and G = `g31`. Logs go to `tmp/airwindows-batch2/ab2-n13/logs/`.

1. `$ENV cargo check -p music-things-airwindows --all-targets`: exit 0.
2. `$ENV cargo nextest run -p music-things-airwindows --test parity_batch2_g31`: exit 0,
   testsRun >= 6, failures 0.
3. `find crates/airwindows/src/plugins/batch2/g31 crates/airwindows/tests/parity_batch2_g31.rs -name '*.rs' -print0 | xargs -0 rustfmt --edition 2021 --check`:
   exit 0.
4. `test -z "$($ENV cargo clippy -p music-things-airwindows --all-targets --message-format short -- -D warnings 2>&1 | grep -E 'src/plugins/batch2/g31/|tests/parity_batch2_g31\.rs')"`:
   exit 0.
5. `test -z "$(find crates/airwindows/src/plugins/batch2/g31 -name '*.rs' -exec wc -l {} + | awk '$2 != "total" && $1 >= 1000')"`:
   exit 0.
6. `test "$(grep -c '::ENTRY' crates/airwindows/src/plugins/batch2/g31/mod.rs)" -eq 6`: exit 0.
7. `! grep -rnE 'todo!|unimplemented!|#\[ignore' crates/airwindows/src/plugins/batch2/g31 crates/airwindows/tests/parity_batch2_g31.rs`:
   exit 0.
8. Diagnostic: `$ENV cargo nextest run -p music-things-airwindows --test no_alloc --test catalog`.
   It must show no failure naming a g31 plugin.

## Completion Criteria

- [ ] All 6 plugins ported, registered in order (cmd 6) and passing parity (cmd 2).
- [ ] Each `ParityRecord` and module header records the class and measured max-abs.
- [ ] Commands 1-7 exit 0, with logs. Command 8 shows no g31 failure.
- [ ] Only writePaths changed (pre/post hashes). Progress Log is updated per common section 11.

## Progress Log

### Session: 2026-11-01
**Tasks Completed**: Plan created (not started).
