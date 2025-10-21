# Validation Playbook

Steps for verifying the inline diff plugin end-to-end and inspecting generated output.

## 1. Rebuild the Plugin
```sh
nix-shell -p rustup --command 'cd wasm-experiments/inline-diff-plugin && rustup run stable cargo build --release --target wasm32-unknown-unknown'
```
This produces `wasm-experiments/inline-diff-plugin/target/wasm32-unknown-unknown/release/inline_diff_plugin.wasm`.

## 2. Run Unit Tests
```sh
nix-shell -p rustup --command 'cd wasm-experiments/inline-diff-plugin && rustup run stable cargo test'
```
Confirms the Myers diff logic stays deterministic and UTF-8 handling works.

## 3. Compile the Typst Demo (PDF)
```sh
typst compile --root . wasm-experiments/inline-diff-plugin/demo.typ wasm-experiments/inline-diff-plugin/demo.pdf
```
Inspect the PDF with a viewer (`xdg-open`, `evince`, etc.) to ensure:
- `ca{+r+}t` and `let x = 1{+0+};` render as expected.
- The error message shows the ASCII replacement characters for non-UTF-8 inputs.

## 4. Export to PNG for Quick Glance
```sh
typst compile --root . wasm-experiments/inline-diff-plugin/demo.typ wasm-experiments/inline-diff-plugin/demo.png
```
View the PNG using an image tool of choice (`feh`, `display`, `sxiv`). This avoids launching a full PDF viewer when sanity-checking small changes.

## 5. Capture Rendered Output Digest
Optional one-liner to generate a hash of the rendered PDF for regression tracking:
```sh
shasum -a 256 wasm-experiments/inline-diff-plugin/demo.pdf
```

## 6. Clean Artifacts
```sh
rm -f wasm-experiments/inline-diff-plugin/demo.pdf wasm-experiments/inline-diff-plugin/demo.png
```
Use after capturing snapshots to keep the repo tidy (build outputs stay untracked thanks to `.gitignore`).
