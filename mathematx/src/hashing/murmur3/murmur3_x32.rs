use super::{murmur3_constants::MURMUR_SEED_X32, murmur3_x32_hash::murmur3_x32_hash};
use crate::utils::constants::chunk_sizes::{
    SIZE_16_BYTES, SIZE_32_BYTES, SIZE_4_BYTES, SIZE_8_BYTES,
};

pub fn murmur3_x32(data: &[u8]) -> u32 {
    if data.is_empty() {
        return MURMUR_SEED_X32;
    }

    murmur3_x32_hash(data, MURMUR_SEED_X32, SIZE_4_BYTES)
}

pub fn murmur3_x32_4byte_chunks(data: &[u8]) -> u32 {
    if data.is_empty() {
        return MURMUR_SEED_X32;
    }

    murmur3_x32_hash(data, MURMUR_SEED_X32, SIZE_4_BYTES)
}

pub fn murmur3_x32_8byte_chunks(data: &[u8]) -> u32 {
    if data.is_empty() {
        return MURMUR_SEED_X32;
    }

    murmur3_x32_hash(data, MURMUR_SEED_X32, SIZE_8_BYTES)
}

pub fn murmur3_x32_16byte_chunks(data: &[u8]) -> u32 {
    if data.is_empty() {
        return MURMUR_SEED_X32;
    }

    murmur3_x32_hash(data, MURMUR_SEED_X32, SIZE_16_BYTES)
}

pub fn murmur3_x32_32byte_chunks(data: &[u8]) -> u32 {
    if data.is_empty() {
        return MURMUR_SEED_X32;
    }

    murmur3_x32_hash(data, MURMUR_SEED_X32, SIZE_32_BYTES)
}
