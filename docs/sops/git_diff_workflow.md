# SOP: Git Diff Workflow

## Purpose
This Standard Operating Procedure (SOP) outlines the best practices for utilizing `git diff` within the `meta-introspector` project to ensure effective code review, change understanding, and adherence to Extreme Programming (XP) principles.

## Core Principles (XP Integration)
*   **Collective Ownership**: Everyone is responsible for the code. `git diff` facilitates understanding changes made by others.
*   **Continuous Integration**: Frequent, small commits are encouraged. `git diff` is crucial for reviewing these small, incremental changes.
*   **Small Releases**: Changes are released frequently. `git diff` helps in verifying the scope and impact of each release.

## Common Git Diff Commands and Their Use Cases

### 1. `git diff`
*   **Description**: Shows changes in the working directory that are not yet staged for commit. This is useful for reviewing your local modifications before deciding what to stage.
*   **Usage**: `git diff`
*   **XP Relevance**: Helps in self-reviewing small changes before staging, aligning with the "small releases" and "continuous integration" principles.

### 2. `git diff --staged` (or `git diff --cached`)
*   **Description**: Shows changes between the staging area and the last commit. This is essential for reviewing what will actually be included in your next commit.
*   **Usage**: `git diff --staged`
*   **XP Relevance**: Critical for ensuring that only intended changes are committed, supporting "small releases" and "continuous integration".

### 3. `git diff <commit1> <commit2>`
*   **Description**: Compares the differences between two specific commits. This is useful for understanding the evolution of a feature or bug fix over time.
*   **Usage**: `git diff HEAD~1 HEAD` (compares last two commits) or `git diff <commit_hash_1> <commit_hash_2>`
*   **XP Relevance**: Aids in understanding the history of changes, supporting "collective ownership" and knowledge sharing.

### 4. `git diff <branch1> <branch2>`
*   **Description**: Compares the tips of two different branches. This is commonly used before merging branches to see what changes will be introduced.
*   **Usage**: `git diff main feature/my-new-feature`
*   **XP Relevance**: Essential for pre-merge reviews, ensuring that new features integrate smoothly and align with "continuous integration".

### 5. `git diff <file_path>`
*   **Description**: Shows changes for a specific file in the working directory that are not yet staged.
*   **Usage**: `git diff src/main.rs`
*   **XP Relevance**: Allows focused review of changes to a single file, useful during pair programming or when addressing specific feedback.

## Interpreting Git Diff Output

`git diff` output typically uses a "unified diff" format:

*   Lines starting with `--- a/<file>` and `+++ b/<file>` indicate the original and new files, respectively.
*   Lines starting with `@@ -<start_line>,<num_lines> +<start_line>,<num_lines> @@` are "hunk headers" indicating the line numbers and number of lines affected in both files.
*   Lines starting with `-` are deletions from the original file.
*   Lines starting with `+` are additions to the new file.
*   Lines starting with a space are context lines (unchanged).

## Best Practices for Code Review with Git Diff

*   **Review Small Diffs Frequently**: In line with XP, review changes as they are made, rather than waiting for large pull requests.
*   **Focus on Intent**: Understand *why* the change was made, not just *what* was changed.
*   **Look for Side Effects**: Consider if the changes introduce unintended consequences.
*   **Check for Consistency**: Ensure new code adheres to existing coding standards and patterns.
*   **Provide Constructive Feedback**: Use `git diff` as a tool for discussion and improvement.
