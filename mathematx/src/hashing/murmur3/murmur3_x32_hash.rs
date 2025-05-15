use super::murmur3_constants::{C1_32, C2_32, M_32, N_32, R1_32, R2_32};

#[inline(always)]
pub fn murmur3_x32_hash(data: &[u8], seed: u32, byte_size: usize) -> u32 {
    if data.is_empty() {
        return seed;
    }

    let mut hash = seed;
    let chunks = data.chunks_exact(byte_size);
    let remainder = chunks.remainder();

    process_chunks(byte_size, &mut hash, chunks);
    process_remainder(&mut hash, remainder);
    finalize_hash(data, &mut hash);
    hash
}

#[inline(always)]
fn process_chunks(byte_size: usize, hash: &mut u32, chunks: std::slice::ChunksExact<'_, u8>) {
    // Since we are processing 4 bytes per iteration, the number of chunks is simply the (byte_size / 4)
    let num_chunks = byte_size / 4;

    for chunk in chunks {
        for i in 0..num_chunks {
            let start = i * 4;

            let byte_chunk = u32::from_le_bytes([
                chunk[start],
                chunk[start + 1],
                chunk[start + 2],
                chunk[start + 3],
            ]);

            let mut k = byte_chunk;
            k = k.wrapping_mul(C1_32);
            k = k.rotate_left(R1_32);
            k = k.wrapping_mul(C2_32);

            *hash ^= k;
            *hash = hash.rotate_left(R2_32);
            *hash = hash.wrapping_mul(M_32).wrapping_add(N_32);
        }
    }
}

#[inline(always)]
fn process_remainder(hash: &mut u32, remainder: &[u8]) {
    let mut tail = 0;
    for (i, &byte) in remainder.iter().enumerate() {
        // TODO: Double check this because it was added due to an overflow exception. 
        //          Double check that this is the correct value.
        if i > 3 {
            break;
        }

        tail |= (byte as u32) << (i * 8);
    }

    if !remainder.is_empty() {
        tail = tail.wrapping_mul(C1_32);
        tail = tail.rotate_left(R1_32);
        tail = tail.wrapping_mul(C2_32);
        *hash ^= tail;
    }
}

// TODO: Magic values, replace with variables
#[inline(always)]
fn finalize_hash(data: &[u8], hash: &mut u32) {
    *hash ^= data.len() as u32;
    *hash ^= hash.wrapping_shr(16);
    *hash = hash.wrapping_mul(0x85ebca6b);
    *hash ^= hash.wrapping_shr(13);
    *hash = hash.wrapping_mul(0xc2b2ae35);
    *hash ^= hash.wrapping_shr(16);
}
