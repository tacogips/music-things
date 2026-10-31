# music-things

Reusable, host-agnostic Rust synth and effect DSP crates.

- Crates extracted from [vactr](https://github.com/tacogips/vactr) so the
  synth and effect kernels can be reused outside the vactr engine.
- Rust ports of permissively licensed open-source synth and effect code, such
  as the components bundled with the Behringer KICK and IGNITE open firmware
  (Airwindows, TinySoundFont, pitch-shifting and pitch-tracking code), ported
  from their original upstream sources.

Every crate builds for `wasm32-unknown-unknown` and has no runtime
dependencies unless noted.

## Development

```sh
mise install
mise run test
mise run build-wasm
```

## License

MIT. Ported third-party code keeps its original license; see
`THIRD_PARTY_NOTICES.md`.
