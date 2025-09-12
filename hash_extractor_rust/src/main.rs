use regex::Regex;
use std::fs::File;
use std::io::{self, BufReader, BufRead, Write};
use std::collections::HashSet;

mod github_counter;
mod domain_reporter;
mod file_type_reporter;

fn main() -> io::Result<()> {
    let file_path = "extracted_urls.txt"; // Path to your URL list file
    let input_file = File::open(file_path)?;
    let reader = BufReader::new(input_file);

    let output_file_path = "unique_urls_report.txt"; // Output file name
    let output_file = File::create(output_file_path)?;
    let mut writer = io::BufWriter::new(output_file);

    // Regex to find hexadecimal strings between 6 and 40 characters long
    let re = Regex::new(r"[0-9a-fA-F]{6,40}").unwrap();

    let mut unique_urls: HashSet<String> = HashSet::new();
    let mut processed_count = 0;

    writeln!(writer, "Processing URLs, replacing hashes, and printing only unique ones:")?;

    for line_result in reader.lines() {
        let line = line_result?;
        let modified_line = re.replace_all(&line, "[HASH]").to_string();

        // If the modified URL is new, insert it and print it
        if unique_urls.insert(modified_line.clone()) {
            writeln!(writer, "{}", modified_line)?;
        }
        processed_count += 1;
    }

    writeln!(writer, "\nTotal unique URLs (after hash replacement): {}", unique_urls.len())?;
    writeln!(writer, "Total URLs processed (including duplicates): {}", processed_count)?;

    // Call the GitHub repo counter
    run_github_repo_counter()?;

    // Call the Domain Report generator
    run_domain_report()?;

    // Call the File Type Report generator
    run_file_type_report()?;

    Ok(())
}

fn run_github_repo_counter() -> io::Result<()> {
    let file_path = "extracted_urls.txt"; // Input file for the counter
    let input_file = File::open(file_path)?;
    let reader = BufReader::new(input_file);

    github_counter::count_github_repos(reader)?;
    Ok(())
}

fn run_domain_report() -> io::Result<()> {
    let file_path = "extracted_urls.txt"; // Input file for the report
    let input_file = File::open(file_path)?;
    let mut reader = BufReader::new(input_file);

    domain_reporter::generate_domain_report(&mut reader)?;
    Ok(())
}

fn run_file_type_report() -> io::Result<()> {
    let file_path = "extracted_urls.txt"; // Input file for the report
    let input_file = File::open(file_path)?;
    let mut reader = BufReader::new(input_file);

    file_type_reporter::generate_file_type_report(&mut reader)?;
    Ok(())
}