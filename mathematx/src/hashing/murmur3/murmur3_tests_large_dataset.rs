use crate::hashing::murmur3::{
    murmur3_x32::murmur3_x32,
    murmur3_x64::murmur3_x64,
};
use std::fs;
use std::path::Path;

#[test]
fn large_dataset_hash_test() {
    let file_path = "src/hashing/.utils/large_test_string_for_hashing.txt";
    let path = Path::new(file_path);

    println!("Reading file: {}", file_path);
    let data = fs::read(path).expect("Unable to read large dataset file");

    println!("Running Murmur3 hash tests on large dataset...");
    let hash_32bit = murmur3_x32(&data);
    let hash_64bit = murmur3_x64(&data);

    println!("32-bit hash: {} | 64-bit hash: {}", hash_32bit, hash_64bit);

    // Replace these values after the first run with actual expected hashes
    let expected_32bit = 1826523034;
    let expected_64bit = 13977607506082318676;

    assert_eq!(
        hash_32bit, expected_32bit,
        "32-bit hash mismatch for large dataset"
    );

    assert_eq!(
        hash_64bit, expected_64bit,
        "64-bit hash mismatch for large dataset"
    );
}
