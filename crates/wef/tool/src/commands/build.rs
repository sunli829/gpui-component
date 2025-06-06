use std::{
    fs::File,
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::{Context, Result};
use askama::Template;
use cargo_metadata::{Metadata, MetadataCommand};

use crate::internal::{InfoPlist, add_cef_framework, add_helper};

fn execute_path(
    metadata: &Metadata,
    target_dir: &Path,
    package: Option<&str>,
    bin: Option<&str>,
    example: Option<&str>,
) -> Result<PathBuf> {
    let package = if let Some(package_name) = package {
        metadata
            .workspace_packages()
            .into_iter()
            .find(|package| package.name.as_str() == package_name)
    } else {
        metadata.workspace_default_packages().into_iter().next()
    }
    .ok_or_else(|| anyhow::anyhow!("No package found in the workspace"))?;

    let target_name = if let Some(bin_name) = bin {
        bin_name
    } else if let Some(example_name) = example {
        example_name
    } else {
        package.name.as_str()
    };

    let target = package
        .targets
        .iter()
        .find(|target| target.name == target_name)
        .ok_or_else(|| {
            if let Some(bin_name) = bin {
                anyhow::anyhow!("no bin target named `{}`", bin_name)
            } else if let Some(example_name) = example {
                anyhow::anyhow!("no example target named `{}`", example_name)
            } else {
                anyhow::anyhow!("no target named `{}`", target_name)
            }
        })?;

    anyhow::ensure!(
        target.is_bin() || target.is_example(),
        "target `{}` is not a binary or example",
        target_name
    );

    match std::env::consts::OS {
        "macos" => Ok(target_dir.join(&target.name)),
        "windows" => Ok(target_dir.join(&target.name).with_extension("exe")),
        "linux" => Ok(target_dir.join(&target.name)),
        _ => Err(anyhow::anyhow!(
            "Unsupported platform: {}",
            std::env::consts::OS
        )),
    }
}

fn bundle_macos_app(
    exec_path: &Path,
    cef_root: &Path,
    release: bool,
    wef_version: Option<&str>,
    wef_path: Option<&Path>,
) -> Result<PathBuf> {
    let filename = exec_path.file_name().unwrap();
    let app_path = exec_path
        .parent()
        .unwrap()
        .join(format!("{}.app", filename.to_string_lossy()));

    let macos_path = app_path.join("Contents").join("MacOS");
    std::fs::create_dir_all(&macos_path).context("create app directory")?;

    std::fs::copy(exec_path, macos_path.join(filename)).context("copy binary to app bundle")?;

    let plist_path = app_path.join("Contents").join("Info.plist");
    InfoPlist {
        name: filename.to_string_lossy().to_string(),
        identifier: "wef.bundle.test".to_string(),
    }
    .write_into(&mut File::create(&plist_path)?)
    .with_context(|| format!("create file at {}", plist_path.display()))?;

    add_cef_framework(cef_root, &app_path, release, false)?;
    add_helper(&app_path, wef_version, wef_path, release, false)?;
    Ok(macos_path.join(filename))
}

pub(crate) fn build(
    package: Option<String>,
    bin: Option<String>,
    example: Option<String>,
    release: bool,
    wef_version: Option<&str>,
    wef_path: Option<&Path>,
) -> Result<PathBuf> {
    let cef_root = crate::internal::find_cef_root();
    println!("Using CEF_ROOT: {}", cef_root.display());

    let metadata = MetadataCommand::new()
        .current_dir(std::env::current_dir().unwrap())
        .exec()?;

    let mut command = Command::new("cargo");

    command.arg("build");

    if let Some(package) = &package {
        command.arg("--package").arg(package);
    }

    if let Some(bin) = &bin {
        command.arg("--bin").arg(bin);
    }

    if let Some(example) = &example {
        command.arg("--example").arg(example);
    }

    if release {
        command.arg("--release");
    }

    anyhow::ensure!(command.status()?.success(), "failed to build the project");

    let target_dir = metadata
        .target_directory
        .join(if release { "release" } else { "debug" });

    match std::env::consts::OS {
        "macos" => {
            let exec_path = execute_path(
                &metadata,
                target_dir.as_std_path(),
                package.as_deref(),
                bin.as_deref(),
                example.as_deref(),
            )?;
            bundle_macos_app(&exec_path, &cef_root, release, wef_version, wef_path)
        }
        "windows" | "linux" => {
            add_cef_framework(&cef_root, target_dir.as_std_path(), release, false)?;
            execute_path(
                &metadata,
                target_dir.as_std_path(),
                package.as_deref(),
                bin.as_deref(),
                example.as_deref(),
            )
        }
        _ => {
            anyhow::bail!("Unsupported platform: {}", std::env::consts::OS);
        }
    }
}
