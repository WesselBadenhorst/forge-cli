use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use clap::Parser;

static ENV_EXAMPLE: &[u8] = include_bytes!("../assets/env.example");
static DJANGO_DEV: &[u8] = include_bytes!("../assets/django/settings/dev.py");
static DJANGO_PROD: &[u8] = include_bytes!("../assets/django/settings/prod.py");
static MAKEFILE: &[u8] = include_bytes!("../assets/Makefile");

/// Forge CLI - scaffolds new SaaS projects
#[derive(Parser)]
#[command(author="Wessel Badenhorst", version, about="Forge new SaaS apps quickly")]
struct Args {
    project_name: String,

    #[arg(long)]
    no_install: bool,
}

fn create_project_dir(project_root: &Path) -> anyhow::Result<()> {
    if project_root.exists() {
        anyhow::bail!(
            "Directory '{}' already exists",
            project_root.display()
        );
    }
    fs::create_dir(project_root)?;

    Ok(())
}

fn create_backend(project_root: &Path) -> anyhow::Result<()> {
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

fn setup_django(project_root: &Path) -> anyhow::Result<()> {
    println!("🧱 Setting up Django...");
    let backend_dir = Path::new(project_root).join("backend");

    Command::new("uv")
        .args([
            "add",
            "django",
            "djangorestframework",
            "django-allauth",
            "python-dotenv",
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

fn create_settings_dir(project_root: &Path) -> anyhow::Result<PathBuf> {
    let settings_dir = project_root
        .join("backend")
        .join("app")
        .join("settings");

    std::fs::create_dir(&settings_dir)?;

    Ok(settings_dir)
}

fn move_settings_py(project_root: &Path, settings_dir: &Path) -> anyhow::Result<()> {
    let app_dir = project_root.join("backend").join("app");

    let old = app_dir.join("settings.py");
    let new = settings_dir.join("base.py");

    if !old.exists() {
        anyhow::bail!("Expected settings.py not found");
    }

    std::fs::rename(old, new)?;

    Ok(())
}

fn create_init_py(settings_dir: &Path) -> anyhow::Result<()> {
    let init_py = settings_dir.join("__init__.py");
    std::fs::write(init_py, "")?;

    Ok(())
}

fn rewrite_base_dir(settings_dir: &Path) -> anyhow::Result<()> {
    let base_py_path = settings_dir.join("base.py");
    let content = fs::read_to_string(&base_py_path)?;

    let mut lines: Vec<String> = content.lines().map(String::from).collect();

    let has_pathlib = lines
        .iter()
        .any(|l| l.trim() == "from pathlib import Path");

    if !has_pathlib {
        // Insert at top, before anything else
        lines.insert(0, "from pathlib import Path".to_string());
        lines.insert(1, "".to_string());
    }

    let mut replaced = false;

    for line in lines.iter_mut() {
        if line.trim().starts_with("BASE_DIR =") {
            *line = "BASE_DIR = Path(__file__).resolve().parent.parent.parent"
                .to_string();
            replaced = true;
            break;
        }
    }

    if !replaced {
        anyhow::bail!("BASE_DIR definition not found in base.py");
    }

    fs::write(base_py_path, lines.join("\n"))?;
    Ok(())
}

fn inject_dotenv(settings_dir: &Path) -> anyhow::Result<()> {
    let base_py_path = settings_dir.join("base.py");
    let content = fs::read_to_string(&base_py_path)?;
    let mut lines: Vec<String> = content.lines().map(String::from).collect();

    let has_os = lines.iter().any(|l| l.trim() == "import os");
    let has_dotenv = lines
        .iter()
        .any(|l| l.trim() == "from dotenv import load_dotenv");

    // Find insertion point for imports (top of file, after pathlib if present)
    let mut insert_at = 0;
    for (i, line) in lines.iter().enumerate() {
        if line.trim() == "from pathlib import Path" {
            insert_at = i + 1;
            break;
        }
    }

    if !has_os {
        lines.insert(insert_at, "import os".to_string());
        insert_at += 1;
    }

    if !has_dotenv {
        lines.insert(insert_at, "from dotenv import load_dotenv".to_string());
    }

    let mut injected = false;

    for i in 0..lines.len() {
        if lines[i].trim().starts_with("BASE_DIR =") {
            // check if dotenv already injected
            if i + 1 < lines.len() && lines[i + 1].contains("load_dotenv") {
                injected = true;
                break;
            }

            lines.insert(
                i + 1,
                r#"load_dotenv(BASE_DIR / ".env")"#.to_string(),
            );
            injected = true;
            break;
        }
    }

    if !injected {
        anyhow::bail!("Failed to inject dotenv: BASE_DIR not found");
    }

    fs::write(base_py_path, lines.join("\n"))?;

    Ok(())
}

fn install_depts(project_root: &Path) -> anyhow::Result<()> {
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

fn create_frontend(project_root: &Path) -> anyhow::Result<()> {
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

fn asset_path(relative: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("assets")
        .join(relative)
}

fn write_env_files(project_root: &Path) -> anyhow::Result<()> {
    let backend_dir = project_root.join("backend");

    let env_example_path = backend_dir.join(".env.example");
    std::fs::write(env_example_path, ENV_EXAMPLE)?;

    let env_path = backend_dir.join(".env");
    if !env_path.exists() {
        std::fs::write(env_path, ENV_EXAMPLE)?;
    }

    Ok(())
}

fn write_env_settings(settings_dir: &Path) -> anyhow::Result<()> {
    std::fs::write(settings_dir.join("dev.py"), DJANGO_DEV)?;
    std::fs::write(settings_dir.join("prod.py"), DJANGO_PROD)?;
    Ok(())
}

fn write_makefile(project_root: &Path) -> anyhow::Result<()> {
    std::fs::write(project_root.join("Makefile"), MAKEFILE)?;
    
    Ok(())
}

fn main() -> anyhow::Result<()>{
    let args = Args::parse();

    let project_root = std::env::current_dir()?
        .join(&args.project_name);

    println!("🔥 Forging project: {}", args.project_name);

    create_project_dir(&project_root)?;

    create_frontend(&project_root)?;

    if args.no_install {
        println!("⚠️  Skipping dependency installation");
    } else {
        install_depts(&project_root)?;
    } 

    create_backend(&project_root)?;
    setup_django(&project_root)?;

    let settings_dir = create_settings_dir(&project_root)?;
    move_settings_py(&project_root, &settings_dir)?;
    create_init_py(&settings_dir)?;
    
    // Fix Django Defaults
    rewrite_base_dir(&settings_dir)?;
    inject_dotenv(&settings_dir)?;

    // Forge-owned files
    write_env_settings(&settings_dir)?;
    write_env_files(&project_root)?;

    write_makefile(&project_root)?;

    println!("✅ Project forged successfully!");
    println!("➡️  cd {} && make dev", args.project_name);

    Ok(())
}
