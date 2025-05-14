use crate::hashing::fowler_noll_vo_1::fowler_noll_vo_1_x32::{
    hash_fnv1_x32, hash_fnv1_x32_16byte_chunks, hash_fnv1_x32_32byte_chunks,
    hash_fnv1_x32_4byte_chunks, hash_fnv1_x32_8byte_chunks,
};
use crate::hashing::fowler_noll_vo_1::fowler_noll_vo_1_x64::{
    hash_fnv1_x64, hash_fnv1_x64_16byte_chunks, hash_fnv1_x64_32byte_chunks,
    hash_fnv1_x64_64byte_chunks, hash_fnv1_x64_8byte_chunks,
};
use crate::utils::snapshot::loader::load_snapshots;

/***
 * NOTE: Snapshots don't contain massive dataset entries like massive strings, because this algorithm is not intended for
 *          that functionality.
 */

use std::sync::{Arc, Barrier};
use std::thread;

const TEST_THREAD_COUNT: usize = 100;

#[test]
fn thread_safety_base_test() {
    run_thread_safety_test(
        "src/hashing/fowler_noll_vo_1/.snapshots/fowler_noll_vo_1_base_snap.json",
        hash_fnv1_x32,
        hash_fnv1_x64,
        TEST_THREAD_COUNT
    );
}

#[test]
fn thread_safety_4byte_test() {
    run_thread_safety_test(
        "src/hashing/fowler_noll_vo_1/.snapshots/fowler_noll_vo_1_4byte_snap.json",
        hash_fnv1_x32_4byte_chunks,
        hash_fnv1_x64,
        TEST_THREAD_COUNT
    );
}

#[test]
fn thread_safety_8byte_test() {
    run_thread_safety_test(
        "src/hashing/fowler_noll_vo_1/.snapshots/fowler_noll_vo_1_8byte_snap.json",
        hash_fnv1_x32_8byte_chunks,
        hash_fnv1_x64_8byte_chunks,
        TEST_THREAD_COUNT
    );
}

#[test]
fn thread_safety_16byte_test() {
    run_thread_safety_test(
        "src/hashing/fowler_noll_vo_1/.snapshots/fowler_noll_vo_1_16byte_snap.json",
        hash_fnv1_x32_16byte_chunks,
        hash_fnv1_x64_16byte_chunks,
        TEST_THREAD_COUNT
    );
}

#[test]
fn thread_safety_32byte_test() {
    run_thread_safety_test(
        "src/hashing/fowler_noll_vo_1/.snapshots/fowler_noll_vo_1_32byte_snap.json",
        hash_fnv1_x32_32byte_chunks,
        hash_fnv1_x64_32byte_chunks,
        TEST_THREAD_COUNT
    );
}

#[test]
fn thread_safety_64byte_test() {
    run_thread_safety_test(
        "src/hashing/fowler_noll_vo_1/.snapshots/fowler_noll_vo_1_64byte_snap.json",
        hash_fnv1_x32_32byte_chunks,
        hash_fnv1_x64_64byte_chunks,
        TEST_THREAD_COUNT
    );
}

fn run_thread_safety_test(
    snapshot_path: &str,
    hash_32_func: fn(&[u8]) -> u32,
    hash_64_func: fn(&[u8]) -> u64,
    number_of_threads: usize
) {
    let snapshots = load_snapshots(snapshot_path);
    let num_threads = number_of_threads;
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

                // Execute the hash functions
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
