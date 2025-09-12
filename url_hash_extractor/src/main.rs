
use regex::Regex;
use std::fs::File;
use std::io::{self, BufReader, BufRead};

fn main() -> io::Result<()> {
    let file_path = "../extracted_urls.txt"; // Path to your URL list file
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    // Regex to find 40-character hexadecimal strings (git hashes)
    let re = Regex::new(r"[0-9a-fA-F]{40}").unwrap();

    println!("Extracting 40-character hashes from {}:", file_path);

    for line_result in reader.lines() {
        let line = line_result?;
        for mat in re.find_iter(&line) {
            println!("{}", mat.as_str());
        }
    }

    Ok(())
}
