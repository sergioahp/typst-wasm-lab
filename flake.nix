{
  description = "Typst WASM lab development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.11";
  };

  outputs = { self, nixpkgs, ... }:
    let
      systems = [ "x86_64-linux" ];
      forAllSystems = f:
        nixpkgs.lib.genAttrs systems (system:
          let
            pkgs = import nixpkgs { inherit system; };
          in
          f system pkgs);
    in {
      devShells = forAllSystems (system: pkgs:
        {
          default = pkgs.mkShell {
            packages = with pkgs; [
              typst
              rustup
              git
              gh
              pkg-config
              openssl
            ];
            shellHook = ''
              echo "Typst WASM dev shell (${system}) activated. Use rustup to manage toolchains."
            '';
          };
        });
    };
}
