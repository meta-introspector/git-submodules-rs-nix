use clap::Parser;
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use walkdir::WalkDir;
use submodules::ngram_analyzer::{analyze_hierarchical_ngrams, NGramDecomposition};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path to the project root directory (defaults to current directory)
    #[clap(short, long, value_parser, default_value = ".")]
    path: PathBuf,
}

// Define the categories for counts
const CATEGORIES: &[usize] = &[2, 3, 5, 7, 9, 11, 13, 17, 19];

// Function to get the appropriate category for a given count
fn get_category(count: usize) -> usize {
    for &category in CATEGORIES {
        if count <= category {
            return category;
        }
    }
    // If count is greater than all defined categories, assign to the largest category
    *CATEGORIES.last().unwrap_or(&0)
}

fn main() -> Result<(), String> {
    let args = Args::parse();
    let project_root = args.path;
    let max_n = 19; // As specified by the user (primorial[8])

    println!("--- Hierarchical N-gram Report ---");

    // HashMap to store counts of all unique child n-grams (subexpressions)
    let mut subexpression_counts: HashMap<Vec<String>, usize> = HashMap::new();

    for entry in WalkDir::new(&project_root).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() && entry.path().extension().map_or(false, |ext| ext == "rs") {
            let file_path = entry.path();
            let content = fs::read_to_string(file_path)
                .map_err(|e| format!("Failed to read file {}: {}", file_path.display(), e))?;

            let decompositions = analyze_hierarchical_ngrams(&content, max_n);

            for decomp in decompositions {
                for child_ngram in decomp.child_ngrams {
                    *subexpression_counts.entry(child_ngram).or_insert(0) += 1;
                }
            }
        }
    }

    // HashMap to store categorized output
    let mut categorized_output: HashMap<usize, Vec<String>> = HashMap::new();
    for &category in CATEGORIES {
        categorized_output.insert(category, Vec::new());
    }

    // Populate categorized_output
    for (subexpression, count) in subexpression_counts {
        let category = get_category(count);
        let formatted_subexpression = format!("'{}': {}", subexpression.join(" "), count);
        if let Some(vec) = categorized_output.get_mut(&category) {
            vec.push(formatted_subexpression);
        }
    }

    // Write categorized output to files
    println!("--- Writing Categorized Reports ---");
    for &category in CATEGORIES {
        let file_name = format!("counts_{}.txt", category);
        let mut file = fs::File::create(&file_name)
            .map_err(|e| format!("Failed to create file {}: {}", file_name, e))?;

        if let Some(entries) = categorized_output.get(&category) {
            for entry in entries {
                writeln!(file, "{}", entry)
                    .map_err(|e| format!("Failed to write to file {}: {}", file_name, e))?;
            }
        }
        println!("  Wrote to {}", file_name);
    }

    Ok(())
}
