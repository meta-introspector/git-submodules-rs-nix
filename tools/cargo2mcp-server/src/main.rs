use tokio::fs;
use walkdir::WalkDir;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use toml::Value;

#[derive(Debug, Serialize, Deserialize)]
struct CrateInfo {
    name: String,
    version: String,
    path: PathBuf,
    // Add more fields as needed
}

async fn discover_crates(root_dir: &PathBuf) -> Vec<CrateInfo> {
    let mut crates = Vec::new();

    for entry in WalkDir::new(root_dir).into_iter().filter_map(|e| e.ok()) {
        if entry.file_name() == "Cargo.toml" {
            let path = entry.path().to_path_buf();
            if let Ok(content) = fs::read_to_string(&path).await {
                if let Ok(value) = content.parse::<Value>() {
                    if let Some(package) = value.get("package") {
                        if let (Some(name), Some(version)) = (package.get("name"), package.get("version")) {
                            if let (Some(name_str), Some(version_str)) = (name.as_str(), version.as_str()) {
                                crates.push(CrateInfo {
                                    name: name_str.to_string(),
                                    version: version_str.to_string(),
                                    path: path.parent().unwrap().to_path_buf(),
                                });
                            }
                        }
                    }
                }
            }
        }
    }
    crates
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting cargo2mcp server...");

    let root_dir = PathBuf::from("./sources/meta-introspector/"); // Adjust as needed
    let crates = discover_crates(&root_dir).await;

    println!("Discovered crates:");
    for c in crates {
        println!("  - {} (v{}) at {:?}", c.name, c.version, c.path);
    }

    // TODO: Implement MCP server logic here
    // This will involve using rust-mcp-sdk to create an MCP server
    // and expose methods to interact with the discovered crates.

    Ok(())
}