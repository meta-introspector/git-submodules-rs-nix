# SOP: Generating the CRQ Report

## Purpose

This Standard Operating Procedure (SOP) outlines the steps for generating a summary report of all Change Request/Question (CRQ) documents, grouped by their suggested next step. This report provides an overview of the project's current focus areas based on the automated classification of CRQs.

## Prerequisites

*   The `crq_table_generator` Rust project must be built in release mode. Ensure the executable is available at `./target/release/crq_table_generator` relative to the project root.
*   The `generate_crq_summary.sh` script must be available in the `tools/` directory and have execute permissions.
*   CRQ documents must be located in the `docs/crq_standardized/` directory and follow the standard CRQ format.

## Procedure

1.  **Navigate to the Project Root:**
    Open your terminal and navigate to the root directory of the `meta-introspector/submodules` project.

    ```bash
    cd /path/to/meta-introspector/submodules
    ```

2.  **Run the Report Generation Script:**
    Execute the `generate_crq_summary.sh` script.

    ```bash
    ./tools/generate_crq_summary.sh
    ```

3.  **Review the Output:**
    The script will output a table-like summary, listing CRQs grouped by their suggested next step (Develop/Implement, Refactor, Document, or Unknown).

    **Example Output:**
    ```
    Scanning CRQ directory: docs/crq_standardized/

    --- CRQs Grouped by Suggested Next Step ---

    Develop/Implement:
      - CRQ: Deterministic CRQ Parser for Project Next Steps
      - CRQ-025-rust-code-generation-for-lattice-structures-programmatic-construction-of-the-framework
      - ...

    Refactor:
      - CRQ-023: Refactoring Planning and Automation
      - ...

    Document:
      - CRQ-010-sop-documentation-and-cargo-lock-update
      - ...

    Unknown:
      - CRQ-008-the-crq-of-crqs
      - ...
    ```

## Troubleshooting

*   **"Error: crq_table_generator executable not found:"**: Ensure you have built the `crq_table_generator` Rust project in release mode. Navigate to the project root and run `cargo build --release`.
*   **"Error reading CRQ directory..."**: Ensure the `docs/crq_standardized/` directory exists and contains readable CRQ files.

## Related Documents

*   [SOP_Using_CRQ_Classifier.md](SOP_Using_CRQ_Classifier.md)
*   [SOP_Creating_New_CRQ.md](SOP_Creating_New_CRQ.md) (To be created)