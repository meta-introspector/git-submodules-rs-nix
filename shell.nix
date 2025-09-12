{description = "Development shell for Rust project";inputs = {nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";rust-overlay.url = "github:oxalica/rust-overlay";};outputs = { self, nixpkgs, rust-overlay }:let system = "aarch64-linux"; pkgs = import nixpkgs {inherit system;overlays = [ rust-overlay.overlays.default ];};toolchain = pkgs.rust-bin.nightly.latest;in {devShells.${system}.default = pkgs.mkShell {buildInputs = [toolchain pkgs.git pkgs.pkg-config pkgs.openssl pkgs.valgrind pkgs.emacs pkgs.emacsPackages.magit];
  shellHook = ''
    export EMACSLOADPATH="${pkgs.lib.makeSearchPathOutput "share/emacs/site-lisp" [
      pkgs.emacsPackages.magit
    ]}"
  '';};};}