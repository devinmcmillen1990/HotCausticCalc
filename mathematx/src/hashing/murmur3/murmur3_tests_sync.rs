use super::murmur3_x32::{
    murmur3_x32, murmur3_x32_4byte_chunks, murmur3_x32_8byte_chunks,
    murmur3_x32_16byte_chunks, murmur3_x32_32byte_chunks,
};
use super::murmur3_x64::{
    murmur3_x64, murmur3_x64_8byte_chunks, murmur3_x64_16byte_chunks,
    murmur3_x64_32byte_chunks, murmur3_x64_64byte_chunks,
};
use crate::hashing::murmur3::murmur3_tests_assertions::run_murmur3_hash_tests;

#[test]
fn run_base_tests() {
    run_murmur3_hash_tests(
        "src/hashing/murmur3/.snapshots/murmur3_base_snap.json",
        murmur3_x32,
        murmur3_x64,
    );
}

// #[test]
// fn run_4byte_tests() {
//     run_murmur3_hash_tests(
//         "src/hashing/murmur3/.snapshots/murmur3_4byte_snap.json",
//         murmur3_x32_4byte_chunks,
//         murmur3_x64_8byte_chunks,
//     );
// }

// #[test]
// fn run_8byte_tests() {
//     run_murmur3_hash_tests(
//         "src/hashing/murmur3/.snapshots/murmur3_8byte_snap.json",
//         murmur3_x32_8byte_chunks,
//         murmur3_x64_8byte_chunks,
//     );
// }

// #[test]
// fn run_16byte_tests() {
//     run_murmur3_hash_tests(
//         "src/hashing/murmur3/.snapshots/murmur3_16byte_snap.json",
//         murmur3_x32_16byte_chunks,
//         murmur3_x64_16byte_chunks,
//     );
// }

// #[test]
// fn run_32byte_tests() {
//     run_murmur3_hash_tests(
//         "src/hashing/murmur3/.snapshots/murmur3_32byte_snap.json",
//         murmur3_x32_32byte_chunks,
//         murmur3_x64_32byte_chunks,
//     );
// }

// #[test]
// fn run_64byte_tests() {
//     run_murmur3_hash_tests(
//         "src/hashing/murmur3/.snapshots/murmur3_64byte_snap.json",
//         murmur3_x32_32byte_chunks,  // No 64-byte processing for 32-bit
//         murmur3_x64_64byte_chunks,
//     );
// }
