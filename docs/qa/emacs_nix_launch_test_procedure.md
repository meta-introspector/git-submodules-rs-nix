# Emacs Nix Launch Test Procedure

## Objective
Verify that Emacs launches correctly within the `terlar-emacs-config` Nix environment, and that core packages like Magit are functional.

## Prerequisites
*   Nix is installed and configured.
*   `tmux` is installed and running.
*   The project repository is cloned.
*   The `terlar-emacs-config` submodule is initialized and up-to-date.

## Test Steps

1.  **Ensure Tmux Session:**
    *   Open a terminal and ensure you are within a `tmux` session. If not, start one (e.g., `tmux new -s my-session`).
    *   Navigate to the project root directory: `/data/data/com.termux.nix/files/home/pick-up-nix/source/github/meta-introspector/submodules/`.

2.  **Execute Launch Script:**
    *   Run the Emacs launch script: `./test_emacs_launch.sh`

3.  **Verify Tmux Pane:**
    *   Observe your `tmux` window. A new pane should appear, attempting to launch Emacs.

4.  **Verify Emacs Launch:**
    *   Confirm that Emacs successfully launches in the new pane. There should be no immediate errors or warnings displayed in the Emacs frame or the terminal.

5.  **Verify Magit Functionality:**
    *   Inside the newly launched Emacs instance, press `M-x` (Alt-x or Esc-x).
    *   Type `magit-status` and press Enter.
    *   **Expected:** A Magit status buffer should open, displaying the Git status of the current repository. If the current directory is not a Git repository, it should prompt you to select one or indicate that it's not a Git repository.
    *   Perform a simple Magit operation, e.g., press `s` to stage a file (if there are unstaged changes).

6.  **Verify Other Core Packages (Optional but Recommended):**
    *   Verify `use-package` is working: `M-x describe-function use-package`. It should show documentation.
    *   Verify `corfu` (completion framework): When typing in a buffer, completion suggestions should appear.
    *   Verify `vertico` (minibuffer completion): When using commands that require minibuffer input (e.g., `M-x find-file`), the completion interface should be vertical.

## Expected Outcome
All test steps are completed successfully, and Emacs, Magit, and other core packages function as described.
