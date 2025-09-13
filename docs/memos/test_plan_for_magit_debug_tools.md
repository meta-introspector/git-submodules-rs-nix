# Test Plan: Magit Debugging SOP and Tools

## Objective
To verify the clarity and effectiveness of the `SOP_Debugging_Magit_Nix_Emacs.md` and the functionality of the associated `debug-emacs` scripts (`debug-magit-step1-build-log.sh`, `debug-magit-step2-inspect-store.sh`, `debug-magit-step3-test-magit.sh`). The goal is to ensure these resources effectively aid in diagnosing issues with Magit loading in a Nix Emacs environment.

## Scope
This test plan covers the documentation (`SOP_Debugging_Magit_Nix_Emacs.md`) and the three shell scripts located in `tools/debug-emacs/`.

## Test Cases

### Test Case 1: SOP Clarity and Completeness Review
*   **Description**: Manually review `docs/sops/SOP_Debugging_Magit_Nix_Emacs.md` for:
    *   Clear problem statement and hypothesis.
    *   Logical flow of proposed solution steps.
    *   Completeness of information for each step.
    *   Readability and ease of understanding for someone familiar with Nix and Emacs.
*   **Expected Outcome**: The SOP should be well-structured, easy to follow, and provide sufficient detail to guide a user through the debugging process.

### Test Case 2: `debug-magit-step1-build-log.sh` Functionality
*   **Description**: Execute the script `tools/debug-emacs/debug-magit-step1-build-log.sh`.
*   **Pre-conditions**: None, other than a functional Nix environment.
*   **Expected Outcome**:
    *   The script should execute successfully (exit code 0).
    *   A file named `emacs-env-build.log` should be created in the `.emacs.d/terlar-emacs-config` directory.
    *   The `emacs-env-build.log` file should contain the output of the `nix build .#emacs-env` command, including any build messages, warnings, or errors.

### Test Case 3: `debug-magit-step2-inspect-store.sh` Functionality
*   **Description**: Execute the script `tools/debug-emacs/debug-magit-step2-inspect-store.sh`.
*   **Pre-conditions**: Test Case 2 must have been executed successfully, resulting in a `result` symlink for `emacs-env` in `.emacs.d/terlar-emacs-config`.
*   **Expected Outcome**:
    *   The script should execute successfully (exit code 0).
    *   It should correctly identify and print the Nix store path for `emacs-env`.
    *   A file named `emacs-env-store-contents.log` should be created in the `.emacs.d/terlar-emacs-config` directory.
    *   The `emacs-env-store-contents.log` file should contain a recursive listing of the contents of the identified Nix store path.
    *   **Note**: The redundant `#!/usr/bin/env bash` and `set -x` in the middle of the script should be noted as a minor code quality issue, but should not prevent successful execution.

### Test Case 4: `debug-magit-step3-test-magit.sh` Functionality
*   **Description**: Execute the script `tools/debug-emacs/debug-magit-step3-test-magit.sh`.
*   **Pre-conditions**: Test Cases 2 and 3 must have been executed successfully.
*   **Expected Outcome**:
    *   The script should execute successfully (exit code 0, assuming Magit loads, or a non-zero exit code if Magit fails to load, which is the scenario it's designed to test).
    *   It should attempt to run the existing Emacs batch test script (if `run_emacs_batch_test.sh` exists and is executable).
    *   It should create a temporary Emacs Lisp file, execute it within a `nix develop` environment, and report whether Magit loaded successfully and if the `magit-status` function is available.
    *   The temporary Emacs Lisp file created for testing should be cleaned up after execution.

### Test Case 5: SOP Effectiveness (Diagnostic Walkthrough)
*   **Description**: Simulate a scenario where Magit is not loading in Nix Emacs. Mentally (or actually, if feasible) follow the steps outlined in `SOP_Debugging_Magit_Nix_Emacs.md` and use the `debug-emacs` scripts as directed.
*   **Expected Outcome**: The combination of the SOP and the scripts should provide clear, actionable information that helps pinpoint the root cause of Magit not loading (e.g., build failure, missing files in store, Emacs load path issues).

## Conclusion
Successful execution and positive outcomes for all test cases will confirm that the `SOP_Debugging_Magit_Nix_Emacs.md` and its accompanying `debug-emacs` tools are fit for purpose in debugging Magit loading issues within the Nix Emacs environment. This test plan will be used to validate the effectiveness of these pre-existing debugging resources.