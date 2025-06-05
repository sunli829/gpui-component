use std::{
    fmt::Display,
    path::{Path, PathBuf},
};

use colored::Colorize;

pub(crate) fn find_cef_root(cef_root: Option<&Path>) -> PathBuf {
    if let Some(cef_root) = cef_root {
        println!("Using CEF_ROOT: {}", cef_root.display());
        cef_root.to_path_buf()
    } else if let Ok(cef_root) = std::env::var("CEF_ROOT") {
        println!("Using CEF_ROOT: {}", cef_root);
        PathBuf::from(cef_root)
    } else {
        PathBuf::from("~/.cef")
    }
}

pub(crate) fn print_error(err: impl Display) {
    eprintln!("{}: {}", "Error".red(), err);
}
