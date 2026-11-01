# STFT reference fixtures

This MIT-licensed harness compiles `StftPitchShift.cpp` directly from the pinned
upstream stftPitchShift revision `70c96db087458f8631edbc1bb4068d42ea5b52cc`.
It uses `-std=c++20 -O1 -ffp-contract=off -fno-fast-math -fno-builtin`.

```sh
MUSIC_THINGS_REFS=/path/to/music-things-refs tools/pitch-harness/stft/generate.sh --work-dir tmp/behringer-oss-ports/bop-17b/harness
```

The output is three little-endian float files of 11025 samples using the shared
pitch stimulus. `meta.txt` records the checksum, host, compiler and parameters.
C3 uses an analysis frame of 1024 and synthesis frame of 512 samples.
