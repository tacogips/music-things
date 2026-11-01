# User Q&A

This directory contains items requiring user confirmation or decision.

## Purpose

Store questions, pending decisions, and items awaiting user approval.

## File Naming Convention

| Prefix | Use Case |
|--------|----------|
| `qa-` | Questions/confirmation items |
| `pending-` | Pending decisions |

## Current Items

- [pending-behringer-oss-ports-batch1.md](./pending-behringer-oss-ports-batch1.md) - Behringer OSS ports batch 1: non-blocking defaults (display formatting, stft streaming-wrapper verification, tolerance calibration, fixture regeneration)
- [pending-airwindows-batch3.md](./pending-airwindows-batch3.md) - Airwindows batch 3: edits to fixture_budget.rs and batch1_complete.rs outside the write paths (R1), batch-2 merge resolution, tolerance host
- [qa-example.md](./qa-example.md) - Example: Database Selection (template example)
- [pending-example.md](./pending-example.md) - Example: CLI Output Format (template example)

## Adding New Items

1. Create a new file with appropriate prefix (`qa-` or `pending-`)
2. Include clear description of the question or decision needed
3. List available options if applicable
4. Update this README.md with a reference to the new item
