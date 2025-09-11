use clap::Parser;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

mod crq_parser;
use crq_parser::{determine_next_step, parse_crq, NextStep};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the directory containing CRQ files
    #[arg(short, long, default_value = "docs/crq_standardized")]
    crq_dir: PathBuf,
}

fn main() {
    let args = Args::parse();

    let mut grouped_crqs: HashMap<NextStep, Vec<String>> = HashMap::new();

    println!("Scanning CRQ directory: {}", args.crq_dir.display());

    match fs::read_dir(&args.crq_dir) {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_file() && path.extension().map_or(false, |ext| ext == "md") {
                        if let Ok(content) = fs::read_to_string(&path) {
                            if let Some(crq) = parse_crq(&content) {
                                let next_step = determine_next_step(&content);
                                grouped_crqs
                                    .entry(next_step)
                                    .or_default()
                                    .push(crq.problem_goal.lines().next().unwrap_or("").to_string());
                            } else {
                                eprintln!("Warning: Could not parse CRQ from file: {}", path.display());
                            }
                        } else {
                            eprintln!("Warning: Could not read file: {}", path.display());
                        }
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Error reading CRQ directory {}: {}", args.crq_dir.display(), e);
            return;
        }
    }

    println!("\n--- CRQs Grouped by Suggested Next Step ---");
    for (step, crqs) in &grouped_crqs {
        println!("\n{}:", match step {
            NextStep::Develop => "Develop/Implement",
            NextStep::Refactor => "Refactor",
            NextStep::Document => "Document",
            NextStep::RespondTo => "Respond To / Our Turn",
            NextStep::CoderabbitAIReview => "CoderabbitAI Review (Size < 2500 bytes)", // New case
            NextStep::Unknown => "Unknown",
        });
        for crq_title in crqs {
            println!("  - {}", crq_title);
        }
    }
}
