use std::path::Path;
use crate::assets;
use std::fs;

pub fn write_makefile(project_root: &Path) -> anyhow::Result<()> {
    fs::write(project_root.join("Makefile"), assets::MAKEFILE)?;
    
    Ok(())
}

