# Memo: CRQs as Commit Messages

## Date: September 12, 2025

## Subject: Standardizing Commit Messages with Change Request Quality (CRQ) Documents

This memo serves to formalize the practice of utilizing Change Request Quality (CRQ) documents directly as commit messages. This approach aims to enhance traceability, provide comprehensive context for each change, and ensure consistency across our version control history.

### Rationale:

*   **Enhanced Traceability:** Linking commits directly to detailed CRQ documents provides a clear audit trail for every modification.
*   **Improved Context:** CRQs contain objectives, descriptions, justifications, and expected outcomes, offering a richer understanding of why a change was made, beyond what a typical concise commit message might convey.
*   **Consistency:** Standardizing commit messages to reflect CRQ content promotes a uniform and predictable commit history.
*   **Documentation Integration:** This practice seamlessly integrates our change management process with our version control system, making the commit history a living documentation of project evolution.

### Implementation:

Developers are encouraged to draft their CRQ documents with the intention of using them as the primary content for their commit messages. The `-F` flag with `git commit` should be utilized, pointing to the CRQ markdown file.

Further guidelines on CRQ creation and integration with the commit process can be found in `docs/crq_standardized/CRQ-008-the-crq-of-crqs.md` and `SOP_CRQ_as_Commit_Message.md`.
