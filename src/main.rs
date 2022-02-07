use crate::updater::JenkinsBuildToolsMetaData;
use hyper::body::Buf;
use hyper::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use crate::constants::BoxedResult;

mod constants;
mod updater;
mod server;
mod versions;


#[tokio::main]
async fn main() -> BoxedResult<()> {
    print!("\x1B[2J\x1B[1;1H");
    println!("{}", constants::TITLE);
    println!("\nThank you for using Nozuru, a Rust tool to deploy Spigot jars and deploy Javadocs");
    println!("This tool is still in development, so please report any bugs or issues to {} on Discord, or open an issue on GitHub\n", constants::AUTHOR);
    verify_env();

    check_for_installation().await;

    server::start().await
}

async fn check_for_installation() -> BoxedResult<()> {
    let metadata = std::fs::metadata(constants::get_build_tools_metadata_path());

    if metadata.is_ok() {
    } else {
        println!("Spigot Build Tools folder not found. Installing now...");
        updater::check_and_create_build_folder().await?;
    }

    let needs_update = updater::check_for_update().await?;

    if needs_update {
        updater::download_build_tools_metadata(constants::get_build_tools_metadata_path().as_path()).await?;
        updater::download_build_tools(constants::get_build_tools_path().as_path()).await?;
    }

    Ok(())
}

fn verify_env() {
    let java_version = std::env::var("JAVA_HOME");
    java_version.as_ref().expect("JAVA_HOME not set");
    println!(
        "Using java executable from '{}'",
        java_version.as_ref().unwrap()
    );
}

async fn get_current_version() -> Option<JenkinsBuildToolsMetaData> {
    let metadata = std::fs::metadata(constants::get_build_tools_metadata_path());

    if metadata.is_err() {
        return None;
    }

    let file = File::open(constants::get_build_tools_metadata_path());
    let reader = BufReader::new(file.unwrap());

    let metadata = serde_json::from_reader::<_, JenkinsBuildToolsMetaData>(reader);

    Some(metadata.unwrap())
}
