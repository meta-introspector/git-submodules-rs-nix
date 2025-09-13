use walkdir::WalkDir;
use regex::Regex;
use std::fs;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let url_regex = Regex::new(r"https?://[^\s/$.?#].[^\s]*").unwrap();

    for entry in WalkDir::new(".").into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let path = entry.path();
            if let Ok(content) = fs::read_to_string(path) {
                for mat in url_regex.find_iter(&content) {
                    writeln!(io::stdout(), "{}", mat.as_str())?;
                }
            }
        }
    }
    Ok(())
}