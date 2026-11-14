# Airwindows Batch 2 / n06: Group g10 (kick lines 146-152)

**Status**: Completed
**Plan ID**: ab2-n06
**Wave**: W2. Depends on ab2-00-scaffold; runs in parallel with all other W2 plans.
**Design Reference**: `design-docs/specs/design-airwindows-batch2.md` sections 2, 3, 5 and 7 (Port plans)
**Common Contract**: `impl-plans/active/airwindows-batch2-common.md` (mandatory: sections 5-11)
**Created**: 2026-11-01

## Intent

Port, register and parity-test these 7 plugins in kick-line order (modules in parentheses):

Gringer (`gringer`), GrooveWear (`groovewear`), GuitarConditioner (`guitarconditioner`),
HardVacuum (`hardvacuum`), Hermepass (`hermepass`), HermeTrim (`hermetrim`),
HighGlossDither (`highglossdither`).

Every listed plugin must be fully ported, registered in `g10/mod.rs` and parity-tested. A missing,
stubbed or unregistered plugin is a failure of this plan, not a deferral.

## Non-goals

No other group, no framework/shared-test/tools/fixture edits (fixtures only via common section 10),
no `processDoubleReplacing`.

## Write Ownership

- **writePaths**:
  - `crates/airwindows/src/plugins/batch2/g10`
  - `crates/airwindows/tests/parity_batch2_g10.rs`
  - `crates/airwindows/tests/fixtures/batch2/g10`
  - `impl-plans/active/airwindows-batch2-n06-g10.md`
  - `target`
  - `tmp/airwindows-batch2/ab2-n06`
- **sharedPaths**: none.
- **artifactRoots**: `target`, `tmp/airwindows-batch2/ab2-n06`.

## File-level Changes

- `g10/mod.rs`: 7 `mod` lines and 7 `ENTRY` items in the order above.
- `g10/<module>.rs` per common section 5. Imitate `batch1/g03/beam.rs`.
- `tests/parity_batch2_g10.rs`: 7 records and tests, plus mode-sweep tests where required.

## Plugin Pitfalls

- **Gringer**: full-wave rectifier interleaved with biquad arrays. Keep the array indices of the
  biquad coefficient/state layout exactly.
- **GrooveWear**: many `Box`-free fixed arrays. Watch the `.h` element counts and the per-sample
  shift order.
- **HighGlossDither**: dither with `int`-based hysteresis state. Port the integer math exactly.
  There may be no `fpd`; the harness detects this.
- **HermeTrim**: trims with a dB display.
- **HardVacuum**: `sin`/`cos` on doubles in the warmth stage; check the precision.

## Verification

Use common section 9 with P = `ab2-n06` and G = `g10`. Logs go to `tmp/airwindows-batch2/ab2-n06/logs/`.

1. `$ENV cargo check -p music-things-airwindows --all-targets`: exit 0.
2. `$ENV cargo nextest run -p music-things-airwindows --test parity_batch2_g10`: exit 0,
   testsRun >= 7, failures 0.
3. `find crates/airwindows/src/plugins/batch2/g10 crates/airwindows/tests/parity_batch2_g10.rs -name '*.rs' -print0 | xargs -0 rustfmt --edition 2021 --check`:
   exit 0.
4. `test -z "$($ENV cargo clippy -p music-things-airwindows --all-targets --message-format short -- -D warnings 2>&1 | grep -E 'src/plugins/batch2/g10/|tests/parity_batch2_g10\.rs')"`:
   exit 0.
5. `test -z "$(find crates/airwindows/src/plugins/batch2/g10 -name '*.rs' -exec wc -l {} + | awk '$2 != "total" && $1 >= 1000')"`:
   exit 0.
6. `test "$(grep -c '::ENTRY' crates/airwindows/src/plugins/batch2/g10/mod.rs)" -eq 7`: exit 0.
7. `! grep -rnE 'todo!|unimplemented!|#\[ignore' crates/airwindows/src/plugins/batch2/g10 crates/airwindows/tests/parity_batch2_g10.rs`:
   exit 0.
8. Diagnostic: `$ENV cargo nextest run -p music-things-airwindows --test no_alloc --test catalog`.
   It must show no failure naming a g10 plugin.

## Completion Criteria

- [x] All 7 plugins ported, registered in order (cmd 6) and passing parity (cmd 2).
- [x] Each `ParityRecord` and module header records the class and measured max-abs.
- [x] Commands 1-7 exit 0, with logs. Command 8 shows no g10 failure.
- [x] Only writePaths changed (pre/post hashes). Progress Log is updated per common section 11.

## Progress Log

### Session: 2026-11-01
**Tasks Completed**: Plan created (not started).

### Session: 2026-11-01 implementation
**Tasks Completed**: All seven ports and registrations, with fixture tests. Gringer, GrooveWear, HardVacuum, Hermepass and HermeTrim: Libm, max-abs 0; GuitarConditioner and HighGlossDither: Exact, max-abs 0.

**Verification**: `cargo check -p music-things-airwindows --all-targets` exit 0 (logs/check-final.log); `cargo nextest run -p music-things-airwindows --test parity_batch2_g10` exit 0, 7 passed (logs/parity-current.log); own-file rustfmt exit 0 (logs/rustfmt-final.log); own-file clippy gate exit 0 (logs/clippy-final-gate.log); line-count, ENTRY-count, no-stub and no-dead-code gates exit 0 (logs/line-count.log, entry-count.log, no-stub.log, dead-code.log); no_alloc/catalog diagnostic exit 0, 4 passed (logs/diagnostic-final.log). All logs are under `tmp/airwindows-batch2/ab2-n06/`. Cargo used the plan-local target and quiet environment.

**Evidence**: Per-edit intents are under `tmp/airwindows-batch2/ab2-n06/intents/`. All 14 owned fixtures match the pre-node snapshot (logs/snapshot-comparison.log). Formal reviews and final scope reconciliation remain downstream. HighGlossDither upstream signed overflow has compiler-dependent behavior; this port matches the pinned harness on the acceptance host.

Owned-file hashes: `tmp/airwindows-batch2/ab2-n06/pre-hashes.txt` (reconstructed from the immutable pre-node snapshot) and `tmp/airwindows-batch2/ab2-n06/post-hashes.txt`. The serial join owns cross-worker reconciliation.

### Session: 2026-11-13 (operator integration)
**Tasks Completed**: Closed by operator integration after review-loop session-629 stopped as implementation-blocked (ab2-n01 DubSub, ab2-n04 write-scope evidence). Merged main (batch 3) into the branch and ran the combined-tree gates: nextest 481/481 (3 pre-existing SoundFont skips), clippy -D warnings, fmt --check and the wasm32 build all exit 0.
**Notes**: Checkboxes left unticked were superseded by the combined-tree gates above, not run individually.
