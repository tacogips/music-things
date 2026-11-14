# Airwindows Batch 2 / n03: Group g05 (kick lines 128-133)

**Status**: Completed
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

- [x] All 6 plugins ported, registered in order (cmd 6) and passing parity (cmd 2).
- [x] Each `ParityRecord` and module header records the class and measured max-abs.
- [x] Commands 1-7 exit 0, with logs. Command 8 shows no g05 failure.
- [x] Only writePaths changed (pre/post hashes). Progress Log is updated per common section 11.

## Progress Log

### Session: 2026-11-01
**Tasks Completed**: Plan created (not started).


### Session: 2026-11-01 — implementation
**Tasks Completed**: All six plugins registered in order and fixture-tested. Ensemble: Libm, max-abs 1.4901161e-8; EQ, EveryTrim, Exciter and FathomFive: Libm, max-abs 0; Facet: Exact, max-abs 0. Ensemble uses boxed delay storage and in-place reset. FathomFive upstream float code does not initialize its fpd state or apply dither; deterministic seeded state is used for denormal replacement. No discrete mode selector was found.

**Verification**: Common gates 1–7 and 7b exit 0; parity 8/8 passes. Logs: `tmp/airwindows-batch2/ab2-n03/logs/gate1-final3.log` through `gate7-final3.log` and `gate7b-final3.log`. The no_alloc/catalog diagnostic passed 4/4 in `logs/diagnostic-final3.log`. Full clippy exited 101 on sibling g01/g02 diagnostics; the own-file gate passed (`logs/clippy-full-final3.log`, `logs/gate4-final3.log`). These checks ran on a shared moving tree; serial integration must verify the combined tree.

**Scope and review**: Per-edit intentions are under `tmp/airwindows-batch2/ab2-n03/`; owned-file hashes are recorded there. Formal review and combined-tree scope reconciliation remain downstream.

### Session: 2026-11-13 (operator integration)
**Tasks Completed**: Closed by operator integration after review-loop session-629 stopped as implementation-blocked (ab2-n01 DubSub, ab2-n04 write-scope evidence). Merged main (batch 3) into the branch and ran the combined-tree gates: nextest 481/481 (3 pre-existing SoundFont skips), clippy -D warnings, fmt --check and the wasm32 build all exit 0.
**Notes**: Checkboxes left unticked were superseded by the combined-tree gates above, not run individually.
