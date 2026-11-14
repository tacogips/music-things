#!/bin/sh
# MIT License; Copyright (c) 2026 music-things contributors.
# Permission is granted to use, copy, modify, publish and distribute this
# software with this notice. THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY.
set -eu
if [ "$#" -ne 2 ] || [ "$1" != "--work-dir" ]; then
    printf 'usage: %s --work-dir DIR\n' "$0" >&2
    exit 2
fi
: "${MUSIC_THINGS_REFS:?set MUSIC_THINGS_REFS to the upstream reference directory}"
work_dir=$2
mkdir -p "$work_dir/inputs" crates/soundfont/tests/fixtures/synth
work_dir=$(cd "$work_dir" && pwd)
SOUNDFONT_FIXTURE_OUT="$work_dir/inputs" CARGO_TERM_QUIET=true cargo test -p music-things-soundfont --test synth_parity -- --ignored export_synth_inputs
clang -std=c99 -O1 -ffp-contract=off -fno-fast-math -fno-builtin -I "$MUSIC_THINGS_REFS/TinySoundFont" tools/soundfont-harness/synth_harness.c -lm -o "$work_dir/synth_harness"
"$work_dir/synth_harness" "$work_dir/inputs/synthetic.sf2" "$work_dir/inputs/scenario.mid" crates/soundfont/tests/fixtures/synth/render_s1.f32 crates/soundfont/tests/fixtures/synth/render_s2.f32
python3 - "$work_dir/inputs" > crates/soundfont/tests/fixtures/synth/inputs.txt <<'PY'
import pathlib
import sys
root = pathlib.Path(sys.argv[1])
for name, extension in (("synthetic", "sf2"), ("scenario", "mid")):
    value = 0xcbf29ce484222325
    for byte in (root / (name + "." + extension)).read_bytes():
        value = ((value ^ byte) * 0x100000001b3) & 0xffffffffffffffff
    print(f"{name}={value:016x}")
PY
