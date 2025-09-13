# Builders: Data Extraction and Parsing - Regular Expressions - Basic Matching - 17-Fold Division

This document applies a 17-fold division to the 'Basic Matching' facet of 'Regular Expressions (Regex)' under the 'Builders' archetype, providing a deeper level of granularity for fundamental pattern matching in text.

## 1. Literal Characters

Matching exact characters as they appear in the pattern (e.g., `a`, `B`, `1`, `!`, `@`).

## 2. Any Character (`.`)

Matching any single character, except for newline characters (unless a specific flag is set).

## 3. Escaped Characters (`\`)

Using a backslash to match special regex characters literally (e.g., `\.`, `\*`, `\?`).

## 4. Case Sensitivity

Whether the matching process distinguishes between uppercase and lowercase letters (e.g., `a` matches only `a`, not `A`).

## 5. Whitespace

Matching literal space characters, tabs, or newlines within the pattern.

## 6. Digits (`\d`)

Matching any single numerical digit (equivalent to `[0-9]`).

## 7. Non-Digits (`\D`)

Matching any single character that is not a numerical digit.

## 8. Word Characters (`\w`)

Matching any single letter (a-z, A-Z), number (0-9), or underscore (`_`).

## 9. Non-Word Characters (`\W`)

Matching any single character that is not a word character (e.g., symbols, spaces).

## 10. Whitespace Characters (`\s`)

Matching any single whitespace character, including space, tab, newline, carriage return, and form feed.

## 11. Non-Whitespace Characters (`\S`)

Matching any single character that is not a whitespace character.

## 12. Line Start (`^`)

Matching the beginning of a line or string.

## 13. Line End (`$`)

Matching the end of a line or string.

## 14. Word Boundary (`\b`)

Matching the position between a word character and a non-word character, or the beginning/end of a string.

## 15. Non-Word Boundary (`\B`)

Matching any position that is not a word boundary (e.g., within a word).

## 16. Character Sets (`[...]`)

Matching any one of a set of specified characters (e.g., `[aeiou]` matches any vowel).

## 17. Negated Character Sets (`[^...]`)

Matching any single character *not* in a specified set (e.g., `[^0-9]` matches any non-digit).

---

## Visual Representation (Mermaid Diagram)

```mermaid
graph TD
    A[Builders: Basic Matching] --> B(1. Literal Characters)
    A --> C(2. Any Character (.))
    A --> D(3. Escaped Characters (\))
    A --> E(4. Case Sensitivity)
    A --> F(5. Whitespace)
    A --> G(6. Digits (\d))
    A --> H(7. Non-Digits (\D))
    A --> I(8. Word Characters (\w))
    A --> J(9. Non-Word Characters (\W))
    A --> K(10. Whitespace Characters (\s))
    A --> L(11. Non-Whitespace Characters (\S))
    A --> M(12. Line Start (^))
    A --> N(13. Line End ($))
    A --> O(14. Word Boundary (\b))
    A --> P(15. Non-Word Boundary (\B))
    A --> Q(16. Character Sets ([...]))
    A --> R(17. Negated Character Sets ([^...]))
```

```