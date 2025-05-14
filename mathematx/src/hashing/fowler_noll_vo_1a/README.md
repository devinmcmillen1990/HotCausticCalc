# FNV-1a Hash Overview

The Fowler-Noll-Vo (FNV) Hash algorithm is a simple, non-cryptographic hash function designed for fast, lightweight hashing of small datasets. It was created by Glenn Fowler, Landon Curt Noll, and Phong Vo in the early 1990s.

## Pros and Cons of FNV-1/1a

| **Pros**                           | **Cons**                                      |
| ---------------------------------- | --------------------------------------------- |
| Simple and easy to implement       | Prone to collisions for certain data patterns |
| Fast and low memory usage          | Not cryptographically secure                  |
| Good for small data                | Not ideal for large datasets                  |
| Lightweight and minimal CPU impact | Distribution can be poor for specific inputs  |

## Differences Between FNV-1 and FNV-1a

* **FNV-1:** Multiplies by the prime first, then XORs the byte.
* **FNV-1a:** XORs the byte first, then multiplies by the prime.
* FNV-1a generally has better dispersion and less clustering than FNV-1.

## Constants for FNV-1 and FNV-1a

| **Bit Length** | **Offset Basis**                                     | **FNV Prime**                                |
| -------------- | ---------------------------------------------------- | -------------------------------------------- |
| 32-bit         | `0x811C9DC5`                                         | `0x01000193`                                 |
| 64-bit         | `0xCBF29CE484222325`                                 | `0x100000001B3`                              |
| 128-bit        | `0x6C62272E07BB014262B821756295C58D`                 | `0x00000000000000000001000000000001B3`       |
| 256-bit        | `0xDD268DBCAAC55036CBB5D6B5B6D2D0C72D239D6F1D5D3A7D` | `0x00000000000000000000010000000000000001B3` |

## Why Use FNV-1/1a?

* It is extremely simple and lightweight, making it ideal for hashing small data inputs.
* It is fast due to minimal bitwise operations and minimal computational overhead.
* It is not suitable for large datasets due to its susceptibility to collisions.

## Algorithm Details

1. Initialize hash to the **Offset Basis**.
2. For each byte in the input data:

   * FNV-1: Multiply by the prime, then XOR with the byte.
   * FNV-1a: XOR with the byte, then multiply by the prime.
3. Return the resulting hash value.

## Example Pseudocode for FNV-1a (32-bit):

```
OFFSET_BASIS = 0x811C9DC5
FNV_PRIME = 0x01000193

hash = OFFSET_BASIS

for each byte in data:
    hash = hash XOR byte
    hash = hash * FNV_PRIME

return hash
```