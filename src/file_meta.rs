use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct FileMeta {
    pub path: std::path::PathBuf,
    pub hash: [u8; 32],
    pub size: u64,
    pub modified: u64,
}
