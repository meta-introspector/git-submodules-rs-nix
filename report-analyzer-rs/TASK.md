# Task: Boot into `report-analyzer-rs` and Build with Nix

This task outlines the steps to enter the development environment of the `report-analyzer-rs` crate using Nix and then build the project.

## Prerequisites

*   Nix installed on your system.
*   The `pick-up-nix` repository cloned.

## Steps

1.  **Navigate to the `report-analyzer-rs` directory:**

    ```bash
    cd source/github/meta-introspector/git-submodules-rs-nix/report-analyzer-rs
    ```

2.  **Enter the Nix development shell:**

    This will load all the necessary dependencies (Rust toolchain, etc.) defined in the `flake.nix` (which will be added in a later step).

    ```bash
    nix develop
    ```

    You should see output indicating that the development environment is loaded.

3.  **Build the project:**

    Once inside the Nix development shell, you can build the `report-analyzer-rs` project using Cargo.

    ```bash
    cargo build
    ```

    To build in release mode (optimized for performance):

    ```bash
    cargo build --release
    ```

    The compiled executable will be located in `target/debug/report-analyzer-rs` (for debug builds) or `target/release/report-analyzer-rs` (for release builds).

## Verification

After building, you can run the tool to verify its functionality:

```bash
./target/debug/report-analyzer-rs --help
```

Or, if you built in release mode:

```bash
./target/release/report-analyzer-rs --help
```
