#!/bin/bash
# SPDX-License-Identifier: MIT
set -euo pipefail
work= out= group=
usage() { echo 'usage: generate_batch2.sh --work-dir DIR --out-root DIR [--group gNN] [NAME...]' >&2; exit 2; }
while (($#)); do
 case "$1" in
  --work-dir|--out-root|--group) (($# >= 2)) || usage; case "$1" in --work-dir) work=$2;; --out-root) out=$2;; --group) group=$2;; esac; shift 2;;
  *) break;;
 esac
done
[ -n "$work" ] && [ -n "$out" ] || usage
: "${MUSIC_THINGS_REFS:?Set MUSIC_THINGS_REFS to the pinned upstream reference directory}"
mkdir -p "$work"
intent_dir=$(mktemp -d "$work/fixture-intents.XXXXXX")
root=$(cd "$(dirname "$0")/../.." && pwd)
table=$(cat <<'TABLE'
DitherMeTimbers g01
Doublelay g01
DoublePaul g01
Drive g01
DrumSlam g01
DubCenter g01
Dubly g01
DubSub g01
DustBunny g02
Dynamics g02
Dyno g02
Edge g02
EdIsDim g02
Elation g02
ElectroHat g02
Energy g03
Energy2 g04
Ensemble g05
EQ g05
EveryTrim g05
Exciter g05
Facet g05
FathomFive g05
FireAmp g06
Flipity g07
Floor g07
Flutter g07
Focus g07
Fracture g07
FromTape g08
Galactic g08
Gatelope g08
GlitchShifter g08
Golem g08
GrindAmp g09
Gringer g10
GrooveWear g10
GuitarConditioner g10
HardVacuum g10
Hermepass g10
HermeTrim g10
HighGlossDither g10
HighImpact g11
Highpass g11
Highpass2 g11
Holt g11
Holt2 g11
Hombre g11
Hull g11
Hype g12
Hypersonic g12
HypersonX g12
Infinity g12
Infinity2 g12
Infrasonic g12
Interstage g13
IronOxide5 g13
IronOxideClassic g13
IronOxideClassic2 g13
Isolator g13
Isolator2 g13
LeadAmp g14
LeftoMono g15
LilAmp g16
Logical4 g17
Loud g18
Lowpass g18
Lowpass2 g18
LRFlipTimer g18
Luxor g18
MackEQ g19
Mackity g19
MatrixVerb g19
Melt g19
MidAmp g20
MidSide g21
Mojo g21
Monitoring g22
Monitoring2 g23
Monitoring3 g24
MoNoam g25
MultiBandDistortion g25
MV g26
NaturalizeDither g27
NCSeventeen g27
Neverland g27
Nikola g28
NodeDither g29
Noise g29
NonlinearSpace g30
NotJustAnotherCD g31
NotJustAnotherDither g31
OneCornerClip g31
Pafnuty g31
PaulDither g31
PaulWide g31
PDBuss g32
PDChannel g32
PeaksOnly g32
PhaseNudge g32
PitchDelay g32
PitchNasty g32
PocketVerbs g33
Pockey g34
Pockey2 g34
Podcast g34
PodcastDeluxe g34
Point g34
Pop g34
Pop2 g34
TABLE
)
list=$(sed -n '111,220p' "$MUSIC_THINGS_REFS/kick-airwindows-set.txt" | sed 's/\r$//')
names=$(printf '%s\n' "$table" | awk '{print $1}')
if [ "$(printf '%s\n' "$list" | wc -l | tr -d ' ')" -ne 110 ] || [ "$list" != "$names" ]; then
 echo 'Batch 2 table differs from kick list lines 111-220' >&2; exit 1
fi
while read -r name grp; do
 [ -z "$group" ] || [ "$group" = "$grp" ] || continue
 if (($#)); then selected=0; for requested in "$@"; do [ "$requested" != "$name" ] || selected=1; done; ((selected)) || continue; fi
 upstream=$name
 src="$MUSIC_THINGS_REFS/airwindows/plugins/LinuxVST/src/$upstream"
 desc=$(awk -v n="$upstream" 'index($0,n " ")==1 {sub(/\[coll=.*/, ""); sub(/\r$/, ""); print; exit}' "$MUSIC_THINGS_REFS/airwindows/what.txt")
 [ -n "$desc" ] || { echo "Missing description: $upstream" >&2; exit 1; }
 mkdir -p "$out/batch2/$grp"
 for extension in txt f32; do
  path="$out/batch2/$grp/$name.$extension"
  if [ -f "$path" ]; then cat "$path" > /dev/null; fi
  printf 'Generate %s from pinned upstream %s with the three plan-specified renders and metadata.\n' "$path" "$upstream" > "$intent_dir/$name.$extension.intent"
 done
 clang++ -std=c++17 -O1 -ffp-contract=off -fno-fast-math -fno-builtin -include "$root/tools/airwindows-harness/prelude.h" -I "$root/tools/vst-shim" -I "$root/tools/common" -I "$src" -DAW_CLASS="$upstream" -DAW_HEADER="\"$upstream.h\"" "$src/$upstream.cpp" "$src/${upstream}Proc.cpp" "$root/tools/airwindows-harness/driver.cpp" "$root/tools/vst-shim/audioeffectx.cpp" -o "$work/$name"
 "$work/$name" "$out/batch2/$grp/$name" "$name" "$upstream" "$desc"
 echo "$name: generated"
done <<< "$table"
