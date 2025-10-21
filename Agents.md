# Agent Guidelines

- Environment runs on modern Nix; prefer `nix shell` (note the space) or flakes for tool availability.
- A project `flake.nix` is provided—run `nix develop` for the curated Typst/WASM toolchain. `nix flake show` should evaluate cleanly before iterating.
- Never install tooling system-wide. Keep everything inside this repo, ephemeral shells, or flake-driven dev shells.
- Recommended workflow:
  1. `nix develop` (requires flakes) for the curated toolchain.
  2. Alternatively, run `nix shell nixpkgs#package` for ad-hoc needs.
- Avoid global cache pollution; store experiment artefacts under repo subdirectories only.
