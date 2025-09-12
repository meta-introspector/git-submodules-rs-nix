## SOP: Updating Hardcoded Emacs Package Paths in `init.el`

**SOP ID:** [Auto-generated or manually assigned ID]
**Date:** 2025-09-12
**Author:** Gemini

### 1. Purpose

This Standard Operating Procedure (SOP) outlines the steps required to update the hardcoded Nix store paths for Emacs packages in `~/.emacs.d/init.el`. These paths may change when Nixpkgs is updated, leading to Emacs being unable to find or load certain packages.

### 2. Scope

This SOP applies to developers working on projects that utilize Nix-provided Emacs packages with their paths hardcoded in `~/.emacs.d/init.el`.

### 3. Prerequisites

*   A working Nix environment.
*   Access to the project repository.
*   Basic familiarity with Emacs Lisp and Nix expressions.

### 4. Procedure

1.  **Identify Outdated Paths:**
    *   If Emacs reports errors about missing packages or packages not loading correctly, it's likely that their Nix store paths have changed.
    *   You can also proactively check for updates by running `nix flake update` (if using flakes) or updating your `nixpkgs` channel.

2.  **Retrieve New Nix Store Paths:**
    For each Emacs package whose path needs to be updated, use the `nix eval` command to retrieve its current Nix store path. Replace `[package-name]` with the actual package name (e.g., `magit`, `rustic`).

    ```bash
nix eval --raw --impure --expr 'with import <nixpkgs> {}; pkgs.emacsPackages.[package-name] + "/share/emacs/site-lisp"'
    ```

    *Example for `magit`*:
    ```bash
nix eval --raw --impure --expr 'with import <nixpkgs> {}; pkgs.emacsPackages.magit + "/share/emacs/site-lisp"'
    ```

    This command will output the current Nix store path for the specified package.

3.  **Update `~/.emacs.d/init.el`:**
    *   Open `~/.emacs.d/init.el` in your preferred editor.
    *   Locate the `(add-to-list 'load-path ...)` lines for the packages you are updating.
    *   Replace the old Nix store path with the new path obtained in Step 2.

    *Example update for `magit`*:
    Change from:
    ```elisp
(add-to-list 'load-path "/nix/store/OLD_MAGIT_PATH/share/emacs/site-lisp")
    ```
    To:
    ```elisp
(add-to-list 'load-path "/nix/store/NEW_MAGIT_PATH/share/emacs/site-lisp")
    ```

4.  **Verify Changes:**
    *   Restart Emacs.
    *   Verify that all Emacs packages load correctly and are functional.
    *   Check the `*Messages*` buffer in Emacs for any errors related to package loading.

### 5. Related Documents

*   CRQ: Hardcoding Emacs Package Paths in `init.el` ([Link to CRQ document])

---