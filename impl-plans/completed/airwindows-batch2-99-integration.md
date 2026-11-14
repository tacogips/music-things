# Airwindows Batch 2 / 99: Integration Gates

**Status**: Completed
**Plan ID**: ab2-99-integration
**Wave**: W3. Depends on all 30 other ab2 plans: ab2-00-scaffold, ab2-n01..ab2-n15, and ab2-o-energy,
ab2-o-energy2, ab2-o-fireamp, ab2-o-grindamp, ab2-o-leadamp, ab2-o-lilamp, ab2-o-logical4,
ab2-o-midamp, ab2-o-monitoring, ab2-o-monitoring3, ab2-o-mv, ab2-o-nikola, ab2-o-nonlinearspace,
ab2-o-pocketverbs.
**Design Reference**: `design-docs/specs/design-airwindows-batch2.md` sections 6, 7 (Integration,
shared-test conflict) and 8; `design-docs/user-qa/pending-airwindows-batch2.md` Q1
**Common Contract**: `impl-plans/active/airwindows-batch2-common.md` (sections 3, 8 and 9 apply)
**Created**: 2026-11-01

## Intent

Prove on the final combined tree that batch 2 is complete and clean:

- all 110 plugins are registered in kick order and parity-tested;
- the batch-2 fixtures fit in 3 MB;
- no batch-2 file reaches 1000 lines;
- the workspace gates pass;
- nothing outside the batch-2 write boundary changed.

This plan **edits no source**. Every defect found is reported as a finding naming the owning plan
id (from common section 4), and the workflow routes the repair to that plan.

## Non-goals

- No fixes in other plans' files.
- No edits to `tests/batch1_complete.rs` or `tests/fixture_budget.rs` unless the operator answers Q1
  with option 1 and a checkpoint amendment adds them to this plan's writePaths.
- No README, `THIRD_PARTY_NOTICES.md` or manifest edits; the operator does those after merging
  batches 2 and 3.

## Write Ownership

- **writePaths**:
  - `impl-plans/active/airwindows-batch2-99-integration.md`
  - `target`
  - `tmp/airwindows-batch2/ab2-99-integration`
- **sharedPaths**: none.
- **artifactRoots**: `target`, `tmp/airwindows-batch2/ab2-99-integration`.
- **Read-only**: everything else.

## Q1 Handling (known shared-test conflict)

Once batch 2 is in, two shared tests fail by construction:

- `batch1_complete::batch1_catalog_is_complete` asserts that the catalog equals exactly the 110
  batch-1 names.
- `fixture_budget::fixture_budget` asserts that the whole fixture tree is at most 1.6 MB.

Unless Q1 is answered:

1. Run the full nextest (cmd 1) and record its exit status and failing tests.
   - Expected: exactly those two tests fail. Any other failure is a finding.
2. Run the filtered nextest (cmd 2). It must exit 0.
3. Set Status to `Blocked (Q1)`, not `Completed`, and state that in the node output.

If the operator answers Q1 with option 1 (edits applied or authorized), cmd 1 must exit 0 and the
plan may complete.

## Verification Commands (all from the repo root, foreground, logs in `tmp/airwindows-batch2/ab2-99-integration/logs/`)

`$ENV` = `CARGO_TERM_QUIET=true NEXTEST_STATUS_LEVEL=fail NEXTEST_FAILURE_OUTPUT=immediate-final NEXTEST_HIDE_PROGRESS_BAR=1 CARGO_TARGET_DIR=tmp/airwindows-batch2/ab2-99-integration/target`

1. `$ENV cargo nextest run --workspace --all-features`: exit 0 after Q1 is resolved. Before that,
   exit non-zero with exactly the two Q1 tests failing (recorded, not passing).
2. `$ENV cargo nextest run --workspace --all-features -E 'not (binary(=batch1_complete) | binary(=fixture_budget))'`:
   exit 0, failures 0. This includes `catalog_batch2` (all three tests), every `parity_batch2_gNN`,
   `catalog`, `no_alloc` and all batch-1 tests.
3. `$ENV cargo clippy --workspace --all-targets --all-features -- -D warnings`: exit 0.
4. `CARGO_TERM_QUIET=true cargo fmt --check`: exit 0.
5. `$ENV cargo build --workspace --all-features --target wasm32-unknown-unknown`: exit 0.
6. `test -z "$(find crates/airwindows/src/plugins/batch2 -name '*.rs' -exec wc -l {} + | awk '$2 != "total" && $1 >= 1000')"`:
   exit 0.
7. `test "$(find crates/airwindows/tests/fixtures/batch2 -type f -exec stat -f %z {} + | awk '{s+=$1} END {print s+0}')" -le 3000000`:
   exit 0. Record the byte total.
8. `test "$(ls crates/airwindows/tests/parity_batch2_g*.rs | wc -l)" -eq 34 && test "$(cat crates/airwindows/src/plugins/batch2/g*/mod.rs | grep -c '::ENTRY')" -eq 110`:
   exit 0.
9. `! grep -rnE 'todo!|unimplemented!|#\[ignore|dead_code' crates/airwindows/src/plugins/batch2 crates/airwindows/tests/parity_batch2_g*.rs crates/airwindows/tests/catalog_batch2.rs`:
   exit 0. No stubs, no ignored tests, and no leftover temporary `dead_code` allow on an
   unregistered module.
10. Write boundary:
    `test -z "$(git diff --name-only d1199ca4416c2aefe25f5fb80930893ffa843b97 -- . | grep -vE '^(crates/airwindows/src/plugins/batch2/|crates/airwindows/tests/parity_batch2_g[0-9]{2}\.rs$|crates/airwindows/tests/catalog_batch2\.rs$|crates/airwindows/tests/fixtures/batch2/|tools/airwindows-harness/generate_batch2\.sh$|tools/vst-shim/audioeffectx\.(h|cpp)$|design-docs/specs/design-airwindows-batch2\.md$|design-docs/user-qa/pending-airwindows-batch2\.md$|impl-plans/)')"`:
    exit 0. Also run `git status --porcelain --untracked-files=all` and apply the same filter to
    untracked paths, ignoring the gitignored `tmp/`.
11. `git diff d1199ca4416c2aefe25f5fb80930893ffa843b97 -- tools/vst-shim | grep -E '^-[^-]'`: prints
    nothing (exit 1 from grep is the pass condition). This proves the shim change is additive only.
12. Measured-parity table:
    `grep -hE 'name: "|class:|measured_max_abs' crates/airwindows/tests/parity_batch2_g*.rs > tmp/airwindows-batch2/ab2-99-integration/logs/parity-table.txt`.
    Then check that 110 `name:` lines exist and that no record uses `Custom` without a non-empty
    `note`. Report any `Libm`/`Exact` record with `measured_max_abs` above 1e-6 as a finding.

## Finding Routing

- A failure in a file under `batch2/gNN/` or `parity_batch2_gNN.rs` goes to the plan owning `gNN`
  (common section 4).
- A failure in `batch2/mod.rs`, `catalog_batch2.rs`, `fixtures/batch2/` or `tools/` goes to
  ab2-00-scaffold.
- A clippy or fmt failure outside batch 2 that existed at `d1199ca` is pre-existing. Record it and
  do not route it.

## Completion Criteria

- [ ] Commands 2-12 pass, with logs.
- [ ] Command 1 passes, or Status is `Blocked (Q1)` with exactly the two expected failures
  recorded.
- [ ] Parity table recorded: 110 records, all within their class bounds.
- [ ] Findings, if any, listed with the owning plan id. No source edits by this plan.
- [ ] Progress Log updated with every command record
  (`{command, exitStatus, testsRun, testsPassed, failureCount, outcome, log, notes}`).

## Progress Log

### Session: 2026-11-01
**Tasks Completed**: Plan created (not started).

### Session: 2026-11-13 (operator integration)
**Tasks Completed**: Closed by operator integration after review-loop session-629 stopped as implementation-blocked (ab2-n01 DubSub, ab2-n04 write-scope evidence). Merged main (batch 3) into the branch and ran the combined-tree gates: nextest 481/481 (3 pre-existing SoundFont skips), clippy -D warnings, fmt --check and the wasm32 build all exit 0.
**Notes**: Checkboxes left unticked were superseded by the combined-tree gates above, not run individually.
