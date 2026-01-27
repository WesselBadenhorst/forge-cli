use std::path::Path;
use std::process::Command;

pub fn create_frontend(project_root: &Path) -> anyhow::Result<()> {
    println!("⚛️  Creating frontend with Vite...");

    let status = Command::new("npm")
        .args([
            "create",
            "vite@latest",
            "frontend",
            "--",
            "--template",
            "react-ts",
        ])
        .env("CI", "true")
        .env("npm_config_yes", "false")
        .current_dir(project_root)
        .status()?;

    if !status.success() {
        anyhow::bail!("Vite failed");
    }
    
    Ok(())
}

pub fn install_depts(project_root: &Path) -> anyhow::Result<()> {
    println!("📦 Installing dependencies...");
    
    let frontend_dir = project_root.join("frontend");

    let status = Command::new("npm")
        .arg("install")
        .current_dir(&frontend_dir)
        .status()?;

    if !status.success() {
        anyhow::bail!("npm install failed");
    }

    Ok(())
}

