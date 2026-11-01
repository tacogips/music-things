#!/bin/sh
# MIT License
# 
# Copyright (c) 2026 music-things contributors
# 
# Permission is hereby granted, free of charge, to any person obtaining a copy
# of this software and associated documentation files (the "Software"), to deal
# in the Software without restriction, including without limitation the rights
# to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies
# of the Software, and to permit persons to whom the Software is furnished to do
# so, subject to the following conditions:
# 
# The above copyright notice and this permission notice shall be included in all
# copies or substantial portions of the Software.
# 
# THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
# IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
# FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
# AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
# LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
# OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
# SOFTWARE.
set -eu
if [ "$#" -ne 2 ] || [ "$1" != "--work-dir" ]; then
    printf 'usage: %s --work-dir DIR\n' "$0" >&2
    exit 2
fi
: "${MUSIC_THINGS_REFS:?set MUSIC_THINGS_REFS to the upstream reference directory}"
work_dir=$2
mkdir -p "$work_dir/inputs" crates/soundfont/tests/fixtures/tml
work_dir=$(cd "$work_dir" && pwd)
SOUNDFONT_FIXTURE_OUT="$work_dir/inputs" CARGO_TERM_QUIET=true cargo test -p music-things-soundfont --test midi_parity -- --ignored export_synthetic_midi
clang -std=c99 -O1 -ffp-contract=off -fno-fast-math -fno-builtin \
    -I "$MUSIC_THINGS_REFS/TinySoundFont" tools/soundfont-harness/tml_harness.c -o "$work_dir/tml_harness"
for name in scenario format0 truncated truncated_late bad_meta; do
    "$work_dir/tml_harness" "$work_dir/inputs/$name.mid" > "crates/soundfont/tests/fixtures/tml/$name.txt"
done
python3 - "$work_dir/inputs" > crates/soundfont/tests/fixtures/tml/inputs.txt <<'PY'
import pathlib
import sys

root = pathlib.Path(sys.argv[1])
for name in ("scenario", "format0", "truncated", "truncated_late", "bad_meta"):
    value = 0xcbf29ce484222325
    for byte in (root / (name + ".mid")).read_bytes():
        value = ((value ^ byte) * 0x100000001b3) & 0xffffffffffffffff
    print(f"{name}={value:016x}")
PY
