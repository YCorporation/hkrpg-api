use serde::Deserialize;

#[derive(Deserialize)]
pub struct GameInfo {
    pub id: String,
    pub biz: String,
}
