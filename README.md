# Meta-Introspector Submodules Project

This project focuses on building tools and infrastructure for introspecting and managing Git submodules within the Meta-Introspector ecosystem.

## Key Features

*   **Comprehensive Submodule Collection:** Utilizes the `submodule-collector` Rust program to recursively scan and gather information about Git repositories and their submodules.
*   **Detailed Error Reporting:** The `submodule-collector` is designed to provide insights into issues encountered during submodule processing, including uninitialized or problematic repositories.
*   **Submodule Reporting (Upcoming):** Development is underway to provide a robust reporting function that analyzes the collected submodule data, offering statistics on success/failure rates, duplicate entries, and organizational patterns.

## Getting Started

### Prerequisites

*   Nix package manager
*   Rust toolchain (managed by Nix)

### Building the `submodule-collector`

To build the `submodule-collector` executable:

```bash
nix build .#submodule-collector
```

The executable will be available via a `result` symlink in your current directory (e.g., `result/bin/submodule-collector`).

### Generating a Submodule Report

To generate a comprehensive JSON report of all Git repositories and their submodules within the main project scope:

```bash
/path/to/submodule-collector --root-dir ../../../ --output-file submodule_report_recursive_resilient.json
```

*(Note: Replace `/path/to/submodule-collector` with the actual path to the built executable. The `--root-dir ../../../` assumes you are running this command from the `meta-introspector/submodules` directory and want to scan the parent `pick-up-nix` project.)*

Errors encountered during the scan (e.g., uninitialized submodules) will be printed to `stderr`.

## Documentation

*   **Development Plan:** See `task.md` for the current development roadmap.
*   **CRQs (Change Request Quality):** Refer to the `docs/crq/` directory for detailed change requests, including `CRQ-002-submodule-report-function.md` for the submodule reporting feature.
*   **SOPs (Standard Operating Procedures):** Find debugging guides and other operational procedures in the `docs/sops/` directory, including `sops-debugging-submodule-counting.md`.

---