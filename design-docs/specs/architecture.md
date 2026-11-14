# Architecture Design

This document describes system architecture and design decisions.

## Overview

Architectural patterns, system structure, and technical decisions.

---

## Sections

### Behringer KICK/IGNITE OSS ports

The workspace adds three dependency-free, wasm32-compatible DSP crates, each ported from pinned
upstream sources and never from Music Tribe AGPL code:

- `crates/airwindows` (`music-things-airwindows`): an additive plugin framework (trait, metadata,
  seeded fpd dither, by-name factory and catalog) plus the Airwindows plugins of the KICK set.
- `crates/soundfont` (`music-things-soundfont`): ports of TinySoundFont `tsf.h` and `tml.h`.
- `crates/pitch` (`music-things-pitch`): ports of smbPitchShift, the stftPitchShift core and
  dywapitchtrack.

The facade `crates/music-things` re-exports them behind the optional features `airwindows`,
`soundfont` and `pitch`. Parity with upstream is proven by committed fixtures, which the MIT
reference harnesses under `tools/` generate.

Details are in [design-behringer-oss-ports.md](./design-behringer-oss-ports.md): crate APIs, the
registration layout, porting rules, the parity method, per-component tolerances, plan ownership,
and the additive procedure for batches 2 and 3.

---
