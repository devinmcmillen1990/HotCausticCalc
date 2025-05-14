use crate::hashing::fowler_noll_vo_1::fowler_noll_vo_1_x32::{
    hash_fnv1_x32, hash_fnv1_x32_16byte_chunks, hash_fnv1_x32_32byte_chunks,
    hash_fnv1_x32_4byte_chunks, hash_fnv1_x32_8byte_chunks,
};
use crate::hashing::fowler_noll_vo_1::fowler_noll_vo_1_x64::{
    hash_fnv1_x64, hash_fnv1_x64_16byte_chunks, hash_fnv1_x64_32byte_chunks,
    hash_fnv1_x64_64byte_chunks, hash_fnv1_x64_8byte_chunks,
};
use crate::utils::snapshot::loader::load_snapshots;

// TODO: statically load the snap shots because we don't need all this File IO.
// TODO: Duplicate testing around files
use std::sync::{Arc, Barrier};
use std::thread;

#[test]
fn thread_safety_base_test() {
    let snapshots =
        load_snapshots("src/hashing/fowler_noll_vo_1/.snapshots/fowler_noll_vo_1_base_snap.json");

    let num_threads = 100;
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

                assert_eq!(
                    hash_32bit, expected_32bit,
                    "32-bit FNV-1 mismatch in thread {}",
                    i
                );
                assert_eq!(
                    hash_64bit, expected_64bit,
                    "64-bit FNV-1 mismatch in thread {}",
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

#[test]
fn thread_safety_4byte_test() {
    let snapshots =
        load_snapshots("src/hashing/fowler_noll_vo_1/.snapshots/fowler_noll_vo_1_4byte_snap.json");

    let num_threads = 100;
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
                let hash_32bit = hash_fnv1_x32_4byte_chunks(input);
                let hash_64bit = hash_fnv1_x64(input);

                assert_eq!(
                    hash_32bit, expected_32bit,
                    "32-bit FNV-1 mismatch in thread {}",
                    i
                );
                assert_eq!(
                    hash_64bit, expected_64bit,
                    "64-bit FNV-1 mismatch in thread {}",
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

#[test]
fn thread_safety_8byte_test() {
    let snapshots =
        load_snapshots("src/hashing/fowler_noll_vo_1/.snapshots/fowler_noll_vo_1_8byte_snap.json");

    let num_threads = 100;
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
                let hash_32bit = hash_fnv1_x32_8byte_chunks(input);
                let hash_64bit = hash_fnv1_x64_8byte_chunks(input);

                assert_eq!(
                    hash_32bit, expected_32bit,
                    "32-bit FNV-1 mismatch in thread {}",
                    i
                );
                assert_eq!(
                    hash_64bit, expected_64bit,
                    "64-bit FNV-1 mismatch in thread {}",
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

#[test]
fn thread_safety_16byte_test() {
    let snapshots =
        load_snapshots("src/hashing/fowler_noll_vo_1/.snapshots/fowler_noll_vo_1_16byte_snap.json");

    let num_threads = 100;
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
                let hash_32bit = hash_fnv1_x32_16byte_chunks(input);
                let hash_64bit = hash_fnv1_x64_16byte_chunks(input);

                assert_eq!(
                    hash_32bit, expected_32bit,
                    "32-bit FNV-1 mismatch in thread {}",
                    i
                );
                assert_eq!(
                    hash_64bit, expected_64bit,
                    "64-bit FNV-1 mismatch in thread {}",
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

#[test]
fn thread_safety_32byte_test() {
    let snapshots =
        load_snapshots("src/hashing/fowler_noll_vo_1/.snapshots/fowler_noll_vo_1_32byte_snap.json");

    let num_threads = 100;
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
                let hash_32bit = hash_fnv1_x32_32byte_chunks(input);
                let hash_64bit = hash_fnv1_x64_32byte_chunks(input);

                assert_eq!(
                    hash_32bit, expected_32bit,
                    "32-bit FNV-1 mismatch in thread {}",
                    i
                );
                assert_eq!(
                    hash_64bit, expected_64bit,
                    "64-bit FNV-1 mismatch in thread {}",
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

#[test]
fn thread_safety_64byte_test() {
    let snapshots =
        load_snapshots("src/hashing/fowler_noll_vo_1/.snapshots/fowler_noll_vo_1_64byte_snap.json");

    let num_threads = 100;
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
                let hash_32bit = hash_fnv1_x32_32byte_chunks(input);
                let hash_64bit = hash_fnv1_x64_64byte_chunks(input);

                assert_eq!(
                    hash_32bit, expected_32bit,
                    "32-bit FNV-1 mismatch in thread {}",
                    i
                );
                assert_eq!(
                    hash_64bit, expected_64bit,
                    "64-bit FNV-1 mismatch in thread {}",
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
