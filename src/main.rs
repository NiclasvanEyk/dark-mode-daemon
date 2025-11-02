use dark_mode_daemon::cli;
use dark_mode_daemon::platform;
use std::process::exit;
fn handle_cli_result(result: anyhow::Result<()>) {
    match result {
        Ok(_) => {}
        Err(err) => {
            eprintln!("{}", err);
            exit(1);
        }
    }
}

#[cfg(target_os = "macos")]
fn main() {
    crate::cli::run(crate::platform::macos::MacOsAdapter::default());
}

#[cfg(target_os = "linux")]
#[tokio::main]
async fn main() {
    let detector_factory = crate::platform::linux::LinuxColorModeDetector::default;
    let cli_result = crate::cli::run(detector_factory).await;
    handle_cli_result(cli_result);
}

#[cfg(target_os = "windows")]
fn main() {
    crate::cli::run(crate::platform::dark_light::DarkLightAdapter::default());
}
