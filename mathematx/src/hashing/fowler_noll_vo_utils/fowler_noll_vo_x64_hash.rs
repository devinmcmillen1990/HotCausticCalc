use super::fowler_noll_vo_constants::FNV_X64_PRIME;
use crate::utils::{
    constants::chunk_sizes::{SIZE_16_BYTES, SIZE_32_BYTES, SIZE_8_BYTES},
    prefetch::x64_prefetch::prefetch,
};

// TODO: Replace the duplication with a nice for loop
pub fn hash_x64_chunks(data: &[u8], hash: &mut u64, byte_size: usize) {
    let chunks = data.chunks_exact(byte_size);
    let remainder = chunks.remainder();

    for chunk in chunks {
        let byte_chunk_1 = u64::from_le_bytes([
            chunk[0], chunk[1], chunk[2], chunk[3], chunk[4], chunk[5], chunk[6], chunk[7],
        ]);
        *hash = hash.wrapping_mul(FNV_X64_PRIME);
        *hash ^= byte_chunk_1;

        if byte_size == SIZE_8_BYTES {
            continue;
        }

        let byte_chunk_2 = u64::from_le_bytes([
            chunk[8], chunk[9], chunk[10], chunk[11], chunk[12], chunk[13], chunk[14], chunk[15],
        ]);
        *hash = hash.wrapping_mul(FNV_X64_PRIME);
        *hash ^= byte_chunk_2;

        if byte_size == SIZE_16_BYTES {
            continue;
        }

        let byte_chunk_3 = u64::from_le_bytes([
            chunk[16], chunk[17], chunk[18], chunk[19], chunk[20], chunk[21], chunk[22], chunk[23],
        ]);
        *hash = hash.wrapping_mul(FNV_X64_PRIME);
        *hash ^= byte_chunk_3;

        let byte_chunk_4 = u64::from_le_bytes([
            chunk[24], chunk[25], chunk[26], chunk[27], chunk[28], chunk[29], chunk[30], chunk[31],
        ]);
        *hash = hash.wrapping_mul(FNV_X64_PRIME);
        *hash ^= byte_chunk_4;

        if byte_size == SIZE_32_BYTES {
            continue;
        }

        let byte_chunk_5 = u64::from_le_bytes([
            chunk[32], chunk[33], chunk[34], chunk[35], chunk[36], chunk[37], chunk[38], chunk[39],
        ]);
        *hash = hash.wrapping_mul(FNV_X64_PRIME);
        *hash ^= byte_chunk_5;

        let byte_chunk_6 = u64::from_le_bytes([
            chunk[40], chunk[41], chunk[42], chunk[43], chunk[44], chunk[45], chunk[46], chunk[47],
        ]);
        *hash = hash.wrapping_mul(FNV_X64_PRIME);
        *hash ^= byte_chunk_6;

        let byte_chunk_7 = u64::from_le_bytes([
            chunk[48], chunk[49], chunk[50], chunk[51], chunk[52], chunk[53], chunk[54], chunk[55],
        ]);
        *hash = hash.wrapping_mul(FNV_X64_PRIME);
        *hash ^= byte_chunk_7;

        let byte_chunk_8 = u64::from_le_bytes([
            chunk[56], chunk[57], chunk[58], chunk[59], chunk[60], chunk[61], chunk[62], chunk[63],
        ]);
        *hash = hash.wrapping_mul(FNV_X64_PRIME);
        *hash ^= byte_chunk_8;
    }

    for &byte in remainder {
        #[cfg(target_arch = "x86_64")]
        unsafe {
            prefetch(&byte);
        }

        *hash = hash.wrapping_mul(FNV_X64_PRIME);
        *hash ^= byte as u64;
    }
}
