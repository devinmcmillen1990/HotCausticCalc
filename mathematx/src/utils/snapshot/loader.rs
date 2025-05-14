use serde_json::Value;
use std::fs;
use std::path::Path;

pub fn load_snapshots(file_path: &str) -> Vec<Value> {
    let path = Path::new(file_path);

    let snapshot_content = fs::read_to_string(path).expect("Failed to read snapshot file");
    
    serde_json::from_str(&snapshot_content).expect("Failed to parse snapshot JSON")
}
