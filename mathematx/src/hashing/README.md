## Shallow Depth Hashing

### Purpose
    - Fast lookups

### Collision Resistance
    - Minimal
        * Focus is more on lookup speed instead of security

# Shallow Depth Hashing Algorithms in Rust
Below is a list of commonly used shallow depth hashing algorithms in Rust, categorized by their type, best use cases, and collision resistance. Each algorithm includes additional notes and relevant references.

| Algorithm   | Type        | Best For         | Collision Resistance | Notes | Reference |
|-------------|-------------|------------------|----------------------|-------|-----------|
| **FNV-1**   | Non-Crypto  | Small Data       | Low                  | Simple and fast, but prone to collisions with certain input patterns. Suitable for small datasets. | [FNV-1/FNV-1a](https://en.wikipedia.org/wiki/Fowler%E2%80%93Noll%E2%80%93Vo_hash_function) |
| **FNV-1a**  | Non-Crypto  | Small Data       | Low                  | Slightly better distribution than FNV-1 due to altered multiplication order. | [FNV-1/FNV-1a](https://en.wikipedia.org/wiki/Fowler%E2%80%93Noll%E2%80%93Vo_hash_function) |
| **MurmurHash** | Non-Crypto  | Large Data       | Moderate             | Excellent speed and distribution, often used in databases and large data processing. | [MurmurHash](https://en.wikipedia.org/wiki/MurmurHash) |
| **SipHash** | Crypto      | Hash Maps        | High                 | Default in Rust `HashMap`. Resistant to hash collision attacks, but slower than non-crypto hashes. | [SipHash](https://en.wikipedia.org/wiki/SipHash) |
| **CityHash** | Non-Crypto  | Large Data       | Moderate             | Very fast for large inputs, ideal for file hashing. Google-developed. | [CityHash](https://opensource.googleblog.com/2011/04/introducing-cityhash.html) |
| **MetroHash** | Non-Crypto  | Large Data       | Moderate             | Similar to CityHash but optimized for even faster processing. | [MetroHash](https://github.com/jandrewrogers/MetroHash) |
| **XXHash**  | Non-Crypto  | Large Data       | Moderate             | Extremely fast; optimized for large data processing. | [XXHash](https://xxhash.com/) |
| **WyHash**  | Non-Crypto  | General Purpose  | Moderate             | Good speed and distribution; designed for high throughput and minimal collisions. | [WyHash](https://github.com/wangyi-fudan/wyhash) |
| **SpookyHash** | Non-Crypto  | Large Data       | Moderate             | Produces 128-bit output; very fast for large data sets and file hashing. | [SpookyHash](https://burtleburtle.net/bob/hash/spooky.html) |
