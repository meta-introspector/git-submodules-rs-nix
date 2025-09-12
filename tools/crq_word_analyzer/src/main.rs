use clap::Parser;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use regex::Regex;
use lazy_static::lazy_static;

mod crq_parser;
use crq_parser::{determine_next_step, NextStep};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the directory containing CRQ files
    #[arg(short, long, default_value = "docs/crq_standardized")]
    crq_dir: PathBuf,

    /// Perform sequential response analysis instead of general word analysis
    #[arg(long)]
    sequential_analysis: bool,

    /// Perform individual response classification
    #[arg(long)]
    classify_individual_responses: bool,

    /// Path to a single file to analyze for tokens
    #[arg(short, long)]
    file: Option<PathBuf>,
}

lazy_static! {
    static ref WORD_REGEX: Regex = Regex::new(r"[a-z]+").unwrap();
}

fn extract_tokens(text: &str) -> Vec<String> {
    let words: Vec<String> = WORD_REGEX.find_iter(&text.to_lowercase())
        .map(|m| m.as_str().to_string())
        .collect();

    let mut tokens = Vec::new();

    // Add single words (1-grams)
    tokens.extend(words.clone());

    // Add n-grams
    let n_gram_lengths = vec![2, 3, 5, 7];
    for n in n_gram_lengths {
        if n > words.len() {
            continue;
        }
        for i in 0..=words.len() - n {
            tokens.push(words[i..i + n].join(" "));
        }
    }
    tokens
}

fn count_words(words: &[String], word_counts: &mut HashMap<String, usize>) {
    for word in words {
        *word_counts.entry(word.clone()).or_insert(0) += 1;
    }
}

fn print_top_words(title: &str, word_counts: &HashMap<String, usize>, top_n: usize) {
    let mut sorted_words: Vec<(&String, &usize)> = word_counts.iter().collect();
    sorted_words.sort_by(|a, b| b.1.cmp(a.1));

    println!("\n--- Top {} Words/N-grams for {} ---", top_n, title);
    for (word, count) in sorted_words.iter().take(top_n) {
        println!("{}: {}", word, count);
    }
}

// New enum for individual response types
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum ResponseType {
    SkippedMessage,
    RateLimitMessage,
    ReviewSummary,
    QuestionMessage,
    OtherMessage,
}

// New function to classify individual responses
fn classify_individual_response(content: &str) -> ResponseType {
    let lower_content = content.to_lowercase();

    if lower_content.contains("review skipped") || lower_content.contains("auto reviews are disabled") {
        return ResponseType::SkippedMessage;
    }
    if lower_content.contains("rate limit") || lower_content.contains("over quota") || lower_content.contains("wait") || lower_content.contains("try again later") || lower_content.contains("reset time") {
        return ResponseType::RateLimitMessage;
    }
    if lower_content.contains("what's been accomplished") || lower_content.contains("strategic next steps") || lower_content.contains("immediate action items") || lower_content.contains("review by coderabbit.ai") {
        return ResponseType::ReviewSummary;
    }
    if lower_content.contains("question") || lower_content.contains("clarify") || lower_content.contains("?") {
        return ResponseType::QuestionMessage;
    }

    ResponseType::OtherMessage
}

fn main() {
    let args = Args::parse();

    if let Some(file_path) = args.file {
        match fs::read_to_string(&file_path) {
            Ok(content) => {
                let tokens = extract_tokens(&content);
                println!("Tokens for {}:", file_path.display());
                for token in tokens {
                    println!("  {}", token);
                }
            }
            Err(e) => {
                eprintln!("Error reading file {}: {}", file_path.display(), e);
            }
        }
        return;
    }

    if args.classify_individual_responses {
        let comms_base_dir = PathBuf::from("/data/data/com.termux.nix/files/home/pick-up-nix/source/github/meta-introspector/submodules/analysis_data/comms/git/coderabbitai/");
        let mut classified_responses: HashMap<ResponseType, Vec<String>> = HashMap::new();

        println!("Classifying individual responses from: {}", comms_base_dir.display());

        match fs::read_dir(&comms_base_dir) {
            Ok(crq_dirs) => {
                for crq_dir_entry in crq_dirs {
                    if let Ok(crq_dir_entry) = crq_dir_entry {
                        let crq_path = crq_dir_entry.path();
                        if crq_path.is_dir() {
                            let responses_path = crq_path.join("responses");
                            if let Ok(response_files) = fs::read_dir(&responses_path) {
                                for response_file_entry in response_files {
                                    if let Ok(response_file_entry) = response_file_entry {
                                        let response_path = response_file_entry.path();
                                        if response_path.is_file() && response_path.extension().map_or(false, |ext| ext == "md") {
                                            if let Ok(content) = fs::read_to_string(&response_path) {
                                                let response_type = classify_individual_response(&content);
                                                classified_responses.entry(response_type).or_default().push(response_path.display().to_string());
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Error reading comms base directory {}: {}", comms_base_dir.display(), e);
                return;
            }
        }

        println!("\n--- Individual Response Classification ---");
        for (res_type, paths) in &classified_responses {
            println!("\n{:?}:", res_type);
            for path in paths {
                println!("  - {}", path);
            }
        }

    } else if args.sequential_analysis {
        let comms_base_dir = PathBuf::from("/data/data/com.termux.nix/files/home/pick-up-nix/source/github/meta-introspector/submodules/analysis_data/comms/git/coderabbitai/");
        let mut sequential_word_counts: HashMap<usize, HashMap<String, usize>> = HashMap::new();

        println!("Performing sequential response analysis from: {}", comms_base_dir.display());

        match fs::read_dir(&comms_base_dir) {
            Ok(crq_dirs) => {
                for crq_dir_entry in crq_dirs {
                    if let Ok(crq_dir_entry) = crq_dir_entry {
                        let crq_path = crq_dir_entry.path();
                        if crq_path.is_dir() {
                            let responses_path = crq_path.join("responses");
                            if let Ok(response_files) = fs::read_dir(&responses_path) {
                                for response_file_entry in response_files {
                                    if let Ok(response_file_entry) = response_file_entry {
                                        let response_path = response_file_entry.path();
                                        if response_path.is_file() && response_path.extension().map_or(false, |ext| ext == "md") {
                                            if let Some(file_stem) = response_path.file_stem().and_then(|s| s.to_str()) {
                                                if let Some(seq_num_str) = file_stem.split('_').next() {
                                                    if let Ok(seq_num) = seq_num_str.parse::<usize>() {
                                                        if let Ok(content) = fs::read_to_string(&response_path) {
                                                            let tokens = extract_tokens(&content);
                                                            let entry_counts = sequential_word_counts.entry(seq_num).or_insert_with(HashMap::new);
                                                            count_words(&tokens, entry_counts);
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Error reading comms base directory {}: {}", comms_base_dir.display(), e);
                return;
            }
        }

        let mut sorted_seq_nums: Vec<usize> = sequential_word_counts.keys().cloned().collect();
        sorted_seq_nums.sort();

        for seq_num in sorted_seq_nums {
            if let Some(word_counts) = sequential_word_counts.get(&seq_num) {
                print_top_words(&format!("Response Sequence #{}", seq_num), word_counts, 20);
            }
        }

    } else {
        let mut all_crq_word_counts: HashMap<String, usize> = HashMap::new();
        let mut categorized_crq_word_counts: HashMap<NextStep, HashMap<String, usize>> = HashMap::new();

        println!("Scanning CRQ directory: {}", args.crq_dir.display());

        match fs::read_dir(&args.crq_dir) {
            Ok(entries) => {
                for entry in entries {
                    if let Ok(entry) = entry {
                        let path = entry.path();
                        if path.is_file() && path.extension().map_or(false, |ext| ext == "md") {
                            if let Ok(content) = fs::read_to_string(&path) {
                                // Extract CRQ ID from filename
                                let crq_id = path.file_stem()
                                    .and_then(|s| s.to_str())
                                    .and_then(|s| {
                                        let parts: Vec<&str> = s.splitn(3, '-').collect();
                                        if parts.len() >= 2 && parts[0] == "CRQ" {
                                            Some(format!("{}-{}", parts[0], parts[1]))
                                        } else {
                                            None
                                        }
                                    })
                                    .unwrap_or("UNKNOWN".to_string());

                                let crq_content_for_analysis = content.clone(); // Clone content for analysis
                                let next_step = determine_next_step(&content, &crq_id);

                                let tokens = extract_tokens(&crq_content_for_analysis);
                                count_words(&tokens, &mut all_crq_word_counts);

                                let entry_counts = categorized_crq_word_counts.entry(next_step).or_insert_with(HashMap::new);
                                count_words(&tokens, entry_counts);

                            } else {
                                eprintln!("Warning: Could not read file: {}", path.display());
                            }
                        } else {
                            eprintln!("Warning: Could not read file: {}", path.is_file());
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Error reading CRQ directory {}: {}", args.crq_dir.display(), e);
                return;
            }
        }

        print_top_words("All CRQs", &all_crq_word_counts, 20);

        for (step, word_counts) in &categorized_crq_word_counts {
            let title = match step {
                NextStep::Develop => "Develop/Implement CRQs",
                NextStep::Refactor => "Refactor CRQs",
                NextStep::Document => "Document CRQs",
                NextStep::RespondToHuman => "Respond To / Our Turn CRQs",
                NextStep::ReviewProvided => "Review Provided CRQs",
                NextStep::ReviewSkipped => "Review Skipped CRQs (No Meaningful Response)",
                NextStep::ReviewNeededFromCoderabbitAI => "Review Needed from CoderabbitAI CRQs",
                NextStep::IssueTooLarge => "Issue Too Large CRQs",
                NextStep::OverQuota => "Over Quota CRQs",
                NextStep::Unknown => "Unknown CRQs",
            };
            print_top_words(title, word_counts, 20);
        }
    }
}