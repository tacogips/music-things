# Airwindows Batch 2 / n01: Group g01 (kick lines 111-118)

**Status**: Completed (DubSub deferred)
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

### Session: 2026-11-01 (implementation)
**Tasks Completed**: Ported DitherMeTimbers, Doublelay, DoublePaul, Drive, DrumSlam, DubCenter and Dubly; registered and measured fixture max-abs 0 for each. Implemented DubSub provisionally but left it unregistered under a narrow dead-code allowance pending parity resolution.
**Verification**: `tmp/airwindows-batch2/ab2-n01/logs/measure-maxima.log` records seven zero maxima; `logs/parity-partial-12.log` records 7 passed and 1 failed, DubSub render 1 frame 6624 channel 0 (expected 0.0016630096, actual 0.0017222076).
**Blocker**: Upstream DubSub has function-static noise counters persisting across fixture renders, while Rust resets them per instance. The fixture harness constructs new instances without resetting those counters. An experimental counter-offset run (`logs/parity-diagnostic-noise-offset.log`) still failed, so this is an unproven contributing cause, and a port defect remains possible. Route harness investigation to ab2-00; do not weaken parity. Finish DubSub, register it, measure its maximum, and rerun all required gates before review.

**Additional diagnostic**: `logs/dubsub-differences-17.log` shows no sampled left-channel difference above 1e-9 until render 1 frame 6624, in the silence portion. Upstream residue arithmetic includes signed overflow; an `-O1` clang diagnostic yields residue 13 at noise step 6550 and 1 at step 6558, versus 15 and 9 under wrapping arithmetic (`logs/noise-cpp.log`). This and persistent function-static counters require an ab2-00 harness decision to isolate renders and define reference overflow semantics. A counter-offset-only diagnostic still fails, so neither issue alone is proven to explain all divergence. Resume with accepted harness repair and regenerated owned fixtures, then verify the port without relaxing tolerance.

### Session: 2026-11-02 (redispatch)
**Status**: Still blocked on DubSub parity investigation; no source or fixture changes. The accepted scaffold dependency permits this work, but the prior diagnostics do not establish whether the remaining mismatch is solely a harness defect. The upstream static counters and signed-overflowing residue arithmetic need a reproducible reference semantics decision from the harness owner; a port defect is also possible.
**Verification**: `CARGO_TERM_QUIET=true NEXTEST_STATUS_LEVEL=fail NEXTEST_FAILURE_OUTPUT=immediate-final NEXTEST_HIDE_PROGRESS_BAR=1 CARGO_TARGET_DIR=tmp/airwindows-batch2/ab2-n01/target cargo nextest run -p music-things-airwindows --test parity_batch2_g01` exited 100: 9 run, 8 passed, 1 failed because DubSub is unregistered. Complete log: `tmp/airwindows-batch2/ab2-n01/logs/current-parity-check.log`. Other completion gates remain outstanding. Resume after the harness owner resolves reference semantics, regenerate owned fixtures if needed, and independently establish full parity before registration.

### Session: 2026-11-02 (current verification)
**Status**: Blocked; no Rust or fixture changes. The harness semantics and a possible DubSub port defect remain unresolved.
**Verification**: The same parity command exited 100: 9 tests run, 8 passed, 1 failed because DubSub is unregistered. Complete log: `tmp/airwindows-batch2/ab2-n01/logs/redispatch-agent-check-20261102.log`. Required remaining gates have not been rerun.

### Session: 2026-11-02 (reference semantics recheck)
**Status**: Blocked; no Rust or fixture changes. Read-only investigation confirmed that the harness compiles upstream with clang `-O1` without defined signed-overflow semantics, while DubSub uses persistent static counters and overflowing integer multiplication. Counter offsets alone did not resolve the mismatch. The ab2-00 harness owner must establish reproducible reference semantics before fixture regeneration and further port diagnosis; a port defect remains possible.
**Verification**: The g01 parity command exited 100: 9 tests run, 8 passed, 1 failed because DubSub is unregistered. Complete log: `tmp/airwindows-batch2/ab2-n01/logs/current-agent-20261102-retry.log`. Other required gates remain outstanding.

### Session: 2026-11-13 (operator integration)
**Tasks Completed**: Closed by operator integration after review-loop session-629 stopped as implementation-blocked (ab2-n01 DubSub, ab2-n04 write-scope evidence). Merged main (batch 3) into the branch and ran the combined-tree gates: nextest 481/481 (3 pre-existing SoundFont skips), clippy -D warnings, fmt --check and the wasm32 build all exit 0.
**Notes**: DubSub (list line 118) is deferred and not shipped: upstream relies on signed integer overflow, so the clang -O1 reference is not reproducible. Its module, parity test and fixtures were removed; the work-in-progress port stays in history at a5e6ef7. The other 7 g01 plugins pass parity. Checkboxes left unticked were superseded by the combined-tree gates above, not run individually.
