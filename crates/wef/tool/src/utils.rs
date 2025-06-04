use std::{
    fmt::Display,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};
use colored::Colorize;

/// Return the CEF_ROOT env or default path: `$HOME/.local/share/cef`
pub fn cef_root() -> Result<PathBuf> {
    if let Ok(path) = std::env::var("CEF_ROOT") {
        return Ok(Path::new(&path).to_path_buf());
    }

    Ok(std::env::home_dir()
        .context("faild get home_dir")?
        .join(".local/share/cef"))
}

pub(crate) fn find_cef_root(cef_root: Option<&Path>) -> Result<PathBuf> {
    if let Some(cef_root) = cef_root {
        println!("Using CEF_ROOT: {}", cef_root.display());
        Ok(cef_root.to_path_buf())
    } else if let Ok(cef_root) = std::env::var("CEF_ROOT") {
        println!("Using CEF_ROOT: {}", cef_root);
        Ok(PathBuf::from(cef_root))
    } else {
        let err = anyhow::anyhow!("CEF_ROOT environment variable not set");
        print_error(&err);
        Err(err)
    }
}

pub(crate) fn print_error(err: impl Display) {
    eprintln!("{}: {}", "Error".red(), err);
}
