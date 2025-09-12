# SOP: Launching Emacs with Nix-Managed Configuration in Tmux

## 1. Purpose
This Standard Operating Procedure (SOP) provides step-by-step instructions for launching an Emacs instance configured with the `terlar-emacs-config` Nix environment within a `tmux` session. This ensures that Emacs has access to all necessary Nix-provided packages and configurations, including Magit.

## 2. Scope
This SOP applies to all users working within this project who require a fully functional Emacs environment integrated with the project's Nix-managed dependencies.

## 3. Prerequisites
*   **Nix:** The Nix package manager must be installed and properly configured on your system.
*   **Tmux:** The `tmux` terminal multiplexer must be installed.
*   **Project Repository:** The project repository must be cloned to your local machine.
*   **Submodules:** The `terlar-emacs-config` submodule (located at `.emacs.d/terlar-emacs-config/`) must be initialized and up-to-date. If not, run `git submodule update --init --recursive` from the project root.

## 4. Procedure

### 4.1. Ensure Tmux Session
1.  Open your terminal.
2.  Verify you are inside a `tmux` session. You can check by looking for a `tmux` status bar at the bottom of your terminal, or by running `tmux ls` (if it returns an error or no sessions, you are not in a `tmux` session).
3.  If you are not in a `tmux` session, start a new one: `tmux new -s project-dev` (you can choose any session name).

### 4.2. Navigate to Project Root
1.  Change your current directory to the project root: 
    `cd /data/data/com.termux.nix/files/home/pick-up-nix/source/github/meta-introspector/submodules/`

### 4.3. Launch Emacs
1.  Execute the Emacs launch script:
    `./launch_nix_emacs.sh`

### 4.4. Verify Emacs Launch
1.  Observe your `tmux` window. A new pane should appear, and Emacs should launch within it.
2.  Confirm that Emacs starts without any critical errors or warnings in the Emacs frame or the terminal output.

### 4.5. Verify Functionality (Post-Launch)
1.  **Magit:** Inside the newly launched Emacs instance, press `M-x` (Alt-x or Esc-x), type `magit-status`, and press Enter. A Magit status buffer should open, reflecting the Git status of your current project.
2.  **Other Packages:** Test other expected functionalities, such as completion (Corfu/Vertico), by typing in a buffer or using minibuffer commands.

## 5. Troubleshooting
*   **`nix develop` errors:** If Emacs fails to launch with Nix-related errors, ensure your Nix installation is healthy (`nix-shell --version`, `nix develop --help`). Also, check the `flake.nix` and `shell.nix` files in `.emacs.d/terlar-emacs-config/` for any syntax errors or outdated dependencies.
*   **`tmux` session not found:** If the `launch_nix_emacs.sh` script reports that the `tmux` session was not found, ensure you are running the script from within an active `tmux` session and that the `SESSION_NAME` variable in the script matches your `tmux` session name (default is `gemini-dev-session`).
*   **Packages not loading:** If Emacs launches but packages like Magit are not available, ensure that `shell.nix` in the project root and `flake.nix` in `.emacs.d/terlar-emacs-config/` correctly list the packages, and that `init.el` does not have conflicting hardcoded paths (which should have been removed by previous steps).
