use std::collections::HashMap;
use std::io::{self, Write, BufReader, BufRead}; // Added BufRead
use std::fs::File;
use regex::Regex;
use serde::{Serialize, Deserialize};
use serde_json::json;

// Define the structure for a single repository count for JSON-LD
#[derive(Debug, Serialize, Deserialize)]
pub struct RepositoryCount {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@type")]
    pub _type: String,
    pub url: String,
    pub owner: String,
    pub repo: String,
    pub count: usize,
}

pub fn count_github_repos(reader: BufReader<File>) -> io::Result<()> { // Changed signature
    let github_repo_re = Regex::new(r"https://github.com/([^/]+)/([^/]+)").unwrap();
    let mut repo_counts: HashMap<String, usize> = HashMap::new();

    for line_result in reader.lines() {
        let line = line_result?;
        if let Some(captures) = github_repo_re.captures(&line) {
            let owner = captures.get(1).unwrap().as_str();
            let repo = captures.get(2).unwrap().as_str();
            let full_repo_url = format!("https://github.com/{}/{}", owner, repo);
            *repo_counts.entry(full_repo_url).or_insert(0) += 1;
        }
    }

    let mut repo_list: Vec<RepositoryCount> = Vec::new();
    for (url, count) in repo_counts {
        let parts: Vec<&str> = url.split('/').collect();
        let owner = parts[3];
        let repo = parts[4]; // This might include .git, which is fine for now.

        repo_list.push(RepositoryCount {
            id: url.clone(),
            _type: "RepositoryCount".to_string(),
            url: url.clone(), // Cloned here
            owner: owner.to_string(),
            repo: repo.to_string(),
            count: count,
        });
    }

    // Construct the JSON-LD output
    let json_ld = json!({
        "@context": {
            "RepositoryCount": "http://example.org/RepositoryCount",
            "url": "http://schema.org/url",
            "owner": "http://example.org/owner",
            "repo": "http://example.org/repo",
            "count": "http://example.org/count"
        },
        "@graph": repo_list
    });

    let output_file_path = "github_repo_counts.jsonld";
    let output_file = File::create(output_file_path)?;
    let mut writer = io::BufWriter::new(output_file);

    serde_json::to_writer_pretty(&mut writer, &json_ld)?;
    writeln!(writer, "")?; // Add a newline at the end

    Ok(())
}