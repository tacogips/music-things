#!/bin/bash
# SPDX-License-Identifier: MIT
set -euo pipefail
: "${MUSIC_THINGS_REFS:?Set MUSIC_THINGS_REFS to the pinned upstream reference directory}"
work= out= group=
while (($#)); do case "$1" in --work-dir) work=$2; shift 2;; --out-root) out=$2; shift 2;; --group) group=$2; shift 2;; *) break;; esac; done
[ -n "$work" ] && [ -n "$out" ] || { echo 'usage: generate.sh --work-dir DIR --out-root DIR [--group gNN] [NAME...]' >&2; exit 2; }
mkdir -p "$work" "$out/batch1"
intent_dir=$(mktemp -d "$work/fixture-intents.XXXXXX")
root=$(cd "$(dirname "$0")/../.." && pwd)
line=0
while IFS= read -r name && ((line < 110)); do
 line=$((line+1)); grp=$(printf 'g%02d' $(((line-1)/8+1)))
 [ -z "$group" ] || [ "$group" = "$grp" ] || continue
 if (($#)); then selected=0; for requested in "$@"; do [ "$requested" != "$name" ] || selected=1; done; ((selected)) || continue; fi
 upstream=$name; [ "$name" != Console8BusOut ] || upstream=Console8BussOut
 src="$MUSIC_THINGS_REFS/airwindows/plugins/LinuxVST/src/$upstream"
 desc=$(awk -v n="$upstream" 'index($0,n " ")==1 {sub(/\[coll=.*/, ""); sub(/\r$/, ""); print; exit}' "$MUSIC_THINGS_REFS/airwindows/what.txt")
 [ -n "$desc" ] || { echo "Missing description: $upstream" >&2; exit 1; }
 mkdir -p "$out/batch1/$grp"
 for extension in txt f32; do
  path="$out/batch1/$grp/$name.$extension"
  if [ -f "$path" ]; then cat "$path" > /dev/null; fi
  printf 'Generate %s from pinned upstream %s with the three plan-specified renders and metadata.\n' "$path" "$upstream" > "$intent_dir/$name.$extension.intent"
 done
 clang++ -std=c++17 -O1 -ffp-contract=off -fno-fast-math -fno-builtin -include "$root/tools/airwindows-harness/prelude.h" -I "$root/tools/vst-shim" -I "$root/tools/common" -I "$src" -DAW_CLASS="$upstream" -DAW_HEADER="\"$upstream.h\"" "$src/$upstream.cpp" "$src/${upstream}Proc.cpp" "$root/tools/airwindows-harness/driver.cpp" "$root/tools/vst-shim/audioeffectx.cpp" -o "$work/$name"
 "$work/$name" "$out/batch1/$grp/$name" "$name" "$upstream" "$desc"
 echo "$name: generated"
done < "$MUSIC_THINGS_REFS/kick-airwindows-set.txt"
