use super::murmur3_constants::{C1_64, C2_64, M_64, N_64, R1_64, R2_64};

#[inline(always)]
pub fn murmur3_x64_hash(data: &[u8], seed: u64, byte_size: usize) -> u64 {
    if data.is_empty() {
        return seed;
    }

    let mut hash: u64 = seed;
    let chunks = data.chunks_exact(byte_size);
    let remainder = chunks.remainder();

    process_chunks(byte_size, &mut hash, chunks);
    process_remainder(&mut hash, remainder);
    finalize_hash(data, &mut hash);

    hash
}

#[inline(always)]
fn process_chunks(byte_size: usize, hash: &mut u64, chunks: std::slice::ChunksExact<'_, u8>) {
    let num_chunks = byte_size / 8;

    for chunk in chunks {
        for i in 0..num_chunks {
            let start = i * 8;

            let byte_chunk = u64::from_le_bytes([
                chunk[start],
                chunk[start + 1],
                chunk[start + 2],
                chunk[start + 3],
                chunk[start + 4],
                chunk[start + 5],
                chunk[start + 6],
                chunk[start + 7],
            ]);

            let mut k = byte_chunk;
            k = k.wrapping_mul(C1_64);
            k = k.rotate_left(R1_64);
            k = k.wrapping_mul(C2_64);

            *hash ^= k;
            *hash = hash.rotate_left(R2_64);
            *hash = hash.wrapping_mul(M_64).wrapping_add(N_64);
        }
    }
}

#[inline(always)]
fn process_remainder(hash: &mut u64, remainder: &[u8]) {
    let mut tail: u64 = 0;

    for (i, &byte) in remainder.iter().enumerate() {
        tail |= (byte as u64) << (i * 8);
    }

    if !remainder.is_empty() {
        tail = tail.wrapping_mul(C1_64);
        tail = tail.rotate_left(R1_64);
        tail = tail.wrapping_mul(C2_64);
        *hash ^= tail;
    }
}

#[inline(always)]
fn finalize_hash(data: &[u8], hash: &mut u64) {
    *hash ^= data.len() as u64;
    *hash ^= hash.wrapping_shr(33);
    *hash = hash.wrapping_mul(0xff51afd7ed558ccd);
    *hash ^= hash.wrapping_shr(33);
    *hash = hash.wrapping_mul(0xc4ceb9fe1a85ec53);
    *hash ^= hash.wrapping_shr(33);
}
