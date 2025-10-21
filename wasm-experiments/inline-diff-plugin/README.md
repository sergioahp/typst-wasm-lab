# Inline Diff Plugin TL;DR

## Goal
Expose an inline diff helper that wraps deletions/insertions with `[- ... -]` and `{+ ... +}` markers.

## Build Steps
1. `rustup target add wasm32-unknown-unknown` (via `nix-shell -p rustup --command …` on this box).
2. `nix-shell -p rustup --command 'cd wasm-experiments/inline-diff-plugin && rustup run stable cargo build --release --target wasm32-unknown-unknown'`
3. Find the wasm at `target/wasm32-unknown-unknown/release/inline_diff_plugin.wasm`

## Progression
- **v0**: baseline string passthrough to ensure the protocol wiring works (kept locally while iterating).
- **v1**: added Myers character diff using `similar` crate and wrapped changes in custom markers.
- **v2**: added validation + deterministic error message for non UTF-8 input and regression tests.

## Stumbling Blocks
- Remember to set `crate-type = ["cdylib"]`; otherwise Typst cannot load the module.
- Without `rustup` on $PATH, use `nix-shell -p rustup --command '<cmd>'` to interact with the toolchain.
- The `similar` crate defaults omit the inline helpers; character diffs work fine without extra features but only for valid UTF-8.
- Keep functions side-effect free—Typst may cache plugin results aggressively.
