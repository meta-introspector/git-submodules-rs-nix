{
  description = "Nix flake for report-analyzer-rs";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.05";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            rustc
            cargo
          ];
          shellHook = ''
            echo "Welcome to report-analyzer-rs development shell!"
          '';
        };

        packages.default = pkgs.callPackage ./default.nix { };
      }
    );
}