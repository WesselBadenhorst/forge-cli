use std::fs;
use std::path::Path;
use crate::assets;


pub fn write_env_files(project_root: &Path) -> anyhow::Result<()> {
    println!("𓂃🪶 Writing .env.example file");
    let backend_dir = project_root.join("backend");

    let env_example_path = backend_dir.join(".env.example");
    fs::write(env_example_path, assets::ENV_EXAMPLE)?;

    println!("𓂃🪶 Writing .env file");
    let env_path = backend_dir.join(".env");
    if !env_path.exists() {
        fs::write(env_path, assets::ENV_EXAMPLE)?;
    }

    Ok(())
}

pub fn write_env_settings(settings_dir: &Path) -> anyhow::Result<()> {
    fs::write(settings_dir.join("dev.py"), assets::DJANGO_DEV)?;
    fs::write(settings_dir.join("prod.py"), assets::DJANGO_PROD)?;
    Ok(())
}

