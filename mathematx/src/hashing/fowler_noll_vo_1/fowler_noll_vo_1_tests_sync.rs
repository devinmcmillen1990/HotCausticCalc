use crate::hashing::fowler_noll_vo_1::fowler_noll_vo_1_x32::{
    fnv1_x32, fnv1_x32_16byte_chunks, fnv1_x32_32byte_chunks,
    fnv1_x32_4byte_chunks, fnv1_x32_8byte_chunks,
};
use crate::hashing::fowler_noll_vo_1::fowler_noll_vo_1_x64::{
    fnv1_x64, fnv1_x64_16byte_chunks, fnv1_x64_32byte_chunks,
    fnv1_x64_64byte_chunks, fnv1_x64_8byte_chunks,
};
use crate::hashing::fowler_noll_vo_utils::fowler_noll_vo_test_assertions::run_hash_tests;

/***
 * NOTE: Snapshots don't contain massive dataset entries like massive strings, because this algorithm is not intended for
 *          that functionality.
 */

#[test]
fn run_base_tests() {
    run_hash_tests(
        "src/hashing/fowler_noll_vo_1/.snapshots/fowler_noll_vo_1_base_snap.json",
        fnv1_x32,
        fnv1_x64,
    );
}

#[test]
fn run_4byte_tests() {
    run_hash_tests(
        "src/hashing/fowler_noll_vo_1/.snapshots/fowler_noll_vo_1_4byte_snap.json",
        fnv1_x32_4byte_chunks,
        fnv1_x64, // No 4-byte function for 64-bit
    );
}

#[test]
fn run_8byte_tests() {
    run_hash_tests(
        "src/hashing/fowler_noll_vo_1/.snapshots/fowler_noll_vo_1_8byte_snap.json",
        fnv1_x32_8byte_chunks,
        fnv1_x64_8byte_chunks,
    );
}

#[test]
fn run_16byte_tests() {
    run_hash_tests(
        "src/hashing/fowler_noll_vo_1/.snapshots/fowler_noll_vo_1_16byte_snap.json",
        fnv1_x32_16byte_chunks,
        fnv1_x64_16byte_chunks,
    );
}

#[test]
fn run_32byte_tests() {
    run_hash_tests(
        "src/hashing/fowler_noll_vo_1/.snapshots/fowler_noll_vo_1_32byte_snap.json",
        fnv1_x32_32byte_chunks,
        fnv1_x64_32byte_chunks,
    );
}

#[test]
fn run_64byte_tests() {
    run_hash_tests(
        "src/hashing/fowler_noll_vo_1/.snapshots/fowler_noll_vo_1_64byte_snap.json",
        fnv1_x32_32byte_chunks, // 32-byte function for 32-bit
        fnv1_x64_64byte_chunks,
    );
}