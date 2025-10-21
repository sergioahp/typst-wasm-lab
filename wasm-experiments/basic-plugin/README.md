# Basic Plugin TL;DR

## Goal
Export a simple Typst plugin function that uppercases incoming text.

## Steps
1. `rustup target add wasm32-unknown-unknown`
2. `cargo build --release --target wasm32-unknown-unknown`
3. Output lives at `target/wasm32-unknown-unknown/release/basic_plugin.wasm`
4. Load from Typst with `#let upper = plugin("basic_plugin.wasm")`

## Stumbling Blocks
- The plugin must compile as a `cdylib`; set `crate-type = ["cdylib"]` in `Cargo.toml`.
- `wasm-minimal-protocol` v0.1.0 powers the glue code. Import `initiate_protocol!` once and wrap exported functions with `#[cfg_attr(target_arch = "wasm32", wasm_func)]`.
- Functions must accept `&[u8]` and return `Vec<u8>` (or `Result<Vec<u8>, E>`). Perform your own UTF-8 handling if needed.
- No host I/O: everything must be pure and deterministic.
- `cargo add wasm-minimal-protocol` fails if the index is stale; re-run after any network hiccups.
