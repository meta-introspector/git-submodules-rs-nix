# Meta-Introspector Submodules Project

This project focuses on building tools and infrastructure for introspecting and managing Git submodules within the Meta-Introspector ecosystem.

## Key Features

*   **Comprehensive Submodule Collection:** Utilizes the `submodule-collector` Rust program to recursively scan and gather information about Git repositories and their submodules.
*   **Detailed Error Reporting:** The `submodule-collector` is designed to provide insights into issues encountered during submodule processing. Errors for unopenable repositories are now captured and included in the generated JSON report.
*   **Submodule Reporting:** The `report-analyzer-rs` tool analyzes the collected submodule data, offering statistics on successful vs. failed repositories, identifying duplicate entries, and providing frequency analysis of organizations, repository/submodule names, and common bigrams/trigrams. It also identifies the longest common prefix (LCP) among all paths and URLs for context.

## Getting Started

### Prerequisites

*   Nix package manager
*   Rust toolchain (managed by Nix)

### Building the `submodule-collector`

To build the `submodule-collector` executable:

```bash
cargo build --release -p submodule-collector
```

The executable will be available at `target/release/submodule-collector`.

### Generating a Submodule Report

To generate a comprehensive JSON report of all Git repositories and their submodules within the main project scope:

```bash
./target/release/submodule-collector --root-dir /data/data/com.termux.nix/files/home/pick-up-nix/source/github/meta-introspector/ --output-file submodule_report_recursive_resilient.json
```

*(Note: The `--root-dir` should point to the root of your meta-introspector project.)*

### Building the `report-analyzer-rs`

To build the `report-analyzer-rs` executable:

```bash
cargo build --release -p report-analyzer-rs
```

The executable will be available at `target/release/report-analyzer-rs`.

### Analyzing the Submodule Report

To analyze the generated JSON report:

```bash
./target/release/report-analyzer-rs --report-path submodule_report_recursive_resilient.json
```

## Documentation

*   **Development Plan:** See `task.md` for the current development roadmap.
*   **CRQs (Change Request Quality):** Refer to the `docs/crq/` directory for detailed change requests, including `CRQ-002-submodule-report-function.md` for the submodule reporting feature.
*   **SOPs (Standard Operating Procedures):** Find debugging guides and other operational procedures in the `docs/sops/` directory, including `SOP_Coding_Standards.md`.
