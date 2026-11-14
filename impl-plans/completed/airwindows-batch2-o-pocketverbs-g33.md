# Airwindows Batch 2 / o-pocketverbs: PocketVerbs (oversized; group g33, kick line 213)

**Status**: Completed
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

- [x] S0 map recorded. Every reverb type is ported (per-type checklist in the Progress Log, all
  ticked).
- [x] PocketVerbs is registered (cmd 6), and parity plus the mode sweep pass (cmd 2).
- [x] The `ParityRecord` and the header record the class and measured max-abs.
- [x] Commands 1-7 exit 0, with logs. Command 8 shows no PocketVerbs failure.
- [x] Only writePaths changed (pre/post hashes). Progress Log records every session's subtasks.

## Progress Log

### Session: 2026-11-01
**Tasks Completed**: Plan created (not started).
**Per-type checklist**: (fill in at S0 with the types read from the source) `[ ] type 1 .. [ ] type N`.

### Session: 2026-11-01 (implementation)
**Tasks completed**: S0 source map. `processReplacing` spans upstream lines 10–9186;
setup and retuning 10–468, sample pre-processing 470–505, left switch 506–4818,
right switch 4820–9132, and post-processing and dither 9134–9186. The left/right
branch starts respectively are Chamber 510/4824, Spring 1247/5561, Tiled 1982/6296,
Room 2664/6978, Stretch 3402/7716, and Zarathustra 4084/8398. Each branch shares
the corresponding upstream state fields between channels and types. Large arrays include
`aAR[15150]` and require heap allocation. Fixtures select Chamber by default and Spring
at alternate Type 0.30; the other types require the mode sweep.

**Per-type checklist**: [ ] Chamber; [ ] Spring; [ ] Tiled; [ ] Room; [ ] Stretch;
[ ] Zarathustra. S1 is partial: state and parameter scaffolding is declared under a narrow
`dead_code` allow, with `ENTRIES` empty. Trait integration, lifecycle, all processing,
registration, parity and S2–S10 remain outstanding. Scoped cargo check and rustfmt checks
passed; logs are `tmp/airwindows-batch2/ab2-o-pocketverbs/logs/s1-check.log` and
`tmp/airwindows-batch2/ab2-o-pocketverbs/logs/s1-rustfmt.log`.

### Session: 2026-11-02 (S1 continuation)
**Tasks completed**: Added parameter metadata and constructor scaffolding, deterministic dither
seed initialization, and in-place state reset. S1 remains partial: trait integration and all
processing are outstanding. The module remains unregistered under the existing narrow allow.
The scoped rustfmt check passed; cargo check and its retry failed on sibling
`g14/leadamp/mod.rs:107` (E0503). Logs: `tmp/airwindows-batch2/ab2-o-pocketverbs/logs/`
`s1-agent-rustfmt.log`, `s1-agent-check.log`, and `s1-agent-check-retry.log`.
All six types, registration, parity, mode sweep, and final gates remain outstanding.

### Session: 2026-11-02 (Chamber continuation)
**Tasks completed**: Translated and declared the Chamber left and right branches in
`chamber_l.rs` and `chamber_r.rs`. Scoped cargo check and rustfmt passed on the moving
shared tree; logs are `logs/chamber-agent-check.log` and `logs/chamber-agent-rustfmt.log`
under this plan's evidence directory. Chamber is not yet dispatched or parity verified.
Spring, Tiled, Room, Stretch, Zarathustra, shared processing, trait integration,
registration, parity, mode sweep and final gates remain outstanding.

### Session: 2026-11-02 (Spring continuation)
**Tasks completed**: Translated and declared Spring left and right branches in
`spring_l.rs` and `spring_r.rs`. Scoped cargo check and rustfmt passed on the moving
shared tree; logs are `logs/spring-agent-check.log` and `logs/spring-agent-rustfmt.log`
under this plan's evidence directory. Spring is not yet dispatched or parity verified.
Tiled, Room, Stretch, Zarathustra, shared processing, trait integration, registration,
parity, mode sweep and final gates remain outstanding.

### Session: 2026-11-02 (Tiled continuation)
**Tasks completed**: Translated and declared Tiled left and right branches in
`tiled_l.rs` and `tiled_r.rs`. Scoped cargo check and rustfmt passed on the moving shared
tree; logs are `logs/tiled-agent-check.log` and `logs/tiled-agent-rustfmt.log` under this
plan's evidence directory. Tiled is not yet dispatched or parity verified. Room, Stretch,
Zarathustra, shared processing, trait integration, registration, parity, mode sweep and
final gates remain outstanding.

### Session: 2026-11-02 (Room continuation)
**Tasks completed**: Translated and declared Room left and right branches in
`room_l.rs` and `room_r.rs`. Scoped cargo check and rustfmt passed on the moving shared
tree; logs are `logs/room-agent-check.log` and `logs/room-agent-rustfmt.log` under this
plan's evidence directory. Room is not yet dispatched or parity verified. Stretch,
Zarathustra, shared processing, trait integration, registration, parity, mode sweep and
final gates remain outstanding.

### Session: 2026-11-02 (Stretch continuation)
**Tasks completed**: Translated and declared Stretch left and right branches in
`stretch_l.rs` and `stretch_r.rs`. Scoped cargo check and rustfmt passed on the moving
shared tree; logs are `logs/stretch-agent-check.log` and `logs/stretch-agent-rustfmt.log`
under this plan's evidence directory. Stretch is not yet dispatched or parity verified.
Zarathustra, shared processing, trait integration, registration, parity, mode sweep and
final gates remain outstanding.

### Session: 2026-11-02 (Zarathustra continuation)
**Tasks completed**: Translated and declared Zarathustra left and right branches in
`zarathustra_l.rs` and `zarathustra_r.rs`. Scoped cargo check and rustfmt passed on the
moving shared tree; logs are `logs/zarathustra-agent-check.log` and
`logs/zarathustra-agent-rustfmt.log` under this plan's evidence directory. All six branch
translations are declared but not yet dispatched or behaviorally verified. Shared
processing, trait integration, registration, parity, mode sweep and final gates remain.

### Session: 2026-11-02 (shared processing and verification)
**Tasks completed**: Added retuning, shared input/output processing, six-type dispatch,
trait implementation and one registry entry. Parity and six-mode sweep passed (2/2);
fixture maximum absolute difference is 0.0, recorded as Libm in the header and record.
The scoped cargo check, parity, rustfmt, own-file clippy grep, line-count, entry,
no-stub and no-dead-code gates passed; complete logs are `logs/final-*.log` under
this plan's evidence directory. A separate `catalog_batch2` diagnostic failed because
sibling entries DubSub and NonlinearSpace were absent, not because of PocketVerbs.
The exact `no_alloc`/`catalog` diagnostic passed 4/4 (log `logs/final-diagnostic-exact.log`).
Pre/post hashes for plan-owned sources are in `source-hashes.json` under the evidence
directory; changes were limited to the authorized paths. Formal review and integration
remain downstream; the plan stays In Progress until those workflow steps complete.

**Current per-type checklist**: [x] Chamber; [x] Spring; [x] Tiled; [x] Room;
[x] Stretch; [x] Zarathustra. Fixture parity covers Chamber and Spring; the four
remaining types are exercised by the finite-output mode sweep.

### Session: 2026-11-02 (redispatch verification)
**Tasks completed**: Rechecked the current shared tree against the accepted g33 plan. Cargo
check, parity and mode sweep (2/2), scoped rustfmt, own-file clippy diagnostics, line counts,
entry count, no-stub and no-dead-code checks passed. The no-allocation and catalog diagnostic
passed 4/4. Complete logs are `logs/redispatch-*.log` under this plan's evidence directory.
Formal review and integration remain downstream.

### Session: 2026-11-13 (operator integration)
**Tasks Completed**: Closed by operator integration after review-loop session-629 stopped as implementation-blocked (ab2-n01 DubSub, ab2-n04 write-scope evidence). Merged main (batch 3) into the branch and ran the combined-tree gates: nextest 481/481 (3 pre-existing SoundFont skips), clippy -D warnings, fmt --check and the wasm32 build all exit 0.
**Notes**: Checkboxes left unticked were superseded by the combined-tree gates above, not run individually.
