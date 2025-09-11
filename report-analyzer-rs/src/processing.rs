use crate::types::{Report, Ontology, RepoInfo, SubmoduleInfo, FailedRepoInfo};
use std::collections::HashMap;
use regex::Regex;

pub fn apply_emoji_ontology(text: &str, ontology: &Option<Ontology>) -> String {
    if let Some(ont) = ontology {
        let mut result = text.to_string();
        let mut sorted_keys: Vec<&String> = ont.0.keys().collect();
        sorted_keys.sort_by(|a, b| b.len().cmp(&a.len()));

        for key in sorted_keys {
            if let Some(emoji) = ont.0.get(key) {
                result = result.replace(key, emoji);
            }
        }
        result
    } else {
        text.to_string()
    }
}

pub fn analyze_strings(report: &Report, ontology: &Option<Ontology>) -> Result<Vec<(String, usize)>, Box<dyn std::error::Error>> {
    let mut all_tokens: Vec<String> = Vec::new();
    let tokenizer_re = Regex::new(r"[^a-zA-Z0-9]+")?;

    let mut suggested_rules: Vec<(String, usize)> = Vec::new(); // Store (n_gram, count) for suggestions

    // Collect tokens from successful repositories
    for (url, info) in &report.repositories {
        // Tokenize URL
        for part in tokenizer_re.split(url) {
            if !part.is_empty() {
                all_tokens.push(part.to_lowercase());
            }
        }
        // Tokenize path
        for part in tokenizer_re.split(&info.path) {
            if !part.is_empty() {
                all_tokens.push(part.to_lowercase());
            }
        }
        // Tokenize submodule names and nested repo URLs
        for submodule in &info.submodules {
            for part in tokenizer_re.split(&submodule.name) {
                if !part.is_empty() {
                    all_tokens.push(part.to_lowercase());
                }
            }
            if let Some(nested_repo) = &submodule.nested_repo {
                for part in tokenizer_re.split(&nested_repo.url) {
                    if !part.is_empty() {
                        all_tokens.push(part.to_lowercase());
                    }
                }
            }
        }
    }

    // Collect tokens from failed repositories (from error messages)
    for failed_repo in &report.failed_repositories {
        for part in tokenizer_re.split(&failed_repo.error) {
            if !part.is_empty() {
                all_tokens.push(part.to_lowercase());
            }
        }
    }

    if !all_tokens.is_empty() {
        let mut token_counts: HashMap<String, usize> = HashMap::new();
        for token in &all_tokens {
            *token_counts.entry(token.to_string()).or_insert(0) += 1;
        }

        let mut sorted_token_counts: Vec<(&String, &usize)> = token_counts.iter().collect();
        sorted_token_counts.sort_by(|a, b| b.1.cmp(a.1));

        println!("\n--- Most Frequently Mentioned Tokens ---");
        for (token, count) in sorted_token_counts.iter().take(20) {
            println!("{}: {}", apply_emoji_ontology(token, ontology), count);
        }
    }

    // Generate initial emoji tokens
    let mut current_emoji_tokens: Vec<String> = all_tokens.iter()
        .map(|token| apply_emoji_ontology(token, ontology))
        .collect();

    let n_gram_sizes = vec![1, 2, 3, 5, 7, 11, 13, 17, 19];
    let mut iteration = 0;
    let max_iterations = 20; // A practical limit to prevent infinite loops

    loop {
        iteration += 1;
        println!("\n--- Iteration {} ---", iteration);
        let mut changed = false;
        let previous_emoji_tokens = current_emoji_tokens.clone(); // For convergence check

        let mut next_iteration_tokens: Vec<String> = Vec::new(); // Tokens for the next iteration

        for &n in &n_gram_sizes {
            if current_emoji_tokens.len() >= n {
                let mut n_grams: Vec<String> = Vec::new();
                for i in 0..current_emoji_tokens.len() - (n - 1) {
                    let mut current_n_gram = String::new();
                    for j in 0..n {
                        current_n_gram.push_str(&current_emoji_tokens[i + j]);
                        if j < n - 1 {
                            current_n_gram.push_str(" "); // Add space between emoji tokens
                        }
                    }
                    n_grams.push(current_n_gram);
                }

                let mut n_gram_counts: HashMap<String, usize> = HashMap::new();
                for n_gram in n_grams {
                    *n_gram_counts.entry(n_gram).or_insert(0) += 1;
                }

                let mut sorted_n_gram_counts: Vec<(&String, &usize)> = n_gram_counts.iter().collect();
                sorted_n_gram_counts.sort_by(|a, b| b.1.cmp(a.1));

                println!("\n--- Most Frequently Mentioned {}-grams (Iteration {}) ---", n, iteration);
                for (n_gram, count) in sorted_n_gram_counts.iter().take(10) {
                    let compressed_n_gram = apply_emoji_ontology(n_gram, ontology);
                    println!("{}: {}", compressed_n_gram.replace(" ", ""), count); // Remove spaces for final output

                    // Collect suggestions
                    if n_gram != &compressed_n_gram {
                        suggested_rules.push((n_gram.clone(), *count));
                    }

                    // For the next iteration, we want the compressed version of the n-gram
                    // if it was actually compressed, otherwise the original n-gram.
                    // We also need to ensure we're not adding duplicates or single emojis that are already compressed.
                    next_iteration_tokens.push(compressed_n_gram.replace(" ", ""));
                }
            } else {
                println!("\n--- Not enough tokens to generate {}-grams (Iteration {}) ---", n, iteration);
            }
        }

        // Check for convergence
        if previous_emoji_tokens == next_iteration_tokens || iteration >= max_iterations {
            break;
        }
        current_emoji_tokens = next_iteration_tokens;
    }

    Ok(suggested_rules)
}

pub fn find_longest_common_prefix(strings: &[String]) -> String {
    if strings.is_empty() {
        return String::new();
    }

    let mut lcp = strings[0].clone();
    for s in &strings[1..] {
        let mut new_lcp = String::new();
        for (c1, c2) in lcp.chars().zip(s.chars()) {
            if c1 == c2 {
                new_lcp.push(c1);
            } else {
                break;
            }
        }
        lcp = new_lcp;
        if lcp.is_empty() {
            break;
        }
    }
    lcp
}

pub fn perform_lcp_analysis(report: &Report) -> String {
    let mut all_paths_and_urls: Vec<String> = Vec::new();
    for (url, info) in &report.repositories {
        all_paths_and_urls.push(url.clone());
        all_paths_and_urls.push(info.path.clone());
        for submodule in &info.submodules {
            all_paths_and_urls.push(submodule.url.clone());
            all_paths_and_urls.push(submodule.path.clone());
            if let Some(nested_repo) = &submodule.nested_repo {
                all_paths_and_urls.push(nested_repo.url.clone());
                all_paths_and_urls.push(nested_repo.path.clone());
            }
        }
    }
    for failed_repo in &report.failed_repositories {
        all_paths_and_urls.push(failed_repo.path.clone());
    }
    find_longest_common_prefix(&all_paths_and_urls)
}

pub fn print_lcp_analysis(lcp: &String) {
    if !lcp.is_empty() {
        println!("\n--- Longest Common Prefix (LCP) ---");
        println!("LCP: {}", lcp);
        println!("Paths and URLs below are relative to LCP where applicable.");
    }
}

pub fn analyze_duplicate_urls(report: &Report) -> HashMap<String, Vec<String>> {{
    let mut repo_urls: HashMap<String, Vec<String>> = HashMap::new();
    for (url, info) in &report.repositories {{
        repo_urls.entry(url.clone()).or_default().push(info.path.clone());
    }}
    repo_urls
}}

pub fn print_duplicate_urls(repo_urls: &HashMap<String, Vec<String>>) {{
    let mut duplicate_urls_found = false;
    println!("\n--- Duplicate Repository URLs ---");
    for (url, paths) in repo_urls {{
        if paths.len() > 1 {{
            duplicate_urls_found = true;
            println!("URL: {}", url);
            for path in paths {{
                println!("  - {}", path);
            }}
        }}
    }}
    if !duplicate_urls_found {
        println!("No Duplicate Repository URLs Found ");
    }
}

pub fn analyze_organizations(report: &Report) -> HashMap<String, usize> {
    let mut organizations: Vec<String> = Vec::new();
    let re_org = Regex::new(r"https://github.com/([^/]+)/.*" ).unwrap();

    for url in report.repositories.keys() {
        if let Some(captures) = re_org.captures(url) {
            organizations.push(captures[1].to_string());
        }
    }
    
    for failed_repo in &report.failed_repositories {
        if let Some(captures) = re_org.captures(&failed_repo.error) {
            organizations.push(captures[1].to_string());
        }
    }

    let mut org_counts: HashMap<String, usize> = HashMap::new();
    if !organizations.is_empty() {
        for org in organizations {
            *org_counts.entry(org).or_insert(0) += 1;
        }
    }
    org_counts
}

pub fn print_organizations_analysis(org_counts: &HashMap<String, usize>, ontology: &Option<Ontology>) {
    if !org_counts.is_empty() {
        let mut sorted_org_counts: Vec<(&String, &usize)> = org_counts.iter().collect();
        sorted_org_counts.sort_by(|a, b| b.1.cmp(a.1));

        println!("\n--- Most Frequently Mentioned Organizations ---");
        for (org, count) in sorted_org_counts.iter().take(10) {
            println!("{}: {}", apply_emoji_ontology(org, ontology), count);
        }
    } else {
        println!("\n--- No Organizations Found ---");
    }
}

pub fn analyze_names(report: &Report) -> HashMap<String, usize> {{
    let mut all_names: Vec<String> = Vec::new();
    let re_repo_name = Regex::new(r"https://github.com/[^/]+/([^/]+).*?").unwrap();

    for (url, info) in &report.repositories {{
        // Add repository name from URL
        if let Some(captures) = re_repo_name.captures(url) {{
            all_names.push(captures[1].replace(".git", ""));
        }}
        
        // Add submodule names
        for submodule in &info.submodules {{
            all_names.push(submodule.name.clone());
            if let Some(nested_repo) = &submodule.nested_repo {{
                if let Some(captures) = re_repo_name.captures(&nested_repo.url) {{
                    all_names.push(captures[1].replace(".git", ""));
                }}
            }}
        }}
    }}

    let mut name_counts: HashMap<String, usize> = HashMap::new();
    if !all_names.is_empty() {{
        for name in all_names {{
            *name_counts.entry(name).or_insert(0) += 1;
        }}
    }}
    name_counts
}}

pub fn print_names_analysis(name_counts: &HashMap<String, usize>, ontology: &Option<Ontology>) {{
    if !name_counts.is_empty() {{
        let mut sorted_name_counts: Vec<(&String, &usize)> = name_counts.iter().collect();
        sorted_name_counts.sort_by(|a, b| b.1.cmp(a.1));

        println!("\n--- Most Frequently Mentioned Repository/Submodule Names ---");
        for (name, count) in sorted_name_counts.iter().take(10) {{
            println!("{}: {}", apply_emoji_ontology(name, ontology), count);
        }}
    }} else {{
        println!("\n--- No Repository/Submodule Names Found ---");
    }}
}}

pub fn print_suggested_rules(suggested_rules: &Vec<(String, usize)>) {{
    if !suggested_rules.is_empty() {{
        let mut sorted_suggestions = suggested_rules.clone();
        sorted_suggestions.sort_by(|a, b| b.1.cmp(&a.1));

        println!("\n--- Suggested New Ontology Rules ---");
        for (n_gram, count) in sorted_suggestions.iter().take(5) {{
            println!("\"{{}}\": \"❓\", // Count: {{}}", n_gram.replace(" ", ""), count);
        }}
    }}
}}
