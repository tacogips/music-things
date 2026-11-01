# Airwindows reference harness

Requires the pinned `MUSIC_THINGS_REFS`, Apple arm64 with `long double` equal to `double`, and clang++. Run `MUSIC_THINGS_REFS=/path/to/refs tools/airwindows-harness/generate.sh --work-dir tmp/behringer-oss-ports/bop-01/harness --out-root crates/airwindows/tests/fixtures`. Optional `--group gNN` and plugin names restrict generation. Each executable compiles original upstream C++ in place. Flags disable FMA contraction, fast math and builtin folding to stabilize parity. The prelude asserts the floating-point ABI.

Descriptions use the first what.txt line beginning with the upstream name and a space, with the trailing `[coll=...]` annotation and carriage return removed. Group plans must never edit this harness; report defects to bop-01.
