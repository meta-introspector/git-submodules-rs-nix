use clap::Parser;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::{self, Read};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the .git/config file
    #[arg(long)]
    git_config: Option<PathBuf>,

    /// Path to the .gitmodules file
    #[arg(long)]
    git_modules: Option<PathBuf>,
}

#[derive(Serialize, Deserialize, Debug)]
struct GitConfig {
    #[serde(default)]
    sections: HashMap<String, HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Debug)]
struct GitModules {
    #[serde(default)]
    submodule: HashMap<String, SubmoduleConfig>,
}

#[derive(Serialize, Deserialize, Debug)]
struct SubmoduleConfig {
    path: String,
    url: String,
    #[serde(default)]
    branch: Option<String>,
}

fn parse_git_config(content: &str) -> GitConfig {
    let mut config = GitConfig {
        sections: HashMap::new(),
    };
    let mut current_section: Option<String> = None;

    for line in content.lines() {
        let line = line.trim();
        if line.starts_with('[') && line.ends_with(']') {
            let section_name = line[1..line.len() - 1].to_string();
            current_section = Some(section_name.clone());
            config.sections.insert(section_name, HashMap::new());
        } else if let Some(section) = &current_section {
            if let Some(eq_pos) = line.find('=') {
                let key = line[..eq_pos].trim().to_string();
                let value = line[eq_pos + 1..].trim().to_string();
                if let Some(section_map) = config.sections.get_mut(section) {
                    section_map.insert(key, value);
                }
            }
        }
    }
    config
}

fn parse_git_modules(content: &str) -> GitModules {
    let mut modules = GitModules {
        submodule: HashMap::new(),
    };
    let mut current_submodule_name: Option<String> = None;

    for line in content.lines() {
        let line = line.trim();
        if line.starts_with('[') && line.ends_with(']') {
            let section_name = line[1..line.len() - 1].to_string();
            if section_name.starts_with("submodule \"") && section_name.ends_with('"') {
                let submodule_name = section_name["submodule \"".len()..section_name.len() - 1].to_string();
                current_submodule_name = Some(submodule_name.clone());
                modules.submodule.insert(
                    submodule_name,
                    SubmoduleConfig {
                        path: String::new(),
                        url: String::new(),
                        branch: None,
                    },
                );
            } else {
                current_submodule_name = None; // Not a submodule section
            }
        } else if let Some(submodule_name) = &current_submodule_name {
            if let Some(submodule_config) = modules.submodule.get_mut(submodule_name) {
                if let Some(eq_pos) = line.find('=') {
                    let key = line[..eq_pos].trim();
                    let value = line[eq_pos + 1..].trim().to_string();
                    match key {
                        "path" => submodule_config.path = value,
                        "url" => submodule_config.url = value,
                        "branch" => submodule_config.branch = Some(value),
                        _ => {} // Ignore other keys for now
                    }
                }
            }
        }
    }
    modules
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let mut output = HashMap::new();

    if let Some(git_config_path) = args.git_config {
        let content = fs::read_to_string(&git_config_path)?;
        let parsed_config = parse_git_config(&content);
        output.insert("git_config".to_string(), serde_json::to_value(parsed_config)?);
    }

    if let Some(git_modules_path) = args.git_modules {
        let content = fs::read_to_string(&git_modules_path)?;
        let parsed_modules = parse_git_modules(&content);
        output.insert("git_modules".to_string(), serde_json::to_value(parsed_modules)?);
    }

    let json_output = serde_json::to_string_pretty(&output)?;
    println!("{}", json_output);

    Ok(())
}
