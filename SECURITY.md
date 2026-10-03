# Security Policy

## Supported Versions

| Version | Supported |
|---------|-----------|
| 0.2.x | ✅ |
| < 0.2 | ❌ |

## Reporting a Vulnerability

K5 is a research cryptographic project. If you discover a vulnerability or
security issue:

1. **Do not** open a public GitHub issue
2. Email: `security@kleinner-kantor.org`
3. Include a description of the issue, affected versions, and if possible,
   a proof of concept

You should receive a response within 48 hours.

## Scope

This security policy covers:

- The K5 hash function specification (see SPEC.md)
- The Rust reference implementation (k5-core)
- The Python reference implementation (k5)
- The IETF draft specification

## Out of Scope

- Side-channel attacks on non-constant-time implementations
- Performance issues (see BENCHMARKS.md)
- Third-party dependencies

## Acknowledgments

**Author:** Lois-Kleinner Alpasan
