use super::fowler_noll_vo_1_constants::{
    FNV_X64_OFFSET_BASIS, FNV_X64_PRIME, SIZE_16_BYTES, SIZE_32_BYTES, SIZE_64_BYTES, SIZE_8_BYTES,
};

#[cfg(target_arch = "x86_64")]
#[inline(always)]
unsafe fn prefetch(addr: *const u8) {
    core::arch::x86_64::_mm_prefetch(addr as *const i8, core::arch::x86_64::_MM_HINT_T0);
}

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

fn hash_x64_chunks(data: &[u8], hash: &mut u64, byte_size: usize) {
    let chunks = data.chunks_exact(byte_size);
    let remainder = chunks.remainder();

    // Loop Unrolling - Process multiple bytes per iteration.
    for chunk in chunks {
        let byte_chunk_1 = u64::from_le_bytes([
            chunk[0], chunk[1], chunk[2], chunk[3], chunk[4], chunk[5], chunk[6], chunk[7],
        ]);
        *hash = hash.wrapping_mul(FNV_X64_PRIME);
        *hash ^= byte_chunk_1;

        if byte_size == SIZE_8_BYTES {
            break;
        }

        let byte_chunk_2 = u64::from_le_bytes([
            chunk[8], chunk[9], chunk[10], chunk[11], chunk[12], chunk[13], chunk[14], chunk[15],
        ]);
        *hash = hash.wrapping_mul(FNV_X64_PRIME);
        *hash ^= byte_chunk_2;

        if byte_size == SIZE_16_BYTES {
            break;
        }

        let byte_chunk_3 = u64::from_le_bytes([
            chunk[16], chunk[17], chunk[18], chunk[19], chunk[20], chunk[21], chunk[22], chunk[23],
        ]);
        *hash = hash.wrapping_mul(FNV_X64_PRIME);
        *hash ^= byte_chunk_3;

        let byte_chunk_4 = u64::from_le_bytes([
            chunk[16], chunk[17], chunk[18], chunk[19], chunk[20], chunk[21], chunk[22], chunk[23],
        ]);
        *hash = hash.wrapping_mul(FNV_X64_PRIME);
        *hash ^= byte_chunk_4;

        if byte_size == SIZE_32_BYTES {
            break;
        }

        let byte_chunk_5 = u64::from_le_bytes([
            chunk[24], chunk[25], chunk[26], chunk[27], chunk[28], chunk[29], chunk[30], chunk[31],
        ]);
        *hash = hash.wrapping_mul(FNV_X64_PRIME);
        *hash ^= byte_chunk_5;

        let byte_chunk_6 = u64::from_le_bytes([
            chunk[16], chunk[17], chunk[18], chunk[19], chunk[20], chunk[21], chunk[22], chunk[23],
        ]);
        *hash = hash.wrapping_mul(FNV_X64_PRIME);
        *hash ^= byte_chunk_6;

        let byte_chunk_7 = u64::from_le_bytes([
            chunk[16], chunk[17], chunk[18], chunk[19], chunk[20], chunk[21], chunk[22], chunk[23],
        ]);
        *hash = hash.wrapping_mul(FNV_X64_PRIME);
        *hash ^= byte_chunk_7;

        let byte_chunk_8 = u64::from_le_bytes([
            chunk[16], chunk[17], chunk[18], chunk[19], chunk[20], chunk[21], chunk[22], chunk[23],
        ]);
        *hash = hash.wrapping_mul(FNV_X64_PRIME);
        *hash ^= byte_chunk_8;
    }

    for &byte in remainder {
        #[cfg(target_arch = "x86_64")]
        unsafe {
            prefetch(&byte);
        }

        *hash = hash.wrapping_mul(FNV_X64_PRIME); // Multiply hash with FNV_Prime (64-bit)
        *hash ^= byte as u64; // XOR hash with the byte iteration
    }
}
