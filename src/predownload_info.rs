use super::main_info::{MajorInfo, PatchInfo};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct PredownloadInfo {
    pub major: Option<MajorInfo>,
    pub patches: Vec<PatchInfo>,
}
