use kari_errors::{CliError, Result};

use std::fmt::Write as _;

use colored::Colorize;
use self_update::{backends::github, version::bump_is_greater, Status};

pub struct Updater;

// TODO Add logic for users to easily select release versions.
impl Updater {
    const KARI_BIN_NAME: &'static str = "kari";
    const KARI_REPO_NAME: &'static str = "kari";
    const KARI_REPO_OWNER: &'static str = "KanariQD";

    /// Show all available releases for `leo`.
    pub fn show_available_releases() -> Result<()> {
        let releases = github::ReleaseList::configure()
            .repo_owner(Self::LEO_REPO_OWNER)
            .repo_name(Self::LEO_REPO_NAME)
            .build()
            .map_err(CliError::self_update_error)?
            .fetch()
            .map_err(CliError::could_not_fetch_versions)?;

        let mut output = "\nList of available versions\n".to_string();
        for release in releases {
            let _ = writeln!(output, "  * {}", release.version);
        }

        // Forgo using tracing to list the available versions without a log status.
        println!("{output}");

        Ok(())
    }

    /// Update `leo` to the latest release.
    pub fn update_to_latest_release(show_output: bool) -> Result<Status> {
        let status = github::Update::configure()
            .repo_owner(Self::LEO_REPO_OWNER)
            .repo_name(Self::LEO_REPO_NAME)
            .bin_name(Self::LEO_BIN_NAME)
            .current_version(env!("CARGO_PKG_VERSION"))
            .show_download_progress(show_output)
            .no_confirm(true)
            .show_output(show_output)
            .build()
            .map_err(CliError::self_update_build_error)?
            .update()
            .map_err(CliError::self_update_error)?;

        Ok(status)
    }

    /// Check if there is an available update for `leo` and return the newest release.
    pub fn update_available() -> Result<String> {
        let updater = github::Update::configure()
            .repo_owner(Self::LEO_REPO_OWNER)
            .repo_name(Self::LEO_REPO_NAME)
            .bin_name(Self::LEO_BIN_NAME)
            .current_version(env!("CARGO_PKG_VERSION"))
            .build()
            .map_err(CliError::self_update_error)?;

        let current_version = updater.current_version();
        let latest_release = updater.get_latest_release().map_err(CliError::self_update_error)?;

        if bump_is_greater(&current_version, &latest_release.version).map_err(CliError::self_update_error)? {
            Ok(latest_release.version)
        } else {
            Err(CliError::old_release_version(current_version, latest_release.version).into())
        }
    }

    /// Display the CLI message, if the Leo configuration allows.
    pub fn print_cli() {
        // If the auto update configuration is off, notify the user to update leo.
        if let Ok(latest_version) = Self::update_available() {
            let mut message = "🟢 A new version is available! Run".bold().green().to_string();
            message += &" `leo update` ".bold().white();
            message += &format!("to update to v{latest_version}.").bold().green();

            tracing::info!("\n{}\n", message);
        }
    }
}