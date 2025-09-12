# Emacs + Nix for the Noob: A Journey to a Perfect Dev Environment

## Introduction
If you're an Emacs enthusiast working in a Nix-powered development environment, you might have faced the challenge of getting your beloved editor to play nicely with Nix-managed packages. This post chronicles our journey to achieve a seamless Emacs setup, ensuring all your favorite packages, like Magit, load perfectly from Nix.

## The Problem: A Tale of Two `init.el`s and Hardcoded Paths
Our initial setup was a bit of a mess. We had a main project with its own `shell.nix`, and within it, an Emacs configuration submodule (`terlar-emacs-config`) that also had its own Nix flake (`flake.nix`).

When launching Emacs, we encountered several issues:
1.  **Package Loading Woes:** Packages like Magit weren't reliably loading. We'd get errors or find them missing, even though they were supposedly provided by Nix.
2.  **Hardcoded Paths:** Our main `init.el` was littered with hardcoded `/nix/store/...` paths for Emacs packages. This was a nightmare! Every time a package updated, these paths would change, breaking our configuration.
3.  **Submodule Confusion:** The Emacs instance wasn't correctly picking up the configuration from our `terlar-emacs-config` submodule, which was designed to be the authoritative source.

## The Solution: Embracing Nix's Power
Our journey involved several key steps to untangle this mess and leverage Nix's strengths.

### Step 1: Directing Emacs to the Right Nix Environment
Initially, we were launching Emacs using the main project's `shell.nix`. The first breakthrough was realizing Emacs needed to be launched directly from within the `terlar-emacs-config`'s Nix environment.

We modified our `launch_nix_emacs.sh` script to first `cd` into the `terlar-emacs-config` directory and then use `nix run .#` to execute the Emacs instance defined by that flake. This ensures Emacs starts with the precise environment and configuration intended by `terlar-emacs-config`.

```bash
# Inside launch_nix_emacs.sh
cd .emacs.d/terlar-emacs-config && nix run .# -- --batch -l "$EMACS_LISP_TEST_FILE"
```

### Step 2: Dynamic `load-path` with `EMACSLOADPATH`
The hardcoded `/nix/store/...` paths were a major pain point. Nix provides a much cleaner solution: the `EMACSLOADPATH` environment variable.

We updated our main `shell.nix` to include a `shellHook` that dynamically sets `EMACSLOADPATH`. This variable tells Emacs where to find its Lisp files. By constructing this path from our Nix-provided Emacs packages, Emacs automatically adds them to its `load-path`.

```nix
# Inside shell.nix
pkgs.mkShell {
  buildInputs = [ ... pkgs.emacs pkgs.emacsPackages.magit ];
  shellHook = ''
    export EMACSLOADPATH="${pkgs.lib.makeSearchPathOutput "share/emacs/site-lisp" [
      pkgs.emacsPackages.magit
      # Add other emacsPackages here
    ]}"
  '';
}
```

With `EMACSLOADPATH` in place, we could happily remove all the brittle hardcoded paths from our `init.el`.

### Step 3: Ensuring Cross-Platform Compatibility
We discovered that our `terlar-emacs-config`'s `flake.nix` was only configured for `x86_64-linux`. To ensure it worked on our `aarch64-linux` system, we simply added `aarch64-linux` to the `systems` list in its `flake.nix`.

```nix
# Inside terlar-emacs-config/flake.nix
systems = [ "x86_64-linux" "aarch64-linux" ];
```

### Step 4: Automated Verification with Headless Tests
To ensure our Emacs setup remains robust, we introduced a headless batch test. This test runs Emacs in batch mode, loads our configuration, and programmatically verifies that key packages like Magit and `use-package` are correctly loaded and functional.

This allows for quick, automated checks, especially useful in CI/CD pipelines or before committing changes.

## The Outcome: A Harmonious Emacs Environment
After these changes, our Emacs environment is now a joy to work with:
*   **Reliable Package Loading:** Magit and all other Nix-provided packages load consistently and correctly.
*   **Clean Configuration:** No more hardcoded paths! Our `init.el` is cleaner and more maintainable.
*   **Automated Confidence:** The headless test provides immediate feedback on the health of our Emacs setup.

This journey highlights the power of Nix in creating reproducible and robust development environments, even for complex applications like Emacs. If you're struggling with your Emacs and Nix setup, we hope our experience provides a helpful starting point!
