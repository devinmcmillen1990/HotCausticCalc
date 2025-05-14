// FNV-1/FNV-1a large prime (HEX) number that helps distribute the hash values across the entire output range.
//     - This value is/needs to be large enough to reduce the likelihood of collisions
pub const FNV_X32_OFFSET_BASIS: u32 = 0x811c9dc5;
pub const FNV_X64_OFFSET_BASIS: u64 = 0xcbf29ce484222325;

// Used to help provide beter disperse the hash values throughout the n-bit hash space.
pub const FNV_X32_PRIME: u32 = 0x01000193;
pub const FNV_X64_PRIME: u64 = 0x100000001b3;