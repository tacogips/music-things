#!/bin/sh
set -eu
[ "$#" -eq 2 ] && [ "$1" = --work-dir ] || { echo 'usage: generate.sh --work-dir DIR' >&2; exit 2; }
: "${MUSIC_THINGS_REFS:?set MUSIC_THINGS_REFS}"
mkdir -p "$2" crates/pitch/tests/fixtures/smb
clang++ -std=c++17 -O1 -ffp-contract=off -fno-fast-math -fno-builtin -I tools/pitch-harness/smb/stubs -I tools/common tools/pitch-harness/smb/smb_harness.cpp "$MUSIC_THINGS_REFS/smbPitchShift/smbPitchShift.esp32.cpp" -o "$2/smb_harness"
"$2/smb_harness" 1 crates/pitch/tests/fixtures/smb/c1.i16
"$2/smb_harness" 2 crates/pitch/tests/fixtures/smb/c2.i16
{
  printf 'stimulus_fnv=4ed50bb4be37bb1a\n'
  printf 'clang='; clang++ --version | head -n 1
  printf 'host='; uname -sm
  printf 'quantization=truncate(f32 stimulus * 32768.0) to int16\n'
} > crates/pitch/tests/fixtures/smb/meta.txt
