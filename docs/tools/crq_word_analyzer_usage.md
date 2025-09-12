## `crq_word_analyzer` - Bag of Words Extraction Helper

### Purpose
This document details the functionality of the `crq_word_analyzer` tool to enable the extraction of a "bag of words" (tokens and n-grams) from a specified input file. This functionality is useful for analyzing text content, such as CoderabbitAI communication messages, to identify keywords and patterns.

### Functionality
1.  **`tools/crq_word_analyzer/src/main.rs`**: 
    *   The tool now accepts a command-line argument `--file <PATH>` (or `-f <PATH>`), allowing a single file to be specified for analysis.
    *   When the `--file` argument is provided, the tool reads the content of the specified file, extracts tokens (single words and n-grams) using the existing `extract_tokens` function, and prints them to standard output.

### How to Use
To use this functionality, navigate to the project root directory and run the `crq_word_analyzer` executable with the `--file` argument, followed by the absolute or relative path to the file you wish to analyze.

**Example:**
```bash
cargo build -p crq_word_analyzer
target/debug/crq_word_analyzer --file analysis_data/comms/git/coderabbitai/CRQ-53/responses/001_coderabbitai.md
```
