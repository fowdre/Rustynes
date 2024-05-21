{
  description = "Rust broker-v2 dev environment";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils = {
      url = "github:numtide/flake-utils";
      inputs.nixpkgs.follows = "nixkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    rust-overlay,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [
            (import rust-overlay)
          ];
        };
      in {
        devShell = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            (pkgs.rust-bin.stable.latest.default.override {
                  extensions = [ "rust-src" "cargo" "rustc" ];
            })
            gcc
          ];

          RUST_SRC_PATH = "${pkgs.rust-bin.stable.latest.default.override {
              extensions = [ "rust-src" ];
          }}/lib/rustlib/src/rust/library";


          buildInputs = with pkgs; [
            clippy
            rust-analyzer
            just
            cargo
            rustc
            rustfmt
            clippy
          ];
        };
      }
    );
}
