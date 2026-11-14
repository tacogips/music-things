# Airwindows Batch 2 / n12: Groups g23, g25, g27 and g29 (lines 189, 191-192, 194-196, 198-199)

**Status**: Completed
**Plan ID**: ab2-n12
**Wave**: W2. Depends on ab2-00-scaffold; runs in parallel with all other W2 plans.
**Design Reference**: `design-docs/specs/design-airwindows-batch2.md` sections 2, 3, 5 and 7 (Port plans)
**Common Contract**: `impl-plans/active/airwindows-batch2-common.md` (mandatory: sections 5-11)
**Created**: 2026-11-01

## Intent

Port, register and parity-test these 8 plugins (modules in parentheses):

- g23: Monitoring2 (`monitoring2`)
- g25: MoNoam (`monoam`), MultiBandDistortion (`multibanddistortion`)
- g27: NaturalizeDither (`naturalizedither`), NCSeventeen (`ncseventeen`), Neverland (`neverland`)
- g29: NodeDither (`nodedither`), Noise (`noise`)

The oversized Monitoring3 (g24), MV (g26), Nikola (g28) and NonlinearSpace (g30) sit between these
groups and belong to their own plans.

Every listed plugin must be fully ported, registered in its group's `mod.rs` and parity-tested. A
missing, stubbed or unregistered plugin is a failure of this plan, not a deferral.

## Non-goals

No other group (in particular not g22, g24, g26, g28 or g30), no framework/shared-test/tools/fixture
edits, no `processDoubleReplacing`.

## Write Ownership

- **writePaths**:
  - `crates/airwindows/src/plugins/batch2/g23`
  - `crates/airwindows/src/plugins/batch2/g25`
  - `crates/airwindows/src/plugins/batch2/g27`
  - `crates/airwindows/src/plugins/batch2/g29`
  - `crates/airwindows/tests/parity_batch2_g23.rs`
  - `crates/airwindows/tests/parity_batch2_g25.rs`
  - `crates/airwindows/tests/parity_batch2_g27.rs`
  - `crates/airwindows/tests/parity_batch2_g29.rs`
  - `crates/airwindows/tests/fixtures/batch2/g23`
  - `crates/airwindows/tests/fixtures/batch2/g25`
  - `crates/airwindows/tests/fixtures/batch2/g27`
  - `crates/airwindows/tests/fixtures/batch2/g29`
  - `impl-plans/active/airwindows-batch2-n12-g23-g25-g27-g29.md`
  - `target`
  - `tmp/airwindows-batch2/ab2-n12`
- **sharedPaths**: none.
- **artifactRoots**: `target`, `tmp/airwindows-batch2/ab2-n12`.

## File-level Changes

- Group `mod.rs` files: g23 1, g25 2, g27 3, g29 2 entries, in the order above.
- One module per plugin, per common section 5. Monitoring2 and MultiBandDistortion become module
  directories if they would reach 900 lines.
- One parity test file per group (1, 2, 3 and 2 records and tests), plus mode-sweep tests where
  required.

## Plugin Pitfalls

- **Monitoring2**: a large mode selector (many monitoring modes such as subs, slew, peaks, mono and
  side, plus several dither types). Port every mode branch and add a mode-sweep test. If the file
  nears 900 lines, split the modes into a submodule.
- **MultiBandDistortion**: crossover IIRs plus per-band drive. Keep the band split order.
- **NaturalizeDither, NodeDither**: dither plugins.
  - Integer and `floor` quantization exactly.
  - NodeDither has a delay array indexed by a node-count parameter. `Box` it if it is over 4096
    elements, and keep the index wrap.
  - There may be no `fpd`.
- **Noise**: a `rand()`/fpd-driven noise generator with filter state. Draw every `rand()` from
  `AwRand` in order. Several params are mode-like; add a mode-sweep test if one is an integer
  selector.
- **Neverland, NCSeventeen**: console-style saturation with `sin`/`pow` and sample-rate-scaled
  constants. Check the precision.

## Verification

Use common section 9 with P = `ab2-n12` and G = `g23`, `g25`, `g27`, `g29`. Logs go to
`tmp/airwindows-batch2/ab2-n12/logs/`.

1. `$ENV cargo check -p music-things-airwindows --all-targets`: exit 0.
2. `$ENV cargo nextest run -p music-things-airwindows --test parity_batch2_g23 --test parity_batch2_g25 --test parity_batch2_g27 --test parity_batch2_g29`:
   exit 0, testsRun >= 8, failures 0.
3. `find crates/airwindows/src/plugins/batch2/g23 crates/airwindows/src/plugins/batch2/g25 crates/airwindows/src/plugins/batch2/g27 crates/airwindows/src/plugins/batch2/g29 crates/airwindows/tests/parity_batch2_g23.rs crates/airwindows/tests/parity_batch2_g25.rs crates/airwindows/tests/parity_batch2_g27.rs crates/airwindows/tests/parity_batch2_g29.rs -name '*.rs' -print0 | xargs -0 rustfmt --edition 2021 --check`:
   exit 0.
4. `test -z "$($ENV cargo clippy -p music-things-airwindows --all-targets --message-format short -- -D warnings 2>&1 | grep -E 'src/plugins/batch2/(g23|g25|g27|g29)/|tests/parity_batch2_(g23|g25|g27|g29)\.rs')"`:
   exit 0.
5. `test -z "$(find crates/airwindows/src/plugins/batch2/g23 crates/airwindows/src/plugins/batch2/g25 crates/airwindows/src/plugins/batch2/g27 crates/airwindows/src/plugins/batch2/g29 -name '*.rs' -exec wc -l {} + | awk '$2 != "total" && $1 >= 1000')"`:
   exit 0.
6. `test "$(grep -c '::ENTRY' crates/airwindows/src/plugins/batch2/g23/mod.rs)" -eq 1 && test "$(grep -c '::ENTRY' crates/airwindows/src/plugins/batch2/g25/mod.rs)" -eq 2 && test "$(grep -c '::ENTRY' crates/airwindows/src/plugins/batch2/g27/mod.rs)" -eq 3 && test "$(grep -c '::ENTRY' crates/airwindows/src/plugins/batch2/g29/mod.rs)" -eq 2`:
   exit 0.
7. `! grep -rnE 'todo!|unimplemented!|#\[ignore' crates/airwindows/src/plugins/batch2/g23 crates/airwindows/src/plugins/batch2/g25 crates/airwindows/src/plugins/batch2/g27 crates/airwindows/src/plugins/batch2/g29 crates/airwindows/tests/parity_batch2_g23.rs crates/airwindows/tests/parity_batch2_g25.rs crates/airwindows/tests/parity_batch2_g27.rs crates/airwindows/tests/parity_batch2_g29.rs`:
   exit 0.
8. Diagnostic: `$ENV cargo nextest run -p music-things-airwindows --test no_alloc --test catalog`.
   It must show no failure naming one of the 8 plugins.

## Completion Criteria

- [x] All 8 plugins ported, registered (cmd 6) and passing parity (cmd 2).
- [x] Each `ParityRecord` and module header records the class and measured max-abs.
- [x] Commands 1-7 exit 0, with logs. Command 8 shows no failure for this plan's plugins.
- [x] Only writePaths changed (pre/post hashes). Progress Log is updated per common section 11.

## Progress Log

### Session: 2026-11-01
**Tasks Completed**: Plan created (not started).

### Session: 2026-11-01 — implementation
**Tasks Completed**: Ported and registered all eight plugins. Each module and parity record reports Libm max-abs 0.0 on its fixture. Added selector sweeps for Monitoring2, MoNoam and Noise.
**Evidence**: Focused nextest logs `tmp/airwindows-batch2/ab2-n12/logs/g23-parity.log`, `g25-parity-2.log`, `g27-parity-3.log`, `g29-parity-4.log`; own-file clippy on the combined current tree `own-clippy-final.log` exited 0. Noise required matching pinned clang's overflow/remainder behavior; initial failure retained in `g29-parity-2.log`.
**Remaining**: Rerun the combined final-source gates, diagnostic and scope audit. Formal review follows implementation.

### Session: 2026-11-01 — combined verification
**Tasks Completed**: Gates 1–7 and 7b passed on the combined source; parity ran 11 tests with 11 passed and diagnostic ran 4 tests with 4 passed. Complete logs are `tmp/airwindows-batch2/ab2-n12/logs/final2-*.log`.
**Remaining**: Scope audit and formal downstream review.

### Session: 2026-11-01 — scope evidence
**Evidence**: Owned source pre/post hashes are in `tmp/airwindows-batch2/ab2-n12/scope-hashes.json`; `logs/scope-fixtures.log` compares 16 owned fixture files with no changes. Author edit intentions 001–031 are retained under `tmp/airwindows-batch2/ab2-n12/intents/`; no edits outside assigned writePaths were made by this implementation.
**Remaining**: Serial integration review must audit the combined shared tree.

### Session: 2026-11-13 (operator integration)
**Tasks Completed**: Closed by operator integration after review-loop session-629 stopped as implementation-blocked (ab2-n01 DubSub, ab2-n04 write-scope evidence). Merged main (batch 3) into the branch and ran the combined-tree gates: nextest 481/481 (3 pre-existing SoundFont skips), clippy -D warnings, fmt --check and the wasm32 build all exit 0.
**Notes**: Checkboxes left unticked were superseded by the combined-tree gates above, not run individually.
