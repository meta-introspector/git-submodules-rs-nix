
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
          # Fetch the entire repository including submodules from remote
          repo = builtins.fetchGit {
            url = "https://github.com/jmikedupont2/git-submodules-rs-nix.git";
            rev = "b3da08e8e7fe2bb97b97bce5451d6750081256f8"; # Use the specific revision
            submodules = true; # Fetch submodules
          };
        in
        {
          checks = {
            gitoxide = pkgs.runCommand "gitoxide-tests" {
              src = repo; # Pass the entire repo as source
              buildInputs = [ toolchain ];
            } ''
              cp -r $src/gitoxide . # Copy gitoxide submodule
              cd gitoxide
              cargo test
              touch $out
            '';
            submod = pkgs.runCommand "submod-tests" {
              src = repo; # Pass the entire repo as source
              buildInputs = [ toolchain ];
            } ''
              cp -r $src/submod . # Copy submod submodule
              cd submod
              cargo test
              touch $out
            '';
            magoo = pkgs.runCommand "magoo-tests" {
              src = repo; # Pass the entire repo as source
              buildInputs = [ toolchain ];
            } ''
              cp -r $src/magoo . # Copy magoo submodule
              cd magoo
              cargo test
              touch $out
            '';
          };
        });
    in
    forAllSystems;
}
