# Airwindows Batch 2 (KICK list lines 111-220)

This document covers porting the 110 Airwindows plugins on lines 111-220 of
`kick-airwindows-set.txt`, `DitherMeTimbers` through `Pop2`, into `music-things-airwindows`. It extends
`design-docs/specs/design-behringer-oss-ports.md`, called the base design below, and does not
restate it. The following sections of the base design apply unchanged:

- A "Public API", "Framework internals", "Additive registration", "Porting rules"
- "Parity method (Airwindows)": stimulus, renders R0-R2, seed `0x00C0FFEE`, 128-frame blocks, the
  504 stored frames per render, and the fixture format
- "Tolerances (Airwindows)"
- "Additive procedure for batches 2 and 3"
- Intentional divergences D1, D5 and D6

Batch 2 adds no new public API, framework change, dependency or divergence. The base design is
shared with the concurrent batch-3 run, so batch 2 does not edit it. Everything specific to batch 2
lives here.

Source: issue "Port Airwindows batch 2 (110 plugins, DitherMeTimbers .. Pop2) into
music-things-airwindows" (workflow input; no GitHub number). Upstream: airwindows/airwindows @
`b110fac3991fff7c4655cc2d25d6a3e221f9799c` (MIT), `plugins/LinuxVST/src/<Name>/`. Every batch-2 name
equals its upstream directory/class name (`upstream_name == name`). The only alias in the kick list,
`Console8BusOut`, is on line 77 and belongs to batch 1.

## 1. Write boundary

| Allowed (batch 2 only) | Forbidden |
|------------------------|-----------|
| `crates/airwindows/src/plugins/batch2/**` | `src/plugins/batch1/**`, `src/plugins/batch3/**` |
| `crates/airwindows/tests/parity_batch2_gNN.rs` | framework: `lib.rs`, `effect.rs`, `params.rs`, `dither.rs`, `rand.rs`, `display.rs`, `registry.rs`, `plugins/mod.rs` |
| `crates/airwindows/tests/catalog_batch2.rs` | `tests/catalog.rs`, `tests/batch1_complete.rs`, `tests/fixture_budget.rs`, `tests/no_alloc.rs`, `tests/stimulus_checksum.rs`, `tests/common/**` (read-only reuse via `mod common;`) |
| `crates/airwindows/tests/fixtures/batch2/**` | `tests/fixtures/batch1/**`, `tests/fixtures/shim_format_vectors.txt` |
| new file `tools/airwindows-harness/generate_batch2.sh`; additive-only edits to `tools/vst-shim/audioeffectx.{h,cpp}` | editing existing lines of `generate.sh`, `driver.cpp`, `prelude.h`, `README.md` |
| `design-docs/specs/design-airwindows-batch2.md`, `design-docs/user-qa/pending-airwindows-batch2.md`, `impl-plans/active/airwindows-batch2-*` | `design-behringer-oss-ports.md`, `design-docs/user-qa/README.md`, `impl-plans/active/behringer-oss-ports-dispatch.json`, root `README.md`, `THIRD_PARTY_NOTICES.md`, all `Cargo.toml`, `Cargo.lock` |

Two shared tests break as soon as batch 2 lands, and both are in the forbidden column. Section 7
covers them, and the open question Q1 decides how they get fixed.

## 2. Grouping (registration order = kick-list order)

`catalog()` returns entries in kick-list order: BATCHES, then GROUPS, then ENTRIES. To keep that
order while giving every oversized plugin its own plan and its own directory, batch 2 uses
**34 groups in kick-list order**:

- Each oversized plugin (an upstream `<Name>Proc.cpp` over 1000 lines) is a singleton group, marked
  `*` below.
- Each run of normal plugins between two oversized ones is split evenly into groups of at most 8.

| Group | Lines | Plugins |
|-------|-------|---------|
| g01 | 111-118 | DitherMeTimbers, Doublelay, DoublePaul, Drive, DrumSlam, DubCenter, Dubly, DubSub |
| g02 | 119-125 | DustBunny, Dynamics, Dyno, Edge, EdIsDim, Elation, ElectroHat |
| g03* | 126 | Energy (1448) |
| g04* | 127 | Energy2 (1292) |
| g05 | 128-133 | Ensemble, EQ, EveryTrim, Exciter, Facet, FathomFive |
| g06* | 134 | FireAmp (1707) |
| g07 | 135-139 | Flipity, Floor, Flutter, Focus, Fracture |
| g08 | 140-144 | FromTape, Galactic, Gatelope, GlitchShifter, Golem |
| g09* | 145 | GrindAmp (1650) |
| g10 | 146-152 | Gringer, GrooveWear, GuitarConditioner, HardVacuum, Hermepass, HermeTrim, HighGlossDither |
| g11 | 153-159 | HighImpact, Highpass, Highpass2, Holt, Holt2, Hombre, Hull |
| g12 | 160-165 | Hype, Hypersonic, HypersonX, Infinity, Infinity2, Infrasonic |
| g13 | 166-171 | Interstage, IronOxide5, IronOxideClassic, IronOxideClassic2, Isolator, Isolator2 |
| g14* | 172 | LeadAmp (1770) |
| g15 | 173 | LeftoMono |
| g16* | 174 | LilAmp (1204) |
| g17* | 175 | Logical4 (1699) |
| g18 | 176-180 | Loud, Lowpass, Lowpass2, LRFlipTimer, Luxor |
| g19 | 181-184 | MackEQ, Mackity, MatrixVerb, Melt |
| g20* | 185 | MidAmp (1290) |
| g21 | 186-187 | MidSide, Mojo |
| g22* | 188 | Monitoring (1042) |
| g23 | 189 | Monitoring2 |
| g24* | 190 | Monitoring3 (1050) |
| g25 | 191-192 | MoNoam, MultiBandDistortion |
| g26* | 193 | MV (1532) |
| g27 | 194-196 | NaturalizeDither, NCSeventeen, Neverland |
| g28* | 197 | Nikola (2047) |
| g29 | 198-199 | NodeDither, Noise |
| g30* | 200 | NonlinearSpace (1447) |
| g31 | 201-206 | NotJustAnotherCD, NotJustAnotherDither, OneCornerClip, Pafnuty, PaulDither, PaulWide |
| g32 | 207-212 | PDBuss, PDChannel, PeaksOnly, PhaseNudge, PitchDelay, PitchNasty |
| g33* | 213 | PocketVerbs (18358) |
| g34 | 214-220 | Pockey, Pockey2, Podcast, PodcastDeluxe, Point, Pop, Pop2 |

The totals are 96 normal plugins in 20 groups plus 14 oversized singletons, which makes 110. Each
group gets its own source directory `src/plugins/batch2/gNN/`, test file `tests/parity_batch2_gNN.rs`
and fixture directory `tests/fixtures/batch2/gNN/`.

Module layout follows the base design: module name = name lowercased, struct = upstream class name.
Normal plugins are `gNN/<module>.rs`. Every oversized plugin is a module directory
`gNN/<module>/mod.rs` plus submodules, as in batch-1 `g03/bigamp/`. Every `.rs` file stays under
1000 lines. A normal plugin whose port would reach 1000 lines is split the same way.

## 3. Plans and ownership

Plan ids use the prefix `ab2-` (airwindows batch 2). Plan files go in
`impl-plans/active/airwindows-batch2-<id>.md`, and the dispatch manifest in
`impl-plans/active/airwindows-batch2-dispatch.json`. The batch-1 manifest is never touched. The
evidence root is `tmp/airwindows-batch2` (gitignored by `**/tmp`).

| Plan | Wave | Groups (plugins) | Owns (writePaths, besides `tmp/airwindows-batch2/<id>/`) |
|------|------|------------------|----------------------------------------------------------|
| `ab2-00-scaffold` | W1 | - | `src/plugins/batch2/mod.rs`; all 34 `src/plugins/batch2/gNN/mod.rs`; `tools/airwindows-harness/generate_batch2.sh`; additive `tools/vst-shim/audioeffectx.{h,cpp}`; `tests/fixtures/batch2/**` (initial generation, 220 files); `tests/catalog_batch2.rs` |
| `ab2-n01` | W2 | g01 (8) | `src/plugins/batch2/g01/` (directory), `tests/parity_batch2_g01.rs` |
| `ab2-n02` | W2 | g02 (7) | same pattern for g02 |
| `ab2-n03` | W2 | g05 (6) | g05 |
| `ab2-n04` | W2 | g07 (5) | g07 |
| `ab2-n05` | W2 | g08 (5) | g08 |
| `ab2-n06` | W2 | g10 (7) | g10 |
| `ab2-n07` | W2 | g11 (7) | g11 |
| `ab2-n08` | W2 | g12 (6) | g12 |
| `ab2-n09` | W2 | g13 (6) | g13 |
| `ab2-n10` | W2 | g15 + g18 (6) | g15, g18 |
| `ab2-n11` | W2 | g19 + g21 (6) | g19, g21 |
| `ab2-n12` | W2 | g23 + g25 + g27 + g29 (8) | g23, g25, g27, g29 |
| `ab2-n13` | W2 | g31 (6) | g31 |
| `ab2-n14` | W2 | g32 (6) | g32 |
| `ab2-n15` | W2 | g34 (7) | g34 |
| `ab2-o-energy` .. `ab2-o-pocketverbs` (14 plans) | W2 | one oversized singleton group each (g03, g04, g06, g09, g14, g16, g17, g20, g22, g24, g26, g28, g30, g33) | that group's directory and test file |
| `ab2-99-integration` | W3 | - | read-only on sources; owns only its evidence directory, plus any repairs routed back to the owning plan |

- A port plan owns `gNN/mod.rs` of its groups (it fills `ENTRIES`), and it owns the whole group
  directory and the group's parity test file. It reads the fixtures in `tests/fixtures/batch2/gNN/`
  and does not regenerate them, except as described under harness-defect escalation below.
- Each plan has at most 8 plugins. Multi-group plans exist only so that groups of 1 or 2 plugins
  are not each a separate plan. They do not change ownership: every group still has exactly one owner.
- **No stubs or skips**: every plan states that each listed plugin must be fully ported, registered
  in `ENTRIES`, and covered by a passing parity test. A stub, an unregistered module, a missing
  plugin, or a parity test that is ignored or weakened is a failure of the plan, not a deferral.
  A plan cannot be marked Completed while any of its plugins is missing.
- **PocketVerbs** (`ab2-o-pocketverbs`): the plan splits into ordered subtasks.
  1. State and constructor, parameters and display (`mod.rs`, `state.rs`).
  2. One submodule per reverb type, following the branches of the upstream `processReplacing` type
     switch. A type whose port exceeds 1000 lines is split again, for example `<type>_a.rs`/`<type>_b.rs`.
  3. Registration and the parity test.

  Only `processReplacing` is ported, which is about half of the 18358-line file; the other half is
  `processDoubleReplacing`. The plan may span several sessions. `ENTRIES` of g33 stays empty until
  every type is ported, and the plan is not Completed until the plugin is registered and its parity
  test passes for all three renders.
- **Harness-defect escalation**: if a port plan finds a defect on the harness side, it records the
  defect and marks the plugin blocked. The fix goes to `ab2-00-scaffold`, the owner of `tools/` for
  batch 2, as a follow-up task with additive edits only. The affected plan then regenerates only its
  own `tests/fixtures/batch2/gNN/` with `generate_batch2.sh --group gNN`.

## 4. Harness (additive)

The existing `tools/airwindows-harness/generate.sh` is hardcoded to batch 1 (`line < 110`,
`out/batch1`, groups of 8), so its lines are not edited. The batch-1 tree also never needs to
reproduce batch-2 output. The scaffold adds `tools/airwindows-harness/generate_batch2.sh`:

- Usage: `generate_batch2.sh --work-dir DIR --out-root DIR [--group gNN] [NAME...]`. It requires
  `$MUSIC_THINGS_REFS`.
- It reads lines 111-220 of `$MUSIC_THINGS_REFS/kick-airwindows-set.txt` and fails unless there are
  exactly 110 lines. Each name maps to its group through an explicit table that equals section 2. The
  script fails if a name is missing from the table, or if a table entry is not on those lines.
- Build flags, compilation units, the `what.txt` description rule, and the driver invocation are
  identical to `generate.sh`. It reuses `driver.cpp`, `prelude.h` and `tools/vst-shim` unchanged.
  Output goes to `<out-root>/batch2/gNN/<Name>.{txt,f32}`, in the same format as batch 1.
- Work directory: `tmp/airwindows-batch2/<plan-id>/harness`.

The scaffold builds and runs the harness for all 110 plugins before W2 starts. If an upstream
plugin needs a shim symbol that is missing (for example a host call used by PitchDelay,
GlitchShifter or MatrixVerb), the scaffold adds it to `tools/vst-shim` additively. Existing
declarations and behavior stay byte-for-byte, so batch-1 fixtures stay reproducible. Each new symbol
is listed in the scaffold's progress log. Display helpers (D1) are never changed.

## 5. Parity and tolerances

- Each `tests/parity_batch2_gNN.rs` follows `tests/parity_batch1_g03.rs`:
  - one `ParityRecord` and one `#[test]` per plugin
  - construction through `create_with_seed(name, ..)`, which proves registration
  - the reset-preserves-params check and the fixture metadata checks
  - all three renders, with stored-frame comparison
- Classes and bounds are those of the base design: `Exact`, `Libm` with max-abs <= 1e-6, and
  `Custom(x)` only with a written reason. Every record carries the max-abs measured on
  aarch64-apple-darwin, and the module header doc repeats it. Plans report the measured value per
  plugin in their verification notes.
- An output above its bound is a defect in the port. A bound is raised only by updating this section
  with the reason.
- Large-state plugins (MatrixVerb, PocketVerbs, NonlinearSpace, Galactic, the delays) allocate
  arrays above 4096 elements as `Box<[T]>` in the constructor. `process`, `set_param` and `reset`
  never allocate, and the shared `tests/no_alloc.rs` checks this automatically for every catalog
  entry.

## 6. Batch-2 catalog and budget test (`tests/catalog_batch2.rs`, owned by `ab2-00-scaffold`)

The test holds a `BATCH2: [&str; 110]` list in kick-list order and a group table equal to section 2.
It asserts the following.

1. **Exact registered set, in order**: in `catalog()`, `DitherMeTimbers` sits immediately after
   `DitherMeDiskers`, and the 110 entries starting there equal `BATCH2` in order. This holds whether
   or not batch 3 has been merged, because batch-3 entries follow `Pop2`. Each name appears exactly
   once and `upstream_name == name`. `create(name, 44100.0)` and
   `create_with_seed(name, 96000.0, 1)` return `Some`.
2. **No skipped parity**: for every name:
   - `tests/fixtures/batch2/<gNN>/<Name>.txt` and `<Name>.f32` exist, and the `.f32` is 12,096 bytes.
   - `tests/parity_batch2_<gNN>.rs`, read with `std::fs`, contains `name: "<Name>"` (its `ParityRecord`)
     and no `#[ignore]`.
3. **Batch-2 fixture budget**: the byte total of `tests/fixtures/batch2/` is at most 3,000,000.
   Estimate: 110 x (12,096 + about 1 KB of metadata), about 1.45 MB, so the batch-1 stimulus length
   is kept.

Assertions 1 and 2 fail until every group is filled. That is intended: W2 plans run only their own
test files, and only `ab2-99-integration` runs this test as a gate.

## 7. Gates

All commands use `CARGO_TERM_QUIET=true`, with `CARGO_TARGET_DIR=tmp/airwindows-batch2/<plan-id>/target`.

**Scaffold (`ab2-00`)**:
- `cargo check -p music-things-airwindows --all-targets` passes. With every `ENTRIES` empty, it adds
  nothing to the catalog.
- `generate_batch2.sh` exits 0 for all 110 plugins and writes 220 files.
- `rustfmt --check` passes on its own `.rs` files.

**Port plans (W2)**, per owned group list `G`:
1. `cargo check -p music-things-airwindows --all-targets`
2. `cargo nextest run -p music-things-airwindows --test parity_batch2_gNN` for each owned group
3. `rustfmt --check` on its own files
4. `cargo clippy -p music-things-airwindows --all-targets --message-format short -- -D warnings 2>&1 | grep -E "src/plugins/batch2/(G)/|tests/parity_batch2_(G)\.rs"`
   prints nothing. The plan wraps it as `! ( ... )` or `test -z "$(...)"` so that it exits 0 when no
   own-file diagnostic exists.

They never gate on crate-wide clippy or on the full test suite.

**Integration (`ab2-99`)** on the final tree:

```sh
export CARGO_TERM_QUIET=true NEXTEST_STATUS_LEVEL=fail NEXTEST_FAILURE_OUTPUT=immediate-final NEXTEST_HIDE_PROGRESS_BAR=1
cargo nextest run --workspace --all-features
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo fmt --check
cargo build --workspace --all-features --target wasm32-unknown-unknown
```

It also checks that no `.rs` file under `src/plugins/batch2/` has 1000 or more lines (a `wc -l`
scan), and it records each plugin's measured max-abs.

**Shared-test conflict (blocks the full-nextest gate; open question Q1)**: two shared tests, which
this run must not touch, fail once batch 2 is registered and its fixtures are committed:

- `tests/batch1_complete.rs` asserts `catalog()` names `== BATCH1`, the exact 110 batch-1 names, so
  any batch-2 entry fails it.
- `tests/fixture_budget.rs` asserts that the whole `tests/fixtures/` tree is at most 1,600,000 bytes.
  Batch 1 alone measures 1432 KiB (`du -A -s -k`, about 1.47 MB), so about 0.13 MB is left against
  the roughly 1.45 MB that batch 2 needs. This also contradicts the base design's rule that each
  batch has its own 3 MB budget.

The recommended fix, which the operator must apply or authorize, is two minimal edits:

- `batch1_complete.rs` compares `&names[..BATCH1.len()]` with `BATCH1`.
- `fixture_budget.rs` measures `tests/fixtures/batch1` plus the top-level files only.

Per-batch budgets then live in `catalog_batchN.rs`. Batch 3 hits the same two failures, so the edit
should be made once by the operator, either on `main` or in the merge. Until Q1 is answered,
`ab2-99`:

- does not edit these files;
- runs the full nextest and records the two expected failures;
- runs `cargo nextest run --workspace --all-features -E 'not (binary(batch1_complete) | binary(fixture_budget))'`,
  which must pass;
- stays `Blocked (Q1)` instead of `Completed`.

## 8. Risks

- **Oversized ports skipped or stubbed** (the batch-1 BigAmp/Ditherbox pattern). Mitigations: one
  plan per oversized plugin, the no-stub clause in every plan, and `catalog_batch2.rs` assertions 1-2.
- **PocketVerbs size**: about 9k upstream lines on the float path. Mitigations: ordered subtasks,
  multi-session execution, and registration only when complete.
- **Shared-test conflict**: see section 7 and Q1.
- **Concurrent batch-3 edits to `tools/vst-shim`**: additive edits may still produce textual merge
  conflicts. New shim symbols are appended in a block marked `// batch 2 additions`.
- **Lock and disk pressure** from about 30 parallel plans, each with its own target directory.
  Plans may delete their own `target/` after the evidence is recorded.
- **Parity drift** from FMA contraction or `long double` is ruled out by the harness flags and the
  prelude assertion. Cross-host libm bounds are not calibrated, so the acceptance host is
  aarch64-apple-darwin.
