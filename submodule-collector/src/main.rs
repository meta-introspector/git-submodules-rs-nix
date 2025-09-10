use clap::Parser;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs;
use git2::Repository;
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Root directory to scan for Git repositories
    #[arg(long)]
    root_dir: PathBuf,
    /// Output file for the JSON report
    #[arg(long)]
    output_file: PathBuf,
}

#[derive(Serialize, Deserialize, Debug)]
struct SubmoduleInfo {
    name: String,
    path: String,
    url: String,
    branch: Option<String>,
    // Add more fields as needed from .git/config and .gitmodules
}

#[derive(Serialize, Deserialize, Debug)]
struct RepoInfo {
    path: PathBuf,
    url: String,
    submodules: Vec<SubmoduleInfo>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Report {
    repositories: HashMap<String, RepoInfo>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let mut report = Report {
        repositories: HashMap::new(),
    };

    // 1. Find all Git repositories under the root_dir
    let git_repos = find_git_repositories(&args.root_dir)?;

    for repo_path in git_repos {
        if let Some(repo_info) = process_repository(&repo_path)? {
            report.repositories.insert(repo_info.url.clone(), repo_info);
        }
    }

    // 2. Write the aggregated JSON report
    let json_report = serde_json::to_string_pretty(&report)?;
    fs::write(&args.output_file, json_report)?;

    Ok(())
}

fn find_git_repositories(root_dir: &Path) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let mut repos = Vec::new();
    for entry in WalkDir::new(root_dir)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.file_type().is_dir() {
            let path = entry.path();
            if path.join(".git").exists() || path.join(".git").is_file() {
                // Check if it's a GitHub repository
                if let Ok(repo) = Repository::open(path) {
                    if let Ok(remotes) = repo.remotes() {
                        for remote_name in remotes.iter().filter_map(|s| s) {
                            if let Some(url) = repo.find_remote(remote_name)?.url() {
                                if url.contains("github.com") {
                                    repos.push(path.to_path_buf());
                                    break; // Found a GitHub remote, no need to check others for this repo
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(repos)
}

fn process_repository(repo_path: &Path) -> Result<Option<RepoInfo>, Box<dyn std::error::Error>> {
    let repo = Repository::open(repo_path)?;
    let mut repo_url = String::new();
    if let Ok(remotes) = repo.remotes() {
        for remote_name in remotes.iter().filter_map(|s| s) {
            if let Some(url) = repo.find_remote(remote_name)?.url() {
                if url.contains("github.com") {
                    repo_url = url.to_string();
                    break;
                }
            }
        }
    }

    if repo_url.is_empty() {
        return Ok(None); // Not a GitHub repo or no remote found
    }

    let mut submodules_info = Vec::new();
    if let Ok(submodules) = repo.submodules() {
        for submodule in submodules {
            let name = submodule.name().unwrap_or("").to_string();
            let path = submodule.path().to_string_lossy().to_string();
            let url = submodule.url().unwrap_or("").to_string();
            let branch = submodule.branch().map(|s| s.to_string());

            submodules_info.push(SubmoduleInfo {
                name,
                path,
                url,
                branch,
            });
        }
    }

    Ok(Some(RepoInfo {
        path: repo_path.to_path_buf(),
        url: repo_url,
        submodules: submodules_info,
    }))
}
