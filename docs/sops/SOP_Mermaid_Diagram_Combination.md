# SOP: Mermaid Diagram Combination Procedure

## Objective
To provide a standardized procedure for combining Mermaid diagrams using the `mermaid_combiner` Rust tool.

## Prerequisites
*   Rust programming language and Cargo build system installed.
*   The `mermaid_combiner` tool is built and available (e.g., `cargo build --bin mermaid_combiner`).

## Procedure
1.  **Navigate to the project root:** Ensure your current working directory is the root of the `meta-introspector` repository.
2.  **Run the Mermaid Combiner tool:** Execute the following command in your terminal:
    ```bash
    cargo run --bin mermaid_combiner > docs/memes/combined_diagram.md
    ```
    *   This command will:
        *   Scan all markdown files in `docs/memes/` for Mermaid `graph TD` diagrams.
        *   Extract and combine all found diagrams.
        *   Redirect the output to `docs/memes/combined_diagram.md`.
3.  **Review the combined diagram:** Open `docs/memes/combined_diagram.md` in a Markdown viewer that supports Mermaid rendering (e.g., VS Code with Markdown Preview Enhanced, GitHub).
4.  **Commit and Push (if applicable):** If the combined diagram is satisfactory and represents a desired state, commit the `combined_diagram.md` file to your version control system.
