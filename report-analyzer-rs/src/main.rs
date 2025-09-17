mod types;
mod input;
mod analyze_strings;
mod apply_emojis;
mod nix_analyzer;

use types::{Args, Report, Ontology};
use std::fs;
use clap::Parser;

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

    let suggested_rules = analyze_strings::analyze_strings(&report, &ontology)?;
    analyze_strings::print_suggested_rules_with_emojis(&suggested_rules, &ontology);

    println!("\n--- Nix Analysis ---");
    let nix_file_path = "../../flake.nix";
    match nix_analyzer::analyze_nix_file(nix_file_path) {
        Ok(metrics) => {
            println!("Nix file metrics for {}: {:#?}", nix_file_path, metrics);
        }
        Err(e) => {
            eprintln!("Error analyzing Nix file {}: {}", nix_file_path, e);
        }
    }

    Ok(())
}
