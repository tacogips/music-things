# Airwindows Batch 2: Decisions Awaiting the Operator

**Status**: Pending Decision (Q1 blocks only the final full-nextest gate of `ab2-99-integration`;
porting proceeds)

**Created**: 2026-11-01

**Category**: Architecture / integration

Design: `design-docs/specs/design-airwindows-batch2.md` (section 7).

## Q1: Shared tests that batch 2 must not touch but that fail once it lands (blocking for ab2-99)

- `crates/airwindows/tests/batch1_complete.rs` asserts that `catalog()` names equal the 110 batch-1
  names exactly. Registering any batch-2 (or batch-3) plugin fails it.
- `crates/airwindows/tests/fixture_budget.rs` asserts that `tests/fixtures/` totals at most 1,600,000
  bytes. Batch 1 already measures about 1.47 MB, and batch 2 adds about 1.45 MB. This contradicts the
  base design's rule of 3 MB per batch.

The workflow input forbids editing shared tests, but it also requires
`cargo nextest run --workspace --all-features` to pass on the final tree. Both cannot hold.

Options:

1. **(Recommended)** The operator applies, or explicitly authorizes `ab2-99` to apply, two minimal
   edits:
   - `batch1_complete.rs` compares `&names[..BATCH1.len()]`.
   - `fixture_budget.rs` measures `tests/fixtures/batch1` plus the top-level files, keeping the
     1.6 MB bound.

   Per-batch 3 MB budgets live in `catalog_batch2.rs` and `catalog_batch3.rs`. Applying the edit
   once (on `main`, or while merging batches 2 and 3) avoids a batch-2/batch-3 conflict.
2. Shrink the batch-2 fixtures to fit the remaining 0.13 MB. This is rejected: it would cut parity
   coverage to about 1 KB per plugin, and `batch1_complete.rs` would still fail.

Default until answered: `ab2-99` does not edit the files. It records the two expected failures,
proves that every other test passes with a nextest filter that excludes only these two binaries, and
reports `Blocked (Q1)`.

## Q2: Index entry in `design-docs/user-qa/README.md` (non-blocking)

The README is shared with the batch-3 run, so batch 2 does not edit it. Default: the operator adds
the index line for this file after merging both batches.
