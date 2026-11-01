# Airwindows Batch 2 / n01: Group g01 (kick lines 111-118)

**Status**: Ready
**Plan ID**: ab2-n01
**Wave**: W2. Depends on ab2-00-scaffold; runs in parallel with all other W2 plans.
**Design Reference**: `design-docs/specs/design-airwindows-batch2.md` sections 2, 3, 5 and 7 (Port plans)
**Common Contract**: `impl-plans/active/airwindows-batch2-common.md` (mandatory: sections 5-11)
**Created**: 2026-11-01

## Intent

Port, register and parity-test these 8 plugins in kick-line order (modules in parentheses):

DitherMeTimbers (`dithermetimbers`), Doublelay (`doublelay`), DoublePaul (`doublepaul`),
Drive (`drive`), DrumSlam (`drumslam`), DubCenter (`dubcenter`), Dubly (`dubly`), DubSub (`dubsub`).

Every listed plugin must be fully ported, registered in `g01/mod.rs` and parity-tested. A missing,
stubbed or unregistered plugin is a failure of this plan, not a deferral.

## Non-goals

No other group, no framework/shared-test/tools/fixture edits (fixtures only via common section 10),
no `processDoubleReplacing`.

## Write Ownership

- **writePaths**:
  - `crates/airwindows/src/plugins/batch2/g01`
  - `crates/airwindows/tests/parity_batch2_g01.rs`
  - `crates/airwindows/tests/fixtures/batch2/g01`
  - `impl-plans/active/airwindows-batch2-n01-g01.md`
  - `target`
  - `tmp/airwindows-batch2/ab2-n01`
- **sharedPaths**: none.
- **artifactRoots**: `target`, `tmp/airwindows-batch2/ab2-n01`.

## File-level Changes

- `g01/mod.rs`: 8 `mod` lines and 8 `ENTRY` items in the order above. Replace the scaffold's empty
  `ENTRIES`.
- `g01/<module>.rs` for each plugin, per common section 5. Imitate `batch1/g03/beam.rs`.
- `tests/parity_batch2_g01.rs`: 8 `ParityRecord`s and 8 tests, plus mode-sweep tests where common
  section 7 requires them. Imitate `tests/parity_batch1_g03.rs`.

## Plugin Pitfalls

- **DitherMeTimbers**: a dither/wordlength plugin.
  - Port its integer/floor quantization exactly. A float-to-int cast truncates; `floor` is not
    `round`.
  - Check whether it uses `rand()` or fpd in `processReplacing`. If `rand()`, draw from the
    struct's `AwRand`.
- **Doublelay**: delay buffers.
  - Arrays over 4096 elements become `Box<[T]>`.
  - Keep the upstream wrap and decrement order of the delay counters exactly.
- **DubSub, DubCenter**: many IIR state variables and sample-rate-scaled coefficients. Watch the
  float vs double member types declared in `.h`.
- **DrumSlam, Drive, Dubly, DoublePaul**: check each libm call's argument precision.

## Verification

Use common section 9 with P = `ab2-n01` and G = `g01`. Logs go to `tmp/airwindows-batch2/ab2-n01/logs/`.

1. `$ENV cargo check -p music-things-airwindows --all-targets`: exit 0.
2. `$ENV cargo nextest run -p music-things-airwindows --test parity_batch2_g01`: exit 0,
   testsRun >= 8, failures 0.
3. `find crates/airwindows/src/plugins/batch2/g01 crates/airwindows/tests/parity_batch2_g01.rs -name '*.rs' -print0 | xargs -0 rustfmt --edition 2021 --check`:
   exit 0.
4. `test -z "$($ENV cargo clippy -p music-things-airwindows --all-targets --message-format short -- -D warnings 2>&1 | grep -E 'src/plugins/batch2/g01/|tests/parity_batch2_g01\.rs')"`:
   exit 0.
5. `test -z "$(find crates/airwindows/src/plugins/batch2/g01 -name '*.rs' -exec wc -l {} + | awk '$2 != "total" && $1 >= 1000')"`:
   exit 0.
6. `test "$(grep -c '::ENTRY' crates/airwindows/src/plugins/batch2/g01/mod.rs)" -eq 8`: exit 0.
7. `! grep -rnE 'todo!|unimplemented!|#\[ignore' crates/airwindows/src/plugins/batch2/g01 crates/airwindows/tests/parity_batch2_g01.rs`:
   exit 0.
8. Diagnostic: `$ENV cargo nextest run -p music-things-airwindows --test no_alloc --test catalog`.
   It must show no failure naming a g01 plugin.

## Completion Criteria

- [ ] All 8 plugins ported, registered in order (cmd 6) and passing parity (cmd 2).
- [ ] Each `ParityRecord` and module header records the class and measured max-abs.
- [ ] Commands 1-7 exit 0, with logs. Command 8 shows no g01 failure.
- [ ] Only writePaths changed (pre/post hashes). Progress Log is updated per common section 11.

## Progress Log

### Session: 2026-11-01
**Tasks Completed**: Plan created (not started).
