
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
          repo = pkgs.fetchgit {
            url = "https://github.com/jmikedupont2/git-submodules-rs-nix.git";
            rev = "b3da08e8e7fe2bb97b97bce5451d6750081256f8"; # Use the specific revision
            deepClone = true; # Ensure submodules are fetched
            sha256 = "Oh+QTs0zkWiLC7Az55RDTb2B7xLmVfCiXTfRu7zi9rM=";
          };
        in
        {
          checks = {
            gitoxide = pkgs.runCommand "gitoxide-tests" {
              src = repo; # Pass the specific submodule as source
              buildInputs = [ toolchain ];
              CARGO_HOME = "$TMPDIR/.cargo"; # Set CARGO_HOME to a writable directory
            } ''
              mkdir -p $CARGO_HOME # Create the directory
              cd gitoxide && cargo test
              touch $out
            '';
            submod = pkgs.runCommand "submod-tests" {
              src = repo; # Pass the specific submodule as source
              buildInputs = [ toolchain ];
              CARGO_HOME = "$TMPDIR/.cargo"; # Set CARGO_HOME to a writable directory
            } ''
              mkdir -p $CARGO_HOME # Create the directory
              cd submod && cargo test
              touch $out
            '';
            magoo = pkgs.runCommand "magoo-tests" {
              src = repo; # Pass the specific submodule as source
              buildInputs = [ toolchain ];
              CARGO_HOME = "$TMPDIR/.cargo"; # Set CARGO_HOME to a writable directory
            } ''
              mkdir -p $CARGO_HOME # Create the directory
              cd magoo && cargo test
              touch $out
            '';
          };

          packages = {
            git-config-parser = pkgs.rustPlatform.buildRustPackage {
              pname = "git-config-parser";
              version = "0.1.0"; # Get this from Cargo.toml
              src = ./src/bin/git-config-parser; # Source of the git-config-parser binary
              cargoLock = {
                lockFile = ./src/bin/git-config-parser/Cargo.lock; # Path to the Cargo.lock for this package
              };
              buildInputs = [ toolchain ]; # Use the defined toolchain
            };

            submodules-managed = pkgs.runCommand "submodules-managed" {
              src = repo; # Use the fetched repository as source
              buildInputs = [ pkgs.coreutils ]; # Ensure cp is available
            } ''
              # Create target directories
              mkdir -p $out/gitoxide
              mkdir -p $out/submod
              mkdir -p $out/magoo
              mkdir -p $out/git-submodule-tools

              # Manually copy submodules
              cp -r $src/gitoxide $out/gitoxide
              cp -r $src/submod $out/submod
              cp -r $src/magoo $out/magoo
              cp -r $src/git-submodule-tools $out/git-submodule-tools
            '';
          };
        });
    in
    forAllSystems;
}
