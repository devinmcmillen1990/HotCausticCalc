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

/***
 * NOTE: Snapshots don't contain massive dataset entries like massive strings, because this algorithm is not intended for
 *          that functionality.
 */
#[test]
fn run_base_tests() {
    let snapshots =
        load_snapshots("src/hashing/fowler_noll_vo_1/.snapshots/fowler_noll_vo_1_base_snap.json");

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

#[test]
fn run_4byte_tests() {
    let snapshots =
        load_snapshots("src/hashing/fowler_noll_vo_1/.snapshots/fowler_noll_vo_1_4byte_snap.json");

    for snapshot in snapshots.iter() {
        // Arrange
        let title = snapshot["title"].as_str().unwrap();
        let input = snapshot["input"].as_str().unwrap().as_bytes();

        let expected_32bit = snapshot["expected_32bit"].as_u64().unwrap() as u32;
        let expected_64bit = snapshot["expected_64bit"].as_u64().unwrap() as u64;

        // Act
        println!("Running test: {}", title);

        let hash_32bit = hash_fnv1_x32_4byte_chunks(input);
        let hash_64bit = hash_fnv1_x64(input); // No 4 byte setup for 64-bit

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

#[test]
fn run_8byte_tests() {
    let snapshots =
        load_snapshots("src/hashing/fowler_noll_vo_1/.snapshots/fowler_noll_vo_1_8byte_snap.json");

    for snapshot in snapshots.iter() {
        // Arrange
        let title = snapshot["title"].as_str().unwrap();
        let input = snapshot["input"].as_str().unwrap().as_bytes();

        let expected_32bit = snapshot["expected_32bit"].as_u64().unwrap() as u32;
        let expected_64bit = snapshot["expected_64bit"].as_u64().unwrap() as u64;

        // Act
        println!("Running test: {}", title);

        let hash_32bit = hash_fnv1_x32_8byte_chunks(input);
        let hash_64bit = hash_fnv1_x64_8byte_chunks(input);

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

#[test]
fn run_16byte_tests() {
    let snapshots =
        load_snapshots("src/hashing/fowler_noll_vo_1/.snapshots/fowler_noll_vo_1_16byte_snap.json");

    for snapshot in snapshots.iter() {
        // Arrange
        let title = snapshot["title"].as_str().unwrap();
        let input = snapshot["input"].as_str().unwrap().as_bytes();

        let expected_32bit = snapshot["expected_32bit"].as_u64().unwrap() as u32;
        let expected_64bit = snapshot["expected_64bit"].as_u64().unwrap() as u64;

        // Act
        println!("Running test: {}", title);

        let hash_32bit = hash_fnv1_x32_16byte_chunks(input);
        let hash_64bit = hash_fnv1_x64_16byte_chunks(input);

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

#[test]
fn run_32byte_tests() {
    let snapshots =
        load_snapshots("src/hashing/fowler_noll_vo_1/.snapshots/fowler_noll_vo_1_32byte_snap.json");

    for snapshot in snapshots.iter() {
        // Arrange
        let title = snapshot["title"].as_str().unwrap();
        let input = snapshot["input"].as_str().unwrap().as_bytes();

        let expected_32bit = snapshot["expected_32bit"].as_u64().unwrap() as u32;
        let expected_64bit = snapshot["expected_64bit"].as_u64().unwrap() as u64;

        // Act
        println!("Running test: {}", title);

        let hash_32bit = hash_fnv1_x32_32byte_chunks(input);
        let hash_64bit = hash_fnv1_x64_32byte_chunks(input);

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

#[test]
fn run_64byte_tests() {
    let snapshots =
        load_snapshots("src/hashing/fowler_noll_vo_1/.snapshots/fowler_noll_vo_1_64byte_snap.json");

    for snapshot in snapshots.iter() {
        // Arrange
        let title = snapshot["title"].as_str().unwrap();
        let input = snapshot["input"].as_str().unwrap().as_bytes();

        let expected_32bit = snapshot["expected_32bit"].as_u64().unwrap() as u32;
        let expected_64bit = snapshot["expected_64bit"].as_u64().unwrap() as u64;

        // Act
        println!("Running test: {}", title);

        let hash_32bit = hash_fnv1_x32_32byte_chunks(input); // No 64 byte setup for 62-bit
        let hash_64bit = hash_fnv1_x64_64byte_chunks(input);

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