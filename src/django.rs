use std::path::Path;
use std::fs;

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

fn rewrite_secret_key(settings_dir: &Path) -> anyhow::Result<()> {
    let base_py_path = settings_dir.join("base.py");
    let content = fs::read_to_string(&base_py_path)?;
    let mut lines: Vec<String> = content.lines().map(String::from).collect();

    let mut replaced = false;

    for line in lines.iter_mut() {
        if line.trim().starts_with("SECRET_KEY =") {
            *line = r#"SECRET_KEY = os.getenv("SECRET_KEY", "dev-insecure-secret")"#
                .to_string();
            replaced = true;
            break;
        }
    }

    if !replaced {
        anyhow::bail!("SECRET_KEY definition not found in base.py");
    }

    fs::write(base_py_path, lines.join("\n"))?;
    Ok(())
}

fn rewrite_allowed_hosts(settings_dir: &Path) -> anyhow::Result<()> {
    let base_py_path = settings_dir.join("base.py");
    let content = fs::read_to_string(&base_py_path)?;
    let mut lines: Vec<String> = content.lines().map(String::from).collect();

    let mut replaced = false;

    for line in lines.iter_mut() {
        if line.trim().starts_with("ALLOWED_HOSTS =") {
            *line = r#"ALLOWED_HOSTS = os.getenv("ALLOWED_HOSTS", "").split(",") if not os.getenv("DEBUG") else []"#
                .to_string();
            replaced = true;
            break;
        }
    }

    if !replaced {
        anyhow::bail!("ALLOWED_HOSTS definition not found in base.py");
    }

    fs::write(base_py_path, lines.join("\n"))?;
    Ok(())
}


fn extra_apps_block() -> Vec<String> {
    vec![
        "".into(),
        "THIRD_PARTY_APPS = [".into(),
        "    \"allauth\",".into(),
        "    \"allauth.account\",".into(),
        "    \"allauth.socialaccount\",".into(),
        "    \"allauth.socialaccount.providers.google\",".into(),
        "    \"django.contrib.sites\",".into(),
        "    \"rest_framework\",".into(),
        "]".into(),
        "".into(),
        "CUSTOM_APPS = [".into(),
        "]".into(),
        "".into(),
        "INSTALLED_APPS = CORE_APPS + THIRD_PARTY_APPS + CUSTOM_APPS".into(),
        "".into(),
        "SITE_ID = 1".into(),
    ]
}

fn restructure_installed_apps(settings_dir: &Path) -> anyhow::Result<()> {
    let base_py_path = settings_dir.join("base.py");
    let content = fs::read_to_string(&base_py_path)?;
    let mut lines: Vec<String> = content.lines().map(String::from).collect();

    let mut replaced = false;

    for line in lines.iter_mut() {
        if line.trim().starts_with("INSTALLED_APPS =") {
            *line = r#"CORE_APPS = ["#
                .to_string();
            replaced = true;
            break;
        }
    }

    if !replaced {
        anyhow::bail!("INSTALLED_APPS definition not found in base.py");
    }


    let mut injected = false;
    let mut found = false;

    for i in 0..lines.len() {
        if lines[i].trim().starts_with("CORE_APPS =") {
            found = true;
        }

        if found && lines[i].contains(']') {
            lines.splice(
                (i + 1)..(i + 1),
                extra_apps_block(),
            );
            injected = true;
            break;
        }
    }

    if !injected {
        anyhow::bail!("Failed to inject settings configuraiton");
    }

    fs::write(base_py_path, lines.join("\n"))?;
    Ok(())
}

fn inject_allauth_middleware(settings_dir: &Path) -> anyhow::Result<()> {
    let base_py_path = settings_dir.join("base.py");
    let content = fs::read_to_string(&base_py_path)?;
    let mut lines: Vec<String> = content.lines().map(String::from).collect();

    // Bail early if already present
    if lines.iter().any(|l| l.contains("AccountMiddleware")) {
        return Ok(());
    }
    let mut in_middleware = false;
    let mut indent: Option<String> = None;
    let mut injected = false;

    for i in 0..lines.len() {
        let line = &lines[i];
        let trimmed = line.trim();

        if trimmed.starts_with("MIDDLEWARE =") {
            in_middleware = true;
            continue;
        }

        if in_middleware {
            // Capture indentation from first real entry
            if indent.is_none() && trimmed.starts_with('"') {
                let leading_spaces = line.len() - line.trim_start().len();
                indent = Some(" ".repeat(leading_spaces));
            }

            // Inject just before closing bracket
            if trimmed == "]" {
                let indent = indent.unwrap_or_else(|| "    ".to_string());
                let middleware_line = format!(
                r#"{indent}"allauth.account.middleware.AccountMiddleware","#,
            );

                lines.insert(i, middleware_line);
                injected = true;
                break;
            }
        }
    }

    if !injected {
        anyhow::bail!("Failed to inject allauth AccountMiddleware");
    }

    fs::write(base_py_path, lines.join("\n"))?;

    Ok(())
}

pub fn configure(settings_dir: &Path) -> anyhow::Result<()> {
    rewrite_base_dir(&settings_dir)?;
    inject_dotenv(&settings_dir)?;
    rewrite_secret_key(&settings_dir)?;
    rewrite_allowed_hosts(&settings_dir)?;
    restructure_installed_apps(&settings_dir)?;
    inject_allauth_middleware(&settings_dir)?;
    Ok(())
}

fn rewrite_settings_module(
    file_path: &Path,
    default_module: &str,
) -> anyhow::Result<()> {
    let content = fs::read_to_string(file_path)?;
    let mut lines: Vec<String> = content.lines().map(String::from).collect();

    let mut replaced = false;

    for line in lines.iter_mut() {
        if line.contains("DJANGO_SETTINGS_MODULE") {
            let trimmed = line.trim_start();
            let leading_spaces = line.len() - trimmed.len();
            let indent = " ".repeat(leading_spaces);

            *line = format!(
                r#"{indent}os.environ.setdefault("DJANGO_SETTINGS_MODULE", os.getenv("DJANGO_SETTINGS_MODULE", "{}"))"#,
                default_module
            );
            replaced = true;
            break;
        }
    }
    
    if !replaced {
        anyhow::bail!(
            "DJANGO_SETTINGS_MODULE not found in {}",
            file_path.display()
        );
    }

    fs::write(file_path, lines.join("\n"))?;

    Ok(())
}

fn rewrite_manage_py(backend_dir: &Path) -> anyhow::Result<()> {
    let manage_py_path = backend_dir.join("manage.py");
    rewrite_settings_module(&manage_py_path, "app.settings.dev")?;
    Ok(())
}

fn rewrite_wsgi_py(app_dir: &Path) -> anyhow::Result<()> {
    let wsgi_py_path = app_dir.join("wsgi.py");
    rewrite_settings_module(&wsgi_py_path, "app.settings.prod")?;
    Ok(())
}

fn rewrite_asgi_py(app_dir: &Path) -> anyhow::Result<()> {
    let asgi_py_path = app_dir.join("asgi.py");
    rewrite_settings_module(&asgi_py_path, "app.settings.prod")?;
    Ok(())
}

pub fn rewrite_settings_refs(project_dir: &Path) -> anyhow::Result<()> {
    let backend_dir = project_dir.join("backend");
    rewrite_manage_py(&backend_dir)?;

    let app_dir = backend_dir.join("app");
    rewrite_wsgi_py(&app_dir)?;
    rewrite_asgi_py(&app_dir)?;

    Ok(())
}

