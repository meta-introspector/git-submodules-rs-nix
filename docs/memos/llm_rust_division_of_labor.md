## Memo: LLM as Code Generator, Rust as Executor

**Date:** 2025-09-12

**Subject:** Division of Labor between AI and Rust for Knowledge Base Expansion

This memo outlines a strategic division of labor for expanding our knowledge base, particularly concerning the extraction of RDF facts (URLs) from Wikipedia articles via Wikidata.

**LLM's Role (The Gemini AI Herder):**

The Large Language Model (LLM) will serve as the "AI Herder," responsible for the intelligent generation and orchestration of code. Its primary functions include:

1.  **Understanding Requirements:** Interpreting user requests for data extraction, analysis, or system interaction.
2.  **Code Generation:** Translating these requirements into precise, functional, and idiomatic Rust code. This includes designing data structures, algorithms, and API interactions.
3.  **Tool Selection and Integration:** Identifying and recommending appropriate Rust crates (e.g., `reqwest` for HTTP, `serde_json` for JSON parsing) and demonstrating their usage within the generated code.
4.  **Workflow Definition:** Defining the sequence of operations and data flow for complex tasks.
5.  **Documentation and Explanation:** Providing clear explanations of the generated code and the rationale behind its design.

**Rust's Role (The Digital Sheep):**

Rust, as the "Digital Sheep," will be the robust and reliable executor of the LLM-generated code. Its characteristics make it ideal for this role:

1.  **Performance:** Rust's focus on zero-cost abstractions and memory safety ensures high-performance execution of data-intensive tasks like web scraping and API querying.
2.  **Reliability and Safety:** The strong type system and borrow checker prevent common programming errors, leading to more stable and secure applications.
3.  **Concurrency:** Rust's excellent support for concurrency allows for efficient parallel processing of multiple URLs or API requests.
4.  **Ecosystem:** A rich ecosystem of crates provides readily available solutions for common programming challenges, which the LLM can leverage.
5.  **Deterministic Execution:** Rust code provides predictable and consistent results, crucial for building and maintaining a reliable knowledge base.

**Workflow:**

1.  The LLM receives a task (e.g., "extract all external links from Wikipedia articles related to X").
2.  The LLM generates a Rust program (or a set of functions/modules) designed to accomplish this task.
3.  The generated Rust code is then compiled and executed. This execution can be triggered manually or as part of an automated pipeline.
4.  The output of the Rust program (e.g., a list of extracted URLs) is then integrated into the knowledge base or presented to the user.

This symbiotic relationship leverages the LLM's intelligence for rapid prototyping and complex logic generation, while relying on Rust for the efficient, safe, and reliable execution of critical data processing tasks.