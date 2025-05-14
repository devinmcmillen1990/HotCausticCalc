use crate::hashing::fowler_noll_vo_utils::{fowler_noll_vo_constants::{FNV_X32_OFFSET_BASIS, FNV_X32_PRIME, SIZE_16_BYTES, SIZE_32_BYTES, SIZE_4_BYTES, SIZE_8_BYTES}, fowler_noll_vo_x32_hash::hash_x32_chunks};

/*
    FNV-1a Pseudocode
        1. Initialize hash to OFFSET_BASIS.
        2. For each byte in the data:
            - XOR hash with the byte.
            - Multiply hash by FNV_PRIME.
        3. Return the resulting hash value.

    NOTE: Thread safe because it takes an immutable reference. And since this algorithm is not intended for large datasets, there
            is no need to consider dealing with mutable byte-arrays like we may have to use for more advanced hashing algorithms.
*/

/// Fowler-Noll-Vo 1a Hash (32-bit)
pub fn hash_fnv1a_x32(data: &[u8]) -> u32 {
    if data.is_empty() {
        return FNV_X32_OFFSET_BASIS;
    }
    let mut hash = FNV_X32_OFFSET_BASIS; // Initialize hash to OFFSET_BASIS

    for &byte in data {
        hash = hash.wrapping_mul(FNV_X32_PRIME); // Multiply hash with FNV_Prime (32-bit)
        hash ^= byte as u32; // XOR hash with byte iteration
    }

    hash // Return hash
}

pub fn hash_fnv1a_x32_4byte_chunks(data: &[u8]) -> u32 {
    if data.is_empty() {
        return FNV_X32_OFFSET_BASIS;
    }

    let mut hash = FNV_X32_OFFSET_BASIS;
    hash_x32_chunks(data, &mut hash, SIZE_4_BYTES);
    hash
}

pub fn hash_fnv1a_x32_8byte_chunks(data: &[u8]) -> u32 {
    if data.is_empty() {
        return FNV_X32_OFFSET_BASIS;
    }

    let mut hash = FNV_X32_OFFSET_BASIS;
    hash_x32_chunks(data, &mut hash, SIZE_8_BYTES);
    hash
}

pub fn hash_fnv1a_x32_16byte_chunks(data: &[u8]) -> u32 {
    if data.is_empty() {
        return FNV_X32_OFFSET_BASIS;
    }

    let mut hash = FNV_X32_OFFSET_BASIS;
    hash_x32_chunks(data, &mut hash, SIZE_16_BYTES);
    hash
}

pub fn hash_fnv1a_x32_32byte_chunks(data: &[u8]) -> u32 {
    if data.is_empty() {
        return FNV_X32_OFFSET_BASIS;
    }

    let mut hash = FNV_X32_OFFSET_BASIS;
    hash_x32_chunks(data, &mut hash, SIZE_32_BYTES);
    hash
}