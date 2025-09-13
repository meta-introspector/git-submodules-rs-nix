# Builders: Core Libraries - API Design - Descriptive Naming - 17-Fold Division

This document applies a 17-fold division to the 'Descriptive Naming' facet of 'Clarity and Intuitiveness' under the 'Builders' archetype, providing a deeper level of granularity for using clear, unambiguous names for functions, parameters, and types in API design.

## 1. Clarity

Names should clearly and immediately convey the purpose, function, or content of the entity they represent.

## 2. Conciseness

Names should be as short as possible without sacrificing clarity or introducing ambiguity.

## 3. Consistency

Adhering to established naming conventions (e.g., camelCase, snake_case, PascalCase) across the entire API and codebase.

## 4. Unambiguity

Avoiding names that could have multiple interpretations or be easily confused with other entities.

## 5. Pronounceability

Names should be easy to say and discuss aloud, facilitating verbal communication among team members.

## 6. Searchability

Names should be easy to find in codebases, documentation, and IDEs, often by avoiding single-letter names or common keywords.

## 7. Avoidance of Abbreviations

Unless widely understood and universally accepted within the domain, avoid shortening words to prevent confusion.

## 8. Domain-Specific Language

Using terms and concepts familiar to the problem domain, making the code more intuitive for domain experts.

## 9. Action-Oriented Verbs (for methods)

Methods should typically start with strong, clear verbs that indicate the action they perform (e.g., `calculateTotal`, `getUserData`).

## 10. Noun-Based (for classes/variables)

Classes, variables, and properties should generally use nouns or noun phrases that describe the entity they represent (e.g., `Customer`, `orderId`).

## 11. Contextual Relevance

Names should make sense and be appropriate within their specific scope or context, avoiding overly generic names.

## 12. Avoidance of Hungarian Notation

Not encoding type information or scope into the name (e.g., `strUserName`, `intCount`), as modern IDEs provide this information.

## 13. Singular/Plural Consistency

Using singular nouns for single instances (e.g., `user`) and plural nouns for collections (e.g., `users`, `userList`).

## 14. Readability

Names should be easy to read and scan, avoiding excessive length or awkward combinations of words.

## 15. Reflect Level of Abstraction

Names should match the level of detail or abstraction they represent, avoiding mixing high-level and low-level terms.

## 16. Avoidance of Generic Names

Names like `data`, `item`, `temp`, `value`, or `obj` should be avoided as they convey little meaning without context.

## 17. Evolution and Refactoring

Considering how naming conventions might adapt over time and during refactoring, and the impact of name changes on existing code.

---

## Visual Representation (Mermaid Diagram)

```mermaid
graph TD
    A[Builders: Descriptive Naming] --> B(1. Clarity)
    A --> C(2. Conciseness)
    A --> D(3. Consistency)
    A --> E(4. Unambiguity)
    A --> F(5. Pronounceability)
    A --> G(6. Searchability)
    A --> H(7. Avoidance of Abbreviations)
    A --> I(8. Domain-Specific Language)
    A --> J(9. Action-Oriented Verbs (for methods))
    A --> K(10. Noun-Based (for classes/variables))
    A --> L(11. Contextual Relevance)
    A --> M(12. Avoidance of Hungarian Notation)
    A --> N(13. Singular/Plural Consistency)
    A --> O(14. Readability)
    A --> P(15. Reflect Level of Abstraction)
    A --> Q(16. Avoidance of Generic Names)
    A --> R(17. Evolution and Refactoring)
```
