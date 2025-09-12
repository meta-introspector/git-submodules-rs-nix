use clap::Parser;
use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;
use submodules::lattice_model::PredicateClassifier;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path to the project root directory (defaults to current directory)
    #[clap(short, long, value_parser, default_value = ".")]
    path: PathBuf,
}

fn main() -> Result<(), String> {
    let args = Args::parse();
    let project_root = args.path;

    let crq_dir = project_root.join("docs").join("crq_standardized");
    let sop_dir = project_root.join("docs").join("sops");

    let global_predicates = vec![
        "rust", "python", "javascript", "java", "c", "cpp",
        "toml", "md", "tex", "rs", "py", "js", "java",
        "config", "test", "doc", "example", "src", "bin", "lib",
        "lattice", "meme", "vibe", "math", "llm", "compiler", "submodule"
    ];

    println!("--- Bag of Words Report ---");

    // Process CRQ documents
    println!("\n--- CRQ Documents ---");
    for entry in WalkDir::new(&crq_dir).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() && entry.path().extension().map_or(false, |ext| ext == "md") {
            let file_path = entry.path();
            let content = fs::read_to_string(file_path)
                .map_err(|e| format!("Failed to read file {}: {}", file_path.display(), e))?;

            let classifier = PredicateClassifier::new(global_predicates.iter().map(|&s| s).collect());
            let predicates_bool = classifier.extract_word_predicates(&content);

            let found_words: Vec<String> = classifier.get_target_predicates().iter()
                .zip(predicates_bool.into_iter())
                .filter_map(|(word, wp)| {
                    if wp.0 {
                        Some(word.clone())
                    } else {
                        None
                    }
                })
                .collect();

            println!("\nFile: {}", file_path.display());
            println!("Words: {:?}", found_words);
        }
    }

    // Process SOP documents
    println!("\n--- SOP Documents ---");
    for entry in WalkDir::new(&sop_dir).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() && entry.path().extension().map_or(false, |ext| ext == "md") {
            let file_path = entry.path();
            let content = fs::read_to_string(file_path)
                .map_err(|e| format!("Failed to read file {}: {}", file_path.display(), e))?;

            let classifier = PredicateClassifier::new(global_predicates.iter().map(|&s| s).collect());
            let predicates_bool = classifier.extract_word_predicates(&content);

            let found_words: Vec<String> = classifier.get_target_predicates().iter()
                .zip(predicates_bool.into_iter())
                .filter_map(|(word, wp)| {
                    if wp.0 {
                        Some(word.clone())
                    } else {
                        None
                    }
                })
                .collect();

            println!("\nFile: {}", file_path.display());
            println!("Words: {:?}", found_words);
        }
    }

    Ok(())
}