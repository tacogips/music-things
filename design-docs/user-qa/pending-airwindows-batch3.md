# Airwindows Batch 3: Decisions Awaiting Confirmation

**Status**: Pending Decision (non-blocking; implementation proceeds with the defaults)

**Created**: 2026-11-01

**Category**: Scope / Verification

## Decisions Needed

Each default is recorded in `design-docs/specs/design-airwindows-batch3.md`.

1. **R1: edits to two batch-1-era tests outside the batch-3 write paths.**
   - The facts:
     - `crates/airwindows/tests/fixture_budget.rs` caps the whole `tests/fixtures/` tree at 1,600,000 bytes, and 1,406,647 bytes are already used.
     - `crates/airwindows/tests/batch1_complete.rs` asserts that `catalog()` holds exactly the 110 batch-1 names.
     - Batch-3 fixtures (about 1.4 MB, within the 3 MB batch budget) and batch-3 registration make both tests fail.
     - The acceptance criteria require the full workspace nextest run to pass.
   - Default, done by plan `aw-b3-00-bootstrap`:
     - In `fixture_budget.rs`, the 1.6 MB assertion excludes `tests/fixtures/batch3/`.
     - A new `tests/fixture_budget_batch3.rs` caps `tests/fixtures/batch3/` at 3,000,000 bytes.
     - In `batch1_complete.rs`, the exact-equality assertion becomes a prefix assertion over the first 110 catalog entries.
   - Options:
     - (a) Accept the default.
     - (b) Fit batch 3 into the remaining headroom of about 193 KB, which is shared with batch 2. That needs under 1.8 KB per plugin, so it is rejected as a loss of parity coverage.
     - (c) Have the operator make the edits after merging. Batch 3's final nextest run would then fail until that is done.
   - If the default is rejected, the integration plan reports the full-nextest acceptance as blocked and does not weaken any batch-3 test.
2. **Merge with batch 2.** Batch 2 may make the analogous edits to the same two files. The default resolution at merge: exclude both batch directories from the 1.6 MB check, and keep the prefix assertion in `batch1_complete.rs`.
3. **Cross-host tolerance.** As for batch 1, the `Libm` 1e-6 bound is measured only on aarch64-apple-darwin. Default: accept.
