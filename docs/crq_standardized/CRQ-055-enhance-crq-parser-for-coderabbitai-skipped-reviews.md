**CRQ: Enhance CRQ Parser for CoderabbitAI Skipped Reviews**

**Problem/Goal:**
To enhance the `crq_parser` within the `crq_table_generator` project to automatically identify CRQs where CoderabbitAI has skipped an automated review, and classify them as requiring a manual review request to CoderabbitAI.

**Proposed Solution:**

1.  **New `NextStep` Variant:**
    *   Add `RequestCoderabbitAIReview` to the `NextStep` enum in `crq_parser.rs`.

2.  **`crq_parser.rs` Modifications:**

    *   **New Function: `check_coderabbitai_comms(crq_id: &str) -> bool`**
        *   This function will check communication logs for a given `crq_id`.
        *   It will construct the path to CoderabbitAI responses (e.g., `analysis_data/comms/git/coderabbitai/{crq_id}/responses/`).
        *   It will read file content and search for keywords/patterns indicating a skipped review (e.g., "Review skipped", "Auto reviews are disabled").
        *   It will return `true` if a skipped review message is found, `false` otherwise.

    *   **Modify `determine_next_step(crq_content: &str, crq_id: &str) -> NextStep`**
        *   The function will accept `crq_id` as an additional argument.
        *   The classification logic will be updated:
            1.  **CoderabbitAI Review (Size-based):** (Existing rule) If content size < 2500 bytes, classify as `CoderabbitAIReview`.
            2.  **Request CoderabbitAI Review (Comms-based):** (NEW RULE) If `check_coderabbitai_comms(crq_id)` returns `true`, classify as `RequestCoderabbitAIReview`. This rule takes precedence over general "Respond To / Our Turn".
            3.  **Respond To / Our Turn:** (Existing rule) If neither of above, and CRQ contains relevant keywords, classify as `RespondTo`.
            4.  **Develop/Refactor/Document/Unknown:** (Existing rules) Fallback if no other specific classification applies.

3.  **`main.rs` Modifications:**

    *   **Extract `crq_id`:** Extract CRQ ID from filename and pass to `determine_next_step`.
    *   **Update `match` statement:** Add a new case for `NextStep::RequestCoderabbitAIReview` for display.

**Justification/Impact:**
This enhancement will make the `crq_parser` more intelligent and context-aware by incorporating external communication context into its classification decisions. It will automate the identification of CRQs that require manual intervention to trigger a CoderabbitAI review, improving workflow efficiency and ensuring that skipped reviews are addressed proactively.

**Progress Update (2025-09-11):**

The core implementation of the proposed solution has been completed and integrated into the `crq_table_generator`. This includes:

*   Refined `NextStep` enum with new states: `ReviewProvided`, `ReviewSkipped (No Meaningful Response)`, `ReviewNeeded from CoderabbitAI`, and `Respond To / Our Turn`.
*   Implementation of `check_coderabbitai_comms` to analyze communication logs for skipped reviews and meaningful responses.
*   Updated `determine_next_step` with a prioritized state machine logic, incorporating n-gram analysis and communication log data.
*   The `crq_table_generator` now produces a detailed report reflecting these new classifications.

This phase of the CRQ is considered complete. Further work will be tracked under new CRQs, such as `CRQ-046-crq-classification-via-comms-analysis-and-state-machine.md` for more advanced comms analysis.
