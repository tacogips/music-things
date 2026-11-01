# Airwindows Batch 3 15: Integration (catalog assertion, final gates, design record)

**Status**: Ready
**Plan ID**: aw-b3-15
**Design Reference**: design-docs/specs/design-airwindows-batch3.md (Tests: catalog_batch3.rs; Verification: Integration; Intentional divergences)
**Depends On**: aw-b3-00, aw-b3-01 .. aw-b3-14 (wave W3, serial, last)
**Created**: 2026-11-01
**Last Updated**: 2026-11-01

## Intent and Context

After all 14 group plans have landed, this plan does four things:

- It adds the batch-3 registration assertion `tests/catalog_batch3.rs`.
- It runs the crate-wide and workspace-wide gates. No group plan runs them, which avoids the batch-1
  shared-crate deadlock.
- It checks that the batch is complete (109 plugins, no stubs).
- It records the measured parity outcome in the design doc and the user-QA file.

**Completeness rule**: every one of the 109 batch-3 plugins must be fully ported, registered and
parity-tested. A missing, stubbed, skipped or blocked plugin is a failure of the batch. This plan must not
report success while one remains. It must not hide one by weakening an assertion. If such a plugin is
found, the gap is routed back to the owning group plan (see Failure handling).

## Non-goals

- Do not port or edit any plugin file, `gNN/mod.rs` or `parity_batch3_gNN.rs`. Group plans own those.
- Do not edit:
  - the framework files, `src/plugins/mod.rs`, `src/plugins/batch3/mod.rs`
  - `batch1/` or `batch2/`
  - `tests/catalog.rs`, `tests/common/`, `tests/fixture_budget.rs`, `tests/batch1_complete.rs`
  - `tools/`, the root `README.md`, `THIRD_PARTY_NOTICES.md`, `Cargo.toml`, `Cargo.lock`
- Do not run `cargo fmt` without `--check`, and do not run `cargo clippy --fix`. Formatting and lint fixes
  belong to the owning plans.
- Do not move plan files. The workflow's plan-archive node archives them, and the operator updates
  totals and notices after batch 2 merges.

## Write Ownership (fanout contract)

- writePaths:
  - `crates/airwindows/tests/catalog_batch3.rs`
  - `design-docs/specs/design-airwindows-batch3.md`
  - `design-docs/user-qa/pending-airwindows-batch3.md`
  - `impl-plans/active/airwindows-batch3-15-integration.md`
  - `target`
  - `tmp/airwindows-batch3/aw-b3-15`
- sharedPaths: none
- artifactRoots (digest-only): `target`, `tmp/airwindows-batch3/aw-b3-15`

## Common Execution Rules

- Evidence directory: `tmp/airwindows-batch3/aw-b3-15/`. It holds:
  - `intent.md`, written once and never edited
  - `pre-hashes.txt` and `post-hashes.txt`
  - a `logs/` directory
- Environment for every cargo command: `CARGO_TERM_QUIET=true CARGO_TARGET_DIR=tmp/airwindows-batch3/aw-b3-15/target NEXTEST_STATUS_LEVEL=fail NEXTEST_FAILURE_OUTPUT=immediate-final NEXTEST_HIDE_PROGRESS_BAR=1`.
- Re-read each file just before editing it. Never run git checkout, stash, reset or commit.

## Deliverables

### 1. `crates/airwindows/tests/catalog_batch3.rs` (new)

Imitate `crates/airwindows/tests/batch1_complete.rs`.

- `const BATCH3: [&str; 109]` lists the 109 names in kick-list order (lines 221-329), exactly as in the
  design Grouping table. Take the authoritative order from
  `sed -n '221,329p' "$MUSIC_THINGS_REFS/kick-airwindows-set.txt"`. If aw-b3-00 logged a TABLE MISMATCH,
  the list file wins.
- `#[test] fn batch3_catalog_is_complete()`:
  - Collect `catalog()` names.
  - Assert `names.len() >= BATCH3.len()`.
  - Assert that the last `BATCH3.len()` names equal `BATCH3`, in order. The check uses the suffix because
    `BATCHES` is ordered batch1, batch2, batch3, which keeps it valid before and after batch 2 merges.
  - Then, for each name:
    - `create(name, 48_000.0).is_some()`, with message `"failed to create {name}"`
    - `create_with_seed(name, 44_100.0, 1).is_some()`
    - the catalog entry's `upstream_name == name`
- Pitfalls:
  - Do not assert the total catalog length. Batch 2 will change it.
  - Do not use a `HashSet`-only comparison. Order matters.
  - `uLawDecode` and `uLawEncode` keep their lowercase `u`.

### 2. Design and user-QA record

- `design-docs/specs/design-airwindows-batch3.md`:
  - Add a short "Outcome" subsection under Verification. It records the per-class plugin counts and every
    plugin whose measured max-abs is nonzero.
  - List any `Custom(x)` with x > 1e-6 under Intentional divergences, with its reason. If there are none,
    leave the section's "None beyond ..." wording unchanged.
  - Fix the Grouping table only if aw-b3-00 logged a TABLE MISMATCH.
- `design-docs/user-qa/pending-airwindows-batch3.md`: append one line under R1 that states the R1
  edits landed in aw-b3-00 and cites the log path. Leave the status Pending, because the operator decides.

## Failure handling

- When a gate fails in a file owned by a group plan or aw-b3-00:
  - Do not edit it.
  - Record the failing command, the file, the diagnostic and the owning plan id in the Progress Log and
    in the verification notes.
  - Leave this plan In Progress. The review loop sends the fix to the owner, and this plan reruns its
    gates afterwards.
- A missing or stubbed plugin is a failure. Report it with the plugin name and the owning group.

## Tests (situation -> expected)

- All 109 plugins are registered in order, so `batch3_catalog_is_complete` passes.
- Negative control, recorded in `mutationEvidence` and reverted afterwards:
  - Temporarily replace one `BATCH3` entry with `NoSuchPlugin`.
  - The test fails.
  - Restore the entry and confirm the file hash matches the pre-mutation hash.

## Verification Commands (evidence)

Logs go to `tmp/airwindows-batch3/aw-b3-15/logs/`. Prefix each command with
`export CARGO_TERM_QUIET=true NEXTEST_STATUS_LEVEL=fail NEXTEST_FAILURE_OUTPUT=immediate-final NEXTEST_HIDE_PROGRESS_BAR=1 CARGO_TARGET_DIR=tmp/airwindows-batch3/aw-b3-15/target`.

1. `cargo nextest run -p music-things-airwindows --test catalog_batch3`. Expect exit 0, 1 test passed.
2. `cargo nextest run --workspace --all-features`. Expect exit 0 and failureCount 0. Record testsRun.
   This run includes all 14 `parity_batch3_gNN` (109 tests), `catalog`, `no_alloc`, `fixture_budget`,
   `fixture_budget_batch3` and `batch1_complete`.
3. `cargo clippy --workspace --all-targets --all-features -- -D warnings`. Expect exit 0.
4. `cargo fmt --check`. Expect exit 0.
5. `cargo build --workspace --all-features --target wasm32-unknown-unknown`. Expect exit 0.
6. Completeness checks:
   - `cat crates/airwindows/tests/parity_batch3_g*.rs | grep -c '^#\[test\]'` prints `109`.
   - `cat crates/airwindows/src/plugins/batch3/g*/mod.rs | grep -c '::ENTRY'` prints `109`.
   - `grep -rnE 'todo!|unimplemented!|#\[ignore' crates/airwindows/src/plugins/batch3 crates/airwindows/tests/parity_batch3_*.rs crates/airwindows/tests/catalog_batch3.rs` prints nothing.
7. Size checks:
   - `find crates/airwindows/src/plugins/batch3 crates/airwindows/tests/parity_batch3_*.rs crates/airwindows/tests/catalog_batch3.rs -name '*.rs' -exec wc -l {} + | awk '$2!="total" && $1>=1000 {bad=1; print} END {exit bad}'`. Expect exit 0.
   - `find crates/airwindows/tests/fixtures/batch3 -type f | wc -l` prints `218`.
8. Scope check: `git status --porcelain` shows changes only under the batch-3 write paths of plans
   aw-b3-00..15. That means:
   - `crates/airwindows/src/plugins/batch3/`
   - `crates/airwindows/tests/{parity_batch3_*,catalog_batch3,fixture_budget,fixture_budget_batch3,batch1_complete}.rs`
   - `crates/airwindows/tests/fixtures/batch3/`
   - `tools/airwindows-harness/generate_batch3.sh`
   - `tools/vst-shim/`, additive
   - `design-docs/`
   - `impl-plans/`

   Record the list.

Record each command as `{command, exitStatus: 0, testsRun, testsPassed, failureCount: 0, outcome: "passed", log, notes}`.

## Completion Criteria

- [ ] `catalog_batch3.rs` asserts the exact 109-name suffix in order, plus create, create_with_seed and the upstream name, and it passes.
- [ ] Verifications 1-8 pass on the combined tree, with complete logs and exit statuses recorded.
- [ ] 109 parity tests, 109 registered entries, 218 fixture files, no stubs or ignores, and every `.rs` file under 1000 lines.
- [ ] The design doc Outcome records per-class counts and any nonzero max-abs. Any Custom > 1e-6 is listed as a divergence.
- [ ] The user-QA file records the R1 landing. Only writePaths were changed by this plan.

## Progress Log

### Session: 2026-11-01
**Tasks Completed**: Plan created (not started)
**Notes**: -
