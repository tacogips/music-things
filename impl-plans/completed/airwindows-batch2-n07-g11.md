# Airwindows Batch 2 / n07: Group g11 (kick lines 153-159)

**Status**: Completed
**Plan ID**: ab2-n07
**Wave**: W2. Depends on ab2-00-scaffold; runs in parallel with all other W2 plans.
**Design Reference**: `design-docs/specs/design-airwindows-batch2.md` sections 2, 3, 5 and 7 (Port plans)
**Common Contract**: `impl-plans/active/airwindows-batch2-common.md` (mandatory: sections 5-11)
**Created**: 2026-11-01

## Intent

Port, register and parity-test these 7 plugins in kick-line order (modules in parentheses):

HighImpact (`highimpact`), Highpass (`highpass`), Highpass2 (`highpass2`), Holt (`holt`),
Holt2 (`holt2`), Hombre (`hombre`), Hull (`hull`).

Every listed plugin must be fully ported, registered in `g11/mod.rs` and parity-tested. A missing,
stubbed or unregistered plugin is a failure of this plan, not a deferral.

## Non-goals

No other group, no framework/shared-test/tools/fixture edits (fixtures only via common section 10),
no `processDoubleReplacing`.

## Write Ownership

- **writePaths**:
  - `crates/airwindows/src/plugins/batch2/g11`
  - `crates/airwindows/tests/parity_batch2_g11.rs`
  - `crates/airwindows/tests/fixtures/batch2/g11`
  - `impl-plans/active/airwindows-batch2-n07-g11.md`
  - `target`
  - `tmp/airwindows-batch2/ab2-n07`
- **sharedPaths**: none.
- **artifactRoots**: `target`, `tmp/airwindows-batch2/ab2-n07`.

## File-level Changes

- `g11/mod.rs`: 7 `mod` lines and 7 `ENTRY` items in the order above.
- `g11/<module>.rs` per common section 5. Imitate `batch1/g03/beam.rs`.
- `tests/parity_batch2_g11.rs`: 7 records and tests, plus mode-sweep tests where required.

## Plugin Pitfalls

- **Holt, Holt2**: Holt-Winters double-exponential filters. Holt2 has many poles.
  - The pole count is an integer derived from a parameter; port the loop bounds exactly.
- **Hull**: a large circular buffer with a sample-rate-dependent cycle.
  - `Box` the buffer.
  - Keep the `hullp` counter and the wrap logic exactly.
- **Hombre**: a fixed short delay with a sample-rate-scaled length (integer truncation).
- **Highpass, Highpass2**: tight/loose IIR variants with alternating-sample state (`fpFlip`). Keep
  the flip toggle order.

## Verification

Use common section 9 with P = `ab2-n07` and G = `g11`. Logs go to `tmp/airwindows-batch2/ab2-n07/logs/`.

1. `$ENV cargo check -p music-things-airwindows --all-targets`: exit 0.
2. `$ENV cargo nextest run -p music-things-airwindows --test parity_batch2_g11`: exit 0,
   testsRun >= 7, failures 0.
3. `find crates/airwindows/src/plugins/batch2/g11 crates/airwindows/tests/parity_batch2_g11.rs -name '*.rs' -print0 | xargs -0 rustfmt --edition 2021 --check`:
   exit 0.
4. `test -z "$($ENV cargo clippy -p music-things-airwindows --all-targets --message-format short -- -D warnings 2>&1 | grep -E 'src/plugins/batch2/g11/|tests/parity_batch2_g11\.rs')"`:
   exit 0.
5. `test -z "$(find crates/airwindows/src/plugins/batch2/g11 -name '*.rs' -exec wc -l {} + | awk '$2 != "total" && $1 >= 1000')"`:
   exit 0.
6. `test "$(grep -c '::ENTRY' crates/airwindows/src/plugins/batch2/g11/mod.rs)" -eq 7`: exit 0.
7. `! grep -rnE 'todo!|unimplemented!|#\[ignore' crates/airwindows/src/plugins/batch2/g11 crates/airwindows/tests/parity_batch2_g11.rs`:
   exit 0.
8. Diagnostic: `$ENV cargo nextest run -p music-things-airwindows --test no_alloc --test catalog`.
   It must show no failure naming a g11 plugin.

## Completion Criteria

- [x] All 7 plugins ported, registered in order (cmd 6) and passing parity (cmd 2).
- [x] Each `ParityRecord` and module header records the class and measured max-abs.
- [x] Commands 1-7 exit 0, with logs. Command 8 shows no g11 failure.
- [x] Only writePaths changed (pre/post hashes). Progress Log is updated per common section 11.

## Progress Log

### Session: 2026-11-01
**Tasks Completed**: Plan created (not started).


### Session: 2026-11-01 implementation
**Tasks Completed**: All seven ports registered and tested. Highpass2: Exact, max-abs 0.0;
HighImpact, Highpass, Holt, Holt2, Hombre and Hull: Libm, max-abs 0.0, measured on arm64 Darwin.
**Verification**: Gates 1–7 and 7b exited 0; parity 7/7 passed. Diagnostic no_alloc/catalog
exited 0, 4/4 passed. Complete logs: `tmp/airwindows-batch2/ab2-n07/logs/final2-*.log`;
measurement: `tmp/airwindows-batch2/ab2-n07/logs/measurement.log`. Pre/post owned hashes and
per-edit intentions are in `tmp/airwindows-batch2/ab2-n07/`. The earlier clippy warning for
upstream approximate constants was resolved by a narrow function-level allowance.
**Notes**: Upstream Hull uses bPointer/cPointer rather than the hinted hullp; Holt and Holt2
use progressive stage wet controls rather than an integer pole selector. Formal review remains
downstream. Verification ran on the shared working tree.

### Session: 2026-11-13 (operator integration)
**Tasks Completed**: Closed by operator integration after review-loop session-629 stopped as implementation-blocked (ab2-n01 DubSub, ab2-n04 write-scope evidence). Merged main (batch 3) into the branch and ran the combined-tree gates: nextest 481/481 (3 pre-existing SoundFont skips), clippy -D warnings, fmt --check and the wasm32 build all exit 0.
**Notes**: Checkboxes left unticked were superseded by the combined-tree gates above, not run individually.
