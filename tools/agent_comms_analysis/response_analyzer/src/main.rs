use std::{fs, io};
use std::path::Path;
use serde::{Deserialize, Serialize};
use walkdir::WalkDir;
use regex::Regex;
use serde_json;

#[derive(Debug, Deserialize, Serialize)]
struct CommentMetadata {
    crq: String,
    #[serde(rename = "messageId")]
    message_id: String,
    timestamp: String,
    author: String,
}

#[derive(Debug, Serialize)]
struct AnalyzedComment {
    metadata: CommentMetadata,
    body: String,
    is_coderabbitai_comment: bool,
    is_too_large_response: bool,
    is_review_request: bool,
    comment_length_chars: usize,
    comment_word_count: usize,
}

fn main() -> io::Result<()> {
    println!("Starting response analysis...");

    let comms_dir = Path::new("analysis_data/comms/git");

    if !comms_dir.exists() {
        eprintln!("Error: 'analysis_data/comms/git' directory not found. Please ensure the mirroring script has been run and data collected.");
        return Ok(());
    }

    let too_large_response_re = Regex::new(r"too large response").unwrap(); // Placeholder regex
    let review_request_re = Regex::new(r"(?i)review the ticket|try now").unwrap(); // Case-insensitive

    for entry in WalkDir::new(comms_dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() && path.extension().map_or(false, |ext| ext == "md") {
            let content = fs::read_to_string(path)?;

            // Split content into YAML front matter and body
            let parts: Vec<&str> = content.splitn(3, "---").collect();
            if parts.len() < 3 {
                // Not a valid Markdown file with YAML front matter
                continue;
            }

            let yaml_str = parts[1];
            let body = parts[2].trim().to_string();

            let metadata: CommentMetadata = match serde_yaml::from_str(yaml_str) {
                Ok(meta) => meta,
                Err(_) => {
                    // This might be pr_description.md, which has different metadata
                    // For now, we only care about comments with specific metadata
                    continue;
                }
            };

            // Analyze comment
            let is_coderabbitai_comment = metadata.author == "coderabbitai";
            let is_too_large_response = too_large_response_re.is_match(&body);
            let is_review_request = review_request_re.is_match(&body);
            let comment_length_chars = body.chars().count();
            let comment_word_count = body.split_whitespace().count();

            let analyzed_comment = AnalyzedComment {
                metadata,
                body,
                is_coderabbitai_comment,
                is_too_large_response,
                is_review_request,
                comment_length_chars,
                comment_word_count,
            };

            println!("{}", serde_json::to_string_pretty(&analyzed_comment)?);
        }
    }

    println!("Analysis finished.");
    Ok(())
}