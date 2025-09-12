# Word Classification Module

This document describes the `word_classifier` module, which encapsulates generic word classification logic for use across the project.

## Purpose

The `src/word_classifier.rs` module was created to centralize the functionality related to identifying the presence of specific words (predicates) within a given text. This promotes reusability and separation of concerns, allowing other modules to utilize this logic without needing to implement it themselves.

## Key Components

### `PredicateClassifier`

- **Role**: This struct is responsible for classifying text based on a predefined set of target predicates (words).
- **Functionality**:
    - It is initialized with a list of words to look for.
    - Its `extract_word_predicates` method takes a string of text and returns a vector of `WordPredicate` indicating whether each target word is present in the text.

### `WordPredicate`

- **Role**: A simple boolean wrapper that represents the presence or absence of a specific word (predicate) in a text.
- **Functionality**: It holds a single boolean value (`true` if the word is found, `false` otherwise).

## Usage Example

```rust
use submodules::word_classifier::{PredicateClassifier, WordPredicate};

fn main() {
    let target_words = vec!["rust", "programming", "example"];
    let classifier = PredicateClassifier::new(target_words.iter().map(|&s| s).collect());

    let text_to_analyze = "This is an example of Rust programming.";
    let predicates = classifier.extract_word_predicates(text_to_analyze);

    // You can then iterate through predicates and their corresponding target words
    // to see which words were found.
    for (word, word_predicate) in classifier.get_target_predicates().iter().zip(predicates.into_iter()) {
        println!("Word '{}': Found = {}", word, word_predicate.0);
    }
}
```

## Impact on Other Modules

- **`src/lib.rs`**: The `build_zos_lattice` function in `src/lib.rs` now imports and utilizes `PredicateClassifier` from `src/word_classifier.rs` instead of `src/lattice_model.rs`.
- **`src/bin/state-word-reporter.rs`**: This binary now imports `PredicateClassifier` from `src/word_classifier.rs` to perform its bag-of-words analysis on CRQ and SOP documents.
