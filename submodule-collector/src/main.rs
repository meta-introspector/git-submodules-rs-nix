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
    path: String, // Relative path from the parent repository
    url: String,
    branch: Option<String>,
    // Nested RepoInfo for recursive submodules
    #[serde(skip_serializing_if = "Option::is_none")]
    nested_repo: Option<RepoInfo>,
}

#[derive(Serialize, Deserialize, Debug)]
struct RepoInfo {
    path: PathBuf, // Absolute path to the repository on disk
    url: String,
    submodules: Vec<SubmoduleInfo>,
}

#[derive(Serialize, Deserialize, Debug)]
struct FailedRepoInfo {
    path: PathBuf,
    error: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Report {
    repositories: HashMap<String, RepoInfo>,
    failed_repositories: Vec<FailedRepoInfo>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let mut report = Report {
        repositories: HashMap::new(),
        failed_repositories: Vec::new(),
    };

    // 1. Find all Git repositories under the root_dir
    let git_repos = find_git_repositories(&args.root_dir)?;

    for repo_path in git_repos {
        // Process each found repository, including its submodules recursively
        match process_repository(&repo_path) {
            Ok((Some(repo_info), nested_failed_submodules)) => {
                report.repositories.insert(repo_info.url.clone(), repo_info);
                report.failed_repositories.extend(nested_failed_submodules);
            }
            Ok((None, nested_failed_submodules)) => {
                report.failed_repositories.extend(nested_failed_submodules);
                // Repository skipped (e.g., no remote URL or other non-error skip)
            }
            Err(e) => {
                report.failed_repositories.push(FailedRepoInfo {
                    path: repo_path.clone(), // Clone to own the PathBuf
                    error: e.to_string(),
                });
                eprintln!("Error processing repository {}: {}", repo_path.display(), e); // Keep eprintln for immediate feedback
            }
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
        .filter(|e| e.file_type().is_dir())
        .filter(|e| {
            let path = e.path();
            // If the directory is named ".git", skip it.
            // This prevents traversing into .git/objects, .git/refs, etc.
            if path.file_name().map_or(false, |name| name == ".git") {
                return false;
            }
            true
        })
    {
        let path = entry.path();
        let git_path = path.join(".git");

        if git_path.exists() {
            let repo_to_open_path = if git_path.is_file() {
                // Read the gitdir file to get the actual .git directory path
                let content = fs::read_to_string(&git_path)?;
                if let Some(line) = content.lines().next() {
                    if line.starts_with("gitdir: ") {
                        let relative_git_dir = PathBuf::from(line["gitdir: ".len()..].trim());
                        // Resolve relative to the parent of the .git file (which is `path`)
                        // and then canonicalize to get the absolute path
                        path.join(relative_git_dir).canonicalize()? // Canonicalize here
                    } else {
                        // Not a gitdir file, treat as a regular repository working directory
                        path.to_path_buf()
                    }
                } else {
                    path.to_path_buf()
                }
            } else {
                // It's a .git directory, so the path itself is the working directory
                path.to_path_buf()
            };

            // Attempt to open the repository using the resolved path
            match Repository::open(&repo_to_open_path) {
                Ok(_) => {
                    repos.push(path.to_path_buf()); // Push the working directory path
                }
                Err(e) => {
                    eprintln!("Warning: Could not open Git repository at {}: {}", path.display(), e);
                }
            }
        }
    }
    Ok(repos)
}

fn process_repository(repo_path: &Path) -> Result<(Option<RepoInfo>, Vec<FailedRepoInfo>), Box<dyn std::error::Error>> {
    let mut failed_submodules_collected = Vec::new();
    let repo = match Repository::open(repo_path) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("DEBUG: Repository::open failed for {}: {}", repo_path.display(), e);
            return Err(format!("Error opening repository {}: {}", repo_path.display(), e).into());
        }
    };

    let mut repo_url = String::new();
    if let Ok(remotes) = repo.remotes() {
        if let Some(remote_name) = remotes.iter().filter_map(|s| s).next() { // Get the first remote
            if let Some(url) = repo.find_remote(remote_name)?.url() {
                repo_url = url.to_string();
            }
        }
    }

    if repo_url.is_empty() {
        eprintln!("Warning: No remote URL found for repository {}", repo_path.display());
        return Ok((None, Vec::new()));
    }

    let mut submodules_info = Vec::new();
    if let Ok(submodules) = repo.submodules() {
        for mut submodule in submodules { // `mut` is needed to open the submodule
            let name = submodule.name().unwrap_or("").to_string();
            let relative_path = submodule.path().to_string_lossy().to_string();
            let url = submodule.url().unwrap_or("").to_string();
            let branch = submodule.branch().map(|s| s.to_string());

            let absolute_submodule_path = repo_path.join(&relative_path);
            let mut nested_repo_info: Option<RepoInfo> = None;

            // Recursively process the submodule if it's a valid repository
            // and it's not the current repository (to prevent infinite loops in case of misconfigured submodules)
            if absolute_submodule_path.exists() && absolute_submodule_path.is_dir() && absolute_submodule_path != repo_path {
                match process_repository(&absolute_submodule_path) {
                    Ok((Some(info), nested_failed_submodules)) => {
                        nested_repo_info = Some(info);
                        failed_submodules_collected.extend(nested_failed_submodules);
                    }
                    Ok((None, nested_failed_submodules)) => {
                        failed_submodules_collected.extend(nested_failed_submodules);
                        // Submodule skipped (e.g., no remote URL)
                    }
                    Err(e) => {
                        eprintln!("DEBUG: Nested process_repository failed for {}: {}", absolute_submodule_path.display(), e);
                        failed_submodules_collected.push(FailedRepoInfo {
                            path: absolute_submodule_path.clone(),
                            error: e.to_string(),
                        });
                        eprintln!("Error processing submodule {}: {}", absolute_submodule_path.display(), e);
                    }
                }
            }

            submodules_info.push(SubmoduleInfo {
                name,
                path: relative_path, // Store relative path here
                url,
                branch,
                nested_repo: nested_repo_info,
            });
        }
    }

        Ok((Some(RepoInfo {
        path: repo_path.to_path_buf(), // Store absolute path here
        url: repo_url,
        submodules: submodules_info,
    }), failed_submodules_collected))
}