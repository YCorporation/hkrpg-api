use super::package_info::{GamePkgInfo, AudioPkgInfo};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct MainInfo {
    pub major: MajorInfo,
    pub patches: Vec<PatchInfo>,
}

#[derive(Deserialize)]
pub struct MajorInfo {
    pub version: String,
    pub game_pkgs: Vec<GamePkgInfo>,
    pub audio_pkgs: Vec<AudioPkgInfo>,
    pub res_list_url: String,
}

#[derive(Deserialize)]
pub struct PatchInfo {
    pub version: String,
    pub game_pkgs: Vec<GamePkgInfo>,
    pub audio_pkgs: Vec<AudioPkgInfo>,
    pub res_list_url: String,
}
