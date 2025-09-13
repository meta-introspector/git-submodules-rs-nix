use std::{
    collections::{HashMap, HashSet},
    fs,
    path::{Path, PathBuf},
};

use regex::{Regex, RegexBuilder};

// Define a struct to represent a Mermaid node
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Node {
    id: String,
    label: String,
}

// Define a struct to represent a Mermaid edge
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Edge {
    source: String,
    target: String,
    label: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base_path = PathBuf::from("/data/data/com.termux.nix/files/home/pick-up-nix/source/github/meta-introspector/submodules/docs/memes");
    let mut all_mermaid_diagrams = String::new();

    // Collect all markdown files
    let markdown_files = find_markdown_files(&base_path)?;

    println!("Found {} markdown files.", markdown_files.len());

    for file_path in markdown_files {
        let content = fs::read_to_string(&file_path)?;
        let mermaid_blocks = extract_mermaid_blocks(&content);
        if !mermaid_blocks.is_empty() {
            println!("Extracted {} mermaid blocks from {:?}", mermaid_blocks.len(), file_path);
        }
        for block in mermaid_blocks {
            if block.contains("graph TD") {
                all_mermaid_diagrams.push_str(&block);
                all_mermaid_diagrams.push_str("\n"); // Add a newline for separation
            }
        }
    }

    println!("Total mermaid diagrams content length: {}", all_mermaid_diagrams.len());

    let combined_diagram = combine_mermaid_diagrams(&all_mermaid_diagrams);

    println!("`mermaid\n{}\n`", combined_diagram);

    Ok(())
}

fn find_markdown_files(dir: &Path) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let mut markdown_files = Vec::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            markdown_files.extend(find_markdown_files(&path)?);
        } else if path.extension().map_or(false, |ext| ext == "md") {
            markdown_files.push(path);
        }
    }
    Ok(markdown_files)
}

fn extract_mermaid_blocks(content: &str) -> Vec<String> {
    let re = RegexBuilder::new(r"```mermaid\n(.*?)```")
        .dot_matches_new_line(true) // Enable DOTALL mode
        .build()
        .unwrap();
    re.captures_iter(content)
        .filter_map(|cap| cap.get(1).map(|m| m.as_str().to_string()))
        .collect()
}

fn combine_mermaid_diagrams(all_diagrams: &str) -> String {
    let mut nodes: HashMap<String, String> = HashMap::new(); // id -> label
    let mut edges: HashSet<(String, String, Option<String>)> = HashSet::new(); // (source, target, label)

    let node_re = Regex::new(r"([A-Za-z0-9]+)\(([^)]*)\)").unwrap();
    let edge_re = Regex::new(r"([A-Za-z0-9]+)\s*-->\s*(?:\|([^|]*?)\|\s*)?([A-Za-z0-9]+)").unwrap();

    for line in all_diagrams.lines() {
        // Extract nodes
        for cap in node_re.captures_iter(line) {
            let id = cap[1].to_string();
            let label = cap[2].to_string();
            nodes.entry(id).or_insert(label);
        }

        // Extract edges
        for cap in edge_re.captures_iter(line) {
            let source = cap[1].to_string();
            let target = cap[3].to_string();
            let label = cap.get(2).map(|m| m.as_str().to_string());
            edges.insert((source, target, label));
        }
    }

    let mut combined_graph = String::from("graph TD\n");

    // Add nodes to the combined graph
    for (id, label) in nodes.iter() {
        combined_graph.push_str(&format!("    {}(\"{}\")\n", id, label));
    }

    // Add edges to the combined graph
    for (source, target, label) in edges.iter() {
        if let Some(l) = label {
            combined_graph.push_str(&format!("    {} -- \"{}\" --> {}\n", source, l, target));
        } else {
            combined_graph.push_str(&format!("    {} --> {}\n", source, target));
        }
    }

    combined_graph
}
