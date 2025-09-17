{ pkgs ? import <nixpkgs> {} }:

pkgs.rustPlatform.buildRustPackage rec {
  pname = "report-analyzer-rs";
  version = "0.1.0"; # Match the version in Cargo.toml

  src = ./.; # Source is the current directory

  cargoLock = {
    lockFile = ./Cargo.lock;
  };

  # Add any build inputs required by your Rust project
  buildInputs = with pkgs; [
    # Add any system dependencies needed for building
  ];

  # Optional: Add check phase if you have tests
  # doCheck = true;

  meta = with pkgs.lib; {
    description = "A Rust crate for analyzing reports and data.";
    homepage = "https://github.com/railwayapp/nixpacks"; # Update with correct homepage
    license = licenses.mit; # Match your project's license
    platforms = platforms.linux;
  };
}