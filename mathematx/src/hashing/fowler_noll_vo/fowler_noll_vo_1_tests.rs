use crate::hashing::fowler_noll_vo::fowler_noll_vo_1::{hash_fnv1_x32, hash_fnv1_x64};
use crate::utils::snapshot::loader::load_snapshots;

/***
 * NOTE: Snapshots don't contain massive dataset entries like massive strings, because this algorithm is not intended for 
 *          that functionality.
 */
#[test]
fn run_tests() {
    let snapshots = load_snapshots("src/hashing/fowler_noll_vo/fowler_noll_vo_1_snap.json");

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



use std::sync::{Arc, Barrier};
use std::thread;

#[test]
fn thread_safety_test() {
    let snapshots = load_snapshots("src/hashing/fowler_noll_vo/fowler_noll_vo_1_snap.json");
    
    let num_threads = 10;
    let barrier = Arc::new(Barrier::new(num_threads));
    let snapshots = Arc::new(snapshots);

    let mut handles = Vec::new();

    for i in 0..num_threads {
        let barrier_clone = Arc::clone(&barrier);
        let snapshots_clone = Arc::clone(&snapshots);

        let handle = thread::spawn(move || {
            // Wait for all threads to be ready
            barrier_clone.wait();

            for snapshot in snapshots_clone.iter() {
                let input = snapshot["input"].as_str().unwrap().as_bytes();
                let expected_32bit = snapshot["expected_32bit"].as_u64().unwrap() as u32;
                let expected_64bit = snapshot["expected_64bit"].as_u64().unwrap() as u64;

                // Execute FNV-1
                let hash_32bit = hash_fnv1_x32(input);
                let hash_64bit = hash_fnv1_x64(input);

                assert_eq!(hash_32bit, expected_32bit, "32-bit FNV-1 mismatch in thread {}", i);
                assert_eq!(hash_64bit, expected_64bit, "64-bit FNV-1 mismatch in thread {}", i);
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().expect("Thread panicked");
    }
}
