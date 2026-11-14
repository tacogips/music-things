#!/bin/sh
# SPDX-License-Identifier: MIT
set -eu
if [ "$#" -ne 2 ] || [ "$1" != "--work-dir" ]; then
  printf 'usage: %s --work-dir DIR\n' "$0" >&2
  exit 2
fi
: "${MUSIC_THINGS_REFS:?MUSIC_THINGS_REFS is required}"
root=$(CDPATH= cd -- "$(dirname -- "$0")/../../.." && pwd)
work=$2
mkdir -p "$work"
fixture="$root/crates/pitch/tests/fixtures/stft"
mkdir -p "$fixture"
clang++ -std=c++20 -O1 -ffp-contract=off -fno-fast-math -fno-builtin \
  -I "$MUSIC_THINGS_REFS/stftPitchShift/cpp" \
  "$root/tools/pitch-harness/stft/stft_harness.cpp" \
  "$MUSIC_THINGS_REFS/stftPitchShift/cpp/StftPitchShift/StftPitchShift.cpp" \
  -o "$work/stft_harness"
"$work/stft_harness" "$fixture" > "$work/checksum.txt"
{
  cat "$work/checksum.txt"
  clang++ --version | head -n 1
  uname -sm
  printf '%s\n' 'upstream=70c96db087458f8631edbc1bb4068d42ea5b52cc' \
    'c1=1024/1024 hop=256 factors=1.5 quefrency=0.001 distortion=1 normalization=off' \
    'c2=1024/1024 hop=256 factors=0.75,1.25 quefrency=0 distortion=1 normalization=on' \
    'c3=1024/512 hop=128 factors=1.25 quefrency=0 distortion=1 normalization=off' \
    'samples=11025 sample_rate=44100 format=f32le'
} > "$fixture/meta.txt"
cat "$fixture/meta.txt"
