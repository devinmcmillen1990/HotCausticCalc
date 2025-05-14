// FNV-1/FNV-1a large prime (HEX) number that helps distribute the hash values across the entire output range.
//     - This value is/needs to be large enough to reduce the likelihood of collisions
pub const FNV_X32_OFFSET_BASIS: u32 = 0x811c9dc5;
pub const FNV_X64_OFFSET_BASIS: u64 = 0xcbf29ce484222325;

// Used to help provide beter disperse the hash values throughout the n-bit hash space.
pub const FNV_X32_PRIME: u32 = 0x01000193;
pub const FNV_X64_PRIME: u64 = 0x100000001b3;

/*********************************************************************************************************/

/*
    FNV-1 Pseudocode
        1. Initialize hash to OFFSET_BASIS.
        2. For each byte in the data:
            - Multiply hash by FNV_PRIME.
            - XOR hash with the byte.
        3. Return the resulting hash value.
*/
/// Folwer-Noll-Vo 1 Hash (32-bit)
pub fn hash_fnv1_x32(data: &[u8]) -> u32
{
    let mut hash = FNV_X32_OFFSET_BASIS;            // Initialize hash to OFFSET_BASIS

    for &byte in data {
        hash = hash.wrapping_mul(FNV_X32_PRIME);    // Multiply hash with FNV_Prime (32-bit)
        hash ^= byte as u32;                        // XOR hash with the byte iteration
    }

    hash                                            // Return hash
}

/// Fowler-Noll-Vo 1 Hash (64-bit)
pub fn hash_fnv1_x64(data: &[u8]) -> u64
{
    let mut hash = FNV_X64_OFFSET_BASIS;            // Initialize hash to OFFSET_BASIS

    for &byte in data {
        hash = hash.wrapping_mul(FNV_X64_PRIME);    // Multiply hash with FNV_Prime (64-bit)
        hash ^= byte as u64;                        // XOR hash with the byte iteration
    }

    hash                                            // Return hash
}

/*********************************************************************************************************/

/*
    FNV-1a Pseudocode
        1. Initialize hash to OFFSET_BASIS.
        2. For each byte in the data:
            - XOR hash with the byte.
            - Multiply hash by FNV_PRIME.
        3. Return the resulting hash value.
*/
/// Fowler-Noll-Vo 1a Hash (32-bit)
pub fn hash_fnv1a_x32(data: &[u8]) -> u32
{
    let mut hash = FNV_X32_OFFSET_BASIS;       // Initialize hash to OFFSET_BASIS

    for &byte in data {
        hash ^= byte as u32;                        // XOR hash with byte iteration
        hash = hash.wrapping_mul(FNV_X32_PRIME);    // Multiply hash with FNV_Prime (32-bit)
        
    }

    hash                                            // Return hash
}

/// Fowler-Noll-Vo 1a Hash (64-bit)
pub fn hash_fnv1a_x64(data: &[u8]) -> u64
{
    let mut hash = FNV_X64_OFFSET_BASIS;       // Initialize hash to OFFSET_BASIS

    for &byte in data {
        hash ^= byte as u64;                        // XOR hash with the byte iteration
        hash = hash.wrapping_mul(FNV_X64_PRIME);    // Multiply hash with FNV_Prime (64-bit)
    }

    hash                                            // Return hash
}
