{
  description = "Nix devenv";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
      {
        # `nix develop`
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            pkg-config
            rustup
            # rust-bin.nightly.latest.default
            cargo-udeps
            cargo-nextest
            cargo-watch
            cargo-insta
            cargo-expand
            bacon
            act
          ];
        };
      }
    );
}
