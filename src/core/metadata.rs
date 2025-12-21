use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct FolderEntry {
    pub relative_path: String,
    pub data: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
pub struct Payload {
    pub is_dir: bool,          // 👈 NEW
    pub root_name: String,
    pub entries: Vec<FolderEntry>,
}
