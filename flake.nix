
{
  description = "A flake for testing git submodules";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";

    gitoxide = {
      url = "path:./gitoxide";
      flake = false;
    };
    submod = {
      url = "path:./submod";
      flake = false;
    };
    magoo = {
      url = "path:./magoo";
      flake = false;
    };
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, gitoxide, submod, magoo }@inputs:
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
              src = gitoxide;
              buildInputs = [ toolchain ];
            } ''
              cd $src
              cargo test
              touch $out
            '';
            submod = pkgs.runCommand "submod-tests" {
              src = submod;
              buildInputs = [ toolchain ];
            } ''
              cd $src
              cargo test
              touch $out
            '';
            magoo = pkgs.runCommand "magoo-tests" {
              src = magoo;
              buildInputs = [ toolchain ];
            } ''
              cd $src
              cargo test
              touch $out
            '';
          };
        });
    in
    forAllSystems;
}
