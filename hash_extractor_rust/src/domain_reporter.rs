use std::collections::HashMap;
use std::io::{self, BufRead, Write};
use regex::Regex;
use serde::Serialize;
use serde_json::json;
use url::Url; // Added import

#[derive(Debug, Serialize)]
struct DomainCount {
    domain: String,
    count: usize,
}

pub fn generate_domain_report(reader: &mut impl BufRead) -> io::Result<()> {
    let mut domain_counts: HashMap<String, usize> = HashMap::new();

    for line_result in reader.lines() {
        let line = line_result?;
        if let Ok(parsed_url) = Url::parse(&line) {
            if let Some(host) = parsed_url.host_str() {
                // Simple heuristic to get base domain: take last two segments
                let domain_parts: Vec<&str> = host.split('.').collect();
                let domain = if domain_parts.len() >= 2 {
                    domain_parts[domain_parts.len() - 2..].join(".")
                } else {
                    host.to_string()
                };
                *domain_counts.entry(domain).or_insert(0) += 1;
            }
        }
    }

    let mut report_list: Vec<DomainCount> = Vec::new();
    for (domain, count) in domain_counts {
        report_list.push(DomainCount { domain, count });
    }

    let output_file_path = "domain_report.json";
    let output_file = std::fs::File::create(output_file_path)?;
    let mut writer = io::BufWriter::new(output_file);

    serde_json::to_writer_pretty(&mut writer, &report_list)?;
    writeln!(writer, "")?;

    Ok(())
}