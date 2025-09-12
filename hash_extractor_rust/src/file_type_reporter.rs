use std::collections::HashMap;
use std::io::{self, BufRead, Write};
use regex::Regex; // Still needed for regex on path segment
use serde::Serialize;
use serde_json::json;
use url::Url; // Added import

#[derive(Debug, Serialize)]
struct FileTypeCount {
    extension: String,
    count: usize,
}

pub fn generate_file_type_report(reader: &mut impl BufRead) -> io::Result<()> {
    let mut file_type_counts: HashMap<String, usize> = HashMap::new();

    for line_result in reader.lines() {
        let line = line_result?;
        if let Ok(parsed_url) = Url::parse(&line) {
            if let Some(segments) = parsed_url.path_segments() {
                if let Some(last_segment) = segments.last() {
                    // Look for an extension in the last path segment
                    if let Some(dot_index) = last_segment.rfind('.') {
                        let extension = last_segment[dot_index + 1..].to_lowercase();
                        // Basic validation: ensure it's not just a dot or empty
                        if !extension.is_empty() && extension.chars().all(|c| c.is_alphanumeric()) {
                            *file_type_counts.entry(extension).or_insert(0) += 1;
                        }
                    }
                }
            }
        }
    }

    let mut report_list: Vec<FileTypeCount> = Vec::new();
    for (extension, count) in file_type_counts {
        report_list.push(FileTypeCount { extension, count });
    }

    let output_file_path = "file_type_report.json";
    let output_file = std::fs::File::create(output_file_path)?;
    let mut writer = io::BufWriter::new(output_file);

    serde_json::to_writer_pretty(&mut writer, &report_list)?;
    writeln!(writer, "")?;

    Ok(())
}