// src/ngram_analyzer.rs

pub struct NGramDecomposition {
    pub parent_ngram: Vec<String>,
    pub child_ngrams: Vec<Vec<String>>,
}

pub fn decompose_ngram(ngram: &[String]) -> Vec<Vec<String>> {
    let mut decomposed = Vec::new();
    let n = ngram.len();

    if n <= 1 {
        return decomposed; // Cannot decompose 1-grams or empty n-grams further
    }

    // Generate all sub-ngrams from 1-gram up to (n-1)-gram
    for i in 1..n { // i is the size of the sub-ngram
        for window in ngram.windows(i) {
            decomposed.push(window.iter().cloned().collect());
        }
    }
    decomposed
}

pub fn analyze_hierarchical_ngrams(text: &str, max_n: usize) -> Vec<NGramDecomposition> {
    let words = tokenize_text(text);
    let mut hierarchical_data = Vec::new();

    for n in 2..=max_n { // Start from 2-grams to decompose
        let ngrams = generate_ngrams(&words, n);
        for ngram in ngrams {
            let decomposed_ngrams = decompose_ngram(&ngram);
            if !decomposed_ngrams.is_empty() {
                hierarchical_data.push(NGramDecomposition {
                    parent_ngram: ngram,
                    child_ngrams: decomposed_ngrams,
                });
            }
        }
    }
    hierarchical_data
}

pub fn tokenize_text(text: &str) -> Vec<String> {
    text.to_lowercase()
        .split(|c: char| !c.is_alphanumeric())
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect()
}

pub fn generate_ngrams(words: &[String], n: usize) -> Vec<Vec<String>> {
    if words.len() < n || n == 0 {
        return Vec::new();
    }

    words.windows(n)
         .map(|window| window.iter().cloned().collect())
         .collect()
}

pub struct NGramClassifier {
    target_ngrams: Vec<Vec<String>>,
}

impl NGramClassifier {
    pub fn new(ngrams: Vec<Vec<String>>) -> Self {
        Self { target_ngrams: ngrams }
    }

    pub fn extract_ngram_presence(&self, text: &str) -> Vec<bool> {
        let words = tokenize_text(text);
        let mut presence = Vec::new();

        for target_ngram in &self.target_ngrams {
            let n = target_ngram.len();
            let mut found = false;
            if n > 0 {
                for window in words.windows(n) {
                    if window.iter().cloned().collect::<Vec<String>>() == *target_ngram {
                        found = true;
                        break;
                    }
                }
            }
            presence.push(found);
        }
        presence
    }

    pub fn get_target_ngrams(&self) -> &Vec<Vec<String>> {
        &self.target_ngrams
    }
}