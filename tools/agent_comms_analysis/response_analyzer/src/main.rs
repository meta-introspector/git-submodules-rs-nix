use std::{fs, io, collections::HashMap};
use std::path::{Path, PathBuf};
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
    speaker_type: String,
}

fn organize_files_by_author(base_dir: &Path) -> io::Result<()> {
    println!("Organizing files by author...");

    let crq_regex = Regex::new(r"(CRQ-\d+|PR-\d+)").unwrap();

    // Collect all files to move first to avoid issues with modifying directory during iteration
    let mut files_to_move: Vec<(PathBuf, PathBuf)> = Vec::new();

    for entry in WalkDir::new(base_dir).into_iter().filter_map(|e| e.ok()) {
        let old_path = entry.path();

        if old_path.is_file() && old_path.extension().map_or(false, |ext| ext == "md") {
            let content = fs::read_to_string(old_path)?;

            let parts: Vec<&str> = content.splitn(3, "---").collect();
            if parts.len() < 3 {
                // Not a valid Markdown file with YAML front matter, skip for now
                continue;
            }

            let yaml_str = parts[1];
            let metadata: CommentMetadata = match serde_yaml::from_str(yaml_str) {
                Ok(meta) => meta,
                Err(_) => {
                    // This might be pr_description.md or other files without expected metadata
                    // For now, we only care about comments with specific metadata
                    continue;
                }
            };

            let author = metadata.author;

            let crq_id_match = crq_regex.find(old_path.to_str().unwrap());
            let crq_id = match crq_id_match {
                Some(m) => m.as_str(),
                None => {
                    eprintln!("Could not find CRQ ID in path: {:?}", old_path);
                    continue;
                }
            };

            let file_name = old_path.file_name().unwrap().to_str().unwrap();
            let file_stem = old_path.file_stem().unwrap().to_str().unwrap();
            let extension = old_path.extension().unwrap().to_str().unwrap();

            let mut new_dir = PathBuf::from(base_dir);
            new_dir.push(&author);
            new_dir.push(crq_id);

            // Handle 'responses' subdirectory if present in the original path
            if old_path.components().any(|c| c.as_os_str() == "responses") {
                new_dir.push("responses");
            }

            fs::create_dir_all(&new_dir)?;

            let new_file_name = format!("{}_{}.{}", file_stem, author, extension);
            let new_path = new_dir.join(&new_file_name);

            // Only add to list if the new path is different from the old path
            if old_path != new_path {
                files_to_move.push((old_path.to_path_buf(), new_path));
            }
        }
    }

    for (old_path, new_path) in files_to_move {
        fs::rename(&old_path, &new_path)?;
        println!("Moved '{:?}' to '{:?}'", old_path, new_path);
    }

    println!("File organization finished.");
    Ok(())
}

fn main() -> io::Result<()> {
    let comms_dir = Path::new("analysis_data/comms/git");

    if !comms_dir.exists() {
        eprintln!("Error: 'analysis_data/comms/git' directory not found. Please ensure the mirroring script has been run and data collected.");
        return Ok(())
    }

    // Organize files first
    organize_files_by_author(comms_dir)?;

    println!("Starting response analysis...");
    let too_large_response_re = Regex::new(r"too large response").unwrap(); // Placeholder regex
    let review_request_re = Regex::new(r"(?i)review the ticket|try now").unwrap(); // Case-insensitive

    let mut speaker_phrases: HashMap<String, HashMap<String, usize>> = HashMap::new();

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

            // Identify speaker type
            let speaker_type = if metadata.author == "coderabbitai" {
                "agent".to_string()
            } else {
                "user".to_string()
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
                speaker_type,
            };

            // Collect phrases for repeated phrase analysis
            let author_phrases = speaker_phrases.entry(analyzed_comment.metadata.author.clone()).or_insert_with(HashMap::new);
            for line in analyzed_comment.body.lines() {
                let trimmed_line = line.trim().to_string();
                if !trimmed_line.is_empty() {
                    *author_phrases.entry(trimmed_line).or_insert(0) += 1;
                }
            }

            println!("{}", serde_json::to_string_pretty(&analyzed_comment)?);
        }
    }

    println!("\n--- Repeated Phrases Analysis ---");
    for (author, phrases) in speaker_phrases {
        println!("\nAuthor: {}", author);
        for (phrase, count) in phrases {
            if count > 1 { // Only show phrases that appear more than once
                println!("  \"{}\": {}", phrase, count);
            }
        }
    }

    println!("Analysis finished.");
    Ok(())
}
