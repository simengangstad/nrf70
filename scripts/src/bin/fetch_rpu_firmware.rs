use std::{fs::File, io::Write};

use clap::Parser;

use log::{error, info};

#[derive(Parser)]
struct Cli {
    /// Commit of sdk-nrfxlib
    commit: String,

    /// Directory to place the firmware binaries
    output_directory: std::path::PathBuf,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    env_logger::builder()
        .format_timestamp(None)
        .format_target(false)
        .filter_level(log::LevelFilter::Info)
        .init();

    let args = Cli::parse();

    let firmware_types = [
        "default",
        "offloaded_raw_tx",
        "radio_test",
        "scan_only",
        "system_with_raw",
    ];

    if !args.output_directory.exists() {
        error!("Output directory {:?} does not exist", args.output_directory);
        return Ok(());
    }

    for firmware_type in firmware_types {
        let url = format!(
            "https://github.com/nrfconnect/sdk-nrfxlib/raw/{}/nrf_wifi/bin/zephyr/{}/nrf70.bin",
            args.commit, firmware_type
        );

        let response = reqwest::get(&url).await?;

        if !reqwest::StatusCode::is_success(&response.status()) {
            error!(
                "Non OK status code ({}) when fetching firmware '{}' from {}",
                response.status(),
                firmware_type,
                url
            );

            continue;
        }

        let output_path = args.output_directory.join(format!("{}.bin", firmware_type));
        let mut file = File::create(&output_path)?;

        let content = response.bytes().await?;
        file.write_all(&content)?;

        info!(
            "Fetched firmware '{}' ({} bytes). Written to: {:?}",
            firmware_type,
            content.len(),
            output_path
        );
    }

    Ok(())
}
