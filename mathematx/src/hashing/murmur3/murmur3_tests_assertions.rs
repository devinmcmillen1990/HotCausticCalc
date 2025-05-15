use crate::utils::snapshot::loader::load_snapshots;
use std::{
    sync::{Arc, Barrier},
    thread,
};

#[cfg(test)]
pub fn run_murmur3_hash_tests(
    snapshot_path: &str,
    hash_32_func: fn(&[u8]) -> u32,
    hash_64_func: fn(&[u8]) -> u64,
) {
    let snapshots = load_snapshots(snapshot_path);

    for snapshot in snapshots.iter() {
        let title = snapshot["title"].as_str().unwrap();
        let input = snapshot["input"].as_str().unwrap().as_bytes();
        let expected_32bit = snapshot["expected_32bit"].as_u64().unwrap() as u32;
        let expected_64bit = snapshot["expected_64bit"].as_u64().unwrap();

        println!("Running test: {}", title);

        let hash_32bit = hash_32_func(input);
        let hash_64bit = hash_64_func(input);

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

/// Run MurmurHash3 thread safety tests for both 32-bit and 64-bit versions.
#[cfg(test)]
pub fn run_murmur3_thread_safety_test(
    snapshot_path: &str,
    hash_32_func: fn(&[u8]) -> u32,
    hash_64_func: fn(&[u8]) -> u64,
    number_of_threads: usize,
) {
    let snapshots = load_snapshots(snapshot_path);
    let barrier = Arc::new(Barrier::new(number_of_threads));
    let snapshots = Arc::new(snapshots);

    let mut handles = Vec::new();

    for i in 0..number_of_threads {
        let barrier_clone = Arc::clone(&barrier);
        let snapshots_clone = Arc::clone(&snapshots);

        let handle = thread::spawn(move || {
            barrier_clone.wait();

            for snapshot in snapshots_clone.iter() {
                let input = snapshot["input"].as_str().unwrap().as_bytes();
                let expected_32bit = snapshot["expected_32bit"].as_u64().unwrap() as u32;
                let expected_64bit = snapshot["expected_64bit"].as_u64().unwrap();

                let hash_32bit = hash_32_func(input);
                let hash_64bit = hash_64_func(input);

                assert_eq!(
                    hash_32bit, expected_32bit,
                    "32-bit hash mismatch in thread {}",
                    i
                );
                assert_eq!(
                    hash_64bit, expected_64bit,
                    "64-bit hash mismatch in thread {}",
                    i
                );
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().expect("Thread panicked");
    }
}
