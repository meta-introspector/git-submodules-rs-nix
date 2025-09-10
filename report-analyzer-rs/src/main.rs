use clap::Parser;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, BTreeMap};
use std::fs;
use regex::Regex;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
struct SubmoduleInfo {
    name: String,
    path: String,
    url: String,
    branch: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nested_repo: Option<RepoInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
struct RepoInfo {
    path: String,
    url: String,
    submodules: Vec<SubmoduleInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
struct FailedRepoInfo {
    path: String,
    error: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Report {
    repositories: HashMap<String, RepoInfo>,
    failed_repositories: Vec<FailedRepoInfo>,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the submodule report JSON file
    #[arg(long)]
    report_path: String,
    /// Optional: Path to an ontology JSON file for emoji mapping
    #[arg(long)]
    ontology_path: Option<PathBuf>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Ontology(HashMap<String, String>);

fn apply_emoji_ontology(text: &str, ontology: &Option<Ontology>) -> String {
    if let Some(ont) = ontology {
        let mut result = text.to_string();
        // Sort keys by length in descending order to prioritize longer matches (n-grams)
        let mut sorted_keys: Vec<&String> = ont.0.keys().collect();
        sorted_keys.sort_by(|a, b| b.len().cmp(&a.len()));

        for key in sorted_keys {
            if let Some(emoji) = ont.0.get(key) {
                // Replace all occurrences of the key with the emoji
                result = result.replace(key, emoji);
            }
        }
        result
    } else {
        text.to_string()
    }
}

fn analyze_strings(report: &Report, ontology: &Option<Ontology>) -> Result<(), Box<dyn std::error::Error>> {
    let mut all_tokens: Vec<String> = Vec::new();
    let tokenizer_re = Regex::new(r"[^a-zA-Z0-9]+")?; // Split on non-alphanumeric characters

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

    let n_gram_sizes = vec![1, 2, 3, 5, 7, 11, 13, 17, 19];

    for &n in &n_gram_sizes {
        if all_tokens.len() >= n {
            let mut n_grams: Vec<String> = Vec::new();
            for i in 0..all_tokens.len() - (n - 1) {
                let mut current_n_gram = String::new();
                for j in 0..n {
                    current_n_gram.push_str(&all_tokens[i + j]);
                    if j < n - 1 {
                        current_n_gram.push_str(" ");
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

            println!("\n--- Most Frequently Mentioned {}-grams ---", n);
            for (n_gram, count) in sorted_n_gram_counts.iter().take(10) {
                println!("{}: {}", apply_emoji_ontology(n_gram, ontology), count);
            }
        } else {
            println!("\n--- Not enough tokens to generate {}-grams ---", n);
        }
    }

    Ok(())
}

// Function to find the longest common prefix among a list of strings
fn find_longest_common_prefix(strings: &[String]) -> String {
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let report_content = fs::read_to_string(&args.report_path)?;
    let report: Report = serde_json::from_str(&report_content)?;

    let ontology: Option<Ontology> = if let Some(path) = &args.ontology_path {
        let ontology_content = fs::read_to_string(path)?;
        Some(serde_json::from_str(&ontology_content)?)
    } else {
        None
    };

    let successful_repos = report.repositories.len();
    let failed_repos = report.failed_repositories.len();

    println!("--- Submodule Report Analysis ---");
    println!("Total successful repositories: {}", successful_repos);
    println!("Total failed repositories: {}", failed_repos);

    // Collect all paths and URLs for LCP analysis
    let mut all_paths_and_urls: Vec<String> = Vec::new();
    for (url, info) in &report.repositories {
        all_paths_and_urls.push(url.clone());
        all_paths_and_urls.push(info.path.clone());
        for submodule in &info.submodules {
            all_paths_and_urls.push(submodule.url.clone());
            all_paths_and_urls.push(submodule.path.clone()); // Relative path, but still useful for LCP
            if let Some(nested_repo) = &submodule.nested_repo {
                all_paths_and_urls.push(nested_repo.url.clone());
                all_paths_and_urls.push(nested_repo.path.clone());
            }
        }
    }
    for failed_repo in &report.failed_repositories {
        all_paths_and_urls.push(failed_repo.path.clone());
        // We don't have a URL for failed repos directly, but the error might contain parts of it
        // For now, we'll just use the path.
    }

    let lcp = find_longest_common_prefix(&all_paths_and_urls);
    if !lcp.is_empty() {
        println!("\n--- Longest Common Prefix (LCP) ---");
        println!("LCP: {}", lcp);
        println!("Paths and URLs below are relative to LCP where applicable.");
    }

    // Identify duplicate repository entries (by URL)
    let mut repo_urls: HashMap<String, Vec<String>> = HashMap::new();
    for (url, info) in &report.repositories {
        repo_urls.entry(url.clone()).or_default().push(info.path.clone());
    }

    let mut duplicate_urls_found = false;
    println!("\n--- Duplicate Repository URLs ---");
    for (url, paths) in repo_urls {
        if paths.len() > 1 {
            duplicate_urls_found = true;
            println!("URL: {}", url);
            for path in paths {
                println!("  - {}", path);
            }
        }
    }
    if !duplicate_urls_found {
        println!("No Duplicate Repository URLs Found ");
    }

    // Analysis of most frequently mentioned organizations (from repository URLs)
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

    if !organizations.is_empty() {
        let mut org_counts: HashMap<String, usize> = HashMap::new();
        for org in organizations {
            *org_counts.entry(org).or_insert(0) += 1;
        }

        let mut sorted_org_counts: Vec<(&String, &usize)> = org_counts.iter().collect();
        sorted_org_counts.sort_by(|a, b| b.1.cmp(a.1));

        println!("\n--- Most Frequently Mentioned Organizations ---");
        for (org, count) in sorted_org_counts.iter().take(10) {
            println!("{}: {}", apply_emoji_ontology(org, &ontology), count);
        }
    } else {
        println!("\n--- No Organizations Found ---");
    }

    // Analysis of most frequently mentioned names or strings (submodule names, repository names)
    let mut all_names: Vec<String> = Vec::new();
    let re_repo_name = Regex::new(r"https://github.com/[^/]+/([^/]+).*").unwrap();

    for (url, info) in &report.repositories {
        // Add repository name from URL
        if let Some(captures) = re_repo_name.captures(url) {
            all_names.push(captures[1].replace(".git", ""));
        }
        
        // Add submodule names
        for submodule in &info.submodules {
            all_names.push(submodule.name.clone());
            if let Some(nested_repo) = &submodule.nested_repo {
                if let Some(captures) = re_repo_name.captures(&nested_repo.url) {
                    all_names.push(captures[1].replace(".git", ""));
                }
            }
        }
    }

    if !all_names.is_empty() {
        let mut name_counts: HashMap<String, usize> = HashMap::new();
        for name in all_names {
            *name_counts.entry(name).or_insert(0) += 1;
        }

        let mut sorted_name_counts: Vec<(&String, &usize)> = name_counts.iter().collect();
        sorted_name_counts.sort_by(|a, b| b.1.cmp(a.1));

        println!("\n--- Most Frequently Mentioned Repository/Submodule Names ---");
        for (name, count) in sorted_name_counts.iter().take(10) {
            println!("{}: {}", apply_emoji_ontology(name, &ontology), count);
        }
    } else {
        println!("\n--- No Repository/Submodule Names Found ---");
    }

    analyze_strings(&report, &ontology)?;

    Ok(())
}