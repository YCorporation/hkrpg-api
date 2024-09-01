use super::game_info::GameInfo;
use super::main_info::MainInfo;
use super::predownload_info::PredownloadInfo;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Data {
    pub game_packages: Vec<GamePackage>,
}

#[derive(Deserialize)]
pub struct GamePackage {
    pub game: GameInfo,
    pub main: MainInfo,
    pub pre_download: PredownloadInfo,
}
