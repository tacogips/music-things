#!/bin/bash
# SPDX-License-Identifier: MIT
# Usage: generate_batch3.sh --work-dir DIR --out-root DIR [--group gNN] [NAME...]
# MUSIC_THINGS_REFS must point to the pinned upstream reference directory.
set -euo pipefail
: "${MUSIC_THINGS_REFS:?Set MUSIC_THINGS_REFS to the pinned upstream reference directory}"
work= out= group=
while (($#)); do case "$1" in --work-dir) work=$2; shift 2;; --out-root) out=$2; shift 2;; --group) group=$2; shift 2;; *) break;; esac; done
[ -n "$work" ] && [ -n "$out" ] || { echo 'usage: generate_batch3.sh --work-dir DIR --out-root DIR [--group gNN] [NAME...]' >&2; exit 2; }
mkdir -p "$work" "$out/batch3"
root=$(cd "$(dirname "$0")/../.." && pwd)
line=0; generated=0
while IFS= read -r name || [ -n "$name" ]; do
 line=$((line+1))
 ((line <= 329)) || break
 ((line >= 221)) || continue
 name=${name%$'\r'}
 grp=$(printf 'g%02d' $(((line-221)/8+1)))
 [ -z "$group" ] || [ "$group" = "$grp" ] || continue
 if (($#)); then selected=0; for requested in "$@"; do [ "$requested" != "$name" ] || selected=1; done; ((selected)) || continue; fi
 upstream=$name
 src="$MUSIC_THINGS_REFS/airwindows/plugins/LinuxVST/src/$upstream"
 desc=$(awk -v n="$upstream" 'index($0,n " ")==1 {sub(/\[coll=.*/, ""); sub(/\r$/, ""); print; exit}' "$MUSIC_THINGS_REFS/airwindows/what.txt")
 [ -n "$desc" ] || { echo "Missing description: $upstream" >&2; exit 1; }
 mkdir -p "$out/batch3/$grp"
 clang++ -std=c++17 -O1 -ffp-contract=off -fno-fast-math -fno-builtin -include "$root/tools/airwindows-harness/prelude.h" -I "$root/tools/vst-shim" -I "$root/tools/common" -I "$src" -DAW_CLASS="$upstream" -DAW_HEADER="\"$upstream.h\"" "$src/$upstream.cpp" "$src/${upstream}Proc.cpp" "$root/tools/airwindows-harness/driver.cpp" "$root/tools/vst-shim/audioeffectx.cpp" -o "$work/$name"
 "$work/$name" "$out/batch3/$grp/$name" "$name" "$upstream" "$desc"
 generated=$((generated+1))
 echo "$name: generated"
done < "$MUSIC_THINGS_REFS/kick-airwindows-set.txt"

if [ -z "$group" ] && (($# == 0)) && ((generated != 109)); then
 echo "Expected 109 plugins, generated $generated" >&2
 exit 1
fi
