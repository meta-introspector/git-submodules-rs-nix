
{
  description = "A flake for testing git submodules";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }@inputs:
    let
      forAllSystems = flake-utils.lib.eachDefaultSystem (system:
        let
          overlays = [ (import rust-overlay) ];
          pkgs = import nixpkgs {
            inherit system overlays;
          };
          toolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        in
        {
          checks = {
            gitoxide = pkgs.runCommand "gitoxide-tests" {
              src = self;
              buildInputs = [ toolchain pkgs.git ];
            } ''
              cd $src
              git submodule update --init --recursive gitoxide
              cd gitoxide
              cargo test
              touch $out
            '';
            submod = pkgs.runCommand "submod-tests" {
              src = self;
              buildInputs = [ toolchain pkgs.git ];
            } ''
              cd $src
              git submodule update --init --recursive submod
              cd submod
              cargo test
              touch $out
            '';
            magoo = pkgs.runCommand "magoo-tests" {
              src = self;
              buildInputs = [ toolchain pkgs.git ];
            } ''
              cd $src
              git submodule update --init --recursive magoo
              cd magoo
              cargo test
              touch $out
            '';
          };
        });
    in
    forAllSystems;
}
