use std::path::PathBuf;

pub const TITLE: &str = "_____   __
___  | / /_______________  ____________  __
__   |/ /_  __ \\__  /_  / / /_  ___/  / / /
_  /|  / / /_/ /_  /_/ /_/ /_  /   / /_/ /
/_/ |_/  \\____/_____/\\__,_/ /_/    \\__,_/";

pub const AUTHOR: &str = "AndyIsCool5463#5230";
pub const SPIGOT_BUILD_TOOLS_URL: &str = "https://hub.spigotmc.org/jenkins/job/BuildTools/lastSuccessfulBuild/artifact/target/BuildTools.jar";

pub const SPIGOT_BUILD_TOOLS_API_URL: &str =
    "https://hub.spigotmc.org/jenkins/job/BuildTools/lastSuccessfulBuild/api/json";

const BUILD_TOOLS_DIR: &str = "build-tools";

const BUILD_TOOLS_JAR: &str = "BuildTools.jar";

const BUILD_TOOLS_METADATA: &str = "build-tools.json";

pub fn get_build_tools_path() -> PathBuf {
    std::path::Path::new(".")
        .join(&BUILD_TOOLS_DIR)
        .join(&BUILD_TOOLS_JAR)
}

pub fn get_build_tools_metadata_path() -> PathBuf {
    std::path::Path::new(".")
        .join(&BUILD_TOOLS_DIR)
        .join(&BUILD_TOOLS_METADATA)
}
