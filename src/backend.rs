use std::path::{Path, PathBuf};
use std::process::Command;
use std::fs;

pub fn create(project_root: &Path) -> anyhow::Result<()> {
    println!("🐍 Creating backend with uv...");

    let backend_dir = project_root.join("backend");
    fs::create_dir(&backend_dir)?;

    let status = Command::new("uv")
        .arg("init")
        .current_dir(&backend_dir)
        .status()?;

    if !status.success() {
        anyhow::bail!("uv init failed");
    }

    Ok(())
}

pub fn setup_django(project_root: &Path) -> anyhow::Result<()> {
    println!("🧱 Setting up Django...");
    let backend_dir = Path::new(project_root).join("backend");

    Command::new("uv")
        .args([
            "add",
            "django",
            "djangorestframework",
            "django-allauth",
            "jwt",
            "python-dotenv",
            "psycopg[binary]",
            "requests",
        ])
        .current_dir(&backend_dir)
        .status()?;

    Command::new("uv")
        .arg("sync")
        .current_dir(&backend_dir)
        .status()?;

    Command::new("uv")
        .args([
            "run",
            "django-admin",
            "startproject",
            "app",
            ".",

        ])
        .current_dir(&backend_dir)
        .status()?;
    
    Ok(())
}

pub fn create_settings_dir(project_root: &Path) -> anyhow::Result<PathBuf> {
    let settings_dir = project_root
        .join("backend")
        .join("app")
        .join("settings");

    fs::create_dir(&settings_dir)?;

    Ok(settings_dir)
}

pub fn move_settings_py(project_root: &Path, settings_dir: &Path) -> anyhow::Result<()> {
    let app_dir = project_root.join("backend").join("app");

    let old = app_dir.join("settings.py");
    let new = settings_dir.join("base.py");

    if !old.exists() {
        anyhow::bail!("Expected settings.py not found");
    }

    fs::rename(old, new)?;

    Ok(())
}

pub fn create_init_py(settings_dir: &Path) -> anyhow::Result<()> {
    let init_py = settings_dir.join("__init__.py");
    fs::write(init_py, "")?;

    Ok(())
}

