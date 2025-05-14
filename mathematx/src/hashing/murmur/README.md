# MurmurHash Overview

MurmurHash is a non-cryptographic hashing algorithm developed by Austin Appleby in 2008. It is designed for fast, high-quality hashing with excellent dispersion and low collision rates. The algorithm is widely used in data structures like hash tables, Bloom filters, and distributed systems.

## Pros and Cons of MurmurHash

| **Pros**                      | **Cons**                                  |
| ----------------------------- | ----------------------------------------- |
| Extremely fast                | Not cryptographically secure              |
| Excellent distribution        | Vulnerable to hash collision attacks      |
| Low collision rate            | Requires little-endian byte order         |
| Suitable for large datasets   | More complex than simpler hashes like FNV |
| High throughput for bulk data | Requires multiple rounds for best results |

## Variants of MurmurHash

| **Version**     | **Output Size** | **Details**                                    |
| --------------- | --------------- | ---------------------------------------------- |
| **MurmurHash1** | 32-bit          | Original version, outdated and not recommended |
| **MurmurHash2** | 32-bit/64-bit   | Faster, but minor flaws in dispersion          |
| **MurmurHash3** | 32-bit/128-bit  | Improved dispersion, highly optimized          |

## Why Use MurmurHash?

* It provides a good balance between speed and dispersion, making it ideal for large datasets and data structures.
* Its implementation is relatively simple, but it uses a series of bitwise operations to achieve effective mixing.
* It is a common choice for non-cryptographic use cases, such as data indexing, caching, and Bloom filters.

## Algorithm Details

MurmurHash works by:

1. Splitting the input data into 4-byte chunks (or 8-byte for 64-bit).
2. Applying a series of bitwise operations (rotate, multiply, XOR).
3. Finalizing with an avalanche step to ensure even bit distribution.

## MurmurHash3 Pseudocode (32-bit)

```
Initialize hash to a seed value
For each 4-byte chunk:
    - Multiply by a constant
    - Rotate bits
    - XOR with hash
Final Step:
    - XOR with length of input
    - Multiply by a constant
    - Apply a final XOR step to spread bits
```

## Example Pseudocode for MurmurHash3 (32-bit)

```
seed = 0x5bd1e995
hash = seed

For each 4-byte chunk:
    - Multiply by 0x5bd1e995
    - Rotate left by 15 bits
    - XOR with hash

Final Step:
    - XOR with length of input
    - Multiply by a constant
    - Apply a final XOR step to spread bits
```