use super::fowler_noll_vo_1a_constants::{FNV_X32_OFFSET_BASIS, FNV_X32_PRIME, FNV_X64_OFFSET_BASIS, FNV_X64_PRIME};

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
pub fn hash_fnv1a_x32(data: &[u8]) -> u32
{
    let mut hash = FNV_X32_OFFSET_BASIS;       // Initialize hash to OFFSET_BASIS

    for &byte in data {
        hash = hash.wrapping_mul(FNV_X32_PRIME);    // Multiply hash with FNV_Prime (32-bit)
        hash ^= byte as u32;                        // XOR hash with byte iteration        
    }

    hash                                            // Return hash
}

/// Fowler-Noll-Vo 1a Hash (64-bit)
pub fn hash_fnv1a_x64(data: &[u8]) -> u64
{
    let mut hash = FNV_X64_OFFSET_BASIS;       // Initialize hash to OFFSET_BASIS

    for &byte in data {
        hash = hash.wrapping_mul(FNV_X64_PRIME);    // Multiply hash with FNV_Prime (64-bit)
        hash ^= byte as u64;                        // XOR hash with the byte iteration
    }

    hash                                            // Return hash
}
