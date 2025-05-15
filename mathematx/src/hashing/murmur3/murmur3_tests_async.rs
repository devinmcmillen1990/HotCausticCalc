use super::{
    murmur3_tests_assertions::run_murmur3_thread_safety_test,
    murmur3_x32::{
        murmur3_x32, murmur3_x32_16byte_chunks, murmur3_x32_32byte_chunks,
        murmur3_x32_4byte_chunks, murmur3_x32_8byte_chunks,
    },
    murmur3_x64::{
        murmur3_x64, murmur3_x64_16byte_chunks, murmur3_x64_32byte_chunks,
        murmur3_x64_64byte_chunks, murmur3_x64_8byte_chunks,
    },
};

const TEST_THREAD_COUNT: usize = 100;

#[test]
fn thread_safety_base_test() {
    run_murmur3_thread_safety_test(
        "src/hashing/murmur3/.snapshots/murmur3_base_snap.json",
        murmur3_x32,
        murmur3_x64,
        TEST_THREAD_COUNT,
    )
}

#[test]
fn thread_safety_4byte_test() {
    run_murmur3_thread_safety_test(
        "src/hashing/murmur3/.snapshots/murmur3_4byte_snap.json",
        murmur3_x32_4byte_chunks,
        murmur3_x64_8byte_chunks,
        TEST_THREAD_COUNT,
    );
}

#[test]
fn thread_safety_8byte_test() {
    run_murmur3_thread_safety_test(
        "src/hashing/murmur3/.snapshots/murmur3_8byte_snap.json",
        murmur3_x32_8byte_chunks,
        murmur3_x64_8byte_chunks,
        TEST_THREAD_COUNT,
    );
}

#[test]
fn thread_safety_16byte_test() {
    run_murmur3_thread_safety_test(
        "src/hashing/murmur3/.snapshots/murmur3_16byte_snap.json",
        murmur3_x32_16byte_chunks,
        murmur3_x64_16byte_chunks,
        TEST_THREAD_COUNT,
    );
}

#[test]
fn thread_safety_32byte_test() {
    run_murmur3_thread_safety_test(
        "src/hashing/murmur3/.snapshots/murmur3_32byte_snap.json",
        murmur3_x32_32byte_chunks,
        murmur3_x64_32byte_chunks,
        TEST_THREAD_COUNT,
    );
}

#[test]
fn thread_safety_64byte_test() {
    run_murmur3_thread_safety_test(
        "src/hashing/murmur3/.snapshots/murmur3_64byte_snap.json",
        murmur3_x32_32byte_chunks,
        murmur3_x64_64byte_chunks,
        TEST_THREAD_COUNT,
    );
}
