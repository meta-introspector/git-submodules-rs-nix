use anyhow::{Result, Context};
use git2::Repository;
use std::path::PathBuf;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The command to execute within each submodule
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    command: Vec<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let repo_path = PathBuf::from("."); // Current directory is the main repository
    let repo = Repository::open(&repo_path)
        .context(format!("Failed to open repository at: {}", repo_path.display()))?;

    println!("Scanning for submodules...");

    let submodules = repo.submodules().context("Failed to get submodules")?;

    if submodules.is_empty() {
        println!("No submodules found.");
        return Ok(())
    }

    for mut submodule in submodules {
        let path = submodule.path().to_path_buf();
        let name = submodule.name().unwrap_or("unknown");
        println!("Found submodule: {} at path: {}", name, path.display());

        // Execute command within the submodule
        if !args.command.is_empty() {
            println!("Executing command in submodule {}: {:?}", name, args.command);

            let mut command_exec = std::process::Command::new(&args.command[0]);
            command_exec.args(&args.command[1..]);
            command_exec.current_dir(&path);

            let output = command_exec.output()
                .context(format!("Failed to execute command in submodule {}", name))?;

            if output.status.success() {
                println!("Stdout:\n{}", String::from_utf8_lossy(&output.stdout));
            } else {
                eprintln!("Command failed in submodule {}:", name);
                eprintln!("Stderr:\n{}", String::from_utf8_lossy(&output.stderr));
            }
        }
    }

    Ok(())
}