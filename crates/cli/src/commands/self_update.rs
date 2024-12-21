use clap::Parser;

use mago_feedback::error;
use mago_feedback::info;
use mago_feedback::warn;
use mago_service::config::Configuration;
use self_update::cargo_crate_version;
use self_update::errors::Error;
use self_update::Status;

const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");
const TARGET: &str = env!("TARGET");
const REPO_OWNER: &str = "carthage-software";
const REPO_NAME: &str = "mago";
const BIN_NAME: &str = "mago";
const ISSUE_URL: &str = "https://github.com/carthage-software/mago/issues/new";

#[derive(Parser, Debug)]
#[command(
    name = "self-update",
    about = "Check for updates or update Mago to the latest version",
    long_about = r#"The `self-update` command allows you to update Mago to the latest version
or check if updates are available. By default, it fetches the latest release
from GitHub and replaces the current binary with the updated version.

You can also specify additional options like downloading a specific version
or suppressing output."#
)]
pub struct SelfUpdateCommand {
    #[arg(long, short, help = "Check for updates but do not install them")]
    pub check: bool,

    #[arg(long, help = "Do not show progress bars")]
    pub no_progress: bool,

    #[arg(long, help = "Do not ask for confirmation")]
    pub no_confirm: bool,

    #[arg(long, help = "Toggle update output information")]
    pub no_output: bool,

    #[arg(long, help = "Update to a specific version")]
    pub tag: Option<String>,
}

pub fn execute(command: SelfUpdateCommand, _configuration: Configuration) -> i32 {
    info!("Current version: {}", CURRENT_VERSION);

    let mut status_builder = self_update::backends::github::Update::configure();
    status_builder
        .repo_owner(REPO_OWNER)
        .repo_name(REPO_NAME)
        .target(TARGET)
        .bin_name(BIN_NAME)
        .current_version(cargo_crate_version!())
        .bin_path_in_archive("{{ bin }}-{{ version }}-{{ target }}/{{ bin }}")
        .show_download_progress(!command.no_progress)
        .show_output(!command.no_output)
        .no_confirm(command.no_confirm);

    if let Some(tag) = command.tag {
        status_builder.target_version_tag(&tag);
    }

    let status = status_builder.build().unwrap_or_else(update_error);

    if command.check {
        let release = status.get_latest_release().unwrap_or_else(update_error);

        info!("Latest release: {}", release.version);
        info!("Date: {}", release.date);

        return 0;
    }

    let status = status.update().unwrap_or_else(update_error);

    match status {
        Status::UpToDate(latest_version) => {
            info!("Already up-to-date with the latest version {}", latest_version);
        }
        Status::Updated(latest_version) => {
            info!("Updated to version {}", latest_version);
        }
    }

    0
}

fn update_error<T>(error: Error) -> T {
    println!();

    let mut code = 1;
    match error {
        Error::Network(e) => {
            error!("Network error occurred: {}", e);
            error!("Check your connection and try again.");
        }
        Error::Release(e) => {
            if e.contains("No asset found for target") {
                error!("No release assets found for the target '{}'.", TARGET);
                error!("This may happen if your target is not supported by our official builds.");
                error!("If you built this binary yourself, recompile the new version or use your original installation method.");
                error!("Binaries downloaded from GitHub should not encounter this error.");
                error!("Need help? Open an issue at {}.", ISSUE_URL);
            } else {
                error!("Failed to fetch release information: {}", e);
                error!("This could indicate a problem with GitHub API or repository configuration.");
                error!("Please open an issue at {} with the error details.", ISSUE_URL);
            }
        }
        Error::Io(e) => {
            error!("I/O error occurred: {}", e);
            error!("Ensure you have sufficient permissions and disk space.");
        }
        Error::Update(e) => {
            if e.contains("Update aborted") {
                warn!("Update aborted by user.");

                code = 0;
            } else {
                error!("Update error occurred: {}", e);
                error!("Please verify the installation directory and file permissions.");
            }
        }
        _ => {
            error!("An unexpected error occurred: {}", error);
            error!("Please open an issue at {} with the error details.", ISSUE_URL);
        }
    }

    std::process::exit(code);
}
