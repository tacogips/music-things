# dywapitchtrack reference fixtures

`generate.sh --work-dir DIR` compiles the pinned dywapitchtrack 0.1.0.1 C source
in place using `MUSIC_THINGS_REFS`, and writes the pitch sequences and shared
f32 stimulus FNV-1a checksum to `crates/pitch/tests/fixtures/dywa/`. The work
directory holds the executable and intermediate output. Each window uses a
fresh upstream tracker and non-overlapping 44100 Hz stimulus windows.
