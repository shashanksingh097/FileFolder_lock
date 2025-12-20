use std::fs;
use rand::{rngs::OsRng, RngCore};

pub fn destroy_file(path: &str) {
    let size = fs::metadata(path).unwrap().len() as usize;

    let mut garbage = vec![0u8; size];
    OsRng.fill_bytes(&mut garbage);

    fs::write(path, garbage).unwrap();
    fs::rename(path, format!("{}.destroyed", path)).unwrap();
}
