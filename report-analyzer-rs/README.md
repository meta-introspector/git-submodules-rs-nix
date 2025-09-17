# Report Analyzer RS

This Rust crate is designed for analyzing various reports and data, particularly in the context of Git submodules and Nix environments.

## Features

*   **Submodule Report Analysis**: Analyzes collected submodule data, generating statistics on successful/failed repositories, duplicate entries, and frequency analysis of organizations, repository/submodule names, and common n-grams.
*   **N-gram Analysis**: Generates n-grams (2-grams, 3-grams, etc.) and their graph edges to analyze word counts and identify patterns in textual data.
*   **CRQ Classification**: Implements logic for classifying Change Requests (CRQs) based on predefined thresholds and communication logs.
*   **Context Introspection Integration**: Supports the project's focus on context introspection by mapping project files into a lattice structure based on extracted predicates.

## Nix Analysis Capabilities

This tool now includes experimental capabilities for analyzing Nix files. It can parse Nix expressions, extract various metrics (e.g., number of attributes, let bindings, function arguments, list elements, imports, and `callPackage` invocations), and identify dependencies.

### Usage

To analyze a Nix file, you can run the tool with the `--nix-file` argument:

```bash
cargo run -- --nix-file <path-to-nix-file>
```

Example:

```bash
cargo run -- --nix-file ../../flake.nix
```

The output will include metrics and identified dependencies from the parsed Nix file.
