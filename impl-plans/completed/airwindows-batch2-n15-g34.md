# Airwindows Batch 2 / n15: Group g34 (kick lines 214-220)

**Status**: Completed
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

- [x] All 7 plugins ported, registered in order (cmd 6) and passing parity (cmd 2).
- [x] Each `ParityRecord` and module header records the class and measured max-abs.
- [x] Commands 1-7 exit 0, with logs. Command 8 shows no g34 failure.
- [x] Only writePaths changed (pre/post hashes). Progress Log is updated per common section 11.

## Progress Log

### Session: 2026-11-01
**Tasks Completed**: Plan created (not started).

### Session: 2026-11-01 implementation

All seven plugins are registered and covered by fixture parity: Pockey, Pockey2, Podcast, PodcastDeluxe, Point, Pop and Pop2 each have Libm class and measured max-abs 0 on arm64 Darwin. Pockey's upstream is u-law encoding and decimation; the plan hint was imprecise. Pop owns boxed delay buffers reset in place.

- Cargo check: exit 0, `tmp/airwindows-batch2/ab2-n15/logs/check-final.log`.
- Group nextest: exit 0, 7 passed, 0 failed, `tmp/airwindows-batch2/ab2-n15/logs/parity-final-02.log`.
- Own rustfmt, line count, ENTRY count, no-stub and no-dead-code gates: exit 0; logs `rustfmt-final-02.log`, `lines-final.log`, `entries-final.log`, `stubs-final.log`, `dead-code-final.log` under the plan logs directory.
- Own-file clippy grep: exit 0, `clippy-own-final-03.log`; full clippy exits 101 on concurrent sibling files, recorded in `clippy-full-final-03.log`, and is not this plan's gate.
- no_alloc/catalog diagnostic: exit 0, 4 passed, 0 failed, `diagnostic-final-02.log`.
- Scoped pre/post hashes and per-edit intent notes are under `tmp/airwindows-batch2/ab2-n15/`. The shared working tree was moving during verification; serial integration must recheck combined state. Formal review and finalization remain downstream.

Self-check: replaced exact equality against the measured values with the Libm 1e-6 class bound, retaining the zero measurements in records and the earlier exact-comparison calibration log `parity-final-02.log`. Final-source reruns: check exit 0 (`check-final-02.log`), parity 7 passed (`parity-final-03.log`), no_alloc/catalog 4 passed (`diagnostic-final-03.log`), own rustfmt, clippy grep, line, registration, no-stub and no-dead-code gates all exit 0 (`rustfmt-final-03.log`, `clippy-gate-final-02.log`, `lines-final-02.log`, `entries-final-02.log`, `stubs-final-02.log`, `dead-code-final-02.log`).

### Session: 2026-11-02 verification

The existing seven registered ports and parity tests were retained. Fresh checks on the shared working tree passed: cargo check (0), parity (7 passed), no_alloc/catalog diagnostic (4 passed), and scoped rustfmt, clippy, line count, registration, no-stub and no-dead-code gates (all 0). Complete logs are under `tmp/airwindows-batch2/ab2-n15/logs/redispatch-*.log`. All seven retain Libm class with measured max-abs 0. Formal review and final integration remain downstream; the shared tree can change concurrently.

### Session: 2026-11-13 (operator integration)
**Tasks Completed**: Closed by operator integration after review-loop session-629 stopped as implementation-blocked (ab2-n01 DubSub, ab2-n04 write-scope evidence). Merged main (batch 3) into the branch and ran the combined-tree gates: nextest 481/481 (3 pre-existing SoundFont skips), clippy -D warnings, fmt --check and the wasm32 build all exit 0.
**Notes**: Checkboxes left unticked were superseded by the combined-tree gates above, not run individually.
