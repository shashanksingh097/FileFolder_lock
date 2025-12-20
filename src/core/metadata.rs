use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Payload {
    pub original_name: String,
    pub file_data: Vec<u8>,
}
