#[cfg(test)]
mod tests;

mod assets;
mod backend;
mod cli;
mod django;
mod env;
mod fs;
mod frontend;
mod makefile;

use std::path::{Path, PathBuf};
use std::process::Command;

use clap::Parser;
use cli::Args;

fn require_cmd(cmd: &str, install_hint: &str) -> anyhow::Result<()> {
    let ok = Command::new(cmd)
        .arg("--version")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .is_ok();

    if !ok {
        anyhow::bail!(
            "{} is required but not fount.\n{}",
            cmd,
            install_hint,
        );
    }
    Ok(())
}

fn resolve_project_root(args: &Args) -> anyhow::Result<PathBuf> {
    let cwd = std::env::current_dir()?;

    if let Some(name) = &args.project_name {
        if name == "." || name.ends_with('/') {
            return Ok(cwd.join(name).canonicalize()?);
        }

        // If it looks like a path
        if name.contains('/') {
            return Ok(Path::new(name).canonicalize()?);
        }

        // Otherwise treat as a new folder name
        return Ok(cwd.join(name));
    }
    
    anyhow::bail!("Please provide a project name or '.'");
}

fn ensure_uv() -> anyhow::Result<()> {
    if Command::new("uv").arg("--version").output().is_ok() {
        return Ok(());
    }

    println!("📦 Installing uv…");

    let status = Command::new("sh")
        .args([
            "-c",
            "curl -Ls https://astral.sh/uv/install.sh | sh",
        ])
        .status()?;

    if !status.success() {
        anyhow::bail!("Failed to install uv");
    }

    Ok(())
}

fn main() -> anyhow::Result<()>{
    let args = Args::parse();

    require_cmd("git", "Install git from https://git-scm.com")?;
    require_cmd("python3", "Install Python 3.11+")?;
    require_cmd("node", "Install Node.js from https://node.org")?;
    require_cmd("npm", "npm should come with Node.js")?;

    ensure_uv()?;

    let project_root = resolve_project_root(&args)?;

    fs::create_project_dir(&project_root)?;

    frontend::create_frontend(&project_root)?;

    if args.no_install {
        println!("⚠️  Skipping dependency installation");
    } else {
        frontend::install_depts(&project_root)?;
    } 

    backend::create(&project_root)?;
    backend::setup_django(&project_root)?;

    let settings_dir = backend::create_settings_dir(&project_root)?;
    backend::move_settings_py(&project_root, &settings_dir)?;
    backend::create_init_py(&settings_dir)?;
    
    django::configure(&settings_dir)?;
    django::rewrite_settings_refs(&project_root)?;

    // Forge-owned files
    env::write_env_settings(&settings_dir)?;
    env::write_env_files(&project_root)?;

    makefile::write_makefile(&project_root)?;

    println!("✅ Project forged successfully!");
    println!("➡️  cd {} && make dev", project_root.display());

    Ok(())
}
