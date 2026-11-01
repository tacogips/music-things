# Airwindows Batch 2 / n05: Group g08 (kick lines 140-144)

**Status**: Ready
**Plan ID**: ab2-n05
**Wave**: W2. Depends on ab2-00-scaffold; runs in parallel with all other W2 plans.
**Design Reference**: `design-docs/specs/design-airwindows-batch2.md` sections 2, 3, 5 and 7 (Port plans)
**Common Contract**: `impl-plans/active/airwindows-batch2-common.md` (mandatory: sections 5-11)
**Created**: 2026-11-01

## Intent

Port, register and parity-test these 5 plugins in kick-line order (modules in parentheses):

FromTape (`fromtape`), Galactic (`galactic`), Gatelope (`gatelope`),
GlitchShifter (`glitchshifter`), Golem (`golem`).

Every listed plugin must be fully ported, registered in `g08/mod.rs` and parity-tested. A missing,
stubbed or unregistered plugin is a failure of this plan, not a deferral.

## Non-goals

No other group, no framework/shared-test/tools/fixture edits (fixtures only via common section 10),
no `processDoubleReplacing`.

## Write Ownership

- **writePaths**:
  - `crates/airwindows/src/plugins/batch2/g08`
  - `crates/airwindows/tests/parity_batch2_g08.rs`
  - `crates/airwindows/tests/fixtures/batch2/g08`
  - `impl-plans/active/airwindows-batch2-n05-g08.md`
  - `target`
  - `tmp/airwindows-batch2/ab2-n05`
- **sharedPaths**: none.
- **artifactRoots**: `target`, `tmp/airwindows-batch2/ab2-n05`.

## File-level Changes

- `g08/mod.rs`: 5 `mod` lines and 5 `ENTRY` items in the order above.
- `g08/<module>.rs` per common section 5. A port that would reach 900 lines becomes a module
  directory, as `batch1/g03/bigamp/` does.
- `tests/parity_batch2_g08.rs`: 5 records and tests, plus mode-sweep tests where required.

## Plugin Pitfalls

- **Galactic**: a large reverb.
  - Many delay arrays (a..m, L/R) become `Box<[T]>` sized as in `.h`.
  - The delay lengths are scaled by sample rate. Keep the exact integer truncation, and keep the
    `count` wrap checks.
  - Expect a module directory if it nears the limit.
- **GlitchShifter**: a pitch/glitch buffer with `rand()`-free but stateful splice logic.
  - Keep integer index arithmetic (`int` vs `double`) exactly.
  - `Box` the large buffer.
- **Gatelope**: gate plus envelope filter state machine. Keep the comparison operators exactly.
- **FromTape**: tape emulation. Check fpd/`rand()` usage.
- **Golem**: a two-mic phase/offset tool with a fractional delay and a mode selector. Port every
  mode and add a mode-sweep test.

## Verification

Use common section 9 with P = `ab2-n05` and G = `g08`. Logs go to `tmp/airwindows-batch2/ab2-n05/logs/`.

1. `$ENV cargo check -p music-things-airwindows --all-targets`: exit 0.
2. `$ENV cargo nextest run -p music-things-airwindows --test parity_batch2_g08`: exit 0,
   testsRun >= 5, failures 0.
3. `find crates/airwindows/src/plugins/batch2/g08 crates/airwindows/tests/parity_batch2_g08.rs -name '*.rs' -print0 | xargs -0 rustfmt --edition 2021 --check`:
   exit 0.
4. `test -z "$($ENV cargo clippy -p music-things-airwindows --all-targets --message-format short -- -D warnings 2>&1 | grep -E 'src/plugins/batch2/g08/|tests/parity_batch2_g08\.rs')"`:
   exit 0.
5. `test -z "$(find crates/airwindows/src/plugins/batch2/g08 -name '*.rs' -exec wc -l {} + | awk '$2 != "total" && $1 >= 1000')"`:
   exit 0.
6. `test "$(grep -c '::ENTRY' crates/airwindows/src/plugins/batch2/g08/mod.rs)" -eq 5`: exit 0.
7. `! grep -rnE 'todo!|unimplemented!|#\[ignore' crates/airwindows/src/plugins/batch2/g08 crates/airwindows/tests/parity_batch2_g08.rs`:
   exit 0.
8. Diagnostic: `$ENV cargo nextest run -p music-things-airwindows --test no_alloc --test catalog`.
   It must show no failure naming a g08 plugin.

## Completion Criteria

- [ ] All 5 plugins ported, registered in order (cmd 6) and passing parity (cmd 2).
- [ ] Each `ParityRecord` and module header records the class and measured max-abs.
- [ ] Commands 1-7 exit 0, with logs. Command 8 shows no g08 failure.
- [ ] Only writePaths changed (pre/post hashes). Progress Log is updated per common section 11.

## Progress Log

### Session: 2026-11-01
**Tasks Completed**: Plan created (not started).
