# Implementation Plans

This directory contains implementation plans that translate design documents into actionable implementation specifications.

## Purpose

Implementation plans bridge design documents (what to build) and actual code (how to build). They provide:
- Clear deliverables without code
- Trait and function specifications
- Dependency mapping for concurrent execution
- Progress tracking across sessions

## Directory Structure

```
impl-plans/
├── README.md              # This file
├── active/                # Currently active implementation plans
│   └── <feature>.md       # One file per feature being implemented
├── completed/             # Completed implementation plans (archive)
│   └── <feature>.md       # Completed plans for reference
└── templates/             # Plan templates
    └── plan-template.md   # Standard plan template
```

## File Size Limits

**IMPORTANT**: Implementation plan files must stay under 400 lines to prevent OOM errors.

| Metric | Limit |
|--------|-------|
| Line count | MAX 1000 lines |
| Modules per plan | MAX 8 modules |
| Tasks per plan | MAX 10 tasks |

Large features are split into multiple related plans with cross-references.

## Active Plans

| Plan | Status | Design Reference | Last Updated |
|------|--------|------------------|--------------|
| (No active plans. The batch-1 dispatch manifest stays at `active/behringer-oss-ports-dispatch.json` as the run record.) | - | - | - |

## Completed Plans

Behringer OSS ports batch 1. Session 578 accepted bop-00..bop-03, bop-05..bop-14 and bop-16a..bop-17c and committed them at 49052ec. Review-loop session-624 accepted bop-04, bop-15 and bop-18. The bop-18 combined-tree gates passed (`tmp/behringer-oss-ports/bop-18/logs/`). Batches 2 and 3 (kick list line 111 onward) will be new plans.

| Plan | Completed | Design Reference |
|------|-----------|------------------|
| [behringer-oss-ports-00-scaffold](completed/behringer-oss-ports-00-scaffold.md) (bop-00, W1) | 2026-11-01 | design-docs/specs/design-behringer-oss-ports.md |
| [behringer-oss-ports-01-airwindows-framework](completed/behringer-oss-ports-01-airwindows-framework.md) (bop-01, W2) | 2026-11-01 | design-docs/specs/design-behringer-oss-ports.md#a-airwindows-cratesairwindows |
| [behringer-oss-ports-02-aw-b1-g01](completed/behringer-oss-ports-02-aw-b1-g01.md) (bop-02, W3) | 2026-11-01 | design-docs/specs/design-behringer-oss-ports.md#batch-1-grouping-fixed-by-line-of-kick-airwindows-settxt |
| [behringer-oss-ports-03-aw-b1-g02](completed/behringer-oss-ports-03-aw-b1-g02.md) (bop-03, W3) | 2026-11-01 | design-docs/specs/design-behringer-oss-ports.md#batch-1-grouping-fixed-by-line-of-kick-airwindows-settxt |
| [behringer-oss-ports-04-aw-b1-g03](completed/behringer-oss-ports-04-aw-b1-g03.md) (bop-04, W3) | 2026-11-01 | design-docs/specs/design-behringer-oss-ports.md#batch-1-grouping-fixed-by-line-of-kick-airwindows-settxt |
| [behringer-oss-ports-05-aw-b1-g04](completed/behringer-oss-ports-05-aw-b1-g04.md) (bop-05, W3) | 2026-11-01 | design-docs/specs/design-behringer-oss-ports.md#batch-1-grouping-fixed-by-line-of-kick-airwindows-settxt |
| [behringer-oss-ports-06-aw-b1-g05](completed/behringer-oss-ports-06-aw-b1-g05.md) (bop-06, W3) | 2026-11-01 | design-docs/specs/design-behringer-oss-ports.md#batch-1-grouping-fixed-by-line-of-kick-airwindows-settxt |
| [behringer-oss-ports-07-aw-b1-g06](completed/behringer-oss-ports-07-aw-b1-g06.md) (bop-07, W3) | 2026-11-01 | design-docs/specs/design-behringer-oss-ports.md#batch-1-grouping-fixed-by-line-of-kick-airwindows-settxt |
| [behringer-oss-ports-08-aw-b1-g07](completed/behringer-oss-ports-08-aw-b1-g07.md) (bop-08, W3) | 2026-11-01 | design-docs/specs/design-behringer-oss-ports.md#batch-1-grouping-fixed-by-line-of-kick-airwindows-settxt |
| [behringer-oss-ports-09-aw-b1-g08](completed/behringer-oss-ports-09-aw-b1-g08.md) (bop-09, W3) | 2026-11-01 | design-docs/specs/design-behringer-oss-ports.md#batch-1-grouping-fixed-by-line-of-kick-airwindows-settxt |
| [behringer-oss-ports-10-aw-b1-g09](completed/behringer-oss-ports-10-aw-b1-g09.md) (bop-10, W3) | 2026-11-01 | design-docs/specs/design-behringer-oss-ports.md#batch-1-grouping-fixed-by-line-of-kick-airwindows-settxt |
| [behringer-oss-ports-11-aw-b1-g10](completed/behringer-oss-ports-11-aw-b1-g10.md) (bop-11, W3) | 2026-11-01 | design-docs/specs/design-behringer-oss-ports.md#batch-1-grouping-fixed-by-line-of-kick-airwindows-settxt |
| [behringer-oss-ports-12-aw-b1-g11](completed/behringer-oss-ports-12-aw-b1-g11.md) (bop-12, W3) | 2026-11-01 | design-docs/specs/design-behringer-oss-ports.md#batch-1-grouping-fixed-by-line-of-kick-airwindows-settxt |
| [behringer-oss-ports-13-aw-b1-g12](completed/behringer-oss-ports-13-aw-b1-g12.md) (bop-13, W3) | 2026-11-01 | design-docs/specs/design-behringer-oss-ports.md#batch-1-grouping-fixed-by-line-of-kick-airwindows-settxt |
| [behringer-oss-ports-14-aw-b1-g13](completed/behringer-oss-ports-14-aw-b1-g13.md) (bop-14, W3) | 2026-11-01 | design-docs/specs/design-behringer-oss-ports.md#batch-1-grouping-fixed-by-line-of-kick-airwindows-settxt |
| [behringer-oss-ports-15-aw-b1-g14](completed/behringer-oss-ports-15-aw-b1-g14.md) (bop-15, W3) | 2026-11-01 | design-docs/specs/design-behringer-oss-ports.md#batch-1-grouping-fixed-by-line-of-kick-airwindows-settxt |
| [behringer-oss-ports-16a-soundfont-midi](completed/behringer-oss-ports-16a-soundfont-midi.md) (bop-16a, W2) | 2026-11-01 | design-docs/specs/design-behringer-oss-ports.md#b-soundfont-cratessoundfont |
| [behringer-oss-ports-16b-soundfont-font](completed/behringer-oss-ports-16b-soundfont-font.md) (bop-16b, W2) | 2026-11-01 | design-docs/specs/design-behringer-oss-ports.md#b-soundfont-cratessoundfont |
| [behringer-oss-ports-16c-soundfont-synth](completed/behringer-oss-ports-16c-soundfont-synth.md) (bop-16c, W3) | 2026-11-01 | design-docs/specs/design-behringer-oss-ports.md#b-soundfont-cratessoundfont |
| [behringer-oss-ports-17a-pitch-smb](completed/behringer-oss-ports-17a-pitch-smb.md) (bop-17a, W2) | 2026-11-01 | design-docs/specs/design-behringer-oss-ports.md#c-pitch-cratespitch |
| [behringer-oss-ports-17b-pitch-stft](completed/behringer-oss-ports-17b-pitch-stft.md) (bop-17b, W2) | 2026-11-01 | design-docs/specs/design-behringer-oss-ports.md#c-pitch-cratespitch |
| [behringer-oss-ports-17c-pitch-dywa](completed/behringer-oss-ports-17c-pitch-dywa.md) (bop-17c, W2) | 2026-11-01 | design-docs/specs/design-behringer-oss-ports.md#c-pitch-cratespitch |
| [behringer-oss-ports-18-integration](completed/behringer-oss-ports-18-integration.md) (bop-18, W4) | 2026-11-01 | design-docs/specs/design-behringer-oss-ports.md#d-facade-cratesmusic-things |

## Phase Dependencies (for impl-exec-auto)

**IMPORTANT**: This section is used by impl-exec-auto to determine which plans to load.
Only plans from eligible phases should be read to minimize context loading.

### Phase Status

| Phase | Status | Depends On |
|-------|--------|------------|
| 1 | NOT_STARTED | - |
| 2 | BLOCKED | Phase 1 |
| 3 | BLOCKED | Phase 2 |

### Phase to Plans Mapping

```
PHASE_TO_PLANS = {
  1: [
    # Add Phase 1 plan files here
  ],
  2: [
    # Add Phase 2 plan files here
  ],
  3: [
    # Add Phase 3 plan files here
  ]
}
```

## Workflow

### Creating a New Plan

1. Use the `/impl-plan` command with a design document reference
2. Or manually create a plan using `templates/plan-template.md`
3. Save to `active/<feature-name>.md`
4. Update this README with the new plan entry
5. **IMPORTANT**: If plan exceeds 400 lines, split into multiple files

### Working on a Plan

1. Read the active plan
2. Select a subtask to work on (consider parallelization)
3. Implement following the deliverable specifications
4. Update task status and progress log
5. Mark completion criteria as done

### Completing a Plan

1. Verify all completion criteria are met
2. Update status to "Completed"
3. Move file from `active/` to `completed/`
4. Update this README

## Guidelines

- Plans contain NO implementation code
- Plans specify traits, functions, and file structures
- Subtasks should be as independent as possible for parallel execution
- Always update progress log after each session
- **Keep each plan file under 400 lines** - split if necessary
