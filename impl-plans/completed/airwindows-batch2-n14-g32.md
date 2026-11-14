# Airwindows Batch 2 / n14: Group g32 (kick lines 207-212)

**Status**: Completed
**Plan ID**: ab2-n14
**Wave**: W2. Depends on ab2-00-scaffold; runs in parallel with all other W2 plans.
**Design Reference**: `design-docs/specs/design-airwindows-batch2.md` sections 2, 3, 5 and 7 (Port plans)
**Common Contract**: `impl-plans/active/airwindows-batch2-common.md` (mandatory: sections 5-11)
**Created**: 2026-11-01

## Intent

Port, register and parity-test these 6 plugins in kick-line order (modules in parentheses):

PDBuss (`pdbuss`), PDChannel (`pdchannel`), PeaksOnly (`peaksonly`), PhaseNudge (`phasenudge`),
PitchDelay (`pitchdelay`), PitchNasty (`pitchnasty`).

Every listed plugin must be fully ported, registered in `g32/mod.rs` and parity-tested. A missing,
stubbed or unregistered plugin is a failure of this plan, not a deferral.

## Non-goals

No other group, no framework/shared-test/tools/fixture edits (fixtures only via common section 10),
no `processDoubleReplacing`.

## Write Ownership

- **writePaths**:
  - `crates/airwindows/src/plugins/batch2/g32`
  - `crates/airwindows/tests/parity_batch2_g32.rs`
  - `crates/airwindows/tests/fixtures/batch2/g32`
  - `impl-plans/active/airwindows-batch2-n14-g32.md`
  - `target`
  - `tmp/airwindows-batch2/ab2-n14`
- **sharedPaths**: none.
- **artifactRoots**: `target`, `tmp/airwindows-batch2/ab2-n14`.

## File-level Changes

- `g32/mod.rs`: 6 `mod` lines and 6 `ENTRY` items in the order above.
- `g32/<module>.rs` per common section 5. PitchDelay becomes a module directory if it would reach
  900 lines.
- `tests/parity_batch2_g32.rs`: 6 records and tests, plus mode-sweep tests where required.

## Plugin Pitfalls

- **PitchDelay**: a pitch-shifting delay with large buffers, fractional read positions and
  regeneration.
  - `Box` the buffers.
  - Keep the `double` read-position arithmetic, the integer truncation of the read index and the
    interpolation order exactly.
  - This plugin is the one most likely to need a shim symbol. If the fixture is missing or wrong,
    escalate per common section 10.
- **PitchNasty**: a glitchy pitch shifter with a buffer and splice logic. Same index-precision
  concerns as PitchDelay.
- **PeaksOnly**: a cascade of short fixed delays (`aL..eL` arrays) with sign flips. Keep the
  per-stage index wrap.
- **PhaseNudge**: per-sample phase smear arrays (`d[]`, `dp`). Keep the wrap.
- **PDBuss, PDChannel**: console-style `sin`/`asin` saturation. Check the precision and the clamp
  to `[-1, 1]` before `asin`.

## Verification

Use common section 9 with P = `ab2-n14` and G = `g32`. Logs go to `tmp/airwindows-batch2/ab2-n14/logs/`.

1. `$ENV cargo check -p music-things-airwindows --all-targets`: exit 0.
2. `$ENV cargo nextest run -p music-things-airwindows --test parity_batch2_g32`: exit 0,
   testsRun >= 6, failures 0.
3. `find crates/airwindows/src/plugins/batch2/g32 crates/airwindows/tests/parity_batch2_g32.rs -name '*.rs' -print0 | xargs -0 rustfmt --edition 2021 --check`:
   exit 0.
4. `test -z "$($ENV cargo clippy -p music-things-airwindows --all-targets --message-format short -- -D warnings 2>&1 | grep -E 'src/plugins/batch2/g32/|tests/parity_batch2_g32\.rs')"`:
   exit 0.
5. `test -z "$(find crates/airwindows/src/plugins/batch2/g32 -name '*.rs' -exec wc -l {} + | awk '$2 != "total" && $1 >= 1000')"`:
   exit 0.
6. `test "$(grep -c '::ENTRY' crates/airwindows/src/plugins/batch2/g32/mod.rs)" -eq 6`: exit 0.
7. `! grep -rnE 'todo!|unimplemented!|#\[ignore' crates/airwindows/src/plugins/batch2/g32 crates/airwindows/tests/parity_batch2_g32.rs`:
   exit 0.
8. Diagnostic: `$ENV cargo nextest run -p music-things-airwindows --test no_alloc --test catalog`.
   It must show no failure naming a g32 plugin.

## Completion Criteria

- [x] All 6 plugins ported, registered in order (cmd 6) and passing parity (cmd 2).
- [x] Each `ParityRecord` and module header records the class and measured max-abs.
- [x] Commands 1-7 exit 0, with logs. Command 8 shows no g32 failure.
- [x] Only writePaths changed (pre/post hashes). Progress Log is updated per common section 11.

## Progress Log

### Session: 2026-11-01
**Tasks Completed**: Plan created (not started).

### Session: 2026-11-01 implementation
**Tasks Completed**: Six float-path ports and ordered registration; fixture parity 6/6, each Libm max-abs 0.0. Scoped check, rustfmt, clippy, line, entry, stub and dead-code gates passed; no_alloc/catalog diagnostic 4/4 passed. Logs and per-edit intentions: `tmp/airwindows-batch2/ab2-n14/`. Pre-node hashes derive from the runtime snapshot. Owned-path pre/post hashes and per-edit intentions are recorded; serial review must reconcile concurrent sibling changes. Formal review and finalization remain downstream.

### Session: 2026-11-02 verification
**Tasks Completed**: Rechecked the existing six ports and plan alignment. Scoped gates passed; parity 6/6 and no_alloc/catalog 4/4 passed. An initial parity build encountered a transient sibling LeadAmp compile error; the retry passed. Fresh logs: `tmp/airwindows-batch2/ab2-n14/logs/recheck-*.log`. Checks ran on the shared moving tree; formal review and finalization remain downstream.

### Session: 2026-11-13 (operator integration)
**Tasks Completed**: Closed by operator integration after review-loop session-629 stopped as implementation-blocked (ab2-n01 DubSub, ab2-n04 write-scope evidence). Merged main (batch 3) into the branch and ran the combined-tree gates: nextest 481/481 (3 pre-existing SoundFont skips), clippy -D warnings, fmt --check and the wasm32 build all exit 0.
**Notes**: Checkboxes left unticked were superseded by the combined-tree gates above, not run individually.
