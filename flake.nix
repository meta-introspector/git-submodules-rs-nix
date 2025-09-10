
{
  description = "A flake for testing git submodules";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

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

  outputs = { self, nixpkgs, gitoxide, submod, magoo }@inputs:
    let
      systems = [ "x86_64-linux" "aarch64-linux" ];
      forAllSystems = nixpkgs.lib.genAttrs systems;
    in
    {
      checks = forAllSystems (system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
          rust-toolchain = pkgs.callPackage (
            { rust-bin }:
            rust-bin.fromRustupToolchainFile {
              file = ./rust-toolchain.toml;
            }
          ) {};
        in
        {
          gitoxide = pkgs.runCommand "gitoxide-tests" {
            src = gitoxide;
            buildInputs = [ rust-toolchain ];
          } ''
            cd $src
            cargo test
            touch $out
          '';
          submod = pkgs.runCommand "submod-tests" {
            src = submod;
            buildInputs = [ rust-toolchain ];
          } ''
            cd $src
            cargo test
            touch $out
          '';
          magoo = pkgs.runCommand "magoo-tests" {
            src = magoo;
            buildInputs = [ rust-toolchain ];
          } ''
            cd $src
            cargo test
            touch $out
          '';
        });
    };
}
