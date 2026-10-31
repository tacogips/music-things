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
| [behringer-oss-ports-00-scaffold](active/behringer-oss-ports-00-scaffold.md) (bop-00, W1) | Ready | design-docs/specs/design-behringer-oss-ports.md | 2026-10-31 |
| [behringer-oss-ports-01-airwindows-framework](active/behringer-oss-ports-01-airwindows-framework.md) (bop-01, W2) | Ready | design-docs/specs/design-behringer-oss-ports.md#a-airwindows-cratesairwindows | 2026-10-31 |
| behringer-oss-ports-02..15-aw-b1-g01..g14 (bop-02..bop-15, W3; one plan per 8 kick-list lines) | Ready | design-docs/specs/design-behringer-oss-ports.md#batch-1-grouping-fixed-by-line-of-kick-airwindows-settxt | 2026-10-31 |
| [behringer-oss-ports-16a-soundfont-midi](active/behringer-oss-ports-16a-soundfont-midi.md) (bop-16a, W2) | Ready | design-docs/specs/design-behringer-oss-ports.md#b-soundfont-cratessoundfont | 2026-10-31 |
| [behringer-oss-ports-16b-soundfont-font](active/behringer-oss-ports-16b-soundfont-font.md) (bop-16b, W2) | Ready | design-docs/specs/design-behringer-oss-ports.md#b-soundfont-cratessoundfont | 2026-10-31 |
| [behringer-oss-ports-16c-soundfont-synth](active/behringer-oss-ports-16c-soundfont-synth.md) (bop-16c, W3) | Ready | design-docs/specs/design-behringer-oss-ports.md#b-soundfont-cratessoundfont | 2026-10-31 |
| [behringer-oss-ports-17a-pitch-smb](active/behringer-oss-ports-17a-pitch-smb.md) (bop-17a, W2) | Ready | design-docs/specs/design-behringer-oss-ports.md#c-pitch-cratespitch | 2026-10-31 |
| [behringer-oss-ports-17b-pitch-stft](active/behringer-oss-ports-17b-pitch-stft.md) (bop-17b, W2) | Ready | design-docs/specs/design-behringer-oss-ports.md#c-pitch-cratespitch | 2026-10-31 |
| [behringer-oss-ports-17c-pitch-dywa](active/behringer-oss-ports-17c-pitch-dywa.md) (bop-17c, W2) | Ready | design-docs/specs/design-behringer-oss-ports.md#c-pitch-cratespitch | 2026-10-31 |
| [behringer-oss-ports-18-integration](active/behringer-oss-ports-18-integration.md) (bop-18, W4) | Ready | design-docs/specs/design-behringer-oss-ports.md#d-facade-cratesmusic-things | 2026-10-31 |

## Completed Plans

| Plan | Completed | Design Reference |
|------|-----------|------------------|
| (No completed plans yet) | - | - |

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
