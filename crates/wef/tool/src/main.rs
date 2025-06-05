mod commands;
mod internal;

use std::path::PathBuf;

use clap::{Parser, Subcommand};
use colored::Colorize;

use crate::internal::{CefBuildsPlatform, DEFAULT_CEF_VERSION};

#[derive(Subcommand)]
enum Commands {
    /// Download CEF framework
    Init {
        /// Target path
        #[clap(long)]
        path: Option<PathBuf>,
        /// CEF version
        #[clap(long, default_value = DEFAULT_CEF_VERSION)]
        version: String,
        /// Platform
        #[clap(long, default_value = "auto")]
        platform: CefBuildsPlatform,
        /// Force download even if the file already exists
        #[clap(long, short, default_value_t = false)]
        force: bool,
    },
    /// Compile a local package and all of its dependencies
    Build {
        /// Package to build (see `cargo help pkgid`)
        #[clap(long, short, value_name = "SPEC")]
        package: Option<String>,
        /// Build only the specified binary
        #[clap(long, value_name = "NAME")]
        bin: Option<String>,
        /// Build only the specified example
        #[clap(long, value_name = "NAME")]
        example: Option<String>,
        /// Build artifacts in release mode, with optimizations
        #[clap(long, short)]
        release: bool,
        /// Use the specified Wef version
        ///
        /// If not specified, use the latest version
        #[clap(long)]
        wef_version: Option<String>,
        /// Specify the source code path of the local Wef library instead of the
        /// published version
        #[clap(long)]
        wef_path: Option<PathBuf>,
    },
    /// Run a binary or example of the local package
    Run {
        /// Package to build (see `cargo help pkgid`)
        #[clap(long, short, value_name = "SPEC")]
        package: Option<String>,
        /// Build only the specified binary
        #[clap(long, value_name = "NAME")]
        bin: Option<String>,
        /// Build only the specified example
        #[clap(long, value_name = "NAME")]
        example: Option<String>,
        /// Build artifacts in release mode, with optimizations
        #[clap(long, short)]
        release: bool,
        /// Use the specified Wef version
        ///
        /// If not specified, use the latest version
        #[clap(long)]
        wef_version: Option<String>,
        /// Specify the source code path of the local Wef library instead of the
        /// published version
        #[clap(long)]
        wef_path: Option<PathBuf>,
        #[arg(last = true)]
        args: Vec<String>,
    },
    // /// Add helper processes to the app
    // AddHelper {
    //     /// Target app path
    //     app_path: PathBuf,
    //     /// Use the specified Wef version
    //     ///
    //     /// If not specified, use the latest version
    //     #[clap(long)]
    //     wef_version: Option<String>,
    //     /// Specify the source code path of the local Wef library instead of the
    //     /// published version
    //     #[clap(long)]
    //     wef_path: Option<PathBuf>,
    //     /// Build artifacts in release mode, with optimizations
    //     #[clap(long, short)]
    //     release: bool,
    //     /// Force adding the helper processes even if they already exist
    //     #[clap(long, short, default_value_t = false)]
    //     force: bool,
    // },
    // /// Add CEF framework to the app
    // AddFramework {
    //     /// Target app path
    //     app_path: PathBuf,
    //     /// CEF root path
    //     #[clap(long, env = "CEF_ROOT")]
    //     cef_root: Option<PathBuf>,
    //     /// Build artifacts in release mode, with optimizations
    //     #[clap(long, short)]
    //     release: bool,
    //     /// Force adding the framework even if it already exists
    //     #[clap(long, short, default_value_t = false)]
    //     force: bool,
    // },
}

/// Wef CLI tool
#[derive(Parser)]
#[clap(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

fn main() {
    let cli = Cli::parse();

    let res = match cli.command {
        Commands::Init {
            path,
            version,
            platform,
            force,
        } => commands::init(path, version, platform, force),
        Commands::Build {
            package,
            bin,
            example,
            release,
            wef_version,
            wef_path,
        } => commands::build(
            package,
            bin,
            example,
            release,
            wef_version.as_deref(),
            wef_path.as_deref(),
        )
        .map(|_| ()),
        Commands::Run {
            package,
            bin,
            example,
            release,
            wef_version,
            wef_path,
            args,
        } => commands::run(
            package,
            bin,
            example,
            release,
            wef_version.as_deref(),
            wef_path.as_deref(),
            args,
        ),
    };

    if let Err(err) = res {
        eprintln!("{}: {}", "Error".red(), err);
        std::process::exit(-1);
    }
}
