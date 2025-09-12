#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct WordPredicate(pub bool);

impl WordPredicate {
    pub fn new(value: bool) -> Self { WordPredicate(value) }
}

/// Represents a simple classifier based on predicate presence.
pub struct PredicateClassifier {
    target_predicates: Vec<String>,
}

impl PredicateClassifier {
    pub fn new(predicates: Vec<&str>) -> Self {
        Self { target_predicates: predicates.into_iter().map(|s| s.to_lowercase()).collect() }
    }

    /// Extracts WordPredicates from text based on the classifier's target predicates.
    pub fn extract_word_predicates(&self, text: &str) -> Vec<WordPredicate> {
        let lower_text = text.to_lowercase();
        self.target_predicates.iter()
            .map(|p| WordPredicate(lower_text.contains(p)))
            .collect()
    }

    pub fn get_target_predicates(&self) -> &Vec<String> {
        &self.target_predicates
    }
}
