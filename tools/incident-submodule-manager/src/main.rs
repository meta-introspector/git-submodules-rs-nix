use anyhow::{Result, Context};
use git2::{Repository, Signature};
use std::path::{Path, PathBuf};
use std::fs;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The ID of the incident to create (e.g., CRQ-001)
    #[arg(long)]
    incident_id: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let incident_id = args.incident_id;

    let template_path = PathBuf::from("qa/templates/incident");
    let incident_repo_dir_name = format!("qa/incidents/{}", incident_id);
    let incident_repo_path = PathBuf::from(&incident_repo_dir_name);

    // Ensure the incident directory does not exist
    if incident_repo_path.exists() {
        fs::remove_dir_all(&incident_repo_path)
            .context(format!("Failed to remove existing incident directory: {}", incident_repo_path.display()))?;
    }

    // 1. Create the incident directory
    fs::create_dir_all(&incident_repo_path)
        .context(format!("Failed to create incident directory: {}", incident_repo_path.display()))?;

    // 2. Copy contents from the template submodule
    println!("Copying template files from {} to {}...", template_path.display(), incident_repo_path.display());
    for entry in fs::read_dir(&template_path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            let file_name = path.file_name().context("Invalid file name")?;
            fs::copy(&path, incident_repo_path.join(file_name))
                .context(format!("Failed to copy file from {} to {}", path.display(), incident_repo_path.join(file_name).display()))?;
        }
    }

    // 3. Initialize a new Git repository in the incident directory
    let incident_repo = Repository::init(&incident_repo_path)
        .context(format!("Failed to initialize git repository at: {}", incident_repo_path.display()))?;

    println!("Initialized new git repository at: {}", incident_repo_path.display());

    // 4. Modify issue.md to include incident_id
    let issue_md_path = incident_repo_path.join("issue.md");
    let original_content = fs::read_to_string(&issue_md_path)
        .context(format!("Failed to read issue.md from: {}", issue_md_path.display()))?;
    let modified_content = original_content.replace("# Incident Template", &format!("# Incident Report - {}", incident_id));
    fs::write(&issue_md_path, modified_content)
        .context(format!("Failed to write modified issue.md to: {}", issue_md_path.display()))?;

    println!("Modified issue.md at: {}", issue_md_path.display());

    // 5. Add and commit all files to the new repository
    let mut index = incident_repo.index().context("Failed to get repository index")?;
    index.add_all(["*"].iter(), git2::IndexAddOption::DEFAULT, None)
        .context("Failed to add all files to index")?;
    index.write().context("Failed to write index")?;

    let tree_id = index.write_tree().context("Failed to write tree")?;
    let tree = incident_repo.find_tree(tree_id).context("Failed to find tree")?;

    let signature = Signature::now("Gemini CLI", "gemini@google.com").context("Failed to create signature")?;

    incident_repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
        &format!("Initial incident report for {}", incident_id),
        &tree,
        &[] // No parents for the initial commit
    ).context("Failed to create initial commit")?;

    println!("Committed initial incident report to the new repository.");

    Ok(())
}
