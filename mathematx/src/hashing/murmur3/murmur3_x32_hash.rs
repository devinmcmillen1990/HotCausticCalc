use super::murmur3_constants::{C1_32, C2_32, M_32, N_32, R1_32, R2_32};

/*
Bench Results

The 32-bit chunk sizes show minimal variance in execution time, ranging from approximately 2.95 ms to 3.02 ms, regardless of the chunk size (4, 8, 16, 32 bytes).

    This suggests that the overhead of managing smaller chunks is not being mitigated effectively.

    We should expect 16-byte and 32-byte chunk sizes to outperform 4-byte and 8-byte chunk sizes, but that is not 
    clearly evident here.


32-bit Processing Analysis:
    The performance across all chunk sizes is quite close, suggesting a bottleneck:

        The process_chunks function might not be fully optimized to leverage the larger chunk sizes effectively.

        Potential optimization: Apply loop unrolling or SIMD instructions to maximize throughput for larger chunks.
*/

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
        // this has to do this the byte size because (8 * 4 == 32) and anything larger than (i == 3) will cause a shift overflow
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
