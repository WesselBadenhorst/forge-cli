use std::fs;
use std::path::Path;
use std::process::Command;
use clap::Parser;

/// Forge CLI - scaffolds new SaaS projects
#[derive(Parser)]
#[command(author="Wessel Badenhorst", version, about="Forge new SaaS apps quickly")]
struct Args {
    project_name: String,

    #[arg(long)]
    no_install: bool,
}

fn create_project_dir(name: &str) -> anyhow::Result<()> {
    let path = Path::new(name);
    if path.exists() {
        anyhow::bail!("Directory '{}' already exists", name);
    }
    fs::create_dir(name)?;

    Ok(())
}

fn create_backend(project_name: &str) -> anyhow::Result<()> {
    println!("🐍 Creating backend with uv...");

    let backend_dir = Path::new(project_name).join("backend");
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

fn create_frontend(project_name: &str) -> anyhow::Result<()> {
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
        .current_dir(project_name)
        .status()?;

    if !status.success() {
        anyhow::bail!("Vite failed");
    }
    
    Ok(())
}

fn main() -> anyhow::Result<()>{
    let args = Args::parse();

    println!("🔥 Forging project: {}", args.project_name);

    if args.no_install {
        println!("⚠️  Skipping dependency installation");
    } 

    create_project_dir(&args.project_name)?;
    create_backend(&args.project_name)?;
    create_frontend(&args.project_name)?;

    Ok(())
}
