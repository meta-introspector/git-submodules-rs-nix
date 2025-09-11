use clap::Parser;
use std::fs;
use std::path::PathBuf;

mod crq_parser;
use crq_parser::{determine_next_step, parse_crq, NextStep};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the CRQ file
    #[arg(short, long)]
    file: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();

    let crq_content = if let Some(file_path) = args.file {
        match fs::read_to_string(&file_path) {
            Ok(content) => {
                println!("Reading CRQ from file: {}", file_path.display());
                content
            }
            Err(e) => {
                eprintln!("Error reading file {}: {}", file_path.display(), e);
                return;
            }
        }
    } else {
        println!("No CRQ file specified. Please provide a file using --file <path>.");
        // For now, we'll just exit if no file is provided. 
        // TODO: Implement reading from stdin if no file is provided
        return;
    };

    match parse_crq(&crq_content) {
        Some(crq) => {
            println!("\n-- Parsed CRQ --");
            println!("Problem/Goal: {}
", crq.problem_goal);
            println!("Proposed Solution: {}
", crq.proposed_solution);
            println!("Justification/Impact: {}
", crq.justification_impact);

            let next_step = determine_next_step(&crq_content);
            println!("-- Suggested Next Step --");
            match next_step {
                NextStep::Develop => println!("Action: Develop/Implement new features."),
                NextStep::Refactor => println!("Action: Refactor existing code."),
                NextStep::Document => println!("Action: Create or update documentation/SOPs."),
                NextStep::Unknown => println!("Action: Unable to determine a specific next step. Further analysis may be required."),
            }
        }
        None => {
            eprintln!("Error: Could not parse CRQ content. Please ensure it follows the expected format.");
        }
    }
}
