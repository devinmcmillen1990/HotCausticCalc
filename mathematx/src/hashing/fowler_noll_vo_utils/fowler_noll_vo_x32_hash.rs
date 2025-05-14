use super::fowler_noll_vo_constants::{FNV_X32_PRIME, SIZE_16_BYTES, SIZE_4_BYTES, SIZE_8_BYTES};
use crate::hashing::fowler_noll_vo_utils::x64_prefetch::prefetch;

pub fn hash_x32_chunks(data: &[u8], hash: &mut u32, byte_size: usize) {
    let chunks = data.chunks_exact(byte_size);
    let remainder = chunks.remainder();

    for chunk in chunks {
        let byte_chunk_1 = u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
        *hash = hash.wrapping_mul(FNV_X32_PRIME);
        *hash ^= byte_chunk_1;

        if byte_size == SIZE_4_BYTES {
            continue;
        }

        let byte_chunk_2 = u32::from_le_bytes([chunk[4], chunk[5], chunk[6], chunk[7]]);
        *hash = hash.wrapping_mul(FNV_X32_PRIME);
        *hash ^= byte_chunk_2;

        if byte_size == SIZE_8_BYTES {
            continue;
        }

        let byte_chunk_3 = u32::from_le_bytes([chunk[8], chunk[9], chunk[10], chunk[11]]);
        *hash = hash.wrapping_mul(FNV_X32_PRIME);
        *hash ^= byte_chunk_3;

        let byte_chunk_4 = u32::from_le_bytes([chunk[12], chunk[13], chunk[14], chunk[15]]);
        *hash = hash.wrapping_mul(FNV_X32_PRIME);
        *hash ^= byte_chunk_4;

        if byte_size == SIZE_16_BYTES {
            continue;
        }

        let byte_chunk_5 = u32::from_le_bytes([chunk[16], chunk[17], chunk[18], chunk[19]]);
        *hash = hash.wrapping_mul(FNV_X32_PRIME);
        *hash ^= byte_chunk_5;

        let byte_chunk_6 = u32::from_le_bytes([chunk[20], chunk[21], chunk[22], chunk[23]]);
        *hash = hash.wrapping_mul(FNV_X32_PRIME);
        *hash ^= byte_chunk_6;

        let byte_chunk_7 = u32::from_le_bytes([chunk[24], chunk[25], chunk[26], chunk[27]]);
        *hash = hash.wrapping_mul(FNV_X32_PRIME);
        *hash ^= byte_chunk_7;

        let byte_chunk_8 = u32::from_le_bytes([chunk[28], chunk[29], chunk[30], chunk[31]]);
        *hash = hash.wrapping_mul(FNV_X32_PRIME);
        *hash ^= byte_chunk_8;
    }

    for &byte in remainder {
        #[cfg(target_arch = "x86_64")]
        unsafe {
            prefetch(&byte);
        }

        *hash = hash.wrapping_mul(FNV_X32_PRIME);
        *hash ^= byte as u32;
    }
}
