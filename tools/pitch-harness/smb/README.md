# smbPitchShift reference harness

This MIT-licensed harness compiles the pinned smbPitchShift 1.2 source in place, with a minimal ESP32 include stub. Run from the repository root:

```sh
MUSIC_THINGS_REFS=/Users/taco/gits/tacogips/music-things-refs tools/pitch-harness/smb/generate.sh --work-dir tmp/behringer-oss-ports/bop-17a/harness
```

Each configuration runs in a separate process because upstream state is static. Input is truncated to int16 and output is stored as little-endian int16; the harness checks the stimulus hash and output bounds.
