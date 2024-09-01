pub mod data;
pub mod game_info;
pub mod main_info;
pub mod package_info;
pub mod predownload_info;

use data::Data;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct HkrpgApi {
    pub retcode: u8,
    pub message: String,
    pub data: Data,
}
