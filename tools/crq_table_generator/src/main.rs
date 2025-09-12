use clap::Parser;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use regex::Regex;
use std::io::Write;
use std::process::Command;

mod crq_parser;
use crq_parser::{determine_next_step, parse_crq, check_coderabbitai_comms, NextStep, CommsAnalysisResult};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the directory containing CRQ files
    #[arg(short, long, default_value = "docs/crq_standardized")]
    crq_dir: PathBuf,

    /// Path to the task.md file for PR number lookup
    #[arg(long, default_value = "task.md")]
    task_file: PathBuf,

    /// Generate gh pr comment commands instead of a detailed report
    #[arg(long)]
    generate_commands: bool,

    /// Output the generated commands to a script file and make it executable
    #[arg(long)]
    output_script: Option<PathBuf>,
}

struct CrqReportEntry {
    title: String,
    next_step: NextStep,
    comms_analysis: CommsAnalysisResult,
}

fn main() {
    let args = Args::parse();

    let mut crq_reports: HashMap<String, CrqReportEntry> = HashMap::new();
    let mut crq_to_pr_map: HashMap<String, String> = HashMap::new();

    // Read task.md to build CRQ to PR number map
    match fs::read_to_string(&args.task_file) {
        Ok(content) => {
            for line in content.lines() {
                if let Some(captures) = Regex::new(r"^\*\s+(\d+):\s+(CRQ-\d+):").unwrap().captures(line) { // Simplified regex
                    let pr_number = captures.get(1).unwrap().as_str().to_string();
                    let crq_id = captures.get(2).unwrap().as_str().to_string();
                    crq_to_pr_map.insert(crq_id.clone(), pr_number);
                }
            }
        }
        Err(e) => {
            eprintln!("Error reading task.md file {}: {}", args.task_file.display(), e);
            return;
        }
    }

    println!("Scanning CRQ directory: {}", args.crq_dir.display());

    match fs::read_dir(&args.crq_dir) {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_file() && path.extension().map_or(false, |ext| ext == "md") {
                        if let Ok(content) = fs::read_to_string(&path) {
                            // Extract CRQ ID from filename (e.g., "CRQ-021" from "CRQ-021-add-crq-pr-script.md")
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

                            if let Some(crq) = parse_crq(&content) {
                                let comms_analysis = check_coderabbitai_comms(&crq_id);
                                let next_step = determine_next_step(&content, &crq_id);

                                crq_reports.insert(
                                    crq_id.clone(),
                                    CrqReportEntry {
                                        title: crq.problem_goal.lines().next().unwrap_or("").to_string(),
                                        next_step,
                                        comms_analysis,
                                    },
                                );
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

    if args.generate_commands {
        let mut commands_to_write = Vec::new();
        commands_to_write.push("#!/usr/bin/env bash".to_string());

        for crq_id in crq_reports.keys().cloned().collect::<Vec<String>>() {
            if let Some(entry) = crq_reports.get(&crq_id) {
                if let NextStep::ReviewSkipped = entry.next_step {
                    if let Some(pr_number) = crq_to_pr_map.get(&crq_id) {
                        println!("gh pr comment {} -b \"@coderabbitai review\"", pr_number);
                    } else {
                        eprintln!("Warning: PR number not found for CRQ: {}", crq_id);
                    }
                }
            }
        }

        if let Some(output_path) = args.output_script {
            match fs::File::create(&output_path) {
                Ok(mut file) => {
                    for cmd in commands_to_write {
                        writeln!(file, "{}", cmd).expect("Could not write command to file");
                    }
                    // Make the script executable
                    let output = Command::new("chmod")
                        .arg("+x")
                        .arg(&output_path)
                        .output()
                        .expect("Failed to execute chmod");

                    if output.status.success() {
                        println!("Commands written to {} and made executable.", output_path.display());
                    } else {
                        eprintln!("Error making script executable: {}", String::from_utf8_lossy(&output.stderr));
                    }
                }
                Err(e) => {
                    eprintln!("Error creating output script file {}: {}", output_path.display(), e);
                }
            }
        } else {
            // Print to stdout if no output_script is specified
            for cmd in commands_to_write {
                println!("{}", cmd);
            }
        }
    } else {
        println!("\n--- CRQ Detailed Report ---");
        println!("{:<15} {:<20} {:<15} {:<20} {:<40}", "CRQ Number", "Count Responses", "Total Size (bytes)", "Skipped Review", "Suggested Next Step");
        println!("{:-<15} {:-<20} {:-<15} {:-<20} {:-<40}", "", "", "", "", "");

        let mut sorted_crq_ids: Vec<String> = crq_reports.keys().cloned().collect();
        sorted_crq_ids.sort();

        for crq_id in sorted_crq_ids {
            if let Some(entry) = crq_reports.get(&crq_id) {
                let next_step_str = match entry.next_step {
                    NextStep::Develop => "Develop/Implement",
                    NextStep::Refactor => "Refactor",
                    NextStep::Document => "Document",
                    NextStep::RespondToHuman => "Respond To / Our Turn",
                    NextStep::ReviewProvided => "Review Provided",
                    NextStep::ReviewSkipped => "Review Skipped (No Meaningful Response)",
                    NextStep::ReviewNeededFromCoderabbitAI => "Review Needed from CoderabbitAI",
                    NextStep::IssueTooLarge => "Issue Too Large", // New case
                    NextStep::OverQuota => "Over Quota", // New case
                    NextStep::Unknown => "Unknown",
                };
                println!("{:<15} {:<20} {:<15} {:<20} {:<40}",
                         crq_id,
                         entry.comms_analysis.response_count,
                         entry.comms_analysis.total_size,
                         if entry.comms_analysis.skipped_review_present { "Yes" } else { "No" },
                         next_step_str
                );
            }
        }
    }
}
