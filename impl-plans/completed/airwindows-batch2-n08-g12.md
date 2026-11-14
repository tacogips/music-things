# Airwindows Batch 2 / n08: Group g12 (kick lines 160-165)

**Status**: Completed
**Plan ID**: ab2-n08
**Wave**: W2. Depends on ab2-00-scaffold; runs in parallel with all other W2 plans.
**Design Reference**: `design-docs/specs/design-airwindows-batch2.md` sections 2, 3, 5 and 7 (Port plans)
**Common Contract**: `impl-plans/active/airwindows-batch2-common.md` (mandatory: sections 5-11)
**Created**: 2026-11-01

## Intent

Port, register and parity-test these 6 plugins in kick-line order (modules in parentheses):

Hype (`hype`), Hypersonic (`hypersonic`), HypersonX (`hypersonx`), Infinity (`infinity`),
Infinity2 (`infinity2`), Infrasonic (`infrasonic`).

Every listed plugin must be fully ported, registered in `g12/mod.rs` and parity-tested. A missing,
stubbed or unregistered plugin is a failure of this plan, not a deferral.

## Non-goals

No other group, no framework/shared-test/tools/fixture edits (fixtures only via common section 10),
no `processDoubleReplacing`.

## Write Ownership

- **writePaths**:
  - `crates/airwindows/src/plugins/batch2/g12`
  - `crates/airwindows/tests/parity_batch2_g12.rs`
  - `crates/airwindows/tests/fixtures/batch2/g12`
  - `impl-plans/active/airwindows-batch2-n08-g12.md`
  - `target`
  - `tmp/airwindows-batch2/ab2-n08`
- **sharedPaths**: none.
- **artifactRoots**: `target`, `tmp/airwindows-batch2/ab2-n08`.

## File-level Changes

- `g12/mod.rs`: 6 `mod` lines and 6 `ENTRY` items in the order above.
- `g12/<module>.rs` per common section 5. A port that would reach 900 lines becomes a module
  directory.
- `tests/parity_batch2_g12.rs`: 6 records and tests, plus mode-sweep tests where required.

## Plugin Pitfalls

- **Infinity, Infinity2**: reverbs with many delay arrays.
  - Arrays over 4096 elements become `Box<[T]>`.
  - Keep the feedback matrix order and the sample-rate scaling (`cycle`/`cycleEnd` integer logic)
    exactly.
- **Hypersonic, HypersonX, Infrasonic**: cascaded biquad arrays (`fixA..fixG` style) with
  coefficients computed from the sample rate.
  - Keep the biquad index constants (for example `fix_freq`, `fix_reso`, `fix_a0`, ...) and the
    computation order.
- **Hype**: a multi-stage softening pipeline with fixed filters.

## Verification

Use common section 9 with P = `ab2-n08` and G = `g12`. Logs go to `tmp/airwindows-batch2/ab2-n08/logs/`.

1. `$ENV cargo check -p music-things-airwindows --all-targets`: exit 0.
2. `$ENV cargo nextest run -p music-things-airwindows --test parity_batch2_g12`: exit 0,
   testsRun >= 6, failures 0.
3. `find crates/airwindows/src/plugins/batch2/g12 crates/airwindows/tests/parity_batch2_g12.rs -name '*.rs' -print0 | xargs -0 rustfmt --edition 2021 --check`:
   exit 0.
4. `test -z "$($ENV cargo clippy -p music-things-airwindows --all-targets --message-format short -- -D warnings 2>&1 | grep -E 'src/plugins/batch2/g12/|tests/parity_batch2_g12\.rs')"`:
   exit 0.
5. `test -z "$(find crates/airwindows/src/plugins/batch2/g12 -name '*.rs' -exec wc -l {} + | awk '$2 != "total" && $1 >= 1000')"`:
   exit 0.
6. `test "$(grep -c '::ENTRY' crates/airwindows/src/plugins/batch2/g12/mod.rs)" -eq 6`: exit 0.
7. `! grep -rnE 'todo!|unimplemented!|#\[ignore' crates/airwindows/src/plugins/batch2/g12 crates/airwindows/tests/parity_batch2_g12.rs`:
   exit 0.
8. Diagnostic: `$ENV cargo nextest run -p music-things-airwindows --test no_alloc --test catalog`.
   It must show no failure naming a g12 plugin.

## Completion Criteria

- [x] All 6 plugins ported, registered in order (cmd 6) and passing parity (cmd 2).
- [x] Each `ParityRecord` and module header records the class and measured max-abs.
- [x] Commands 1-7 exit 0, with logs. Command 8 shows no g12 failure.
- [x] Plan-owned edits and pre/post hashes recorded; Progress Log updated per common section 11.

## Progress Log

### Session: 2026-11-01
**Tasks Completed**: Plan created (not started).

### Session: 2026-11-01 (implementation)
**Tasks Completed**: Hype, Hypersonic, HypersonX, Infinity, Infinity2 and Infrasonic;
each is Libm with measured max-abs 0.0 on aarch64-apple-darwin. HypersonX has a
seven-mode smoke sweep. All six entries are registered in kick order.
**Verification**: `logs/check-agent-final.log` exit 0; `logs/parity2.log` exit 0,
7 tests passed; `logs/rustfmt2.log`, `logs/clippy-gate.log`, `logs/lines2.log`,
`logs/entries2.log`, `logs/stubs2.log`, `logs/deadcode2.log` exit 0. Diagnostic
`logs/diagnostic.log` exit 0, 4 tests passed. Logs are under
`tmp/airwindows-batch2/ab2-n08/`. `pre-hashes.txt` and `post-hashes.txt` record
owned files; per-edit intents are under `intents/`. These checks ran on the shared
moving tree. Formal integrity and adversarial review remain downstream.

### Session: 2026-11-13 (operator integration)
**Tasks Completed**: Closed by operator integration after review-loop session-629 stopped as implementation-blocked (ab2-n01 DubSub, ab2-n04 write-scope evidence). Merged main (batch 3) into the branch and ran the combined-tree gates: nextest 481/481 (3 pre-existing SoundFont skips), clippy -D warnings, fmt --check and the wasm32 build all exit 0.
**Notes**: Checkboxes left unticked were superseded by the combined-tree gates above, not run individually.
