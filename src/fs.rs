use std::fs;
use std::path::Path;
use std::process::Command;

pub fn create_project_dir(project_root: &Path) -> anyhow::Result<()> {
    if project_root.exists() {
        if project_root.read_dir()?.next().is_some() {
            anyhow::bail!(
                "Directory '{}' already exists",
                project_root.display()
            );
        }
    }
    fs::create_dir(project_root)?;
    Ok(())
}


pub fn initialize_git(project_root: &Path) -> anyhow::Result<()> {
    println!("Initialize git repo...");

    let status = Command::new("git")
        .arg("init")
        .current_dir(&project_root)
        .status()?;

    if !status.success() {
        anyhow::bail!("Git init failed");
    }

    Ok(())
}
