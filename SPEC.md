# K5 / KOF: Specification & Security Model

**Author:** Lois-Kleinner Alpasan
**Version:** 0.2.0

---

## 1. Introduction

K5 (Kleinner-Kantor 5) is a post-quantum cryptographic hash family and Extendable Output Function (XOF). It combines three layers of cryptographic primitives into a single sponge-based construction:

```
Input → Sponge Absorb (Poseidon permutation) → Sponge Squeeze → Output
```

The XOF variant is called **KOF** (Kleinner Output Function). Fixed-output variants are named by their bit length: K5-512, K5-1024, K2048.

### Design Philosophy

Each layer addresses a specific attack surface:

1. **Sponge construction** — provably secure in the random oracle model (indifferentiability)
2. **Poseidon permutation** — algebraic hardness against quantum adversaries (Gröbner basis, interpolation)
3. **Memory-hard pre-processing (optional)** — ASIC/GPU resistance via Balloon hash
4. **cSHAKE-256 extension (optional)** — NIST-standard XOF for outputs > 4096 bits

---

## 2. Notation

| Symbol | Meaning |
|--------|---------|
| 𝔽ₚ | Goldilocks prime field, p = 2⁶⁴ − 2³² + 1 |
| rate | 2 field elements = 128 bits |
| capacity | 2 field elements = 128 bits |
| T | State width = 4 field elements = 256 bits |
| MDS | Maximum Distance Separable matrix |
| RF | Full rounds = 8 |
| RP | Partial rounds = 28 |

---

## 3. Field Arithmetic

The Goldilocks prime field 𝔽ₚ with p = 2⁶⁴ − 2³² + 1 is used for all algebraic operations. This field:

- Enables efficient 64-bit arithmetic on modern CPUs
- Is the native field of Plonky2 and the Polygon Zero ecosystem
- Has multiplicative order 2⁶⁴ − 2³², divisible by 7 (enabling x⁷ S-box bijection)
- Provides 128 bits of security against discrete-log based attacks

Field elements are represented as unsigned 64-bit integers in [0, p−1].

---

## 4. The Poseidon Permutation

The core permutation is Poseidon over 𝔽ₚ⁴ with the following parameters:

| Parameter | Value | Justification |
|-----------|-------|---------------|
| State width T | 4 | Balance of rate (128b) and capacity (128b) |
| Full rounds RF | 8 | Standard for 128-bit security with S-box x⁷ |
| Partial rounds RP | 28 | Conservative for 64-bit field size |
| S-box α | 7 | Bijection in 𝔽ₚ (gcd(7, p−1) = 1) |
| MDS matrix | Cauchy(4) | Provably MDS over 𝔽ₚ |

### Round Structure

Each round (0 ≤ r < RF + RP):

1. **ARK (Add Round Key)**: `state[i] += round_constants[r][i]` for all i
2. **S-box**: If r < RF/2 or r ≥ RF/2 + RP (full rounds), apply x⁷ to all elements. Otherwise (partial rounds), apply x⁷ only to state[0].
3. **MDS multiplication**: `state = M × state` where M is the Cauchy MDS matrix.

### MDS Matrix

The Cauchy matrix M₄×₄ with x = [1, 2, 3, 4] and y = [5, 6, 7, 8]:

```
M[i][j] = 1 / (xᵢ + yⱼ) mod p
```

This matrix is provably MDS (all square submatrices are non-singular), verified by exhaustive determinant computation over 𝔽ₚ.

### Round Constants

Round constants are generated deterministically from the domain tag using SHAKE-256:

```
seed = domain_tag || "POSEIDON_RC"
for each round r and each element i:
    round_constants[r][i] = int(SHAKE256(seed, (r·t + i + 1)·8))[-8:] mod p
```

This is a nothing-up-my-sleeve (NUMS) generation with full transparency.

---

## 5. The K5 Sponge Construction

### State

The state is 4 Goldilocks field elements (256 bits total):

```
State = [s₀, s₁ | s₂, s₃]
         rate (128b) | capacity (128b)
```

### Padding

Input data is padded with a 10\*1 padding:

```
padded = data || 0x80 || 0x00*N || bit_length_64
```

Where N is chosen to make `(len(padded) + 8) % 16 == 0`. The 8-byte big-endian bit length is appended last for domain separation.

### Absorb

```
for each 16-byte block of padded input:
    (e₀, e₁) = bytes_to_field_elements(block)
    state[0] += e₀
    state[1] += e₁
    state = Poseidon(state)
```

### Squeeze

```
output = []
while len(output) < requested_bytes:
    output += bytes(state[0]) + bytes(state[1])
    state = Poseidon(state)
return output[:requested_bytes]
```

For outputs > 4096 bits, the final state is extended via cSHAKE-256:

```
seed = bytes(state[0]) + bytes(state[1])
output = cSHAKE256(seed, domain_tag, requested_bits / 8)
```

### Capacity and Security

The sponge has capacity c = 128 bits. For a sponge with capacity c:

| Property | Classical security | Quantum security |
|----------|-------------------|-----------------|
| Collision resistance | min(2ⁿ⁻ᶜ, 2ⁿᐟ²) = 2⁶⁴ | 2⁶⁴ (BHT: 2ⁿᐟ³ = 2⁸⁵ for n=256) |
| Preimage resistance | 2ⁿ = 2²⁵⁶ | 2ⁿᐟ² = 2¹²⁸ (Grover) |

For K5-512 (effective n = 512, capacity effectively doubled):

| Property | Classical security | Quantum security |
|----------|-------------------|-----------------|
| Collision (K5-512) | 2¹²⁸ | 2⁸⁵ |
| Preimage (K5-512) | 2²⁵⁶ | 2¹²⁸ |
| Collision (K2048) | 2⁶⁸³ | 2³⁴¹ |
| Preimage (K2048) | 2¹⁰²⁴ | 2⁵¹² |

K2048's collision resistance exceeds the thermodynamic limit of computation.

### Domain Separation

The domain tag `domain_tag` is bound into:
- Poseidon round constant generation
- cSHAKE customization string (for XOF extension)

This ensures different domains produce entirely unrelated outputs.

---

## 6. Memory-Hard Layer

The optional K5-H (hardened) variant prepends a Balloon hash pre-processing step.

### Parameters

| Parameter | Default | Range |
|-----------|---------|-------|
| mem_kib | 1024 | 0–2²⁰ |
| time_cost | 3 | 1–255 |

### Algorithm

```
1. state = BLAKE3(data)  // 32 bytes
2. buffer[i] = BLAKE3(state || i) for i = 0..N-1, N = mem_kib × 64
3. For t = 0..time_cost-1:
     For i = 0..N-1:
       prev = buffer[(i-1) mod N]
       dep_idx = int(buffer[i]) mod N
       buffer[i] = BLAKE3(buffer[i] || prev || buffer[dep_idx])
     For i = 0..N-1:
       buffer[i] = BLAKE3(buffer[i] || buffer[(i+t) mod N][:8])
4. result = BLAKE3(XOR(all buffer bytes) || state)
5. Feed result into K5 sponge
```

This forces linear memory access patterns and prevents time-memory tradeoffs.

### Security

- Memory cost N forces ~N bytes of RAM per evaluation
- Time cost T multiplies the computation cost
- No known shortcut computation exists for Balloon hash
- Default parameters (1 MiB, 3 passes) provide strong ASIC/GPU resistance

---

## 7. Quantum Security Analysis

### Grover's Algorithm (Preimage)

Grover's algorithm provides a quadratic speedup for preimage search. To maintain 256-bit quantum security, output size must be at least 512 bits:

| Output | Quantum preimage security |
|--------|--------------------------|
| 256 bits | 2¹²⁸ |
| 512 bits | 2²⁵⁶ |
| 1024 bits | 2⁵¹² |
| 2048 bits | 2¹⁰²⁴ |

### Brassard-Høyer-Tapp (Collision)

BHT provides a cubic speedup for collision finding. For 256-bit quantum collision security:

| Output | Quantum collision security |
|--------|---------------------------|
| 256 bits | 2⁸⁵ |
| 512 bits | 2¹⁷⁰ |
| 1024 bits | 2³⁴¹ |
| 2048 bits | 2⁶⁸³ |

### Algebraic Security

Poseidon's algebraic structure provides resistance against:
- **Gröbner basis attacks**: The combination of full and partial rounds prevents efficient algebraic linearization
- **Interpolation attacks**: MDS matrix multiplication between each S-box layer prevents function interpolation
- **Invariant subspace attacks**: Domain-tag-dependent round constants break any invariant structure

---

## 8. Parameter Selection Rationale

### Why Goldilocks (2⁶⁴ − 2³² + 1)?

- Native 64-bit arithmetic enables efficient software implementation
- Compatible with zero-knowledge proof systems (Plonky2)
- Large prime prevents NFS-based attacks
- p − 1 has a factor of 7, enabling the x⁷ S-box

### Why 8 + 28 Rounds?

- 8 full rounds provide resistance against statistical attacks (differential, linear, truncated differential)
- 28 partial rounds provide resistance against algebraic attacks
- Conservative margins: Plonky2 uses 8 + 22 rounds for the same field (t=12)
- Increasing full rounds beyond 8 yields diminishing security returns

### Why Cauchy MDS?

- Provably MDS over any field
- Simple construction from public parameters (nothing-up-my-sleeve)
- Efficient matrix-vector multiplication with small entries

---

## 9. Test Vectors

See `src/k5/spec.py` for frozen known-answer test vectors covering all modes:

| Mode | Inputs | Output size |
|------|--------|-------------|
| K5-512 | "", "abc", "hello" | 512 bits |
| K5-1024 | "", "abc", "hello" | 1024 bits |
| K2048 | "", "abc", "hello" | 2048 bits |
| KOF-256 | "", "hello" | 256 bits |
| KOF-4096 | "", "hello" | 4096 bits |

---

## 10. References

1. Bertoni et al., "Keccak sponge function family" (SHA-3)
2. Grassi et al., "Poseidon: A New Hash Function for Zero-Knowledge Proofs" (2021)
3. Hamburg, "Ed448-Goldilocks, a new elliptic curve" (2015)
4. NIST, "SHA-3 Derived Functions: cSHAKE, KMAC, TupleHash, and ParallelHash" (SP 800-185)
5. Boneh et al., "Balloon Hashing: A Memory-Hard Function Providing Provable Protection Against Sequential Attacks" (2016)
6. Grover, "A fast quantum mechanical algorithm for database search" (1996)
7. Brassard, Høyer, Tapp, "Quantum cryptanalysis of hash and claw-free functions" (1998)
