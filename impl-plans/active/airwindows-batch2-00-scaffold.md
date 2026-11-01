# Airwindows Batch 2 / 00: Scaffold, Harness and Fixtures

**Status**: Ready
**Plan ID**: ab2-00-scaffold
**Wave**: W1 (no dependencies). All W2 plans depend on this plan.
**Design Reference**: `design-docs/specs/design-airwindows-batch2.md` sections 2, 3, 4, 6 and 7
(Scaffold gates)
**Common Contract**: `impl-plans/active/airwindows-batch2-common.md` (mandatory)
**Created**: 2026-11-01
**Last Updated**: 2026-11-01

## Intent and Context

This plan prepares everything the 29 parallel W2 port plans need, so that none of them touches a
shared file:

- batch-2 registration wiring with 34 empty groups
- a batch-2 fixture generator script
- additive shim fixes for any symbol the 110 upstream plugins need
- the 220 committed fixture files
- the batch-level test `tests/catalog_batch2.rs`

`crates/airwindows/src/plugins/batch2/mod.rs` currently contains
`pub(crate) const GROUPS: &[&[PluginEntry]] = &[];`. `plugins/mod.rs` already wires `batch2::GROUPS`
into `BATCHES`, and that file must not change.

The existing `tools/airwindows-harness/generate.sh` is hardcoded to batch 1: it reads 110 lines,
writes to `out/batch1`, and computes the group as `(line-1)/8+1`. Do not edit it. Write a new script
instead.

## Non-goals

- No plugin ports. Every `gNN/mod.rs` keeps `ENTRIES = &[]`.
- No edits to `generate.sh`, `driver.cpp`, `prelude.h` or the harness `README.md`.
- No edits to the framework, `batch1/`, `batch3/`, shared tests (`catalog.rs`, `no_alloc.rs`,
  `batch1_complete.rs`, `fixture_budget.rs`), `tests/common/`, manifests, `Cargo.lock`, the root
  README or `THIRD_PARTY_NOTICES.md`.
- No change to existing shim declarations or behavior. Batch-1 fixtures must stay reproducible.

## Write Ownership

- **writePaths**:
  - `crates/airwindows/src/plugins/batch2` (directory: `mod.rs` plus `g01`..`g34`, each with `mod.rs`)
  - `crates/airwindows/tests/catalog_batch2.rs`
  - `crates/airwindows/tests/fixtures/batch2` (directory: 34 group dirs, 220 files)
  - `tools/airwindows-harness/generate_batch2.sh`
  - `tools/vst-shim/audioeffectx.h`
  - `tools/vst-shim/audioeffectx.cpp`
  - `impl-plans/active/airwindows-batch2-00-scaffold.md`
  - `target`
  - `tmp/airwindows-batch2/ab2-00-scaffold`
- **sharedPaths**: none.
- **artifactRoots**: `target`, `tmp/airwindows-batch2/ab2-00-scaffold`.
- **Ownership hand-off**: after W1, each `batch2/gNN/` directory and each
  `tests/fixtures/batch2/gNN/` directory belongs to the W2 plan listed in common section 4. This
  plan remains the owner of `batch2/mod.rs`, `catalog_batch2.rs`, `generate_batch2.sh` and the shim
  (for escalation follow-ups).

## Deliverables

### D1. `crates/airwindows/src/plugins/batch2/mod.rs`

- Keep the doc line `//! Owned by the batch-2 run.` and `use crate::registry::PluginEntry;`.
- Add `mod g01;` .. `mod g34;` (34 lines, zero-padded).
- Set `pub(crate) const GROUPS: &[&[PluginEntry]] = &[g01::ENTRIES, g02::ENTRIES, ..., g34::ENTRIES];`
  in numeric order. That order is kick-list order, which is what `catalog()` returns.
- Imitate `crates/airwindows/src/plugins/batch1/mod.rs`.

### D2. 34 files `crates/airwindows/src/plugins/batch2/gNN/mod.rs`

Each has exactly:

- a doc line `//! Batch 2 list lines A-B.` with the group's line range from common section 4
  (for example `g03` -> `126`, `g34` -> `214-220`)
- `use crate::registry::PluginEntry;`
- `pub(crate) const ENTRIES: &[PluginEntry] = &[];`

If clippy or rustc warns about an unused import in an empty group, use the form that compiles
warning-free under `-D warnings` and record the choice in the Progress Log. All 34 files must be
identical apart from the line range.

### D3. `tools/airwindows-harness/generate_batch2.sh` (new, executable, `# SPDX-License-Identifier: MIT`)

- CLI: `generate_batch2.sh --work-dir DIR --out-root DIR [--group gNN] [NAME...]`.
  - Exit 2 with usage when `--work-dir` or `--out-root` is missing.
  - Require `$MUSIC_THINGS_REFS`, as `generate.sh` does.
- **Group table**: one embedded `name group` table with exactly 110 rows, equal to common
  section 4.
- **Validation**: read lines 111-220 of `$MUSIC_THINGS_REFS/kick-airwindows-set.txt`. Fail
  (exit 1, with a message) when:
  - fewer than 110 names are read;
  - a name on those lines is missing from the table;
  - a table row is not on those lines;
  - the order differs.
- Per selected plugin:
  - Use upstream = name; there is no alias in batch 2.
  - Extract the description with the same awk rule as `generate.sh`.
  - Compile with the identical clang++ command line as `generate.sh`: same flags, `-include prelude.h`,
    `-I tools/vst-shim`, `-I tools/common`, `-DAW_CLASS`/`-DAW_HEADER`, the same four translation
    units.
  - Run it and write `<out-root>/batch2/<gNN>/<Name>.{txt,f32}`.
  - Print `<Name>: generated`.
- `--group` and the NAME filters behave as in `generate.sh`.
- Pitfalls:
  - Do not reuse the `(line-1)/8+1` group formula.
  - Do not write under `batch1/`.
  - Do not change the compile flags (`-O1 -ffp-contract=off -fno-fast-math -fno-builtin`).

### D4. Additive shim fixes (`tools/vst-shim/audioeffectx.{h,cpp}`), only if needed

Build all 110 plugins with D3. If one fails to compile or link because a VST/host symbol is
missing, append the minimal declaration or no-op definition in a block delimited by
`// batch 2 additions` comments, at the end of the relevant file (inside the existing class for
member functions).

- Never modify or reorder existing lines. Display helpers (D1 divergence) never change.
- List every added symbol and the plugin that needed it in the Progress Log.
- After any shim change, run D3 plus
  `tools/airwindows-harness/generate.sh --work-dir tmp/airwindows-batch2/ab2-00-scaffold/b1check --out-root tmp/airwindows-batch2/ab2-00-scaffold/b1out --group g03`.
  Then compare its output byte for byte with `crates/airwindows/tests/fixtures/batch1/g03/`
  (`cmp`) to prove that batch-1 output is unchanged.
- If an upstream plugin cannot be built without editing upstream or existing shim lines, stop and
  report it as a design blocker. Never port from a modified upstream.

### D5. Fixtures `crates/airwindows/tests/fixtures/batch2/gNN/<Name>.{txt,f32}`

Run:

`MUSIC_THINGS_REFS=/Users/taco/gits/tacogips/music-things-refs tools/airwindows-harness/generate_batch2.sh --work-dir tmp/airwindows-batch2/ab2-00-scaffold/harness --out-root crates/airwindows/tests/fixtures`

- It must exit 0 and print 110 `generated` lines.
- Result: 220 files, every `.f32` exactly 12,096 bytes, total apparent bytes of
  `tests/fixtures/batch2` at most 3,000,000 (about 1.45 MB expected).
- Commit nothing yourself. The workflow commits.

### D6. `crates/airwindows/tests/catalog_batch2.rs`

Public API only (`music_things_airwindows::{catalog, create, create_with_seed}`) plus `std::fs`. No
`mod common;` is needed.

- `const BATCH2: [(&str, &str); 110]`: `(name, group)` pairs in kick-list order, equal to common
  section 4.
- `#[test] fn batch2_fixture_budget()`:
  - Sum the file lengths under `CARGO_MANIFEST_DIR/tests/fixtures/batch2` recursively; assert the
    total is at most 3,000,000. A missing directory counts as 0 and fails the next test.
  - For every row, assert that `<group>/<Name>.txt` exists and that `<group>/<Name>.f32` has length
    12,096.
- `#[test] fn batch2_catalog_exact()`:
  - Collect `catalog()` names. Find the index of `"DitherMeDiskers"` and assert that the 110
    entries after it equal the `BATCH2` names in order. This works with or without batch 3, whose
    entries follow `Pop2`.
  - For each: `upstream_name == name`, exactly one occurrence in the catalog,
    `create(name, 44100.0).is_some()` and `create_with_seed(name, 96000.0, 1).is_some()`.
- `#[test] fn batch2_parity_coverage()`: for each row, read
  `CARGO_MANIFEST_DIR/tests/parity_batch2_<group>.rs` and assert that it contains
  `name: "<Name>"` and does not contain `#[ignore`. Failure messages name the plugin and group.

The two tests `batch2_catalog_exact` and `batch2_parity_coverage` fail until W2 finishes. That is
by design: do not `#[ignore]` them, and do not soften them.

## Key Pitfalls

- The `GROUPS` order must be g01..g34 numerically. A lexicographic mistake (g1, g10, ...) breaks
  catalog order.
- Fixture budget is measured in apparent bytes (sum of file lengths), not with `du`.
- If any plugin fails to build in the harness, the scaffold is not done. W2 plans must not start
  against missing fixtures.
- Do not hand-edit generated fixtures.

## Tests (situation -> expected)

- `batch2_fixture_budget` after D5 -> passes (220 files, at most 3,000,000 bytes).
- `batch2_catalog_exact` at the end of W1 -> fails with "DitherMeTimbers missing" (expected).
  `batch2_parity_coverage` -> fails (expected). Both pass only after W2.
- `generate_batch2.sh --work-dir X --out-root Y --group g03` -> generates exactly `Energy.{txt,f32}`.
- `generate_batch2.sh` without `--out-root` -> exit 2.
- Shim unchanged, or batch-1 g03 regeneration byte-identical (`cmp` exit 0).

## Verification Commands

`$ENV` = `CARGO_TERM_QUIET=true NEXTEST_STATUS_LEVEL=fail NEXTEST_FAILURE_OUTPUT=immediate-final NEXTEST_HIDE_PROGRESS_BAR=1 CARGO_TARGET_DIR=tmp/airwindows-batch2/ab2-00-scaffold/target`.
Logs go to `tmp/airwindows-batch2/ab2-00-scaffold/logs/`.

1. `$ENV cargo check -p music-things-airwindows --all-targets`: exit 0.
2. `MUSIC_THINGS_REFS=/Users/taco/gits/tacogips/music-things-refs tools/airwindows-harness/generate_batch2.sh --work-dir tmp/airwindows-batch2/ab2-00-scaffold/harness --out-root crates/airwindows/tests/fixtures`:
   exit 0, 110 `generated` lines.
3. `test "$(ls crates/airwindows/tests/fixtures/batch2/g*/*.f32 | wc -l)" -eq 110 && test "$(ls crates/airwindows/tests/fixtures/batch2/g*/*.txt | wc -l)" -eq 110`:
   exit 0.
4. `$ENV cargo nextest run -p music-things-airwindows --test catalog_batch2 -E 'test(batch2_fixture_budget)'`:
   exit 0, 1 test passed.
5. `test "$(find crates/airwindows/tests/fixtures/batch2 -type f -exec stat -f %z {} + | awk '{s+=$1} END {print s+0}')" -le 3000000`:
   exit 0. Record the byte total.
6. `rustfmt --edition 2021 --check crates/airwindows/src/plugins/batch2/mod.rs crates/airwindows/src/plugins/batch2/g*/mod.rs crates/airwindows/tests/catalog_batch2.rs`:
   exit 0.
7. `test -z "$($ENV cargo clippy -p music-things-airwindows --all-targets --message-format short -- -D warnings 2>&1 | grep -E 'src/plugins/batch2/|tests/catalog_batch2\.rs')"`:
   exit 0. These are own-file diagnostics only. (Addresses review finding L2: late scaffold clippy.)
8. `$ENV cargo nextest run -p music-things-airwindows`: every pre-existing test passes. The only
   failures allowed are `catalog_batch2::batch2_catalog_exact`,
   `catalog_batch2::batch2_parity_coverage` and `fixture_budget::fixture_budget`. The last one is
   the expected Q1 conflict from design section 7; record it, do not fix it. This is a
   diagnostic record, not a passing gate.
9. `bash -n tools/airwindows-harness/generate_batch2.sh && test -x tools/airwindows-harness/generate_batch2.sh`:
   exit 0.
10. If D4 changed the shim: the batch-1 g03 regeneration plus `cmp` of all 16 files: exit 0.

## Completion Criteria

- [ ] D1 and D2: 34 groups declared in numeric order; every `gNN/mod.rs` has an empty `ENTRIES`;
  crate compiles (cmd 1).
- [ ] D3: script exists, is executable, validates the table against lines 111-220 (cmds 2 and 9).
- [ ] D4: shim additions are only appended and listed, and batch-1 is byte-identical (cmd 10), or
  no shim change was needed (recorded).
- [ ] D5: 220 fixture files, `.f32` = 12,096 bytes each, total at most 3,000,000 (cmds 3-5).
- [ ] D6: `catalog_batch2.rs` has the three tests; budget test passes (cmd 4).
- [ ] Commands 6 and 7 exit 0. Command 8 is recorded with only the allowed failures.
- [ ] Only writePaths changed (pre/post hashes in the evidence directory).
- [ ] Progress Log has the shim symbol list (or "none"), the fixture byte total and all command
  records.

## Progress Log

### Session: 2026-11-01
**Tasks Completed**: Plan created (not started).
