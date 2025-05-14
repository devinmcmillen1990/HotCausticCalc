use super::fowler_noll_vo::{hash_fnv1_x32, hash_fnv1_x64};
use crate::utils::snapshot::loader::load_snapshots;

// TODO: Have tests for strings, but not for other values.
// TODO: Add tests for massive strings.
// TODO: Provide collision examples

#[test]
fn run_tests() {
    let snapshots = load_snapshots("src/hashing/sequential/fowler_noll_vo_snap.json");

    for snapshot in snapshots.iter() {
        // Arrange
        let title = snapshot["title"].as_str().unwrap();
        let input = snapshot["input"].as_str().unwrap().as_bytes();

        let expected_32bit = snapshot["expected_32bit"].as_u64().unwrap() as u32;
        let expected_64bit = snapshot["expected_64bit"].as_u64().unwrap() as u64;

        // Act
        println!("Running test: {}", title);

        let hash_32bit = hash_fnv1_x32(input);
        let hash_64bit = hash_fnv1_x64(input);

        // Assert
        assert_eq!(
            hash_32bit, expected_32bit,
            "32-bit hash mismatch for {}",
            title
        );
        assert_eq!(
            hash_64bit, expected_64bit,
            "64-bit hash mismatch for {}",
            title
        );
    }
}
