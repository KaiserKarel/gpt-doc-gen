{
  description = "gpt-doc-gen";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs = inputs@{ self, nixpkgs, rust-overlay, flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems =
        [ "x86_64-linux" "aarch64-linux" "aarch64-darwin" "x86_64-darwin" ];
      perSystem = { config, self', inputs', pkgs, system, ... }:
        let
          crane = rec {
            lib = self.inputs.crane.lib.${system};
            stable = lib.overrideToolchain self'.packages.rust-stable;
          };
        in {
          packages = {
            rust-stable = inputs'.rust-overlay.packages.rust.override {
              extensions = [ "rust-src" "rust-analyzer" "clippy" ];
            };
            default = crane.stable.buildPackage {
              src = ./.;
              cargoBuildCommand = "cargo build --release";
            };
          };
          devShells = {
            default = pkgs.mkShell {
              buildInputs = [ self'.packages.rust-stable ]
                ++ (with pkgs; [ bacon rnix-lsp hyperfine cargo-flamegraph ]);
            };
          };
        };
    };
}
