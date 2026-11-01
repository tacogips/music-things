#!/bin/sh
set -eu
if [ "$#" -ne 2 ] || [ "$1" != "--work-dir" ]; then
    echo "usage: $0 --work-dir DIR" >&2
    exit 2
fi
: "${MUSIC_THINGS_REFS:?Set MUSIC_THINGS_REFS to the pinned upstream references}"
work_dir=$2
root=$(CDPATH= cd -- "$(dirname -- "$0")/../../.." && pwd)
cd "$root"
mkdir -p "$work_dir" crates/pitch/tests/fixtures/dywa
upstream="$MUSIC_THINGS_REFS/dywapitchtrack/dywapitchtrack-0.1.0.1"
clang -std=c99 -O1 -ffp-contract=off -fno-fast-math -fno-builtin -I "$upstream/include" -I tools/common tools/pitch-harness/dywa/dywa_harness.c "$upstream/cbits/dywapitchtrack.c" -lm -o "$work_dir/dywa_harness"
"$work_dir/dywa_harness" "$work_dir/pitches.txt" "$work_dir/meta.txt"
cp "$work_dir/pitches.txt" crates/pitch/tests/fixtures/dywa/pitches.txt
cp "$work_dir/meta.txt" crates/pitch/tests/fixtures/dywa/meta.txt
