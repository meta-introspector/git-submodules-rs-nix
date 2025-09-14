# Terlar Emacs Configuration (Nix-managed): A N00b Guide

Welcome to Terje's Emacs configuration! This guide will help you get started with this powerful and highly customized Emacs setup, which is managed using Nix.

## What is this Emacs Configuration?

This is a personal Emacs configuration designed for efficiency and a consistent user experience. It's built as a Nix package, meaning all Emacs packages and dependencies are managed by Nix, ensuring a reproducible and isolated environment. This is great for stability, but it means you can't just install packages the usual Emacs way.

## Prerequisites

Before you begin, you'll need:

1.  **Nix**: The Nix package manager must be installed on your system. If you don't have it, follow the official Nix installation guide: [https://nixos.org/download.html](https://nixos.org/download.html)
2.  **Git**: To clone this repository.

## Getting Started: Setting Up Your Emacs Environment

Since this configuration is Nix-managed, you'll typically enter a development shell that provides the correct Emacs environment.

1.  **Navigate to the Configuration Directory**:
    First, ensure you are in the root directory of the `terlar-emacs-config` submodule. If you're in the `meta-introspector` project, it's likely located at `.emacs.d/terlar-emacs-config/`.
    ```bash
    cd /data/data/com.termux.nix/files/home/pick-up-nix/source/github/meta-introspector/submodules/.emacs.d/terlar-emacs-config/
    ```

2.  **Enter the Nix Development Shell**:
    This command will set up an environment where the correct Emacs version and all its packages (including this configuration) are available.
    ```bash
    nix develop
    ```
    You'll know you're in the Nix shell when your terminal prompt changes (often prefixed with `[nix-shell:` or similar).

3.  **Launch Emacs**:
    Once inside the `nix develop` shell, you can simply launch Emacs.
    ```bash
    emacs
    ```
    Emacs should now start with Terje's configuration loaded.

## Your First Steps in Terlar Emacs

This configuration is designed to be keyboard-driven and efficient. Here are some initial pointers:

*   **Minimalist UI**: You'll notice there's no traditional menu bar or tool bar. This is intentional to maximize screen space and encourage keyboard shortcuts.

*   **The Leader Key (`C-,`)**: This is your primary gateway to many custom commands. Press `Control` and `,` (comma) together. A small popup will appear showing you available commands. For example, `C-, n` might open a navigation menu.

*   **Local Leader Key (`C-.`)**: Similar to the leader key, but often used for commands specific to the current major mode (e.g., a programming language mode).

*   **`M-x` (Meta-x)**: This is the standard Emacs way to run any command by name. Press `Alt` and `x` (or `Esc` then `x`). Start typing a command name (e.g., `find-file`) and use `Tab` for completion.

*   **Completion with `Corfu` and `Vertico`**: When you type in the minibuffer (the area at the bottom of Emacs where you enter commands or search terms), you'll see powerful completion suggestions powered by `Corfu` and `Vertico`. Use `Tab` to cycle through suggestions.

*   **Icons Everywhere (`all-the-icons`)**: You'll see various icons in your mode line, Dired buffers, and completion candidates. These are provided by `all-the-icons` to make the UI more visually informative.

*   **Themes**: This configuration includes `readable-typo-theme` and `readable-mono-theme`. You can toggle between light and dark themes using the leader key (e.g., `C-, *`).

## Troubleshooting Common Issues

If you encounter problems, especially with packages not loading, here are some resources:

*   **Magit Not Loading?**: Refer to `docs/sops/SOP_Debugging_Magit_Nix_Emacs.md` for a detailed Standard Operating Procedure on diagnosing and fixing issues with Magit in this Nix-built environment.

*   **Debugging Scripts**: The `tools/debug-emacs/` directory contains scripts (`debug-magit-step1-build-log.sh`, etc.) that can help you gather information about your Emacs build and environment. These are referenced in the SOP.

## Further Exploration

This guide only scratches the surface. For a deep dive into all the features and customizations, I highly recommend reading the main `README.org` file located in the `terlar-emacs-config` directory. It contains extensive details on every aspect of this configuration.

Happy Emacs-ing!