{
  description = "A very basic Rust flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
      };
    };
  };

  outputs = {
    self,
    rust-overlay,
    nixpkgs,
    flake-utils,
    ...
  }: let
    system = "x86_64-linux";
    pkgs = import nixpkgs {inherit system;};
    nativeBuildInputs = with pkgs; [bashInteractive];
    overlays = [(import rust-overlay)];
    rustToolchain = pkgs.pkgsBuildHost.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
  in
    with pkgs; {
      devShells.default = mkShell {
        inherit nativeBuildInputs;
        buildInputs = [rustToolchain];
      };
    };
}
