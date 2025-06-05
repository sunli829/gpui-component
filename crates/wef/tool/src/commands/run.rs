use std::{path::Path, process::Command};

use anyhow::Result;

pub(crate) fn run(
    package: Option<String>,
    bin: Option<String>,
    example: Option<String>,
    release: bool,
    wef_version: Option<&str>,
    wef_path: Option<&Path>,
    args: Vec<String>,
) -> Result<()> {
    let exec_path = crate::commands::build(package, bin, example, release, wef_version, wef_path)?;

    let mut command = match std::env::consts::OS {
        "macos" => {
            let mut command = Command::new("open");
            command.arg(&exec_path);
            command.arg("-W");
            command.arg("--args");
            command
        }
        "windows" | "linux" => Command::new(&exec_path),
        _ => unreachable!(),
    };

    command.args(args).status()?;
    Ok(())
}
