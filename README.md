# Typst WASM Lab

Playground for experimenting with Typst documents and WebAssembly plugins.

## Layout
- `main.typ` – minimal demo showing Typst `highlight` usage.
- `codly-experiments/` – notes about the Codly project and copied Typst plugin docs.
- `reference-code/` – vendored reference crates (`codly` and `wasm-minimal-protocol`).
- `wasm-experiments/` – Rust prototypes compiled to Typst-friendly WebAssembly.

## Build
Both plugins rely on `rustup` and the `wasm32-unknown-unknown` target. On this NixOS setup:

```sh
nix-shell -p rustup --command 'cd wasm-experiments/basic-plugin && rustup run stable cargo build --release --target wasm32-unknown-unknown'
nix-shell -p rustup --command 'cd wasm-experiments/inline-diff-plugin && rustup run stable cargo build --target wasm32-unknown-unknown'
```

`basic-plugin` exports a text uppercaser, while `inline-diff-plugin` annotates character-level diffs.

## License
MIT – see [LICENSE](LICENSE).
