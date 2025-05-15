use crate::{
    hashing::fowler_noll_vo_utils::{
        fowler_noll_vo_constants::{FNV_X64_OFFSET_BASIS, FNV_X64_PRIME},
        fowler_noll_vo_x64_hash::hash_x64_chunks,
    },
    utils::constants::chunk_sizes::{SIZE_16_BYTES, SIZE_32_BYTES, SIZE_64_BYTES, SIZE_8_BYTES},
};

/// Fowler-Noll-Vo 1a Hash (64-bit)
pub fn fnv1a_x64(data: &[u8]) -> u64 {
    if data.is_empty() {
        return FNV_X64_OFFSET_BASIS;
    }

    let mut hash = FNV_X64_OFFSET_BASIS; // Initialize hash to OFFSET_BASIS

    for &byte in data {
        hash = hash.wrapping_mul(FNV_X64_PRIME); // Multiply hash with FNV_Prime (64-bit)
        hash ^= byte as u64; // XOR hash with the byte iteration
    }

    hash // Return hash
}

pub fn fnv1a_x64_8byte_chunks(data: &[u8]) -> u64 {
    if data.is_empty() {
        return FNV_X64_OFFSET_BASIS;
    }

    let mut hash = FNV_X64_OFFSET_BASIS;
    hash_x64_chunks(data, &mut hash, SIZE_8_BYTES);
    hash
}

pub fn fnv1a_x64_16byte_chunks(data: &[u8]) -> u64 {
    if data.is_empty() {
        return FNV_X64_OFFSET_BASIS;
    }

    let mut hash = FNV_X64_OFFSET_BASIS;
    hash_x64_chunks(data, &mut hash, SIZE_16_BYTES);
    hash
}

pub fn fnv1a_x64_32byte_chunks(data: &[u8]) -> u64 {
    if data.is_empty() {
        return FNV_X64_OFFSET_BASIS;
    }

    let mut hash = FNV_X64_OFFSET_BASIS;
    hash_x64_chunks(data, &mut hash, SIZE_32_BYTES);
    hash
}

pub fn fnv1a_x64_64byte_chunks(data: &[u8]) -> u64 {
    if data.is_empty() {
        return FNV_X64_OFFSET_BASIS;
    }

    let mut hash = FNV_X64_OFFSET_BASIS;
    hash_x64_chunks(data, &mut hash, SIZE_64_BYTES);
    hash
}
