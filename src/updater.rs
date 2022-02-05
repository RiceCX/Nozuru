use crate::{constants, get_current_version};
use hyper::body::Buf;
use hyper::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;

pub async fn check_for_update() -> Result<bool, Box<dyn Error + Send + Sync>> {
    let mut needs_update = false;

    if let Some(current_meta) = get_current_version().await {
        let client = Client::new();

        let uri = constants::SPIGOT_BUILD_TOOLS_API_URL.parse()?;

        println!("Checking for updates...");
        let resp = client.get(uri).await?;
        let body = hyper::body::aggregate(resp).await?;

        let json: JenkinsBuildToolsMetaData = serde_json::from_reader(body.reader())?;

        let change_set = json.change_set.items;
        let id = change_set.get(0).unwrap().clone().id;

        if id != current_meta.change_set.items.get(0).unwrap().id {
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

pub async fn download_build_tools() -> Result<(), Box<dyn Error + Send + Sync>> {
    Ok(())
}

#[derive(Serialize, Deserialize, Clone)]
pub struct JenkinsBuildToolsMetaData {
    pub artifacts: JenkinsBuildToolsMetaDataArtifacts,
    pub change_set: JenkinsBuildToolsMetaDataChangeSet,
    pub url: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct JenkinsBuildToolsMetaDataChangeSet {
    pub items: Vec<JenkinsBuildToolsMetaDataChangeSetItem>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct JenkinsBuildToolsMetaDataChangeSetItem {
    pub id: String,
    pub date: String,
    pub timestamp: i64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct JenkinsBuildToolsMetaDataArtifacts {
    pub display_path: String,
    pub file_name: String,
    pub relative_path: String,
}
