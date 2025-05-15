use super::{murmur3_constants::MURMUR_SEED_X64, murmur3_x64_hash::murmur3_x64_hash};
use crate::utils::constants::chunk_sizes::{
    SIZE_16_BYTES, SIZE_32_BYTES, SIZE_64_BYTES, SIZE_8_BYTES,
};

pub fn murmur3_x64(data: &[u8]) -> u64 {
    if data.is_empty() {
        return MURMUR_SEED_X64 as u64;
    }

    murmur3_x64_hash(data, MURMUR_SEED_X64 as u64, SIZE_8_BYTES)
}

pub fn murmur3_x64_8byte_chunks(data: &[u8]) -> u64 {
    if data.is_empty() {
        return MURMUR_SEED_X64 as u64;
    }

    murmur3_x64_hash(data, MURMUR_SEED_X64 as u64, SIZE_8_BYTES)
}

pub fn murmur3_x64_16byte_chunks(data: &[u8]) -> u64 {
    if data.is_empty() {
        return MURMUR_SEED_X64 as u64;
    }

    murmur3_x64_hash(data, MURMUR_SEED_X64 as u64, SIZE_16_BYTES)
}

pub fn murmur3_x64_32byte_chunks(data: &[u8]) -> u64 {
    if data.is_empty() {
        return MURMUR_SEED_X64 as u64;
    }

    murmur3_x64_hash(data, MURMUR_SEED_X64 as u64, SIZE_32_BYTES)
}

pub fn murmur3_x64_64byte_chunks(data: &[u8]) -> u64 {
    if data.is_empty() {
        return MURMUR_SEED_X64 as u64;
    }

    murmur3_x64_hash(data, MURMUR_SEED_X64 as u64, SIZE_64_BYTES)
}
