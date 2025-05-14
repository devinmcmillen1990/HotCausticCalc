use super::fowler_noll_vo_1a_x32::{
    hash_fnv1a_x32, hash_fnv1a_x32_16byte_chunks, hash_fnv1a_x32_32byte_chunks,
    hash_fnv1a_x32_4byte_chunks, hash_fnv1a_x32_8byte_chunks,
};
use super::fowler_noll_vo_1a_x64::{
    hash_fnv1a_x64, hash_fnv1a_x64_16byte_chunks, hash_fnv1a_x64_32byte_chunks,
    hash_fnv1a_x64_64byte_chunks, hash_fnv1a_x64_8byte_chunks,
};
use crate::hashing::fowler_noll_vo_utils::fowler_noll_vo_test_assertions::run_thread_safety_test;

/***
 * NOTE: Snapshots don't contain massive dataset entries like massive strings, because this algorithm is not intended for
 *          that functionality.
 */

const TEST_THREAD_COUNT: usize = 100;

#[test]
fn thread_safety_base_test() {
    run_thread_safety_test(
        "src/hashing/fowler_noll_vo_1a/.snapshots/fowler_noll_vo_1a_base_snap.json",
        hash_fnv1a_x32,
        hash_fnv1a_x64,
        TEST_THREAD_COUNT,
    );
}

#[test]
fn thread_safety_4byte_test() {
    run_thread_safety_test(
        "src/hashing/fowler_noll_vo_1a/.snapshots/fowler_noll_vo_1a_4byte_snap.json",
        hash_fnv1a_x32_4byte_chunks,
        hash_fnv1a_x64,
        TEST_THREAD_COUNT,
    );
}

#[test]
fn thread_safety_8byte_test() {
    run_thread_safety_test(
        "src/hashing/fowler_noll_vo_1a/.snapshots/fowler_noll_vo_1a_8byte_snap.json",
        hash_fnv1a_x32_8byte_chunks,
        hash_fnv1a_x64_8byte_chunks,
        TEST_THREAD_COUNT,
    );
}

#[test]
fn thread_safety_16byte_test() {
    run_thread_safety_test(
        "src/hashing/fowler_noll_vo_1a/.snapshots/fowler_noll_vo_1a_16byte_snap.json",
        hash_fnv1a_x32_16byte_chunks,
        hash_fnv1a_x64_16byte_chunks,
        TEST_THREAD_COUNT,
    );
}

#[test]
fn thread_safety_32byte_test() {
    run_thread_safety_test(
        "src/hashing/fowler_noll_vo_1a/.snapshots/fowler_noll_vo_1a_32byte_snap.json",
        hash_fnv1a_x32_32byte_chunks,
        hash_fnv1a_x64_32byte_chunks,
        TEST_THREAD_COUNT,
    );
}

#[test]
fn thread_safety_64byte_test() {
    run_thread_safety_test(
        "src/hashing/fowler_noll_vo_1a/.snapshots/fowler_noll_vo_1a_64byte_snap.json",
        hash_fnv1a_x32_32byte_chunks,
        hash_fnv1a_x64_64byte_chunks,
        TEST_THREAD_COUNT,
    );
}
