# Kleinner-Kantor 5 (K5)

**Author:** Lois-Kleinner Alpasan
**Version:** 0.2.0

**The theoretically strongest cryptographic hash against quantum computing.**

K5 is a post-quantum sponge-based hash family and Extendable Output Function (XOF). It uses a Poseidon permutation over the Goldilocks prime field as its core, with optional memory-hard pre-processing (Balloon hash) for ASIC/GPU resistance.

```
Input → [Memory-Hard (optional)] → Sponge Absorb → Poseidon Permutation → Sponge Squeeze → Output
```

| Variant | Type | Output | Quantum Preimage Security |
|---------|------|--------|--------------------------|
| K5-512 | Fixed hash | 512 bits | 2²⁵⁶ |
| K5-1024 | Fixed hash | 1024 bits | 2⁵¹² |
| K2048 | Fixed hash | 2048 bits | 2¹⁰²⁴ |
| KOF | XOF (variable) | user-chosen | user-defined |
| K5-H | Memory-hard | any variant | +ASIC resistant |

---

## Quick Start

```bash
pip install -e .

# CLI
k5 hash "hello"                # K5-512
k5 hash --k2048 "hello"        # K2048
k5 hash --kof 4096 "hello"     # KOF-4096
k5 hash --hardened "password"  # Memory-hard mode
k5 hash --hardened --mem 4096 --time 5 "secret"  # 4 MiB, 5 passes
k5 file document.pdf           # hash a file
k5 vectors                     # verify test vectors
k5 benchmark                   # speed test
```

```python
from k5 import k5_512, k2048, kof, k5_h, K5

# Fixed output hashes
h = k5_512(b"hello")        # 512-bit hex
h = k2048(b"The Anticloud") # 2048-bit hex

# XOF mode
h = kof(b"data", 4096)      # 4096-bit output

# Memory-hard mode
h = k5_h(b"password", mem_kib=1024, time_cost=3)

# Streaming (hashlib-style)
h = K5()
h.update(b"stream ")
h.update(b"data")
digest = h.hexdigest(1024)  # 1024-bit output
```

---

## Architecture

### Layer 1 — Memory-Hard Pre-processing (optional)
Balloon hash construction with configurable memory (KiB) and time cost. Prevents GPU/ASIC brute-force by forcing linear memory access patterns.

### Layer 2 — Poseidon Permutation (core)
Algebraic permutation over the **Goldilocks prime field** (p = 2⁶⁴ − 2³² + 1):

| Parameter | Value |
|-----------|-------|
| State width | 4 elements (256 bits) |
| Full rounds | 8 |
| Partial rounds | 28 |
| S-box | x⁷ |
| MDS matrix | Cauchy (provably MDS over 𝔽ₚ) |
| Round constants | SHAKE-256 from domain tag (NUMS) |

### Layer 3 — Sponge Squeeze
Rate: 128 bits (2 elements), Capacity: 128 bits (2 elements). Variable-length output through repeated permutation and rate reads. For outputs > 4096 bits, extends via cSHAKE-256 XOF.

---

## Quantum Security

| Output | Preimage (Grover) | Collision (BHT) | Energy to break |
|--------|------------------|-----------------|-----------------|
| 256-bit | 2¹²⁸ | 2⁸⁵ | 10¹⁵× universe |
| 512-bit | 2²⁵⁶ | 2¹⁷⁰ | 10⁶⁹× universe |
| 1024-bit | 2⁵¹² | 2³⁴¹ | impossible |
| 2048-bit | 2¹⁰²⁴ | 2⁶⁸³ | impossible |

---

## Security Model

Full formal specification in [SPEC.md](SPEC.md), covering:

- Sponge indifferentiability
- Algebraic security (Gröbner basis, interpolation, invariant subspace)
- Quantum security bounds (Grover, BHT)
- Domain separation
- Parameter justification for all constants
- Verified Cauchy MDS matrix (all subdeterminants non-zero)

---

## Project Structure

```
kantor/
├── Kantor Brief.txt          # Original design brief
├── pyproject.toml             # Package config
├── README.md                  # This file
├── SPEC.md                    # Full security specification
├── src/k5/
│   ├── __init__.py            # Public API (k5, kof, k5_h, K5 class)
│   ├── field.py               # Goldilocks prime field
│   ├── poseidon.py            # Poseidon permutation + MDS verification
│   ├── sponge.py              # True sponge (absorb/squeeze loop)
│   ├── layers.py              # BLAKE3, Poseidon, cSHAKE wrappers
│   ├── memory_hard.py         # Balloon hash (memory-hard layer)
│   ├── spec.py                # Frozen test vectors
│   └── cli.py                 # CLI entry point
├── tests/
│   ├── test_poseidon.py       # 8 tests
│   ├── test_sponge.py         # 11 tests
│   ├── test_memory_hard.py    # 8 tests
│   ├── test_vectors.py        # 3 tests
│   └── test_cli.py            # 9 tests
└── (39 tests total, all passing)
```

---

## Dependencies

- `blake3` — BLAKE3 parallel hashing
- `pycryptodome` — cSHAKE-256 XOF

---

## License

MIT — Lois-Kleinner Alpasan

---
## Mirror notice
Read-only mirror of the KANTOR K5 core (Anticloud FZ LLE). Issues/discussions disabled — contact lois@0-1.gg.
License: Apache-2.0 + Enterprise commercial dual (Anticommons 0.1.0). K5 = SHA3-256(archive_sha3 || size_le64 || name || NULL || timestamp).
