pub mod murmur3_x32;
pub mod murmur3_x64;
mod murmur3_x32_hash;
mod murmur3_x64_hash;
mod murmur3_constants;

#[cfg(test)]
pub mod murmur3_tests_assertions;
#[cfg(test)]
mod murmur3_tests_sync;
#[cfg(test)]
mod murmur3_tests_async;