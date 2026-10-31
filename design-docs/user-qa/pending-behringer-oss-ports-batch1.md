# Behringer OSS Ports Batch 1: Defaults Awaiting Confirmation

**Status**: Pending Decision (non-blocking; implementation proceeds with the defaults)

**Created**: 2026-10-31

**Category**: Architecture

## Decisions Needed

Each item already has a default recorded in
`design-docs/specs/design-behringer-oss-ports.md`. Overriding a default changes only the
section named.

1. **Airwindows display strings (D1)**: the Steinberg VST2 SDK is unavailable, so `float2string`,
   `dB2string` and `int2string` follow a clean-room behavioral spec that the MIT shim and Rust share.
   Default: accept it. Character-exact SDK formatting is not claimed.
2. **stftPitchShift streaming wrapper (D2)**: `StftPitchShiftStream` is an addition with no upstream
   equivalent. It is verified against the ported offline path, which itself is a direct port of the
   pinned core and `RFFT.h`. Default: accept this verification.
3. **Cross-host tolerance calibration**: the bounds (Airwindows `Libm` 1e-6, tsf 1e-5, stft 1e-5, smb 3 LSB in the int16 domain) are
   asserted on every host but measured only on aarch64-apple-darwin. Default: accept, and widen per
   component with a documented reason if another host's CI fails.
4. **Fixture regeneration**: fixtures are committed and regenerated manually with the `tools/`
   scripts (which need `$MUSIC_THINGS_REFS` and clang++). They are not regenerated in CI and there is
   no mise task, because `mise.toml` is outside this run's write scope. Default: accept.
