**SOP: Testing `terlar/emacs-config` Submodule Integration**

**Objective:** To verify the successful integration and functionality of the `terlar/emacs-config` Git submodule within the project's Nix development environment.

**Audience:** Developers new to the project or Emacs/Nix.

**Prerequisites:**
*   A working Nix installation.
*   Git installed and configured.
*   Basic familiarity with the command line.

**Procedure:**

**Step 1: Ensure Submodules are Initialized and Updated**

Before working with any submodules, ensure they are correctly initialized and updated. This command fetches the submodule's content and places it in the correct directory.

```bash
git submodule update --init --recursive
```

**Explanation for Noobs:** Think of submodules as separate mini-Git projects living inside our main project. This command makes sure those mini-projects are downloaded and ready to use.

**Step 2: Tangle `init.org` to `init.el`**

The `terlar/emacs-config` uses an Org-mode file (`init.org`) for its configuration. Emacs needs this to be converted into a standard Emacs Lisp file (`init.el`) before it can load it.

```bash
nix develop --command bash -c "emacs --batch -l org --eval '(org-babel-tangle-file "$(pwd)/.emacs.d/terlar-emacs-config/init.org")'"
```

**Explanation for Noobs:** Emacs can read special `.org` files to set itself up. This command tells Emacs to take that `.org` file and turn it into a regular `.el` file that Emacs can understand and use for its settings. We use `$(pwd)` to make sure Emacs knows exactly where to find the file.

**Step 3: Launch Emacs in the Nix Development Environment (Interactive)**

To interactively test the Emacs configuration, enter the Nix development shell and launch Emacs.

```bash
nix develop
```
Once inside the `nix develop` shell, run:
```bash
emacs -l .emacs.d/terlar-emacs-config/init.el
```

**Explanation for Noobs:**
*   `nix develop`: This command puts you into a special "development environment" set up by Nix. This environment has all the tools and libraries our project needs, including the correct version of Emacs and its dependencies.
*   `emacs -l .emacs.d/terlar-emacs-config/init.el`: This launches Emacs and tells it to load the configuration file we just created (`init.el`) from the `terlar-emacs-config` submodule.

**Step 4: Verify Loaded Packages (Interactive)**

Once Emacs is running, you can check the `*Messages*` buffer to see what packages were loaded and if there were any errors.

1.  Press `C-h e` (Ctrl-h then e) to open the `*Messages*` buffer.
2.  Review the messages for any errors or indications of loaded packages.

**Explanation for Noobs:** Inside Emacs, there's a special window called `*Messages*` that shows you everything Emacs has been doing, including loading files and any problems it encountered. This is where you can see if your configuration loaded correctly.

**Step 5: Launch Emacs for Logging Loaded Packages (Programmatic)**

To programmatically log the loaded packages, you can use a helper script. This is useful for automated testing or debugging.

**Helper Script: `test-emacs-config.sh`**

Create a file named `test-emacs-config.sh` in the project root with the following content:

```bash
#!/usr/bin/env bash

# Exit immediately if a command exits with a non-zero status.
set -e

PROJECT_ROOT="$(pwd)"
EMACS_CONFIG_PATH="${PROJECT_ROOT}/.emacs.d/terlar-emacs-config"
INIT_ORG_PATH="${EMACS_CONFIG_PATH}/init.org"
INIT_EL_PATH="${EMACS_CONFIG_PATH}/init.el"
LOG_FILE="${PROJECT_ROOT}/emacs_load_log.txt"

echo "--- Starting Emacs Configuration Test ---" > "$LOG_FILE"
echo "Project Root: $PROJECT_ROOT" >> "$LOG_FILE"
echo "Emacs Config Path: $EMACS_CONFIG_PATH" >> "$LOG_FILE"
echo "Init Org Path: $INIT_ORG_PATH" >> "$LOG_FILE"
echo "Init El Path: $INIT_EL_PATH" >> "$LOG_FILE"
echo "Log File: $LOG_FILE" >> "$LOG_FILE"
echo "" >> "$LOG_FILE"

# Step 1: Ensure submodules are updated
echo "Updating Git submodules..." >> "$LOG_FILE"
git submodule update --init --recursive >> "$LOG_FILE" 2>&1
echo "Submodules updated." >> "$LOG_FILE"
echo "" >> "$LOG_FILE"

# Step 2: Tangle init.org to init.el
echo "Tangling init.org to init.el..." >> "$LOG_FILE"
nix develop --command bash -c "emacs --batch -l org --eval '(org-babel-tangle-file \"$INIT_ORG_PATH\")'" >> "$LOG_FILE" 2>&1
echo "init.org tangled to init.el." >> "$LOG_FILE"
echo "" >> "$LOG_FILE"

# Step 3: Launch Emacs and log loaded features
echo "Launching Emacs in batch mode to log loaded features..." >> "$LOG_FILE"
nix develop --command bash -c "emacs --batch -l \"$INIT_EL_PATH\" --eval '(progn (message \"Emacs loaded init.el from submodule.\") (message \"Loaded features: %S\" feature-list))'" >> "$LOG_FILE" 2>&1
echo "Emacs launch complete. Check '$LOG_FILE' for output." >> "$LOG_FILE"

echo "--- Emacs Configuration Test Complete ---" >> "$LOG_FILE"
```

**Explanation for Noobs:**
*   This is a special script that automates the steps we just did.
*   It first makes sure all the mini-projects (submodules) are up-to-date.
*   Then, it converts the `.org` configuration file into an `.el` file.
*   Finally, it launches Emacs in a special "batch mode" (meaning it runs without showing a window) and tells it to print out all the features (packages and settings) that it successfully loaded.
*   All of this information is saved into a file called `emacs_load_log.txt` in the main project folder.

**How to Run the Helper Script:**

1.  Save the content above as `test-emacs-config.sh` in your project's root directory.
2.  Make the script executable:
    ```bash
    chmod +x test-emacs-config.sh
    ```
3.  Run the script:
    ```bash
    ./test-emacs-config.sh
    ```
4.  After running, check the `emacs_load_log.txt` file for the output.
