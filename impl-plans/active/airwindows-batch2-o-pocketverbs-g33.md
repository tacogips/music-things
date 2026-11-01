# Airwindows Batch 2 / o-pocketverbs: PocketVerbs (oversized; group g33, kick line 213)

**Status**: Ready
**Plan ID**: ab2-o-pocketverbs
**Wave**: W2. Depends on ab2-00-scaffold; runs in parallel with all other W2 plans.
**Design Reference**: `design-docs/specs/design-airwindows-batch2.md` section 3 (PocketVerbs
bullet: ordered subtasks, multi-session, registration only when complete), plus sections 2, 5 and 7
**Common Contract**: `impl-plans/active/airwindows-batch2-common.md` (mandatory: sections 5-11)
**Created**: 2026-11-01

## Intent

Port the upstream Airwindows **PocketVerbs**, a multi-type reverb. Its upstream
`PocketVerbsProc.cpp` is 18358 lines, by far the largest batch-2 source. Only `processReplacing`
is ported, roughly half the file; the rest is `processDoubleReplacing`.

- Register it in `g33/mod.rs`.
- Prove parity with `tests/parity_batch2_g33.rs`.

PocketVerbs must be fully ported, **including every reverb type**, then registered and
parity-tested. A stub, a missing type branch, a type that falls through to dry/no-op where upstream
has code, or an unregistered module is a failure of this plan, not a deferral. This plan may take
several sessions. It stays `In Progress` until every subtask is done and is never reported
`Completed` with partial types.

## Non-goals

No other plugin or group, no framework/shared-test/tools/fixture edits, no `processDoubleReplacing`,
no algorithm refactoring. Do not merge the per-type code into a shared generic engine unless the
operations are identical in order and precision. Duplicated upstream code stays duplicated where it
differs at all.

## Write Ownership

- **writePaths**:
  - `crates/airwindows/src/plugins/batch2/g33`
  - `crates/airwindows/tests/parity_batch2_g33.rs`
  - `crates/airwindows/tests/fixtures/batch2/g33`
  - `impl-plans/active/airwindows-batch2-o-pocketverbs-g33.md`
  - `target`
  - `tmp/airwindows-batch2/ab2-o-pocketverbs`
- **sharedPaths**: none.
- **artifactRoots**: `target`, `tmp/airwindows-batch2/ab2-o-pocketverbs`.

## Module Layout (required: module directory, every file under 1000 lines, target 900 or fewer)

- `g33/mod.rs`: declare `#[allow(dead_code)] mod pocketverbs;` at **S1**, so that every per-type
  cmd 1 really compiles the new code. `ENTRIES` stays `&[]` until **S9**, which adds
  `pocketverbs::ENTRY` and removes the `dead_code` allow (common section 5 item 6).
- `g33/pocketverbs/mod.rs`: header, narrow allow list, params (Type selector display strings,
  Size, Gating, Dry/Wet or whatever `PocketVerbs.cpp` defines), `INFO`/`ENTRY`/`construct`, the
  trait impl, and the per-sample type dispatch, matching upstream's `switch`/`if` on the verb type.
- `g33/pocketverbs/state.rs`: the struct with all delay arrays from `PocketVerbs.h`, each `Box<[T]>`
  when over 4096 elements and sized exactly as declared, the constructor and `reset`, which zeroes
  in place. If the field list alone exceeds 900 lines, split it into `state.rs` plus `state_b.rs`
  using nested sub-structs, with no behavior change.
- `g33/pocketverbs/<type>.rs`: one file per reverb type branch of upstream `processReplacing`
  (expected from `getParameterDisplay`: Chamber, Spring, Tiled, Room, Stretch, Zarathustra; verify
  against the source). A type over 900 lines is split again into `<type>_a.rs`/`<type>_b.rs` at
  upstream statement boundaries.
- Shared pre- and post-type code (input gain, gating, dry/wet, dither) stays in `mod.rs` in
  upstream order.

## Ordered Subtasks (one session may cover several; registration only at S9)

1. **S0**: map `PocketVerbsProc.cpp`: the `processReplacing` line range, the per-type line ranges,
   the shared pre/post sections, and the `.h` arrays used by each type. Record the map in the
   Progress Log with line numbers.
2. **S1**: `state.rs` (all fields and arrays, constructor, seed replay), plus params and display in
   `mod.rs`. Declare `mod pocketverbs;` (with `#[allow(dead_code)]`) in `g33/mod.rs` now. Cmd 1
   must exit 0.
3. **S2-S7**: one subtask per reverb type, in upstream order, each ported completely into its own
   file(s), with the type file declared in `pocketverbs/mod.rs` as soon as it is created. After each
   type, run cmds 1, 3 and 5. Cmd 1 now compiles the type, so a type counts as done only when cmd 1
   exits 0 with that type declared. Follow the keep-compiling rule in common section 8: never leave
   a declared type file that fails to compile.
4. **S8**: `reset`, the shared pre/post code and the dispatch.
5. **S9**: add `pocketverbs::ENTRY` to `ENTRIES` in `g33/mod.rs` and remove the `dead_code` allow,
   then write `tests/parity_batch2_g33.rs` with
   the `ParityRecord`, the parity test **and the required mode-sweep test**. The sweep renders every
   discrete Type value (from upstream's integer cast of the Type param) at 44100 and 96000 Hz and
   asserts no panic and finite output.
6. **S10**: measure max-abs, record it, and run all gates.

## Pitfalls

- The R0/R1/R2 fixtures reach only the Type values selected by the default param and by
  `ALT[0] = 0.30`. The other types are protected only by the mode sweep and review.
  - Port them with the same statement-by-statement care.
  - Record in the Progress Log which types the fixtures exercise.
- Delay lengths often depend on the Size param and on the sample rate, with clamps to the array
  sizes. Port every clamp exactly; the 96 kHz mode sweep exposes missing clamps as panics.
- Per-type state may be reused across types. Keep the same field for the same upstream variable;
  never duplicate a state variable.
- Allocation happens only in the constructor. `no_alloc.rs` exercises `process`, `set_param` and
  `reset`.
- Expect about 15-25 source files. Keep the per-file line check green after every subtask.

## Verification

Use common section 9 with P = `ab2-o-pocketverbs` and G = `g33`. Logs go to
`tmp/airwindows-batch2/ab2-o-pocketverbs/logs/`.

1. `$ENV cargo check -p music-things-airwindows --all-targets`: exit 0.
2. `$ENV cargo nextest run -p music-things-airwindows --test parity_batch2_g33`: exit 0,
   testsRun >= 2 (parity plus mode sweep), failures 0.
3. `find crates/airwindows/src/plugins/batch2/g33 crates/airwindows/tests/parity_batch2_g33.rs -name '*.rs' -print0 | xargs -0 rustfmt --edition 2021 --check`:
   exit 0.
4. `test -z "$($ENV cargo clippy -p music-things-airwindows --all-targets --message-format short -- -D warnings 2>&1 | grep -E 'src/plugins/batch2/g33/|tests/parity_batch2_g33\.rs')"`:
   exit 0.
5. `test -z "$(find crates/airwindows/src/plugins/batch2/g33 -name '*.rs' -exec wc -l {} + | awk '$2 != "total" && $1 >= 1000')"`:
   exit 0.
6. `test "$(grep -c '::ENTRY' crates/airwindows/src/plugins/batch2/g33/mod.rs)" -eq 1 && test -f crates/airwindows/src/plugins/batch2/g33/pocketverbs/mod.rs`:
   exit 0.
7. `! grep -rnE 'todo!|unimplemented!|#\[ignore' crates/airwindows/src/plugins/batch2/g33 crates/airwindows/tests/parity_batch2_g33.rs`:
   exit 0.
8. Diagnostic: `$ENV cargo nextest run -p music-things-airwindows --test no_alloc --test catalog`.
   It must show no failure naming PocketVerbs.

## Completion Criteria

- [ ] S0 map recorded. Every reverb type is ported (per-type checklist in the Progress Log, all
  ticked).
- [ ] PocketVerbs is registered (cmd 6), and parity plus the mode sweep pass (cmd 2).
- [ ] The `ParityRecord` and the header record the class and measured max-abs.
- [ ] Commands 1-7 exit 0, with logs. Command 8 shows no PocketVerbs failure.
- [ ] Only writePaths changed (pre/post hashes). Progress Log records every session's subtasks.

## Progress Log

### Session: 2026-11-01
**Tasks Completed**: Plan created (not started).
**Per-type checklist**: (fill in at S0 with the types read from the source) `[ ] type 1 .. [ ] type N`.
