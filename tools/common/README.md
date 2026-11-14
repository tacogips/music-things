# Shared deterministic stimulus

The MIT-licensed C99 helpers generate deterministic IEEE floating-point input without
transcendental functions. `parabolic(p)` is `4*x*(1-abs(x))`, where `x=2*p-1`.
Airwindows uses 8192 stereo frames: an impulse, a 220/330 Hz parabolic tone
with amplitude ramp 0.05 to 1.2, xorshift32 noise, then silence. Pitch uses
11025 mono frames at 44100 Hz with a 220–330 Hz chirp, its second harmonic,
and xorshift32 noise. All arithmetic is double until the final float conversion.
FNV-1a-64 hashes float bits in little-endian byte order; stereo is interleaved.

Build and run from the workspace root:

```sh
clang -std=c99 -O1 -ffp-contract=off -fno-fast-math -fno-builtin -o tmp/behringer-oss-ports/bop-00/stimulus_check tools/common/stimulus_check.c
tmp/behringer-oss-ports/bop-00/stimulus_check
```
