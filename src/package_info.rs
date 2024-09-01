use serde::Deserialize;

#[derive(Deserialize)]
pub struct GamePkgInfo {
    pub url: String,
    pub md5: String,
    pub size: String,
    pub decompressed_size: String,
}

#[derive(Deserialize)]
pub struct AudioPkgInfo {
    pub language: String,
    pub url: String,
    pub md5: String,
    pub size: String,
    pub decompressed_size: String,
}
