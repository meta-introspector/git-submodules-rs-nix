## 1. Introduction

This document outlines the high-level architecture and design for the Project Context Introspector, a tool aimed at providing a comprehensive understanding of the project's current state and historical context. Its primary goal is to analyze diverse data sources and present insights into project structure, relationships, and evolution.

## 2. Core Components

The Context Introspector will consist of the following core components:

### 2.1 Data Sources

The introspector will consume data from various project artifacts:

*   **File System**: Directory structure, file types, file contents (e.g., source code, documentation, configuration files).
*   **Git History**: Commit messages, authors, timestamps, file changes (diffs), branches, tags, merges.
*   **Configuration Files**:
    *   `.gitmodules`: Submodule definitions (paths, URLs, branches/commits).
    *   `flake.nix`, `flake.lock`: Nix package definitions, dependencies, build instructions.
    *   `Cargo.toml`, `Cargo.lock`: Rust project dependencies, features, metadata.
    *   `submod.toml`: `submod` tool configuration.
*   **Documentation**: SOPs, CRQs, design documents (for explicit relationships and processes).
*   **Conversation Logs (Optional/Future)**: Transcripts of interactions (like this one) to capture implicit context and decision-making.

### 2.2 Data Ingestion & Parsing Layer

This layer is responsible for reading raw data from the sources and parsing it into a structured, machine-readable format.

*   **File System Scanner**: Recursively scans the project directory, identifying files and their types.
*   **Git Parser**: Utilizes Git commands (e.g., `git log`, `git show`, `git ls-tree`) or Git library APIs (e.g., `gitoxide` if integrated) to extract history and repository metadata.
*   **Configuration Parsers**: Specific parsers for TOML (`Cargo.toml`, `submod.toml`), INI-like (`.git/config`, `.gitmodules`), and Nix expression language (`flake.nix`).
*   **Text/Markdown Parsers**: For documentation, to extract headings, links, and potentially structured metadata.

### 2.3 Data Model & Storage

A unified data model will represent the project's context. This model should be flexible enough to capture various entities and their relationships.

*   **Entities**:
    *   **Files/Directories**: Path, type, content hash, metadata (size, last modified).
    *   **Commits**: Hash, author, committer, message, timestamp, parent commits, changed files.
    *   **Repositories**: URL, local path, current revision.
    *   **Submodules**: Name, path, URL, tracked revision, parent repository.
    *   **Packages/Dependencies**: Name, version, source, features.
    *   **SOPs/CRQs**: Title, description, content, related entities.
*   **Relationships**:
    *   `contains` (directory contains file/directory)
    *   `modifies` (commit modifies file)
    *   `depends_on` (package depends on another package)
    *   `defines` (config file defines submodule/package)
    *   `documents` (SOP documents a process)
    *   `enables` (CRQ enables another CRQ)
*   **Storage**: A graph database (e.g., Neo4j, Dgraph) would be ideal for representing entities and relationships. Alternatively, a relational database or even structured JSON/YAML files could be used for simpler implementations.

### 2.4 Analysis & Insight Generation Layer

This layer processes the structured data to derive meaningful insights and identify patterns.

*   **Dependency Graph Builder**: Constructs a graph of inter-component dependencies.
*   **Change History Analyzer**: Tracks the evolution of files, modules, and configurations over time.
*   **Relationship Discoverer**: Identifies implicit connections based on naming conventions, file co-occurrence, or content analysis.
*   **Anomaly Detector (Optional/Future)**: Identifies deviations from expected patterns (e.g., unmanaged submodules, inconsistent configurations).

### 2.5 Presentation Layer

This layer provides various interfaces for users to explore and understand the project context.

*   **CLI Interface**: For querying specific information (e.g., "show dependencies of X", "list commits affecting Y").
*   **Web-based UI (Optional/Future)**: For interactive visualizations of the project graph, timelines, and dashboards.
*   **Report Generator**: Generates static reports (e.g., Markdown, PDF) summarizing key aspects of the context.

## 3. Technology Stack (Initial Thoughts)

*   **Core Language**: Rust (leveraging `gitoxide`, `submod`, `magoo` where applicable).
*   **Parsing**: `serde`, `toml`, custom parsers for Git/Nix.
*   **Graph Data Structures**: Rust crates for graph representation (e.g., `petgraph`).
*   **Storage**: Initial implementation could use in-memory data structures or simple file-based storage (JSON/YAML). For persistence and scalability, a dedicated graph database would be considered.

## 4. Development Phases (High-Level)

1.  **Phase 1: Core Data Ingestion & Model**: Implement parsers for Git history, `.gitmodules`, `flake.nix`, `Cargo.toml`. Define initial data model and in-memory storage.
2.  **Phase 2: Basic Analysis & CLI**: Implement basic analysis functions (e.g., dependency listing, change history for a file) and a simple CLI for querying.
3.  **Phase 3: Advanced Analysis & Persistence**: Explore graph database integration, more sophisticated analysis (e.g., impact analysis, relationship discovery).
4.  **Phase 4: Visualization (Optional/Future)**: Develop a web-based UI for interactive exploration.
