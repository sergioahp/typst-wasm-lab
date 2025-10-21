#!/usr/bin/env zsh
set -euo pipefail

# 1) cd to project root
cd /home/admin/code/typst/diff

# 2) Ensure target directory exists
mkdir -p reference-code/wasm-minimal-protocol

# 3) Download crate tarball via crates.io API
curl -fL https://crates.io/api/v1/crates/wasm-minimal-protocol/0.1.0/download \
  -o reference-code/wasm-minimal-protocol/wasm-minimal-protocol-0.1.0.crate

# 4) Extract into the same directory and remove the .crate file
tar -xzf reference-code/wasm-minimal-protocol/wasm-minimal-protocol-0.1.0.crate \
  -C reference-code/wasm-minimal-protocol
rm reference-code/wasm-minimal-protocol/wasm-minimal-protocol-0.1.0.crate

# 5) Download rendered rustdoc page for src/lib.rs into lib.rs.html
curl -fL "https://docs.rs/wasm-minimal-protocol/0.1.0/src/wasm_minimal_protocol/lib.rs.html" \
  -o reference-code/wasm-minimal-protocol/lib.rs.html

# 6) Final tree listing
ls -R /home/admin/code/typst/diff/reference-code/wasm-minimal-protocol

