# Design References

This directory contains reference materials for system design and implementation.

## External References

| Name | URL | Description |
|------|-----|-------------|
| The Rust Book | https://doc.rust-lang.org/book/ | Official Rust programming language book |
| Rust API Guidelines | https://rust-lang.github.io/api-guidelines/ | Rust API design best practices |
| Rust Design Patterns | https://rust-unofficial.github.io/patterns/ | Common Rust design patterns |

## Behringer OSS Ports (pinned upstream sources)

The operator keeps read-only checkouts under `$MUSIC_THINGS_REFS` (indexed by its `REFERENCES.md`).
They are compiled in place by `tools/` harnesses and never copied into this repository.

| Component | URL | Pinned revision | License |
|-----------|-----|-----------------|---------|
| Airwindows (`plugins/LinuxVST/src/<Name>/`, `what.txt`) | https://github.com/airwindows/airwindows | `b110fac3991fff7c4655cc2d25d6a3e221f9799c` | MIT |
| TinySoundFont (`tsf.h`, `tml.h`) | https://github.com/schellingb/TinySoundFont | `853a0a171759f1ddba0de1442133a75912bbeffa` | MIT |
| stftPitchShift (`cpp/`) | https://github.com/jurihock/stftPitchShift | `70c96db087458f8631edbc1bb4068d42ea5b52cc` | MIT |
| dywapitchtrack (`cbits`) | dywapitchtrack-0.1.0.1 (Antoine Schmitt) | 0.1.0.1 | MIT |
| smbPitchShift (`smbPitchShift.esp32.cpp`) | Stephan M. Bernsee, smbPitchShift 1.2 | 1.2 | Wide Open License |
| KICK Airwindows set (`kick-airwindows-set.txt`) | operator-compiled list of 329 plugin names | - | - |

Music Tribe KICK/IGNITE firmware (AGPL-3.0) is never a reference; see
`design-docs/specs/design-behringer-oss-ports.md`.

## Reference Documents

Reference documents should be organized by topic:

```
references/
├── README.md              # This index file
├── rust/                  # Rust patterns and practices
└── <topic>/               # Other topic-specific references
```

## Adding References

When adding new reference materials:

1. Create a topic directory if it does not exist
2. Add reference documents with clear naming
3. Update this README.md with the reference entry
