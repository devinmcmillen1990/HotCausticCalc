use crate::{hashing::fowler_noll_vo_utils::{
    fowler_noll_vo_constants::{
        FNV_X64_OFFSET_BASIS, FNV_X64_PRIME
    },
    fowler_noll_vo_x64_hash::hash_x64_chunks,
}, utils::{constants::chunk_sizes::{SIZE_16_BYTES, SIZE_32_BYTES, SIZE_64_BYTES, SIZE_8_BYTES}, prefetch::x64_prefetch::prefetch}};

/*
    FNV-1 Pseudocode
        1. Initialize hash to OFFSET_BASIS.
        2. For each byte in the data:
            - Multiply hash by FNV_PRIME.
            - XOR hash with the byte.
        3. Return the resulting hash value.

    NOTE: Thread safe because it takes an immutable reference. And since this algorithm is not intended for large datasets, there
            is no need to consider dealing with mutable byte-arrays like we may have to use for more advanced hashing algorithms.
*/

/// Fowler-Noll-Vo 1 Hash (64-bit)
pub fn hash_fnv1_x64(data: &[u8]) -> u64 {
    let mut hash = FNV_X64_OFFSET_BASIS; // Initialize hash to OFFSET_BASIS

    if data.is_empty() {
        return hash;
    }

    for &byte in data {
        #[cfg(target_arch = "x86_64")]
        unsafe {
            prefetch(&byte);
        }

        hash = hash.wrapping_mul(FNV_X64_PRIME); // Multiply hash with FNV_Prime (64-bit)
        hash ^= byte as u64; // XOR hash with the byte iteration
    }

    hash // Return hash
}

pub fn hash_fnv1_x64_8byte_chunks(data: &[u8]) -> u64 {
    if data.is_empty() {
        return FNV_X64_OFFSET_BASIS;
    }

    let mut hash = FNV_X64_OFFSET_BASIS;
    hash_x64_chunks(data, &mut hash, SIZE_8_BYTES);
    hash
}

pub fn hash_fnv1_x64_16byte_chunks(data: &[u8]) -> u64 {
    if data.is_empty() {
        return FNV_X64_OFFSET_BASIS;
    }

    let mut hash = FNV_X64_OFFSET_BASIS;
    hash_x64_chunks(data, &mut hash, SIZE_16_BYTES);
    hash
}

pub fn hash_fnv1_x64_32byte_chunks(data: &[u8]) -> u64 {
    if data.is_empty() {
        return FNV_X64_OFFSET_BASIS;
    }

    let mut hash = FNV_X64_OFFSET_BASIS;
    hash_x64_chunks(data, &mut hash, SIZE_32_BYTES);
    hash
}

pub fn hash_fnv1_x64_64byte_chunks(data: &[u8]) -> u64 {
    if data.is_empty() {
        return FNV_X64_OFFSET_BASIS;
    }

    let mut hash = FNV_X64_OFFSET_BASIS;
    hash_x64_chunks(data, &mut hash, SIZE_64_BYTES);
    hash
}
