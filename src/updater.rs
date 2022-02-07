use crate::{BoxedResult, constants, get_current_version};
use hyper::body::{Buf, HttpBody};
use hyper::{Body, Client, Method, Request};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;
use hyper_tls::HttpsConnector;

pub async fn check_for_update() -> BoxedResult<bool> {
    let mut needs_update = false;

    if let Some(current_meta) = get_current_version().await {
        // setup http client
        let connector = HttpsConnector::new();
        let client: Client<_, Body> = Client::builder().build(connector);

        let req = Request::builder()
            .method(Method::GET)
            .uri(constants::SPIGOT_BUILD_TOOLS_API_URL)
            .header("User-Agent", "Nozuru/RiceCX 1.0")
            .body(Body::empty())?;

        println!("Checking for updates...");
        let resp = client.request(req).await?;

        let body = hyper::body::aggregate(resp).await?;

        let json: JenkinsBuildToolsMetaData = serde_json::from_reader(body.reader())?;

        let latest = json.id;

        if latest != current_meta.id {
            needs_update = true;
            println!("Spigot Build Tools update available!");
        } else {
            println!("Spigot Build Tools is up to date!");
        }
    } else {
        needs_update = true;
        println!("Spigot Build Tools metadata not found. Downloading build tools...");
    }

    Ok(needs_update)
}

pub fn check_and_create_build_folder() {
    let build_folder = constants::get_build_tools_folder();

    if !Path::new(&build_folder).exists() {
        std::fs::create_dir_all(&build_folder).expect("Failed to create build folder");
    }
}

pub async fn download_build_tools_metadata(file_location: &Path) -> BoxedResult<()> {
    // setup http client
    let connector = HttpsConnector::new();
    let client: Client<_, Body> = Client::builder().build(connector);

    let req = Request::builder()
        .method(Method::GET)
        .uri(constants::SPIGOT_BUILD_TOOLS_API_URL)
        .header("User-Agent", "Nozuru/RiceCX 1.0")
        .body(Body::empty())?;

    println!("Downloading patch metadata...");
    let resp = client.request(req).await?;

    println!("Response code: {}", resp.status());

    let body = hyper::body::aggregate(resp).await?;

    let json: JenkinsBuildToolsMetaData = serde_json::from_reader(body.reader())?;

    println!("JSON: {:#?}", json);

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(file_location)?;

    file.write_all(serde_json::to_string(&json)?.as_bytes())?;

    Ok(())
}

pub async fn download_build_tools(file_location: &Path) -> BoxedResult<()> {
    check_and_create_build_folder();
    println!("Downloading build tools...");
    // setup temp files
    let temp_dir_path = tempfile::tempdir()?.into_path();
    let file_name = match file_location.file_name() {
        Some(name) => name,
        None => return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Invalid file path",
        ))),
    };

    let temp_filepath = temp_dir_path.join(&file_name);


    // setup http client
    let connector = HttpsConnector::new();
    let client: Client<_, Body> = Client::builder().build(connector);

    let mut temp_file = OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .open(&temp_filepath)?;

    let req = Request::builder()
        .method(Method::GET)
        .uri(constants::SPIGOT_BUILD_TOOLS_URL)
        .header("User-Agent", "Nozuru/RiceCX 1.0")
        .body(Body::empty())?;

    let mut res = client.request(req).await?;

    println!("Response status: {}", res.status());

    if !res.status().is_success() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to download build tools",
        )));
    }

    println!("Response headers: {:?}", res.headers());

    let mut bytes = 0;
    let max_bytes = res.headers().get("content-length").unwrap().to_str().unwrap().parse::<usize>()?;
    while let Some(next) = res.data().await {
        let chunk  = next?;
        bytes = bytes + chunk.len();
        println!("Downloaded {}/{}", bytes, max_bytes);

        temp_file.write(&chunk)?;
    }

    temp_file.flush()?;
    println!("Download complete!");

    println!("Moving file from {} to {}", temp_filepath.display(), file_location.display());
    // Move file to final location
    std::fs::copy(&temp_filepath, file_location)?;

    println!("Build tools installed!");
    Ok(())
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct JenkinsBuildToolsMetaData {
    pub id: String,
    pub url: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct JenkinsBuildToolsMetaDataChangeSet {
    pub items: Vec<JenkinsBuildToolsMetaDataChangeSetItem>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct JenkinsBuildToolsMetaDataChangeSetItem {
    pub id: String,
    pub date: String,
    pub timestamp: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct JenkinsBuildToolsMetaDataArtifacts {
    pub display_path: String,
    pub file_name: String,
    pub relative_path: String,
}
