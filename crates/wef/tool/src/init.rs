use anyhow::{Context, Result};

use crate::{download_cef, utils};

pub(crate) fn init() -> Result<()> {
    let cef_root = utils::cef_root()?;
    std::fs::create_dir_all(&cef_root).context("failed to create CEF directory")?;

    println!("CEF path is created at {:?}", cef_root.display());
    println!(
        "\n   [Optional] Set the CEF_ROOT env to `{:?}`\n",
        cef_root.display()
    );

    download_cef::download_cef(&download_cef::DownloadCefSettings {
        path: cef_root.clone(),
        version: None,
        platform: crate::cef_platform::CefBuildsPlatform::Auto,
        force: false,
    })
    .context("failed to download CEF")?;

    println!("\n{}", "-".repeat(80));
    println!("CEF init successfully.");

    Ok(())
}

pub(crate) fn set_env() -> Result<()> {
    let cef_root = utils::cef_root()?;

    unsafe {
        std::env::set_var("CEF_ROOT", cef_root.display().to_string());
    }

    Ok(())
}
