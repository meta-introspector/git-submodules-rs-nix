# CRQ-061: Recursive Tuple Branch for Concurrent Branch Management

## Problem
Managing multiple feature branches concurrently within a single repository can become cumbersome, leading to frequent context switching, merge conflicts, and difficulty in visualizing the overall state of parallel development efforts. Creating separate repositories for every feature leads to repository sprawl and fragmented project views.

## Solution
Implement a "recursive tuple branch" strategy where the main repository contains submodules that point back to itself, but to different branches. This allows developers to check out a "tuple branch" and have multiple feature branches (or even `main`) available as submodules within a single working directory, facilitating concurrent work and holistic project views.

## Motivation:
*   **Avoid Repository Sprawl:** Consolidate related development efforts within a single repository structure, rather than creating numerous independent repositories.
*   **Concurrent Branch Work:** Enable developers to work on and test multiple feature branches simultaneously without constantly switching branches or managing separate clones.
*   **Holistic View:** Provide a unified view of the project's state across different development lines.

## Proposed Implementation (Conceptual):
*   A dedicated directory (e.g., `self/branches/`) within the main repository will house the submodules.
*   Each submodule within `self/branches/` will point to the *same* parent repository, but to a *specific branch* (e.g., `feature_X`, `feature_Y`, `main`).
*   When a developer checks out a "tuple branch," they will initialize and update these submodules, bringing the content of the specified branches into their working directory.

## Considerations & Challenges (Acknowledged):
*   **Complexity:** This is an advanced Git pattern that introduces significant complexity in setup, maintenance, and daily operations.
*   **Self-Referential Nature:** Managing a repository as a submodule of itself can lead to intricate dependency graphs and potential for infinite recursion if not handled with extreme care.
*   **Tooling Compatibility:** Standard Git tools, IDEs, and CI/CD pipelines may not fully support or be optimized for such a self-referential structure, potentially requiring custom scripting or workarounds.
*   **Merge/Rebase Operations:** Operations like merging or rebasing the parent repository can become significantly more complex when submodules point back to the parent's history.
*   **Performance:** A large number of submodules could impact Git operation performance.

## Future Scope (Wikidata Tool):
The `wikidata` tool, intended to be its own binary, can be integrated into this structure as a distinct submodule or managed as a separate project, depending on its specific requirements and the evolution of this "recursive tuple branch" strategy.
