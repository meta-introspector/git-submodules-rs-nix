# SOP: PR Review for Noobs

## 1. Objective

This SOP provides a simplified workflow for reviewing Pull Requests (PRs) for new contributors to the project. It introduces a script that automates the process of gathering the necessary information for a review.

## 2. The `review_pr.sh` Script

The `review_pr.sh` script is located in the `tools/` directory. It takes a PR number as an argument and provides a comprehensive overview of the PR, including its details, file changes (diff), and comments.

### 2.1. Usage

To use the script, run the following command from the root of the project:

```bash
./tools/review_pr.sh <pr_number>
```

Replace `<pr_number>` with the number of the PR you want to review.

### 2.2. Output

The script will output the following information:

*   **PR Details:** The title, author, labels, and description of the PR.
*   **PR Diff:** The changes made to the files in the PR.
*   **PR Comments:** All the comments made on the PR.

## 3. How to Review a PR

1.  **Get the list of open PRs:**
    ```bash
    gh pr list
    ```
2.  **Choose a PR to review.**
3.  **Run the `review_pr.sh` script with the PR number:**
    ```bash
    ./tools/review_pr.sh <pr_number>
    ```
4.  **Analyze the output:**
    *   **Read the PR description:** Understand the purpose of the PR and the changes it introduces.
    *   **Review the diff:** Look at the code changes. Are they correct? Do they follow the project's coding standards?
    *   **Read the comments:** See what other reviewers have said about the PR.
5.  **Provide your feedback:**
    *   If you have any questions or suggestions, add a comment to the PR.
    *   If you approve of the changes, you can approve the PR on GitHub.

## 4. What to Look For

*   **Clarity:** Is the code clear and easy to understand?
*   **Correctness:** Does the code do what it is supposed to do?
*   **Consistency:** Does the code follow the project's style and conventions?
*   **Completeness:** Are there any missing pieces?
*   **Documentation:** Is the code well-documented? Are the comments clear and helpful?
