{
  description = "Nix flake for report-analyzer-rs";

  inputs = {
    nixpkgs.url = "github:meta-introspector/nixpkgs?ref=feature/CRQ-016-nixify";
    flake-utils.url = "github:meta-introspector/flake-utils?ref=feature/CRQ-016-nixify";
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