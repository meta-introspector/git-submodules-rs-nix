# SOP: Using the CRQ Classifier

## Purpose

This Standard Operating Procedure (SOP) outlines the steps for using the `classify_crq.sh` script to determine the suggested next step for a Change Request/Question (CRQ) document. This tool leverages the deterministic CRQ parser to provide automated guidance.

## Prerequisites

*   The `crq-parser` Rust project must be built. Ensure the executable is available at `./target/debug/crq-parser` (or `./target/release/crq-parser` if built in release mode) relative to the project root.
*   The `classify_crq.sh` script must be available in the `tools/` directory and have execute permissions.

## Procedure

1.  **Navigate to the Project Root:**
    Open your terminal and navigate to the root directory of the `meta-introspector/submodules` project.

    ```bash
    cd /path/to/meta-introspector/submodules
    ```

2.  **Run the Classifier Script:**
    Execute the `classify_crq.sh` script, providing the path to the CRQ file you wish to classify.

    ```bash
    ./tools/classify_crq.sh <path_to_your_crq_file.md>
    ```

    **Example:**
    To classify a CRQ located at `docs/crq_standardized/CRQ-054-deterministic-crq-parser.md`:

    ```bash
    ./tools/classify_crq.sh docs/crq_standardized/CRQ-054-deterministic-crq-parser.md
    ```

3.  **Review the Output:**
    The script will output the parsed CRQ content and the suggested next step.

    **Example Output:**
    ```
    Reading CRQ from file: docs/crq_standardized/CRQ-054-deterministic-crq-parser.md

    -- Parsed CRQ --
    Problem/Goal: To establish a clear and automated mechanism for identifying and prioritizing the next steps in the project development lifecycle.

    Proposed Solution: Develop a deterministic CRQ parser capable of analyzing change requests/questions to programmatically determine the subsequent actions required for project progression.

    Justification/Impact: This parser will enable more efficient and consistent project management by providing a structured approach to decision-making, akin to an "AI shepherd" guiding the project towards optimal outcomes. It will reduce ambiguity and streamline the process of identifying critical path items.

    -- Suggested Next Step --
    Action: Develop/Implement new features.
    ```

## Troubleshooting

*   **"Error: CRQ file not found:"**: Double-check the path to your CRQ file. Ensure it is correct and the file exists.
*   **"Error: crq-parser executable not found:"**: Ensure you have built the `crq-parser` Rust project. Navigate to the project root and run `cargo build -p crq-parser`. If you built in release mode, you might need to change the `CRQ_PARSER_PATH` in `classify_crq.sh` to `./target/release/crq-parser`.
*   **"Error reading file..."**: The CRQ file might be corrupted or have incorrect permissions.

## Related Documents

*   [SOP_Generating_CRQ_Report.md](SOP_Generating_CRQ_Report.md) (To be created)
*   [SOP_Creating_New_CRQ.md](SOP_Creating_New_CRQ.md) (To be created)