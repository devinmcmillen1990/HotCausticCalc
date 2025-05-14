// TODO: Move into /sequential/

use super::structs::Vector;
use crate::utils::snapshot::loader::load_snapshots;

const BASE_SNAPSHOT_PATH: &str = "src/vector/.snapshots/base_snapshots.json";

// TODO: This only tests i32, need strings, structs, and other data types.

#[test]
fn run_tests() {
    let snapshots = load_snapshots(BASE_SNAPSHOT_PATH);

    for snapshot in snapshots.iter() {
        let title = snapshot["title"].as_str().unwrap();
        let input = &snapshot["input"];
        let expected = &snapshot["expected_output"];

        println!("Running test: {}", title);

        let data: Vec<i32> = input["data"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_i64().unwrap() as i32)
            .collect();

        let vector = Vector::new_with(data.clone());

        // Test Length
        if let Some(expected_len) = expected.get("length") {
            let expected_len = expected_len.as_u64().unwrap() as usize;
            assert_eq!(vector.len(), expected_len, "Test failed: {}", title);
        }

        // Test Get
        if let Some(get_index) = input.get("get_index") {
            let index = get_index.as_u64().unwrap() as usize;
            let result = vector.get(index);

            if let Some(expected_result) = expected.get("result") {
                if expected_result.is_null() {
                    assert_eq!(result, None, "Test failed: {}", title);
                } else {
                    let expected_val = expected_result.as_i64().unwrap() as i32;
                    assert_eq!(result, Some(&expected_val), "Test failed: {}", title);
                }
            }
        }
    }
}
