use crate::types::{Ontology};
use std::collections::HashMap;
use super::processing::apply_emoji_ontology; // Import apply_emoji_ontology from the original processing module

pub fn print_suggested_rules_with_emojis(suggested_rules: &Vec<(String, usize)>, ontology: &Option<Ontology>) {
    if !suggested_rules.is_empty() {
        let mut sorted_suggestions = suggested_rules.clone();
        sorted_suggestions.sort_by(|a, b| b.1.cmp(&a.1));

        println!("\n--- Suggested New Ontology Rules (with Emojis) ---");
        for (n_gram, count) in sorted_suggestions.iter().take(5) {
            let emoji_n_gram = apply_emoji_ontology(n_gram, ontology);
            println!("\"{}\": \"{}\", // Count: {}", n_gram.replace(" ", ""), emoji_n_gram.replace(" ", ""), count);
        }
    }
}