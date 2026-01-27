use std::fs;
use std::path::Path;

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
