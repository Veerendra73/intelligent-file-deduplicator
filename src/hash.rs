use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};
use sha2::{Sha256, Digest};

pub fn create_hash_map(files: &Vec<String>) -> HashMap<String, String> {
    let mut result = HashMap::new();

    for file in files {
        if let Some(hash) = calculate_hash(file) {
            result.insert(file.clone(), hash);
        }
    }

    result
}

fn calculate_hash(path: &str) -> Option<String> {
    let file = File::open(path).ok()?;
    let mut reader = BufReader::new(file);
    let mut hasher = Sha256::new();
    let mut buffer = [0; 4096];

    while let Ok(n) = reader.read(&mut buffer) {
        if n == 0 { break; }
        hasher.update(&buffer[..n]);
    }

    Some(format!("{:x}", hasher.finalize()))
}
