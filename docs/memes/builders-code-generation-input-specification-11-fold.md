# Builders: Code Generation - Input Specification - 11-Fold Division

This document applies an 11-fold division to the 'Input Specification' facet of 'Code Generation' under the 'Builders' archetype, providing a deeper level of granularity for defining the source or model from which code is generated.

## 1. Abstract Syntax Trees (ASTs)

Using a tree representation of source code as input, allowing for structural analysis and transformation during generation.

## 2. Domain-Specific Languages (DSLs)

Defining a specialized language tailored for expressing the generation logic or the domain model, simplifying complex configurations.

## 3. Configuration Files

Using structured data formats (e.g., YAML, JSON, XML, TOML) to define parameters, settings, or models that drive the code generation process.

## 4. Database Schemas

Generating code (e.g., ORM models, data access layers) based on the definitions of database tables, columns, relationships, and constraints.

## 5. UML Models

Using Unified Modeling Language diagrams (e.g., class diagrams, state machine diagrams) as a visual input for generating code.

## 6. API Definitions

Generating client or server code, documentation, or test stubs from formal API specifications like OpenAPI/Swagger.

## 7. Grammars

Defining formal grammars (e.g., ANTLR, EBNF) to parse input text into a structured representation that can then be used for code generation.

## 8. Metadata Annotations

Embedding generation instructions, directives, or additional metadata directly within existing code or configuration files.

## 9. External Data Sources

Pulling data from external systems, spreadsheets, CSVs, or other data repositories to populate templates or drive generation logic.

## 10. Interactive Prompts

Guiding the user through a series of questions or choices to define the parameters and options for code generation in real-time.

## 11. Version Control Integration

Using information from Git history, branch names, commit messages, or repository structure as input for generating version-specific code or documentation.

---

## Visual Representation (Mermaid Diagram)

```mermaid
graph TD
    A[Builders: Input Specification] --> B(1. Abstract Syntax Trees (ASTs))
    A --> C(2. Domain-Specific Languages (DSLs))
    A --> D(3. Configuration Files)
    A --> E(4. Database Schemas)
    A --> F(5. UML Models)
    A --> G(6. API Definitions)
    A --> H(7. Grammars)
    A --> I(8. Metadata Annotations)
    A --> J(9. External Data Sources)
    A --> K(10. Interactive Prompts)
    A --> L(11. Version Control Integration)
```
