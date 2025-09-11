mod types;
mod input;
mod processing;
use types::{Args, Report, Ontology};
use input::{parse_args, load_data};
use processing::{apply_emoji_ontology, analyze_strings, perform_lcp_analysis, print_lcp_analysis};
use std::fs;
use regex::Regex;
use std::collections::HashMap;





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

    let lcp = processing::perform_lcp_analysis(&report);
    processing::print_lcp_analysis(&lcp);

    let repo_urls = processing::analyze_duplicate_urls(&report);
    processing::print_duplicate_urls(&repo_urls);

    let org_counts = processing::analyze_organizations(&report);
    processing::print_organizations_analysis(&org_counts, &ontology);

    let name_counts = processing::analyze_names(&report);
    processing::print_names_analysis(&name_counts, &ontology);

    let suggested_rules = processing::analyze_strings(&report, &ontology)?;
    processing::print_suggested_rules(&suggested_rules);

    Ok(())
}