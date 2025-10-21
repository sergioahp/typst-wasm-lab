# WASM Experiments

Two Rust-based Typst plugin prototypes live here:

1. `basic-plugin`: uppercases text, perfect for validating the wasm-minimal-protocol wiring.
2. `inline-diff-plugin`: computes inline character-level diffs and annotates insertions/deletions.

Both crates target `wasm32-unknown-unknown` and rely on `wasm-minimal-protocol` v0.1.0.

See each subdirectory `README.md` for a punchy rebuild guide and gotchas discovered along the way.
